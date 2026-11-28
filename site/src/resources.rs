use tera::Tera;
use tokio::sync::RwLock;

use crate::load::SiteCtxt;
use rust_embed::RustEmbed;

/// Static files and templates are embedded into the binary (in release mode) or hot-reloaded
/// from the `frontend/static` directory (in debug mode).
#[derive(RustEmbed)]
#[folder = "frontend/static/"]
#[include = "*.js"]
#[include = "*.css"]
#[include = "*.svg"]
#[include = "*.png"]
#[include = "*.html"]
#[include = "*.woff2"]
#[include = "*.map"]
#[include = "*.json"]
#[include = "*.wasm"]
struct StaticAssets;

/// Frontend source files compiled by `npm`.
#[derive(RustEmbed)]
#[folder = "frontend/dist"]
#[include = "*.js"]
#[include = "*.br"]
#[include = "*.css"]
// Source maps for debugging; only in debug mode since it increases binary size by ~6%
#[cfg_attr(debug_assertions, include = "*.map")]
struct StaticCompiledAssets;

/// HTML template files that will be rendered by `tera`.
#[derive(RustEmbed)]
#[folder = "frontend/templates/"]
#[include = "*.html"]
struct TemplateAssets;

pub enum Payload {
    Compressed(Vec<u8>),
    Uncompressed(Vec<u8>),
}

/// Data that is put into every page, and is then available in the PAGE_DATA global variable.
#[derive(serde::Serialize)]
struct PageData<'a> {
    repository: &'a str,
}

pub struct ResourceResolver {
    tera: RwLock<Tera>,
}

impl ResourceResolver {
    pub fn new() -> anyhow::Result<Self> {
        let tera = load_templates()?;

        Ok(Self {
            tera: RwLock::new(tera),
        })
    }

    pub fn get_static_asset(&self, path: &str, use_compression: bool) -> Option<Payload> {
        if use_compression {
            let compressed_path = path.to_owned() + ".br";
            let data =
                StaticCompiledAssets::get(compressed_path.as_str()).map(|file| file.data.to_vec());
            if let Some(data) = data {
                return Some(Payload::Compressed(data));
            }
        }

        StaticCompiledAssets::get(path)
            .or_else(|| StaticAssets::get(path))
            .map(|file| Payload::Uncompressed(file.data.to_vec()))
    }

    pub async fn get_template(&self, ctxt: &SiteCtxt, path: &str) -> anyhow::Result<Vec<u8>> {
        // Live-reload the template if we're in debug mode
        #[cfg(debug_assertions)]
        {
            *self.tera.write().await = load_templates()?;
        }

        let mut context = tera::Context::new();
        context.insert(
            "data",
            &PageData {
                repository: ctxt.config.repo_full_name().as_str(),
            },
        );
        let rendered = self.tera.read().await.render(path, &context)?;
        Ok(rendered.into_bytes())
    }
}

fn load_templates() -> anyhow::Result<Tera> {
    let templates = TemplateAssets::iter().map(|path| {
        (
            path.to_string(),
            String::from_utf8(TemplateAssets::get(&path).unwrap().data.to_vec()).unwrap(),
        )
    });
    let mut tera = Tera::default();
    tera.add_raw_templates(templates)?;
    Ok(tera)
}
