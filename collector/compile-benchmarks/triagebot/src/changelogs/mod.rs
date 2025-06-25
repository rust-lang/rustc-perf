mod rustc;

use comrak::{nodes::AstNode, Arena, ComrakOptions, ComrakRenderOptions};
use std::collections::HashMap;

#[derive(Copy, Clone, PartialEq, Eq, Debug, serde::Deserialize)]
#[serde(rename_all = "kebab-case")]
pub(crate) enum ChangelogFormat {
    Rustc,
}

pub(crate) struct Changelog {
    versions: HashMap<String, String>,
}

impl Changelog {
    pub(crate) fn parse(format: ChangelogFormat, content: &str) -> anyhow::Result<Self> {
        match format {
            ChangelogFormat::Rustc => rustc::RustcFormat::new(&Arena::new()).parse(content),
        }
    }

    pub(crate) fn version(&self, version: &str) -> Option<&str> {
        self.versions.get(version).map(|s| s.as_str())
    }
}

fn render_for_github_releases<'a>(document: &'a AstNode<'a>) -> anyhow::Result<String> {
    let mut content = Vec::new();
    comrak::format_commonmark(
        document,
        &ComrakOptions {
            render: ComrakRenderOptions {
                // Prevent column width line breaks from appearing in the generated release
                // notes. GitHub Releases insert <br>s for every line break in the markdown,
                // mangling the output.
                width: std::usize::MAX,

                ..ComrakRenderOptions::default()
            },
            ..ComrakOptions::default()
        },
        &mut content,
    )?;
    Ok(String::from_utf8(content)?)
}
