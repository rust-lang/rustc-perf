use chrono::offset::TimeZone;
use chrono::{DateTime, Datelike, NaiveDate, Utc};
use hashbrown::HashMap;
use intern::intern;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::hash;
use std::ops::{Add, Sub};
use std::time::Duration;

pub mod pool;

pub use pool::{Connection, Pool};

intern!(pub struct ProcessStatistic);
intern!(pub struct Crate);

#[derive(Debug, Hash, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Date(pub DateTime<Utc>);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DateParseError {
    pub input: String,
    pub format: String,
    pub error: chrono::ParseError,
}

impl std::str::FromStr for Date {
    type Err = DateParseError;
    fn from_str(s: &str) -> Result<Date, DateParseError> {
        match DateTime::parse_from_rfc3339(s) {
            Ok(value) => Ok(Date(value.with_timezone(&Utc))),
            Err(error) => Err(DateParseError {
                input: s.to_string(),
                format: format!("RFC 3339"),
                error,
            }),
        }
    }
}

impl Date {
    pub fn from_format(date: &str, format: &str) -> Result<Date, DateParseError> {
        match DateTime::parse_from_str(date, format) {
            Ok(value) => Ok(Date(value.with_timezone(&Utc))),
            Err(_) => match Utc.datetime_from_str(date, format) {
                Ok(dt) => Ok(Date(dt)),
                Err(err) => Err(DateParseError {
                    input: date.to_string(),
                    format: format.to_string(),
                    error: err,
                }),
            },
        }
    }

    pub fn ymd_hms(year: i32, month: u32, day: u32, h: u32, m: u32, s: u32) -> Date {
        Date(Utc.ymd(year, month, day).and_hms(h, m, s))
    }

    pub fn start_of_week(&self) -> Date {
        let weekday = self.0.weekday();
        // num_days_from_sunday is 0 for Sunday
        Date(self.0 - chrono::Duration::days(weekday.num_days_from_sunday() as i64))
    }
}

impl fmt::Display for Date {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0.to_rfc3339())
    }
}

impl From<DateTime<Utc>> for Date {
    fn from(datetime: DateTime<Utc>) -> Date {
        Date(datetime)
    }
}

impl PartialEq<DateTime<Utc>> for Date {
    fn eq(&self, other: &DateTime<Utc>) -> bool {
        self.0 == *other
    }
}

impl Sub<chrono::Duration> for Date {
    type Output = Date;
    fn sub(self, rhs: chrono::Duration) -> Date {
        Date(self.0 - rhs)
    }
}

impl Add<chrono::Duration> for Date {
    type Output = Date;
    fn add(self, rhs: chrono::Duration) -> Date {
        Date(self.0 + rhs)
    }
}

impl Serialize for Date {
    fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        serializer.serialize_str(&self.0.to_rfc3339())
    }
}

impl<'de> Deserialize<'de> for Date {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Date, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct DateVisitor;

        impl<'de> serde::de::Visitor<'de> for DateVisitor {
            type Value = Date;

            fn visit_str<E>(self, value: &str) -> ::std::result::Result<Date, E>
            where
                E: serde::de::Error,
            {
                value.parse::<Date>().map_err(|_| {
                    serde::de::Error::invalid_value(serde::de::Unexpected::Str(value), &self)
                })
            }

            fn expecting(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                f.write_str("an RFC 3339 date")
            }
        }

        deserializer.deserialize_str(DateVisitor)
    }
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Sha {
    /// Straight-up bytes of the 40-long hex-encoded sha
    Hex([u8; 20]),
    /// Usually a string ID provided by the user.
    Raw(RawSha),
}

intern!(pub struct RawSha);

impl PartialEq<str> for Sha {
    fn eq(&self, other: &str) -> bool {
        self.to_string() == other
    }
}

fn hex_decode(s: &str) -> Option<[u8; 20]> {
    let mut in_progress = 0;
    let mut v = [0; 20];
    for (idx, ch) in s.chars().enumerate() {
        let offset = if idx % 2 == 0 { 4 } else { 0 };
        in_progress |= (ch.to_digit(16)? as u8) << offset;
        if idx % 2 != 0 {
            v[idx / 2] = in_progress;
            in_progress = 0;
        }
    }
    Some(v)
}

impl<'a> From<&'a str> for Sha {
    fn from(s: &'a str) -> Sha {
        if let Some(v) = hex_decode(s) {
            return Sha::Hex(v);
        }

        Sha::Raw(s.into())
    }
}

impl Serialize for Sha {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        serializer.collect_str(&self)
    }
}

impl<'de> Deserialize<'de> for Sha {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        use serde::de::Visitor;
        struct ShaVisitor;
        impl<'de> Visitor<'de> for ShaVisitor {
            type Value = Sha;

            fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
                f.write_str("a string")
            }

            fn visit_str<E>(self, s: &str) -> Result<Sha, E> {
                Ok(s.into())
            }

            fn visit_borrowed_str<E>(self, s: &'de str) -> Result<Sha, E> {
                Ok(s.into())
            }
        }
        deserializer.deserialize_str(ShaVisitor)
    }
}

impl fmt::Debug for Sha {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self)
    }
}

impl fmt::Display for Sha {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Sha::Hex(hex) => {
                for &b in hex.iter() {
                    write!(f, "{:x}{:x}", b >> 4, b & 0xf)?;
                }
            }
            Sha::Raw(raw) => {
                write!(f, "{}", raw)?;
            }
        }
        Ok(())
    }
}

#[derive(Debug, Copy, Clone, serde::Deserialize, serde::Serialize)]
pub struct Commit {
    pub sha: Sha,
    pub date: Date,
}

impl Commit {
    pub fn is_try(&self) -> bool {
        self.date.0.naive_utc().date() == NaiveDate::from_ymd(2000, 1, 1)
    }
}

impl hash::Hash for Commit {
    fn hash<H: hash::Hasher>(&self, hasher: &mut H) {
        self.sha.hash(hasher);
    }
}

impl PartialEq for Commit {
    fn eq(&self, other: &Self) -> bool {
        self.sha == other.sha
    }
}

impl Eq for Commit {}

impl PartialOrd for Commit {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(&other))
    }
}

impl Ord for Commit {
    fn cmp(&self, other: &Self) -> Ordering {
        self.date
            .cmp(&other.date)
            .then_with(|| self.sha.cmp(&other.sha))
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

intern!(pub struct PatchName);

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

impl fmt::Display for CollectionId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CollectionId::Commit(c) => write!(f, "{} ({})", c.sha, c.date),
            CollectionId::Artifact(id) => write!(f, "{}", id),
        }
    }
}

impl From<Commit> for CollectionId {
    fn from(c: Commit) -> Self {
        Self::Commit(c)
    }
}

#[async_trait::async_trait]
pub trait SeriesType: Sized {
    async fn get(conn: &dyn pool::Connection, series: u32, cid: CollectionIdNumber)
        -> Option<Self>;
    async fn insert(self, conn: &dyn pool::Connection, label: LabelId, cid: CollectionIdNumber);
}

#[async_trait::async_trait]
impl SeriesType for f64 {
    async fn insert(self, conn: &dyn pool::Connection, label: LabelId, cid: CollectionIdNumber) {
        match label.0 {
            1 => conn.insert_pstat(label.1, cid, self).await,
            _ => unreachable!("{}", label.0),
        }
    }

    async fn get(
        conn: &dyn pool::Connection,
        series: u32,
        cid: CollectionIdNumber,
    ) -> Option<Self> {
        conn.get_pstats(series, &[Some(cid)]).await[0]
    }
}

intern!(pub struct QueryLabel);

#[derive(Clone, Debug)]
pub struct QueryDatum {
    pub self_time: Duration,
    pub blocked_time: Duration,
    pub incremental_load_time: Duration,
    pub number_of_cache_hits: u32,
    pub invocation_count: u32,
}

#[async_trait::async_trait]
impl SeriesType for QueryDatum {
    async fn insert(self, conn: &dyn pool::Connection, label: LabelId, cid: CollectionIdNumber) {
        match label.0 {
            2 => conn.insert_self_profile_query(label.1, cid, self).await,
            _ => unreachable!("{}", label.0),
        }
    }
    async fn get(
        conn: &dyn pool::Connection,
        series: u32,
        cid: CollectionIdNumber,
    ) -> Option<Self> {
        conn.get_self_profile_query(series, cid).await
    }
}
#[async_trait::async_trait]
impl SeriesType for String {
    async fn insert(self, conn: &dyn pool::Connection, label: LabelId, cid: CollectionIdNumber) {
        match label.0 {
            0 => conn.insert_error(label.1, cid, self).await,
            _ => unreachable!("{}", label.0),
        }
    }

    async fn get(
        conn: &dyn pool::Connection,
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

pub trait Lookup {
    type Id;
    fn lookup(&self, index: &Index) -> Option<Self::Id>;
}

impl Lookup for DbLabel {
    type Id = u32;
    fn lookup(&self, index: &Index) -> Option<Self::Id> {
        match self {
            DbLabel::Errors { krate } => index.errors.get(krate),
            DbLabel::ProcessStat {
                krate,
                profile,
                cache,
                stat,
            } => index.pstats.get(&(*krate, *profile, *cache, *stat)),
            DbLabel::SelfProfileQuery {
                krate,
                profile,
                cache,
                query,
            } => index.queries.get(&(*krate, *profile, *cache, *query)),
        }
    }
}

impl Lookup for CollectionId {
    type Id = CollectionIdNumber;
    fn lookup(&self, index: &Index) -> Option<Self::Id> {
        Some(match self {
            CollectionId::Commit(c) => CollectionIdNumber(0, index.commits.get(c)?),
            CollectionId::Artifact(a) => CollectionIdNumber(1, index.artifacts.get(a.as_str())?),
        })
    }
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

    pub fn lookup(&self, path: &DbLabel, cid: &CollectionId) -> Option<(u32, CollectionIdNumber)> {
        let cid = cid.lookup(self)?;
        let series = path.lookup(self)?;
        Some((series, cid))
    }

    pub async fn get<T: SeriesType>(
        &self,
        db: &mut dyn pool::Connection,
        path: &DbLabel,
        cid: &CollectionId,
    ) -> Option<T> {
        let (series, cid) = self.lookup(path, cid)?;
        T::get(db, series, cid).await
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
        label: DbLabel,
        conn: &mut dyn pool::Connection,
        cid: CollectionIdNumber,
        point: T,
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
