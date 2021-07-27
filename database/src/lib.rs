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

intern!(pub struct Metric);
intern!(pub struct Benchmark);

#[derive(Debug, PartialEq, Eq)]
pub struct QueuedCommit {
    pub pr: u32,
    pub sha: String,
    pub parent_sha: String,
    pub include: Option<String>,
    pub exclude: Option<String>,
    pub runs: Option<i32>,
}

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

    pub fn empty() -> Date {
        Date::ymd_hms(2000, 1, 1, 1, 1, 1)
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

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct Commit {
    pub sha: String,
    pub date: Date,
}

impl Commit {
    pub fn is_try(&self) -> bool {
        self.date.0.naive_utc().date() == NaiveDate::from_ymd(2000, 1, 1)
            || self.date.0.naive_utc().date() == NaiveDate::from_ymd(2001, 1, 1)
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

/// The compilation profile (i.e., how the crate was built)
#[derive(
    Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, serde::Serialize, serde::Deserialize,
)]
pub enum Profile {
    /// A checked build (i.e., no codegen)
    Check,
    /// A debug build (i.e., low optimizations)
    Debug,
    /// A doc build
    Doc,
    /// An optimized "release" build
    Opt,
}

impl std::str::FromStr for Profile {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s.to_ascii_lowercase().as_str() {
            "check" => Profile::Check,
            "debug" => Profile::Debug,
            "doc" => Profile::Doc,
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
                Profile::Doc => "doc",
            }
        )
    }
}

/// The scenario under test - composed of incremental cache state
/// and sometimes a code change.
///
/// These are usually reported to users in a "flipped" way. For example,
/// `Cache::Empty` means we're doing a "full" build. We present this to users as "full".
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[serde(tag = "variant", content = "name")]
pub enum Scenario {
    /// Empty cache (i.e., full build)
    #[serde(rename = "full")]
    Empty,
    /// Empty cache but still incremental (i.e., a full incremental build)
    #[serde(rename = "incr-full")]
    IncrementalEmpty,
    /// Cache is fully up-to-date (i.e., no code has changed)
    #[serde(rename = "incr-unchanged")]
    IncrementalFresh,
    /// Cache is mostly up-to-date but some code has been changed
    #[serde(rename = "incr-patched")]
    IncrementalPatch(PatchName),
}

intern!(pub struct PatchName);

impl std::str::FromStr for Scenario {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s.to_ascii_lowercase().as_str() {
            "full" => Scenario::Empty,
            "incr-full" => Scenario::IncrementalEmpty,
            "incr-unchanged" => Scenario::IncrementalFresh,
            _ => {
                // FIXME: use str::strip_prefix when stabilized
                if s.starts_with("incr-patched: ") {
                    Scenario::IncrementalPatch(PatchName::from(&s["incr-patched: ".len()..]))
                } else {
                    return Err(format!("{} is not a profile", s));
                }
            }
        })
    }
}

impl fmt::Display for Scenario {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Scenario::Empty => write!(f, "full"),
            Scenario::IncrementalEmpty => write!(f, "incr-full"),
            Scenario::IncrementalFresh => write!(f, "incr-unchanged"),
            Scenario::IncrementalPatch(name) => write!(f, "incr-patched: {}", name),
        }
    }
}

impl Scenario {
    pub fn to_id(&self) -> String {
        match self {
            Scenario::Empty => format!("full"),
            Scenario::IncrementalEmpty => format!("incr-full"),
            Scenario::IncrementalFresh => format!("incr-unchanged"),
            Scenario::IncrementalPatch(name) => format!("incr-patched-{}", name),
        }
    }
}

use std::cmp::Ordering;

// We sort println before all other patches.
impl Ord for Scenario {
    fn cmp(&self, other: &Scenario) -> Ordering {
        match (self, other) {
            (a, b) if a == b => Ordering::Equal,
            (Scenario::Empty, _) => Ordering::Less,
            (Scenario::IncrementalEmpty, Scenario::Empty) => Ordering::Greater,
            (Scenario::IncrementalEmpty, _) => Ordering::Less,
            (Scenario::IncrementalFresh, Scenario::Empty) => Ordering::Greater,
            (Scenario::IncrementalFresh, Scenario::IncrementalEmpty) => Ordering::Greater,
            (Scenario::IncrementalFresh, _) => Ordering::Less,
            (Scenario::IncrementalPatch(_), Scenario::Empty) => Ordering::Greater,
            (Scenario::IncrementalPatch(_), Scenario::IncrementalEmpty) => Ordering::Greater,
            (Scenario::IncrementalPatch(_), Scenario::IncrementalFresh) => Ordering::Greater,
            (Scenario::IncrementalPatch(a), Scenario::IncrementalPatch(b)) => {
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

impl PartialOrd for Scenario {
    fn partial_cmp(&self, other: &Scenario) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Deserialize, Serialize, Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct LabelPath {
    pub benchmark: Option<Benchmark>,
    pub profile: Option<Profile>,
    pub scenario: Option<Scenario>,
    pub metric: Option<Metric>,
    pub query: Option<QueryLabel>,
}

impl LabelPath {
    pub fn new() -> Self {
        assert_eq!(std::mem::size_of::<LabelPath>(), 48);
        Self {
            benchmark: None,
            profile: None,
            scenario: None,
            metric: None,
            query: None,
        }
    }

    pub fn set(&mut self, component: Label) {
        match component {
            Label::Benchmark(b) => self.benchmark = Some(b),
            Label::Profile(p) => self.profile = Some(p),
            Label::Scenario(s) => self.scenario = Some(s),
            Label::Metric(m) => self.metric = Some(m),
            Label::Query(q) => self.query = Some(q),
        }
    }

    pub fn remove(&mut self, component: LabelTag) {
        match component {
            LabelTag::Benchmark => self.benchmark = None,
            LabelTag::Profile => self.profile = None,
            LabelTag::Scenario => self.scenario = None,
            LabelTag::Metric => self.metric = None,
            LabelTag::Query => self.query = None,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum LabelTag {
    Benchmark,
    Profile,
    Scenario,
    Metric,
    Query,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Label {
    Benchmark(Benchmark),
    Profile(Profile),
    Scenario(Scenario),
    Metric(Metric),
    Query(QueryLabel),
}

/// An identifier for a built version of the compiler
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ArtifactId {
    /// A built version of the compiler at an exact commit
    Commit(Commit),
    /// A symbolic tag for a built compiler like "1.51.0"
    Tag(String),
}

impl fmt::Display for ArtifactId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ArtifactId::Commit(c) => write!(f, "{} ({})", c.sha, c.date),
            ArtifactId::Tag(id) => write!(f, "{}", id),
        }
    }
}

impl From<Commit> for ArtifactId {
    fn from(c: Commit) -> Self {
        Self::Commit(c)
    }
}

#[async_trait::async_trait]
pub trait SeriesType: Sized {
    async fn get(
        conn: &dyn pool::Connection,
        series: u32,
        artifact_row_id: ArtifactIdNumber,
    ) -> Option<Self>;
}

#[async_trait::async_trait]
impl SeriesType for f64 {
    async fn get(
        conn: &dyn pool::Connection,
        series: u32,
        artifact_row_id: ArtifactIdNumber,
    ) -> Option<Self> {
        conn.get_pstats(&[series], &[Some(artifact_row_id)]).await[0][0]
    }
}

intern!(pub struct QueryLabel);

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct QueryDatum {
    pub self_time: Duration,
    pub blocked_time: Duration,
    pub incremental_load_time: Duration,
    pub number_of_cache_hits: u32,
    pub invocation_count: u32,
}

#[async_trait::async_trait]
impl SeriesType for QueryDatum {
    async fn get(
        conn: &dyn pool::Connection,
        series: u32,
        artifact_row_id: ArtifactIdNumber,
    ) -> Option<Self> {
        conn.get_self_profile_query(series, artifact_row_id).await
    }
}
#[derive(Hash, Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct LabelId(pub u8, pub u32);

/// A database row ID for an artifact in the artifact table
#[derive(Serialize, Deserialize, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct ArtifactIdNumber(pub u32);

/// Cached Id lookups for many database tables.
///
/// This is a quick way to find what the database id for something.
/// Essentially duplicates of the various database tables (artifacts,
/// error_series, pstat_series, etc.) so that we can avoid a network round-trip.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Index {
    /// Id look for a commit
    commits: Indexed<Commit>,
    /// Id lookup of the errors for a crate
    artifacts: Indexed<Box<str>>,
    /// Id lookup of the errors for a crate
    errors: Indexed<Benchmark>,
    /// Id lookup of a given process stastic profile
    pstats: Indexed<(Benchmark, Profile, Scenario, Metric)>,
    /// Id lookup of a given process query label
    queries: Indexed<(Benchmark, Profile, Scenario, QueryLabel)>,
}

/// An index lookup
///
/// Given a `T` find what its database id is
#[derive(Debug, Clone, Serialize, Deserialize)]
struct Indexed<T> {
    #[serde(with = "index_serde")]
    #[serde(bound = "T: Serialize + serde::de::DeserializeOwned + std::hash::Hash + Eq")]
    map: HashMap<T, u32>,
}

impl<T> std::iter::FromIterator<(u32, T)> for Indexed<T>
where
    T: std::hash::Hash + Eq,
{
    fn from_iter<I: IntoIterator<Item = (u32, T)>>(iter: I) -> Self {
        Self {
            map: iter.into_iter().map(|(idx, v)| (v, idx)).collect(),
        }
    }
}

impl<T> PartialEq for Indexed<T>
where
    T: PartialEq + Eq + std::hash::Hash,
{
    fn eq(&self, other: &Self) -> bool {
        self.map == other.map
    }
}
impl<T> Eq for Indexed<T> where T: PartialEq + Eq + std::hash::Hash {}

impl<T> Default for Indexed<T> {
    fn default() -> Self {
        Indexed {
            map: Default::default(),
        }
    }
}

impl<T> Indexed<T>
where
    T: Clone,
    T: Eq + std::hash::Hash,
{
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
        benchmark: Benchmark,
    },
    ProcessStat {
        benchmark: Benchmark,
        profile: Profile,
        scenario: Scenario,
        metric: Metric,
    },
    SelfProfileQuery {
        benchmark: Benchmark,
        profile: Profile,
        scenario: Scenario,
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
            DbLabel::Errors { benchmark } => index.errors.get(benchmark),
            DbLabel::ProcessStat {
                benchmark,
                profile,
                scenario,
                metric,
            } => index
                .pstats
                .get(&(*benchmark, *profile, *scenario, *metric)),
            DbLabel::SelfProfileQuery {
                benchmark,
                profile,
                scenario,
                query,
            } => index
                .queries
                .get(&(*benchmark, *profile, *scenario, *query)),
        }
    }
}

impl Lookup for ArtifactId {
    type Id = ArtifactIdNumber;
    fn lookup(&self, index: &Index) -> Option<Self::Id> {
        Some(match self {
            ArtifactId::Commit(c) => ArtifactIdNumber(index.commits.get(c)?),
            ArtifactId::Tag(a) => ArtifactIdNumber(index.artifacts.get(a.as_str())?),
        })
    }
}

impl Index {
    pub async fn load(conn: &mut dyn pool::Connection) -> Index {
        conn.load_index().await
    }

    pub fn lookup(
        &self,
        path: &DbLabel,
        artifact_id: &ArtifactId,
    ) -> Option<(u32, ArtifactIdNumber)> {
        let artifact_row_id = artifact_id.lookup(self)?;
        let series = path.lookup(self)?;
        Some((series, artifact_row_id))
    }

    pub async fn get<T: SeriesType>(
        &self,
        db: &mut dyn pool::Connection,
        path: &DbLabel,
        artifact_id: &ArtifactId,
    ) -> Option<T> {
        let (series, artifact_row_id) = self.lookup(path, artifact_id)?;
        T::get(db, series, artifact_row_id).await
    }

    pub fn artifacts(&self) -> impl Iterator<Item = &'_ str> + '_ {
        self.artifacts.map.keys().map(|s| &**s)
    }

    pub fn commits(&self) -> Vec<Commit> {
        let mut commits = self.commits.map.keys().cloned().collect::<Vec<_>>();
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

    pub fn all_errors(&self) -> impl Iterator<Item = Benchmark> + '_ {
        self.errors.map.keys().copied()
    }

    // FIXME: in theory this won't scale indefinitely as there's potentially
    // millions of queries and labels and iterating all of them is eventually
    // going to be impractical. But for now it performs quite well, so we'll go
    // for it as keeping indices around would be annoying.
    pub fn all_pstat_series(
        &self,
    ) -> impl Iterator<Item = &'_ (Benchmark, Profile, Scenario, Metric)> + '_ {
        self.pstats.map.keys()
    }

    // FIXME: in theory this won't scale indefinitely as there's potentially
    // millions of queries and labels and iterating all of them is eventually
    // going to be impractical. But for now it performs quite well, so we'll go
    // for it as keeping indices around would be annoying.
    pub fn all_query_series(
        &self,
    ) -> impl Iterator<Item = &'_ (Benchmark, Profile, Scenario, QueryLabel)> + '_ {
        self.queries.map.keys()
    }

    // FIXME: in theory this won't scale indefinitely as there's potentially
    // millions of queries and labels and iterating all of them is eventually
    // going to be impractical. But for now it performs quite well, so we'll go
    // for it as keeping indices around would be annoying.
    pub fn filtered_queries(
        &self,
        benchmark: Benchmark,
        profile: Profile,
        scenario: Scenario,
    ) -> impl Iterator<Item = QueryLabel> + '_ {
        self.queries
            .map
            .keys()
            .filter(move |path| path.0 == benchmark && path.1 == profile && path.2 == scenario)
            .map(|path| path.3)
            .filter(|q| !q.as_str().starts_with("codegen passes ["))
    }
}

#[derive(Debug)]
pub struct Step {
    pub name: String,
    pub is_done: bool,
    // The amount of time this step has been ongoing (or took, if completed).
    pub duration: Duration,
    pub expected: Duration,
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub struct CollectionId(i32);

impl fmt::Display for CollectionId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
