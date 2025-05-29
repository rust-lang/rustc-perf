use super::Changelog;
use anyhow::Context as _;
use comrak::{
    nodes::{AstNode, NodeHeading, NodeValue},
    Arena, ComrakOptions,
};
use std::collections::HashMap;

pub(super) struct RustcFormat<'a> {
    arena: &'a Arena<AstNode<'a>>,
    current_h1: Option<String>,
    result: Changelog,
}

impl<'a> RustcFormat<'a> {
    pub(super) fn new(arena: &'a Arena<AstNode<'a>>) -> Self {
        RustcFormat {
            arena,
            current_h1: None,
            result: Changelog {
                versions: HashMap::new(),
            },
        }
    }

    pub(super) fn parse(mut self, content: &str) -> anyhow::Result<Changelog> {
        let ast = comrak::parse_document(&self.arena, &content, &ComrakOptions::default());

        let mut section_ast = Vec::new();
        for child in ast.children() {
            let child_data = child.data.borrow();

            if let NodeValue::Heading(NodeHeading { level: 1, .. }) = child_data.value {
                if let Some(h1) = self.current_h1.take() {
                    self.store_version(h1, section_ast)?;
                }

                let Some(h1_child_data) = child.first_child().map(|c| c.data.borrow()) else {
                    anyhow::bail!("unable to retrieve heading (H1) child from changelog");
                };
                self.current_h1 = Some(
                    h1_child_data
                        .value
                        .text()
                        .context("unable to get the text of node below the heading H1")?
                        .to_string(),
                );
                section_ast = Vec::new();
            } else {
                section_ast.push(child);
            }
        }
        if let Some(h1) = self.current_h1.take() {
            self.store_version(h1, section_ast)?;
        }

        Ok(self.result)
    }

    fn store_version(&mut self, h1: String, body: Vec<&'a AstNode<'a>>) -> anyhow::Result<()> {
        // Create a document with only the contents of this section
        let document = self.arena.alloc(NodeValue::Document.into());
        for child in &body {
            document.append(child);
        }

        let content = super::render_for_github_releases(document)?;

        if let Some(version) = h1.split(' ').nth(1) {
            self.result.versions.insert(version.to_string(), content);
        } else {
            println!("skipped version, invalid header: {}", h1);
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const CONTENT: &str = "\
Version 1.45.2 (2020-08-03)
==========================

* [Fix bindings in tuple struct patterns][74954]
* [Link in another section][69033]
* Very very very very very very very very very very very long line that has some
  linebreaks here and there

[74954]: https://github.com/rust-lang/rust/issues/74954

Version 1.45.1 (2020-07-30)
==========================

* [Fix const propagation with references.][73613]
* [rustfmt accepts rustfmt_skip in cfg_attr again.][73078]

[73613]: https://github.com/rust-lang/rust/pull/73613
[73078]: https://github.com/rust-lang/rust/issues/73078

Version 1.44.0 (2020-06-04)
==========================

Language
--------
- [You can now use `async/.await` with `#[no_std]` enabled.][69033]

**Syntax-only changes**

- [Expansion-driven outline module parsing][69838]
```rust
#[cfg(FALSE)]
mod foo {
    mod bar {
        mod baz; // `foo/bar/baz.rs` doesn't exist, but no error!
    }
}
```

These are still rejected semantically, so you will likely receive an error but
these changes can be seen and parsed by macros and conditional compilation.

Internal Only
-------------
These changes provide no direct user facing benefits, but represent significant
improvements to the internals and overall performance of rustc and
related tools.

- [dep_graph Avoid allocating a set on when the number reads are small.][69778]

[69033]: https://github.com/rust-lang/rust/pull/69033/
[69838]: https://github.com/rust-lang/rust/pull/69838/
[69778]: https://github.com/rust-lang/rust/pull/69778/
";

    const EXPECTED_1_45_2: &str = "\
- [Fix bindings in tuple struct patterns](https://github.com/rust-lang/rust/issues/74954)
- [Link in another section](https://github.com/rust-lang/rust/pull/69033/)
- Very very very very very very very very very very very long line that has some linebreaks here and there
";

    #[test]
    fn test_changelog_parsing() -> anyhow::Result<()> {
        let arena = Arena::new();
        let parsed = RustcFormat::new(&arena).parse(CONTENT)?;

        // Ensure the right markdown is generated from each version
        let version_1_45_2 = parsed.version("1.45.2").expect("missing version 1.45.2");
        assert_eq!(EXPECTED_1_45_2, version_1_45_2);

        let version_1_44_0 = parsed.version("1.44.0").expect("missing version 1.44.0");
        assert!(version_1_44_0.contains("Avoid allocating a set"));

        Ok(())
    }
}
