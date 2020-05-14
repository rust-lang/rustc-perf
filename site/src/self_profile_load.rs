use anyhow::Context as _;
use collector::self_profile::SelfProfile;
use serde::de::{self, DeserializeSeed, Deserializer, MapAccess, Visitor};
use std::fmt;
use std::fs;
use std::io::Read;
use std::path::Path;

pub fn deserialize(
    path: &Path,
    krate: crate::db::Crate,
    profile: crate::db::Profile,
    cache: crate::db::Cache,
) -> Option<SelfProfile> {
    let mut file = fs::File::open(path)
        .with_context(|| format!("Failed to open {}", path.display()))
        .unwrap();
    let mut file_contents = Vec::new();
    if path.extension().map_or(false, |e| e == "sz") {
        let mut szip_reader = snap::read::FrameDecoder::new(std::io::BufReader::new(file));
        szip_reader
            .read_to_end(&mut file_contents)
            .with_context(|| format!("Failed to read {}", path.display()))
            .unwrap();
    } else {
        file.read_to_end(&mut file_contents)
            .with_context(|| format!("Failed to read {}", path.display()))
            .unwrap();
    };
    let file_contents = std::str::from_utf8(&file_contents).unwrap();

    let mut json = serde_json::de::Deserializer::from_str(&file_contents);

    let r = json
        .deserialize_struct(
            "CommitData",
            &["benchmarks"],
            CommitDataVisitor {
                krate,
                profile,
                cache,
            },
        )
        .unwrap_or_else(|err| {
            panic!("Failed to parse JSON for {}: {:?}", path.display(), err);
        });
    r
}

#[derive(Copy, Clone)]
struct CommitDataVisitor {
    krate: crate::db::Crate,
    profile: crate::db::Profile,
    cache: crate::db::Cache,
}

impl<'de> serde::de::DeserializeSeed<'de> for CommitDataVisitor {
    type Value = Option<SelfProfile>;
    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // We are deserializing the benchmarks map
        deserializer.deserialize_map(BenchmarksVisitor {
            krate: self.krate,
            profile: self.profile,
            cache: self.cache,
        })
    }
}

struct BenchmarksVisitor {
    krate: crate::db::Crate,
    profile: crate::db::Profile,
    cache: crate::db::Cache,
}

enum Field {
    Expected,
    Other,
}
struct Expected<'a>(&'a str);
impl<'de> DeserializeSeed<'de> for Expected<'_> {
    type Value = Field;
    fn deserialize<D>(self, deserializer: D) -> Result<Field, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct FieldVisitor<'a>(&'a str);

        impl<'de> Visitor<'de> for FieldVisitor<'_> {
            type Value = Field;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                write!(formatter, "{}", self.0)
            }

            fn visit_str<E>(self, value: &str) -> Result<Field, E>
            where
                E: de::Error,
            {
                if self.0 == value {
                    Ok(Field::Expected)
                } else {
                    Ok(Field::Other)
                }
            }
        }

        deserializer.deserialize_identifier(FieldVisitor(self.0))
    }
}

impl<'de> Visitor<'de> for BenchmarksVisitor {
    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: MapAccess<'de>,
    {
        let mut res = None;
        while let Some(key) = map.next_key_seed(Expected(&*self.krate))? {
            match key {
                Field::Other => {
                    let _: serde::de::IgnoredAny = map.next_value()?;
                }
                Field::Expected => {
                    #[derive(serde::Deserialize)]
                    pub struct Benchmark<'a> {
                        #[serde(borrow)]
                        pub runs: Vec<Run<'a>>,
                    }

                    #[derive(serde::Deserialize)]
                    pub struct Run<'a> {
                        #[serde(default, borrow)]
                        pub self_profile: Option<&'a serde_json::value::RawValue>,
                        #[serde(default)]
                        pub check: bool,
                        pub release: bool,
                        pub state: collector::BenchmarkState,
                    }

                    let benchmark: Result<Benchmark, serde::de::IgnoredAny> = map.next_value()?;
                    res = Some(benchmark.ok().and_then(|b| {
                        b.runs
                            .iter()
                            .find(|r| {
                                let matches_profile = match self.profile {
                                    crate::db::Profile::Check => r.check,
                                    crate::db::Profile::Opt => r.release,
                                    crate::db::Profile::Debug => !r.check && !r.release,
                                };
                                let matches_cache = self.cache
                                    == match &r.state {
                                        collector::BenchmarkState::Clean => crate::db::Cache::Empty,
                                        collector::BenchmarkState::IncrementalStart => {
                                            crate::db::Cache::IncrementalEmpty
                                        }
                                        collector::BenchmarkState::IncrementalClean => {
                                            crate::db::Cache::IncrementalFresh
                                        }
                                        collector::BenchmarkState::IncrementalPatched(p) => {
                                            crate::db::Cache::IncrementalPatch(p.name)
                                        }
                                    };
                                matches_profile && matches_cache
                            })
                            .and_then(|r| r.self_profile)
                            .and_then(|r| serde_json::from_str(r.get()).ok())
                    }));
                }
            }
        }

        if let Some(res) = res {
            return Ok(res);
        } else {
            Err(serde::de::Error::missing_field("benchmarks"))
        }
    }

    type Value = Option<SelfProfile>;
    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(formatter, "{}/{}/{}", self.krate, self.profile, self.cache)
    }
}

impl<'de> Visitor<'de> for CommitDataVisitor {
    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: MapAccess<'de>,
    {
        #[derive(serde::Deserialize)]
        #[serde(field_identifier, rename_all = "lowercase")]
        enum Field {
            Benchmarks,
            Commit,
        }
        let mut res = None;
        while let Some(key) = map.next_key()? {
            match key {
                Field::Commit => {
                    let _: serde::de::IgnoredAny = map.next_value()?;
                }
                Field::Benchmarks => {
                    res = Some(map.next_value_seed(self)?);
                }
            }
        }
        if let Some(res) = res {
            return Ok(res);
        } else {
            Err(serde::de::Error::missing_field("benchmarks"))
        }
    }

    type Value = Option<SelfProfile>;
    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(formatter, "{}/{}/{}", self.krate, self.profile, self.cache)
    }
}
