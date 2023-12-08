use chrono::offset::TimeZone;
use chrono::{DateTime, Datelike, Utc};
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
    pub commit_date: Option<Date>,
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
                format: "RFC 3339".to_string(),
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
        Date(Utc.with_ymd_and_hms(year, month, day, h, m, s).unwrap())
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
pub enum CommitType {
    Try,
    Master,
}

impl FromStr for CommitType {
    type Err = String;

    fn from_str(ty: &str) -> Result<Self, Self::Err> {
        match ty {
            "try" => Ok(CommitType::Try),
            "master" => Ok(CommitType::Master),
            _ => Err(format!("Wrong commit type {}", ty)),
        }
    }
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct Commit {
    pub sha: String,
    pub date: Date,
    pub r#type: CommitType,
}

impl Commit {
    pub fn is_try(&self) -> bool {
        matches!(self.r#type, CommitType::Try)
    }
    pub fn is_master(&self) -> bool {
        matches!(self.r#type, CommitType::Master)
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
        Some(self.cmp(other))
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
    /// A Clippy run
    Clippy,
}

impl Profile {
    pub fn as_str(self) -> &'static str {
        match self {
            Profile::Check => "check",
            Profile::Opt => "opt",
            Profile::Debug => "debug",
            Profile::Doc => "doc",
            Profile::Clippy => "clippy",
        }
    }
}

impl std::str::FromStr for Profile {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s.to_ascii_lowercase().as_str() {
            "check" => Profile::Check,
            "debug" => Profile::Debug,
            "doc" => Profile::Doc,
            "opt" => Profile::Opt,
            "clippy" => Profile::Clippy,
            _ => return Err(format!("{} is not a profile", s)),
        })
    }
}

impl fmt::Display for Profile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
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
                if let Some(stripped) = s.strip_prefix("incr-patched: ") {
                    Scenario::IncrementalPatch(PatchName::from(stripped))
                } else {
                    return Err(format!("{} is not a scenario", s));
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
            Scenario::Empty => "full".to_string(),
            Scenario::IncrementalEmpty => "incr-full".to_string(),
            Scenario::IncrementalFresh => "incr-unchanged".to_string(),
            Scenario::IncrementalPatch(name) => format!("incr-patched-{}", name),
        }
    }
}

use std::cmp::Ordering;
use std::str::FromStr;

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

/// The codegen backend used for compilation.
#[derive(
    Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, serde::Serialize, serde::Deserialize,
)]
pub enum CodegenBackend {
    /// The default LLVM backend
    Llvm,
    /// Cranelift codegen backend
    Cranelift,
}

impl CodegenBackend {
    pub fn as_str(self) -> &'static str {
        match self {
            CodegenBackend::Llvm => "llvm",
            CodegenBackend::Cranelift => "cranelift",
        }
    }
}

impl FromStr for CodegenBackend {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s.to_ascii_lowercase().as_str() {
            "llvm" => CodegenBackend::Llvm,
            "cranelift" => CodegenBackend::Cranelift,
            _ => return Err(format!("{} is not a codegen backend", s)),
        })
    }
}

impl fmt::Display for CodegenBackend {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
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

struct ArtifactInfo<'a> {
    name: &'a str,
    date: Option<DateTime<Utc>>,
    kind: &'static str,
}

impl ArtifactId {
    fn info(&self) -> ArtifactInfo {
        let (name, date, ty) = match self {
            Self::Commit(commit) => (
                commit.sha.as_str(),
                Some(commit.date.0),
                if commit.is_try() { "try" } else { "master" },
            ),
            Self::Tag(a) => (a.as_str(), None, "release"),
        };
        ArtifactInfo {
            name,
            date,
            kind: ty,
        }
    }
}

intern!(pub struct QueryLabel);

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
    /// Id lookup of published release artifacts
    artifacts: Indexed<Box<str>>,
    /// Id lookup of compile stat description ids
    /// For legacy reasons called `pstat_series` in the database, and so the name is kept here.
    pstat_series: Indexed<(Benchmark, Profile, Scenario, CodegenBackend, Metric)>,
    /// Id lookup of runtime stat description ids
    runtime_pstat_series: Indexed<(Benchmark, Metric)>,
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
    StatisticDescription {
        benchmark: Benchmark,
        profile: Profile,
        scenario: Scenario,
        backend: CodegenBackend,
        metric: Metric,
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
            DbLabel::StatisticDescription {
                benchmark,
                profile,
                scenario,
                backend,
                metric,
            } => index
                .pstat_series
                .get(&(*benchmark, *profile, *scenario, *backend, *metric)),
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

pub type StatisticalDescriptionId = u32;

impl Index {
    pub async fn load(conn: &mut dyn pool::Connection) -> Index {
        conn.load_index().await
    }

    pub fn lookup(
        &self,
        label: &DbLabel,
        artifact_id: &ArtifactId,
    ) -> Option<(u32, ArtifactIdNumber)> {
        let artifact_row_id = artifact_id.lookup(self)?;
        let stat_description_row_id = label.lookup(self)?;
        Some((stat_description_row_id, artifact_row_id))
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
    pub fn compile_metrics(&self) -> Vec<String> {
        self.pstat_series
            .map
            .keys()
            .map(|(_, _, _, _, metric)| metric)
            .collect::<std::collections::HashSet<_>>()
            .into_iter()
            .map(|s| s.to_string())
            .collect()
    }

    pub fn runtime_metrics(&self) -> Vec<String> {
        self.runtime_pstat_series
            .map
            .keys()
            .map(|(_, metric)| metric)
            .collect::<std::collections::HashSet<_>>()
            .into_iter()
            .map(|s| s.to_string())
            .collect()
    }

    // FIXME: in theory this won't scale indefinitely as there's potentially
    // millions of queries and labels and iterating all of them is eventually
    // going to be impractical. But for now it performs quite well, so we'll go
    // for it as keeping indices around would be annoying.
    pub fn compile_statistic_descriptions(
        &self,
    ) -> impl Iterator<
        Item = (
            &(Benchmark, Profile, Scenario, CodegenBackend, Metric),
            StatisticalDescriptionId,
        ),
    > + '_ {
        self.pstat_series
            .map
            .iter()
            .map(|(test_case, &row_id)| (test_case, row_id))
    }

    pub fn runtime_statistic_descriptions(
        &self,
    ) -> impl Iterator<Item = (&(Benchmark, Metric), StatisticalDescriptionId)> + '_ {
        self.runtime_pstat_series
            .map
            .iter()
            .map(|(test_case, &row_id)| (test_case, row_id))
    }

    pub fn artifact_id_for_commit(&self, commit: &str) -> Option<ArtifactId> {
        self.commits()
            .into_iter()
            .find(|c| c.sha == *commit)
            .map(ArtifactId::Commit)
            .or_else(|| {
                self.artifacts()
                    .find(|a| *a == commit)
                    .map(|a| ArtifactId::Tag(a.to_owned()))
            })
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

#[derive(Debug, Clone, Serialize)]
pub struct CompileBenchmark {
    pub name: String,
    pub category: String,
}

#[derive(Debug)]
pub struct ArtifactCollection {
    pub artifact: ArtifactId,
    pub duration: Duration,
    pub end_time: DateTime<Utc>,
}
