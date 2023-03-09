use std::path::{Path, PathBuf};
use tera::Tera;
use tokio::sync::RwLock;

pub struct TemplateRenderer {
    template_dir: PathBuf,
    tera: RwLock<Tera>,
    // Reload the templates each time for faster iteration time
    livereload: bool,
}

impl TemplateRenderer {
    pub fn new(template_dir: PathBuf, livereload: bool) -> anyhow::Result<Self> {
        let tera = load_templates(&template_dir)?;
        Ok(Self {
            template_dir,
            tera: RwLock::new(tera),
            livereload,
        })
    }

    pub async fn render(&self, name: &str) -> anyhow::Result<String> {
        if self.livereload {
            *self.tera.write().await = load_templates(&self.template_dir)?;
        }

        let context = tera::Context::new();
        let rendered = self.tera.read().await.render(name, &context)?;
        Ok(rendered)
    }
}

fn load_templates(path: &Path) -> anyhow::Result<Tera> {
    let tera = Tera::new(&format!("{}/**/*.html", path.display()))
        .map_err(|error| anyhow::anyhow!("Could not load Tera templates: {error:?}"))?;
    Ok(tera)
}
