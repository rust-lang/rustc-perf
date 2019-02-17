extern crate gll;
extern crate walkdir;

use std::env;
use std::fs;
use std::path::PathBuf;
use walkdir::WalkDir;

fn main() {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());

    // FIXME(eddyb) streamline this process in `gll`.

    // Find all the `.lyg` grammar fragments in `grammar/`.
    let fragments = WalkDir::new("grammar")
        .contents_first(true)
        .into_iter()
        .map(|entry| entry.unwrap())
        .filter(|entry| entry.path().extension().map_or(false, |ext| ext == "lyg"));

    // Start with the builtin rules for proc-macro grammars.
    let mut grammar = gll::proc_macro::builtin();

    // HACK(eddyb) inject a custom builtin - this should be upstreamed to gll!
    {
        use gll::proc_macro::{FlatTokenPat, Pat};

        grammar.define(
            "LIFETIME",
            gll::grammar::eat(Pat(vec![
                FlatTokenPat::Punct {
                    ch: Some('\''),
                    joint: Some(true),
                },
                FlatTokenPat::Ident(None),
            ])),
        );
    }

    // Add in each grammar fragment to the grammar.
    for fragment in fragments {
        let path = fragment.into_path();

        // Inform Cargo about our dependency on the fragment files.
        println!("cargo:rerun-if-changed={}", path.display());

        let src = fs::read_to_string(&path).unwrap();
        let fragment: gll::grammar::Grammar<_> = src.parse().unwrap();
        grammar.extend(fragment);
    }

    // Generate a Rust parser from the combined grammar and write it out.
    fs::write(&out_dir.join("parse.rs"), grammar.generate_rust()).unwrap();
}
