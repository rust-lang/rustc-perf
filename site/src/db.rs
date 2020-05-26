use collector::self_profile::{QueryData, QueryLabel};
pub use collector::BenchmarkName as Crate;
use collector::{Commit, PatchName, ProcessStatistic};
use hashbrown::HashMap;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::time::Duration;

pub mod pool;

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

#[async_trait::async_trait]
pub trait SeriesType: Sized {
    async fn get(
        conn: &mut dyn pool::Connection,
        series: u32,
        cid: CollectionIdNumber,
    ) -> Option<Self>;
    async fn insert(
        &self,
        conn: &mut dyn pool::Connection,
        label: LabelId,
        cid: CollectionIdNumber,
    );
}

#[async_trait::async_trait]
impl SeriesType for f64 {
    async fn insert(
        &self,
        conn: &mut dyn pool::Connection,
        label: LabelId,
        cid: CollectionIdNumber,
    ) {
        match label.0 {
            1 => conn.insert_pstat(label.1, cid, *self).await,
            _ => unreachable!("{}", label.0),
        }
    }

    async fn get(
        conn: &mut dyn pool::Connection,
        series: u32,
        cid: CollectionIdNumber,
    ) -> Option<Self> {
        conn.get_pstat(series, cid).await
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

#[async_trait::async_trait]
impl SeriesType for QueryDatum {
    async fn insert(
        &self,
        conn: &mut dyn pool::Connection,
        label: LabelId,
        cid: CollectionIdNumber,
    ) {
        match label.0 {
            2 => conn.insert_self_profile_query(label.1, cid, self).await,
            _ => unreachable!("{}", label.0),
        }
    }
    async fn get(
        conn: &mut dyn pool::Connection,
        series: u32,
        cid: CollectionIdNumber,
    ) -> Option<Self> {
        conn.get_self_profile_query(series, cid).await
    }
}
#[async_trait::async_trait]
impl SeriesType for String {
    async fn insert(
        &self,
        conn: &mut dyn pool::Connection,
        label: LabelId,
        cid: CollectionIdNumber,
    ) {
        match label.0 {
            0 => conn.insert_error(label.1, cid, self).await,
            _ => unreachable!("{}", label.0),
        }
    }

    async fn get(
        conn: &mut dyn pool::Connection,
        series: u32,
        cid: CollectionIdNumber,
    ) -> Option<Self> {
        conn.get_error(series, cid).await
    }
}

#[derive(Hash, Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct LabelId(pub u8, pub u32);

#[derive(Serialize, Deserialize, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct CollectionIdNumber(u8, u32);

impl CollectionIdNumber {
    pub fn pack(self) -> u32 {
        let mut bytes = u32::to_be_bytes(self.1);
        assert_eq!(bytes[0], 0);
        bytes[0] = self.0;
        u32::from_be_bytes(bytes)
    }
}

#[derive(Clone, Deserialize, Serialize, Default)]
pub struct Index {
    commits: Indexed<Commit>,
    artifacts: Indexed<Box<str>>,

    errors: Indexed<Crate>,
    pstats: Indexed<(Crate, Profile, Cache, ProcessStatistic)>,
    queries: Indexed<(Crate, Profile, Cache, QueryLabel)>,
}

#[derive(Clone, Serialize, Deserialize)]
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
    pub async fn load(conn: &mut dyn pool::Connection) -> Index {
        let bytes = conn.load_index().await;
        bytes
            .map(|s| serde_json::from_slice(&s).unwrap())
            .unwrap_or_default()
    }

    pub async fn store(&self, conn: &mut dyn pool::Connection) {
        conn.store_index(&serde_json::to_vec(self).unwrap()).await;
    }

    pub async fn get<T: SeriesType>(
        &self,
        db: &mut dyn pool::Connection,
        path: &DbLabel,
        cid: &CollectionId,
    ) -> Option<T> {
        let cid = match cid {
            CollectionId::Commit(c) => CollectionIdNumber(0, self.commits.get(c)?),
            CollectionId::Artifact(a) => CollectionIdNumber(1, self.artifacts.get(a.as_str())?),
        };

        match path {
            DbLabel::Errors { krate } => {
                let series = self.errors.get(krate)?;
                T::get(db, series, cid).await
            }
            DbLabel::ProcessStat {
                krate,
                profile,
                cache,
                stat,
            } => {
                let series = self.pstats.get(&(*krate, *profile, *cache, *stat))?;
                T::get(db, series, cid).await
            }
            DbLabel::SelfProfileQuery {
                krate,
                profile,
                cache,
                query,
            } => {
                let series = self.queries.get(&(*krate, *profile, *cache, *query))?;
                T::get(db, series, cid).await
            }
        }
    }

    pub fn artifacts(&self) -> impl Iterator<Item = &'_ str> + '_ {
        self.artifacts.map.keys().map(|s| &**s)
    }

    pub fn commits(&self) -> Vec<Commit> {
        let mut commits = self.commits.map.keys().copied().collect::<Vec<_>>();
        commits.sort();
        commits
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

    pub fn all_errors(&self) -> impl Iterator<Item = Crate> + '_ {
        self.errors.map.keys().copied()
    }

    // FIXME: in theory this won't scale indefinitely as there's potentially
    // millions of queries and labels and iterating all of them is eventually
    // going to be impractical. But for now it performs quite well, so we'll go
    // for it as keeping indices around would be annoying.
    pub fn all_pstat_series(
        &self,
    ) -> impl Iterator<Item = &'_ (Crate, Profile, Cache, ProcessStatistic)> + '_ {
        self.pstats.map.keys()
    }

    // FIXME: in theory this won't scale indefinitely as there's potentially
    // millions of queries and labels and iterating all of them is eventually
    // going to be impractical. But for now it performs quite well, so we'll go
    // for it as keeping indices around would be annoying.
    pub fn all_query_series(
        &self,
    ) -> impl Iterator<Item = &'_ (Crate, Profile, Cache, QueryLabel)> + '_ {
        self.queries.map.keys()
    }

    // FIXME: in theory this won't scale indefinitely as there's potentially
    // millions of queries and labels and iterating all of them is eventually
    // going to be impractical. But for now it performs quite well, so we'll go
    // for it as keeping indices around would be annoying.
    pub fn filtered_queries(
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

    pub async fn insert_labeled<T: SeriesType>(
        &mut self,
        label: &DbLabel,
        conn: &mut dyn pool::Connection,
        cid: CollectionIdNumber,
        point: &T,
    ) {
        let label_id = self.intern_db_label(&label);
        point.insert(conn, label_id, cid).await;
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
