use collector::self_profile::{QueryData, QueryLabel};
pub use collector::BenchmarkName as Crate;
use collector::{Bound, Commit, PatchName, ProcessStatistic, StatId};
use hashbrown::HashMap;
use rocksdb::{ColumnFamilyDescriptor, Options, WriteBatch, DB};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::convert::TryInto;
use std::fmt;
use std::ops::RangeInclusive;
use std::time::Duration;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RunId {
    pub profile: Profile,
    pub state: Cache,
}

#[derive(Debug, Clone, serde::Deserialize)]
#[serde(from = "CollectorRun")]
pub struct Run {
    pub stats: collector::Stats,
    pub profile: Profile,
    pub state: Cache,
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct CollectorRun {
    pub stats: collector::Stats,
    #[serde(default)]
    pub check: bool,
    pub release: bool,
    pub state: collector::BenchmarkState,
}

impl Run {
    pub fn get_stat(&self, stat: StatId) -> Option<f64> {
        self.stats.get(stat)
    }

    pub fn id(&self) -> RunId {
        RunId {
            profile: self.profile,
            state: self.state,
        }
    }
}

impl From<CollectorRun> for Run {
    fn from(c: CollectorRun) -> Run {
        Run {
            stats: c.stats,
            profile: if c.check {
                Profile::Check
            } else if c.release {
                Profile::Opt
            } else {
                Profile::Debug
            },
            state: (&c.state).into(),
        }
    }
}

impl<'a> From<&'a collector::BenchmarkState> for Cache {
    fn from(other: &'a collector::BenchmarkState) -> Self {
        match other {
            collector::BenchmarkState::Clean => Cache::Empty,
            collector::BenchmarkState::IncrementalStart => Cache::IncrementalEmpty,
            collector::BenchmarkState::IncrementalClean => Cache::IncrementalFresh,
            collector::BenchmarkState::IncrementalPatched(p) => Cache::IncrementalPatch(p.name),
        }
    }
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct Benchmark {
    pub runs: Vec<Run>,
    pub name: Crate,
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct CommitData {
    pub commit: Commit,
    // String in Result is the output of the command that failed
    pub benchmarks: BTreeMap<Crate, Result<Benchmark, String>>,
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct ArtifactData {
    pub id: String,
    // String in Result is the output of the command that failed
    pub benchmarks: BTreeMap<Crate, Result<Benchmark, String>>,
}

pub fn data_for(data: &[Commit], is_left: bool, query: Bound) -> Option<Commit> {
    if is_left {
        data.iter().find(|commit| query.left_match(commit)).cloned()
    } else {
        data.iter()
            .rfind(|commit| query.left_match(commit))
            .cloned()
    }
}

pub fn range_subset(data: &[Commit], range: RangeInclusive<Bound>) -> &[Commit] {
    let (a, b) = range.into_inner();

    let left_idx = data.iter().position(|commit| a.left_match(commit));
    let right_idx = data.iter().rposition(|commit| b.left_match(commit));

    if let (Some(left), Some(right)) = (left_idx, right_idx) {
        data.get(left..=right).unwrap_or_else(|| {
            log::error!(
                "Failed to compute left/right indices from {:?}..={:?}",
                a,
                b
            );
            &[]
        })
    } else {
        &[]
    }
}

pub struct ByProfile<T> {
    pub check: T,
    pub debug: T,
    pub opt: T,
}

impl<T> ByProfile<T> {
    pub fn new<E, F>(mut f: F) -> Result<Self, E>
    where
        F: FnMut(Profile) -> Result<T, E>,
    {
        Ok(ByProfile {
            check: f(Profile::Check)?,
            debug: f(Profile::Debug)?,
            opt: f(Profile::Opt)?,
        })
    }
}

impl<T> std::ops::Index<Profile> for ByProfile<T> {
    type Output = T;
    fn index(&self, index: Profile) -> &Self::Output {
        match index {
            Profile::Check => &self.check,
            Profile::Debug => &self.debug,
            Profile::Opt => &self.opt,
        }
    }
}

#[derive(
    Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, serde::Serialize, serde::Deserialize,
)]
pub enum Profile {
    Check,
    Debug,
    Opt,
}

impl std::str::FromStr for Profile {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s.to_ascii_lowercase().as_str() {
            "check" => Profile::Check,
            "debug" => Profile::Debug,
            "opt" => Profile::Opt,
            _ => return Err(format!("{} is not a profile", s)),
        })
    }
}

impl fmt::Display for Profile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Profile::Check => "check",
                Profile::Opt => "opt",
                Profile::Debug => "debug",
            }
        )
    }
}

impl Profile {
    pub fn matches_run(self, run: &RunId) -> bool {
        run.profile == self
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[serde(tag = "variant", content = "name")]
pub enum Cache {
    #[serde(rename = "full")]
    Empty,
    #[serde(rename = "incr-full")]
    IncrementalEmpty,
    #[serde(rename = "incr-unchanged")]
    IncrementalFresh,
    #[serde(rename = "incr-patched")]
    IncrementalPatch(PatchName),
}

impl std::str::FromStr for Cache {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s.to_ascii_lowercase().as_str() {
            "full" => Cache::Empty,
            "incr-full" => Cache::IncrementalEmpty,
            "incr-unchanged" => Cache::IncrementalFresh,
            _ => {
                // FIXME: use str::strip_prefix when stabilized
                if s.starts_with("incr-patched: ") {
                    Cache::IncrementalPatch(PatchName::from(&s["incr-patched: ".len()..]))
                } else {
                    return Err(format!("{} is not a profile", s));
                }
            }
        })
    }
}

impl fmt::Display for Cache {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Cache::Empty => write!(f, "full"),
            Cache::IncrementalEmpty => write!(f, "incr-full"),
            Cache::IncrementalFresh => write!(f, "incr-unchanged"),
            Cache::IncrementalPatch(name) => write!(f, "incr-patched: {}", name),
        }
    }
}

impl Cache {
    pub fn matches_run(self, r: &RunId) -> bool {
        r.state == self
    }
}

use std::cmp::Ordering;

// We sort println before all other patches.
impl Ord for Cache {
    fn cmp(&self, other: &Cache) -> Ordering {
        match (self, other) {
            (a, b) if a == b => Ordering::Equal,
            (Cache::Empty, _) => Ordering::Less,
            (Cache::IncrementalEmpty, Cache::Empty) => Ordering::Greater,
            (Cache::IncrementalEmpty, _) => Ordering::Less,
            (Cache::IncrementalFresh, Cache::Empty) => Ordering::Greater,
            (Cache::IncrementalFresh, Cache::IncrementalEmpty) => Ordering::Greater,
            (Cache::IncrementalFresh, _) => Ordering::Less,
            (Cache::IncrementalPatch(_), Cache::Empty) => Ordering::Greater,
            (Cache::IncrementalPatch(_), Cache::IncrementalEmpty) => Ordering::Greater,
            (Cache::IncrementalPatch(_), Cache::IncrementalFresh) => Ordering::Greater,
            (Cache::IncrementalPatch(a), Cache::IncrementalPatch(b)) => {
                if a == "println" {
                    Ordering::Less
                } else if b == "println" {
                    Ordering::Greater
                } else {
                    a.cmp(b)
                }
            }
        }
    }
}

impl PartialOrd for Cache {
    fn partial_cmp(&self, other: &Cache) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub trait Point {
    type Key: fmt::Debug + PartialEq + Clone;

    fn key(&self) -> &Self::Key;
    fn set_key(&mut self, key: Self::Key);
    fn value(&self) -> Option<f64>;
    fn set_value(&mut self, value: f64);
    fn interpolated(&self) -> bool;
    fn set_interpolated(&mut self);
}

impl<T: Clone + PartialEq + fmt::Debug> Point for (T, Option<f64>) {
    type Key = T;

    fn key(&self) -> &T {
        &self.0
    }
    fn set_key(&mut self, key: T) {
        self.0 = key;
    }
    fn value(&self) -> Option<f64> {
        self.1
    }
    fn set_value(&mut self, value: f64) {
        self.1 = Some(value);
    }
    fn interpolated(&self) -> bool {
        false
    }
    fn set_interpolated(&mut self) {
        // no-op
    }
}

impl<T: Clone + PartialEq + fmt::Debug> Point for (T, f64) {
    type Key = T;

    fn key(&self) -> &T {
        &self.0
    }
    fn set_key(&mut self, key: T) {
        self.0 = key;
    }
    fn value(&self) -> Option<f64> {
        Some(self.1)
    }
    fn set_value(&mut self, value: f64) {
        self.1 = value;
    }
    fn interpolated(&self) -> bool {
        false
    }
    fn set_interpolated(&mut self) {
        // no-op
    }
}

pub use crate::average::average;

#[derive(Deserialize, Serialize, Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct LabelPath {
    pub krate: Option<Crate>,
    pub profile: Option<Profile>,
    pub cache: Option<Cache>,
    pub process_stat: Option<ProcessStatistic>,
    pub query: Option<QueryLabel>,
}

impl LabelPath {
    pub fn new() -> Self {
        assert_eq!(std::mem::size_of::<LabelPath>(), 48);
        Self {
            krate: None,
            profile: None,
            cache: None,
            process_stat: None,
            query: None,
        }
    }

    pub fn set(&mut self, component: Label) {
        match component {
            Label::Crate(c) => self.krate = Some(c),
            Label::Profile(p) => self.profile = Some(p),
            Label::Cache(c) => self.cache = Some(c),
            Label::ProcessStat(p) => self.process_stat = Some(p),
            Label::Query(q) => self.query = Some(q),
        }
    }

    pub fn remove(&mut self, component: LabelTag) {
        match component {
            LabelTag::Crate => self.krate = None,
            LabelTag::Profile => self.profile = None,
            LabelTag::Cache => self.cache = None,
            LabelTag::ProcessStat => self.process_stat = None,
            LabelTag::Query => self.query = None,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum LabelTag {
    Crate,
    Profile,
    Cache,
    ProcessStat,
    Query,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Label {
    Crate(Crate),
    Profile(Profile),
    Cache(Cache),
    ProcessStat(ProcessStatistic),
    Query(QueryLabel),
}

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum CollectionId {
    Commit(Commit),
    Artifact(String),
}

impl From<Commit> for CollectionId {
    fn from(c: Commit) -> Self {
        Self::Commit(c)
    }
}

pub trait SeriesType {
    fn with_bytes(&self, f: impl FnOnce(&[u8]));
    fn from_bytes(bytes: &[u8]) -> Self;
}

impl SeriesType for f64 {
    fn with_bytes(&self, f: impl FnOnce(&[u8])) {
        f(&self.to_bits().to_le_bytes())
    }

    fn from_bytes(bytes: &[u8]) -> Self {
        f64::from_bits(u64::from_le_bytes(bytes.try_into().unwrap()))
    }
}

#[derive(Clone, Debug)]
pub struct QueryDatum {
    pub self_time: Duration,
    pub blocked_time: Duration,
    pub incremental_load_time: Duration,
    pub number_of_cache_hits: u32,
    pub invocation_count: u32,
}

impl QueryDatum {
    pub fn from_query_data(qd: &QueryData) -> Self {
        Self {
            self_time: qd.self_time(),
            blocked_time: qd.blocked_time(),
            incremental_load_time: qd.incremental_load_time(),
            number_of_cache_hits: qd.number_of_cache_hits,
            invocation_count: qd.invocation_count,
        }
    }
}

impl SeriesType for QueryDatum {
    fn with_bytes(&self, f: impl FnOnce(&[u8])) {
        let mut bytes = [0; 32];
        bytes[..8].copy_from_slice(&u64::to_le_bytes(
            self.self_time.as_nanos().try_into().unwrap(),
        ));
        bytes[8..16].copy_from_slice(&u64::to_le_bytes(
            self.blocked_time.as_nanos().try_into().unwrap(),
        ));
        bytes[16..24].copy_from_slice(&u64::to_le_bytes(
            self.incremental_load_time.as_nanos().try_into().unwrap(),
        ));
        bytes[24..28].copy_from_slice(&u32::to_le_bytes(self.number_of_cache_hits));
        bytes[28..32].copy_from_slice(&u32::to_le_bytes(self.invocation_count));
        f(&bytes);
    }

    fn from_bytes(bytes: &[u8]) -> Self {
        QueryDatum {
            self_time: Duration::from_nanos(u64::from_le_bytes(bytes[..8].try_into().unwrap())),
            blocked_time: Duration::from_nanos(u64::from_le_bytes(
                bytes[8..16].try_into().unwrap(),
            )),
            incremental_load_time: Duration::from_nanos(u64::from_le_bytes(
                bytes[16..24].try_into().unwrap(),
            )),
            number_of_cache_hits: u32::from_le_bytes(bytes[24..28].try_into().unwrap()),
            invocation_count: u32::from_le_bytes(bytes[28..32].try_into().unwrap()),
        }
    }
}
impl SeriesType for String {
    fn with_bytes(&self, f: impl FnOnce(&[u8])) {
        f(self.as_bytes())
    }

    fn from_bytes(bytes: &[u8]) -> Self {
        std::str::from_utf8(bytes).unwrap().to_string()
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct LabelId(u8, u32);

impl LabelId {
    fn as_bytes(self) -> [u8; 4] {
        let mut bytes = u32::to_be_bytes(self.1);
        assert_eq!(bytes[0], 0);
        bytes[0] = self.0;
        bytes
    }

    fn from_bytes(mut bytes: [u8; 4]) -> Self {
        let ty = bytes[0];
        bytes[0] = 0;
        Self(ty, u32::from_be_bytes(bytes))
    }
}

#[derive(Serialize, Deserialize, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct CollectionIdNumber(u8, u32);

impl CollectionIdNumber {
    fn as_bytes(self) -> [u8; 4] {
        let mut bytes = u32::to_be_bytes(self.1);
        assert_eq!(bytes[0], 0);
        bytes[0] = self.0;
        bytes
    }

    fn from_bytes(mut bytes: [u8; 4]) -> Self {
        let ty = bytes[0];
        bytes[0] = 0;
        Self(ty, u32::from_be_bytes(bytes))
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct DatabaseKey(pub LabelId, pub CollectionIdNumber);

impl DatabaseKey {
    fn as_bytes(&self) -> [u8; 8] {
        let mut buf = [0; 8];
        buf[..4].copy_from_slice(&self.0.as_bytes());
        buf[4..].copy_from_slice(&self.1.as_bytes());
        buf
    }

    fn from_bytes(key: &[u8]) -> Self {
        Self(
            LabelId::from_bytes(key[..4].try_into().unwrap()),
            CollectionIdNumber::from_bytes(key[4..].try_into().unwrap()),
        )
    }
}

pub fn open(at: &str, ingest: bool) -> DB {
    let mut db_opts = Options::default();
    db_opts.create_if_missing(true);
    db_opts.create_missing_column_families(true);
    db_opts.set_max_open_files(625);
    db_opts.set_keep_log_file_num(3);

    let mut cf_opts = Options::default();
    cf_opts.optimize_level_style_compaction(256 * 1024 * 1024);
    cf_opts.set_compression_type(rocksdb::DBCompressionType::Lz4);
    if ingest {
        db_opts.set_allow_concurrent_memtable_write(false);
        cf_opts.set_allow_concurrent_memtable_write(false);
        cf_opts.set_memtable_factory(rocksdb::MemtableFactory::Vector);
    }
    cf_opts.set_comparator("default", |a, b| {
        DatabaseKey::from_bytes(a).cmp(&DatabaseKey::from_bytes(b))
    });
    let descriptors = vec![
        ColumnFamilyDescriptor::new("default", cf_opts),
        ColumnFamilyDescriptor::new("indices", Options::default()),
    ];

    DB::open_cf_descriptors(&db_opts, &at, descriptors).unwrap()
}

#[derive(Deserialize, Serialize, Default)]
pub struct Index {
    commits: Indexed<Commit>,
    artifacts: Indexed<Box<str>>,

    errors: Indexed<Crate>,
    pstats: Indexed<(Crate, Profile, Cache, ProcessStatistic)>,
    queries: Indexed<(Crate, Profile, Cache, QueryLabel)>,
}

#[derive(Serialize, Deserialize)]
struct Indexed<T> {
    #[serde(with = "index_serde")]
    #[serde(bound = "T: Serialize + serde::de::DeserializeOwned + std::hash::Hash + Eq")]
    map: HashMap<T, u32>,
    next: u32,
}

impl<T> Default for Indexed<T> {
    fn default() -> Self {
        Indexed {
            map: Default::default(),
            // start at 0
            next: 0,
        }
    }
}

impl<T> Indexed<T>
where
    T: Clone,
    T: Eq + std::hash::Hash,
{
    pub fn intern<U>(&mut self, value: U) -> u32
    where
        T: std::borrow::Borrow<U>,
        U: std::hash::Hash + Eq,
        U: Into<T>,
    {
        let next = &mut self.next;
        *self
            .map
            .raw_entry_mut()
            .from_key(&value)
            .or_insert_with(|| {
                let idx = *next;
                // FIXME: search for a deleted entry on wrap?
                *next = idx.checked_add(1).unwrap();
                (value.into(), idx)
            })
            .1
    }

    pub fn get<U>(&self, value: &U) -> Option<u32>
    where
        T: std::borrow::Borrow<U>,
        U: std::hash::Hash + Eq + ?Sized,
    {
        self.map.get(value).copied()
    }
}

mod index_serde {
    use hashbrown::HashMap;
    use serde::de::{DeserializeOwned, MapAccess, Visitor};
    use serde::ser::SerializeMap;
    use serde::{Deserializer, Serialize, Serializer};
    use std::marker::PhantomData;

    pub fn serialize<T, S>(map: &HashMap<T, u32>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
        T: Serialize,
    {
        let mut writer = serializer.serialize_map(Some(map.len()))?;
        for (value, idx) in map.iter() {
            writer.serialize_entry(idx, value)?;
        }
        writer.end()
    }

    struct MapVisitor<T>(PhantomData<T>);

    impl<'de, T> Visitor<'de> for MapVisitor<T>
    where
        T: std::hash::Hash + Eq + DeserializeOwned,
    {
        type Value = HashMap<T, u32>;
        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(formatter, "map u32 -> T")
        }

        fn visit_map<A>(self, mut access: A) -> Result<Self::Value, A::Error>
        where
            A: MapAccess<'de>,
        {
            let mut map = HashMap::with_capacity(access.size_hint().unwrap_or(64));
            while let Some((idx, value)) = access.next_entry()? {
                if map.insert(value, idx).is_some() {
                    return Err(serde::de::Error::invalid_value(
                        serde::de::Unexpected::Other("duplicate"),
                        &"no duplicates",
                    ));
                }
            }
            Ok(map)
        }
    }

    pub fn deserialize<'de, D, T>(deserializer: D) -> Result<HashMap<T, u32>, D::Error>
    where
        D: Deserializer<'de>,
        T: DeserializeOwned + Eq + std::hash::Hash,
    {
        deserializer.deserialize_map(MapVisitor(PhantomData))
    }
}

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum DbLabel {
    Errors {
        krate: Crate,
    },
    ProcessStat {
        krate: Crate,
        profile: Profile,
        cache: Cache,
        stat: ProcessStatistic,
    },
    SelfProfileQuery {
        krate: Crate,
        profile: Profile,
        cache: Cache,
        query: QueryLabel,
    },
}

impl Index {
    pub fn load(db: &DB) -> Index {
        let cf = db.cf_handle("indices").unwrap();
        let indices = db.get_pinned_cf(cf, "index").unwrap();
        if let Some(pinned) = indices {
            serde_json::from_slice(&pinned).unwrap()
        } else {
            Index::default()
        }
    }

    pub fn store(&self, db: &DB) {
        let cf = db.cf_handle("indices").unwrap();
        let serialized = serde_json::to_vec(self).unwrap();
        db.put_cf(cf, "index", serialized).unwrap();
    }

    pub fn get<T: SeriesType>(&self, db: &DB, path: &DbLabel, cid: &CollectionId) -> Option<T> {
        let label_id = match path {
            DbLabel::Errors { krate } => LabelId(0, self.errors.get(krate)?),
            DbLabel::ProcessStat {
                krate,
                profile,
                cache,
                stat,
            } => LabelId(1, self.pstats.get(&(*krate, *profile, *cache, *stat))?),
            DbLabel::SelfProfileQuery {
                krate,
                profile,
                cache,
                query,
            } => LabelId(2, self.queries.get(&(*krate, *profile, *cache, *query))?),
        };
        let cid = match cid {
            CollectionId::Commit(c) => CollectionIdNumber(0, self.commits.get(c)?),
            CollectionId::Artifact(a) => CollectionIdNumber(1, self.artifacts.get(a.as_str())?),
        };
        let key = DatabaseKey(label_id, cid);

        db.get_pinned(key.as_bytes())
            .unwrap()
            .map(|slice| T::from_bytes(&slice))
    }

    // FIXME: in theory this won't scale indefinitely as there's potentially
    // millions of queries and labels and iterating all of them is eventually
    // going to be impractical. But for now it performs quite well, so we'll go
    // for it as keeping indices around would be annoying.
    pub fn stats(&self) -> Vec<String> {
        self.pstats
            .map
            .keys()
            .map(|path| path.3)
            .collect::<std::collections::HashSet<_>>()
            .into_iter()
            .map(|s| s.to_string())
            .collect()
    }

    // FIXME: in theory this won't scale indefinitely as there's potentially
    // millions of queries and labels and iterating all of them is eventually
    // going to be impractical. But for now it performs quite well, so we'll go
    // for it as keeping indices around would be annoying.
    pub fn all_queries(
        &self,
        krate: Crate,
        profile: Profile,
        cache: Cache,
    ) -> impl Iterator<Item = QueryLabel> + '_ {
        self.queries
            .map
            .keys()
            .filter(move |path| path.0 == krate && path.1 == profile && path.2 == cache)
            .map(|path| path.3)
    }

    pub fn insert_labeled<T: SeriesType>(
        &mut self,
        label: &DbLabel,
        batch: &mut WriteBatch,
        cid: CollectionIdNumber,
        point: &T,
    ) {
        let label_id = self.intern_db_label(&label);
        let key = DatabaseKey(label_id, cid);
        point.with_bytes(|value| {
            batch.put(key.as_bytes(), value);
        });
    }

    pub fn intern_db_label(&mut self, label: &DbLabel) -> LabelId {
        match *label {
            DbLabel::Errors { krate } => {
                let id = self.errors.intern(krate);
                LabelId(0, id)
            }
            DbLabel::ProcessStat {
                krate,
                profile,
                cache,
                stat,
            } => {
                let id = self.pstats.intern((krate, profile, cache, stat));
                LabelId(1, id)
            }
            DbLabel::SelfProfileQuery {
                krate,
                profile,
                cache,
                query,
            } => {
                let id = self.queries.intern((krate, profile, cache, query));
                LabelId(2, id)
            }
        }
    }

    pub fn intern_cid(&mut self, cid: &CollectionId) -> CollectionIdNumber {
        match cid {
            CollectionId::Commit(c) => CollectionIdNumber(0, self.commits.intern(*c)),
            CollectionId::Artifact(aid) => {
                CollectionIdNumber(1, self.artifacts.intern(aid.clone().into_boxed_str()))
            }
        }
    }
}
