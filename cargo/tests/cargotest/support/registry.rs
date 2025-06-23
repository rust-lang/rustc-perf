use std::collections::HashMap;
use std::fs::{self, File};
use std::io::prelude::*;
use std::path::{PathBuf, Path};

use flate2::Compression::Default;
use flate2::write::GzEncoder;
use git2;
use hex::ToHex;
use tar::{Builder, Header};
use url::Url;

use support::paths;
use support::git::repo;
use cargo::util::Sha256;

pub fn registry_path() -> PathBuf { paths::root().join("registry") }
pub fn registry() -> Url { Url::from_file_path(&*registry_path()).ok().unwrap() }
pub fn dl_path() -> PathBuf { paths::root().join("dl") }
pub fn dl_url() -> Url { Url::from_file_path(&*dl_path()).ok().unwrap() }

pub struct Package {
    name: String,
    vers: String,
    deps: Vec<Dependency>,
    files: Vec<(String, String)>,
    extra_files: Vec<(String, String)>,
    yanked: bool,
    features: HashMap<String, Vec<String>>,
    local: bool,
}

struct Dependency {
    name: String,
    vers: String,
    kind: String,
    target: Option<String>,
    features: Vec<String>,
}

pub fn init() {
    let config = paths::home().join(".cargo/config");
    t!(fs::create_dir_all(config.parent().unwrap()));
    if fs::metadata(&config).is_ok() {
        return
    }
    t!(t!(File::create(&config)).write_all(format!(r#"
        [registry]
            token = "api-token"

        [source.crates-io]
        registry = 'https://wut'
        replace-with = 'dummy-registry'

        [source.dummy-registry]
        registry = '{reg}'
    "#, reg = registry()).as_bytes()));

    // Init a new registry
    let _ = repo(&registry_path())
        .file("config.json", &format!(r#"
            {{"dl":"{0}","api":"{0}"}}
        "#, dl_url()))
        .build();
    fs::create_dir_all(dl_path().join("api/v1/crates")).unwrap();
}

impl Package {
    pub fn new(name: &str, vers: &str) -> Package {
        init();
        Package {
            name: name.to_string(),
            vers: vers.to_string(),
            deps: Vec::new(),
            files: Vec::new(),
            extra_files: Vec::new(),
            yanked: false,
            features: HashMap::new(),
            local: false,
        }
    }

    pub fn local(&mut self, local: bool) -> &mut Package {
        self.local = local;
        self
    }

    pub fn file(&mut self, name: &str, contents: &str) -> &mut Package {
        self.files.push((name.to_string(), contents.to_string()));
        self
    }

    pub fn extra_file(&mut self, name: &str, contents: &str) -> &mut Package {
        self.extra_files.push((name.to_string(), contents.to_string()));
        self
    }

    pub fn dep(&mut self, name: &str, vers: &str) -> &mut Package {
        self.full_dep(name, vers, None, "normal", &[])
    }

    pub fn feature_dep(&mut self,
                       name: &str,
                       vers: &str,
                       features: &[&str]) -> &mut Package {
        self.full_dep(name, vers, None, "normal", features)
    }

    pub fn target_dep(&mut self,
                      name: &str,
                      vers: &str,
                      target: &str) -> &mut Package {
        self.full_dep(name, vers, Some(target), "normal", &[])
    }

    pub fn dev_dep(&mut self, name: &str, vers: &str) -> &mut Package {
        self.full_dep(name, vers, None, "dev", &[])
    }

    fn full_dep(&mut self,
                name: &str,
                vers: &str,
                target: Option<&str>,
                kind: &str,
                features: &[&str]) -> &mut Package {
        self.deps.push(Dependency {
            name: name.to_string(),
            vers: vers.to_string(),
            kind: kind.to_string(),
            target: target.map(|s| s.to_string()),
            features: features.iter().map(|s| s.to_string()).collect(),
        });
        self
    }

    pub fn yanked(&mut self, yanked: bool) -> &mut Package {
        self.yanked = yanked;
        self
    }

    pub fn publish(&self) -> String {
        self.make_archive();

        // Figure out what we're going to write into the index
        let deps = self.deps.iter().map(|dep| {
            json!({
                "name": dep.name,
                "req": dep.vers,
                "features": dep.features,
                "default_features": true,
                "target": dep.target,
                "optional": false,
                "kind": dep.kind,
            })
        }).collect::<Vec<_>>();
        let cksum = {
            let mut c = Vec::new();
            t!(t!(File::open(&self.archive_dst())).read_to_end(&mut c));
            cksum(&c)
        };
        let line = json!({
            "name": self.name,
            "vers": self.vers,
            "deps": deps,
            "cksum": cksum,
            "features": self.features,
            "yanked": self.yanked,
        }).to_string();

        let file = match self.name.len() {
            1 => format!("1/{}", self.name),
            2 => format!("2/{}", self.name),
            3 => format!("3/{}/{}", &self.name[..1], self.name),
            _ => format!("{}/{}/{}", &self.name[0..2], &self.name[2..4], self.name),
        };

        // Write file/line in the index
        let dst = if self.local {
            registry_path().join("index").join(&file)
        } else {
            registry_path().join(&file)
        };
        let mut prev = String::new();
        let _ = File::open(&dst).and_then(|mut f| f.read_to_string(&mut prev));
        t!(fs::create_dir_all(dst.parent().unwrap()));
        t!(t!(File::create(&dst))
                  .write_all((prev + &line[..] + "\n").as_bytes()));

        // Add the new file to the index
        if !self.local {
            let repo = t!(git2::Repository::open(&registry_path()));
            let mut index = t!(repo.index());
            t!(index.add_path(Path::new(&file)));
            t!(index.write());
            let id = t!(index.write_tree());

            // Commit this change
            let tree = t!(repo.find_tree(id));
            let sig = t!(repo.signature());
            let parent = t!(repo.refname_to_id("refs/heads/master"));
            let parent = t!(repo.find_commit(parent));
            t!(repo.commit(Some("HEAD"), &sig, &sig,
                           "Another commit", &tree,
                           &[&parent]));
        }

        return cksum
    }

    fn make_archive(&self) {
        let mut manifest = format!(r#"
            [package]
            name = "{}"
            version = "{}"
            authors = []
        "#, self.name, self.vers);
        for dep in self.deps.iter() {
            let target = match dep.target {
                None => String::new(),
                Some(ref s) => format!("target.'{}'.", s),
            };
            let kind = match &dep.kind[..] {
                "build" => "build-",
                "dev" => "dev-",
                _ => ""
            };
            manifest.push_str(&format!(r#"
                [{}{}dependencies.{}]
                version = "{}"
            "#, target, kind, dep.name, dep.vers));
        }

        let dst = self.archive_dst();
        t!(fs::create_dir_all(dst.parent().unwrap()));
        let f = t!(File::create(&dst));
        let mut a = Builder::new(GzEncoder::new(f, Default));
        self.append(&mut a, "Cargo.toml", &manifest);
        if self.files.is_empty() {
            self.append(&mut a, "src/lib.rs", "");
        } else {
            for &(ref name, ref contents) in self.files.iter() {
                self.append(&mut a, name, contents);
            }
        }
        for &(ref name, ref contents) in self.extra_files.iter() {
            self.append_extra(&mut a, name, contents);
        }
    }

    fn append<W: Write>(&self, ar: &mut Builder<W>, file: &str, contents: &str) {
        self.append_extra(ar,
                          &format!("{}-{}/{}", self.name, self.vers, file),
                          contents);
    }

    fn append_extra<W: Write>(&self, ar: &mut Builder<W>, path: &str, contents: &str) {
        let mut header = Header::new_ustar();
        header.set_size(contents.len() as u64);
        t!(header.set_path(path));
        header.set_cksum();
        t!(ar.append(&header, contents.as_bytes()));
    }

    pub fn archive_dst(&self) -> PathBuf {
        if self.local {
            registry_path().join(format!("{}-{}.crate", self.name,
                                         self.vers))
        } else {
            dl_path().join(&self.name).join(&self.vers).join("download")
        }
    }
}

pub fn cksum(s: &[u8]) -> String {
    let mut sha = Sha256::new();
    sha.update(s);
    sha.finish().to_hex()
}
