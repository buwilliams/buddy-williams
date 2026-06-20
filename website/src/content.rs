//! Loads structured site content (TOML) and the template environment.
//! Content is data-driven so copy can change without touching Rust.

use serde::{Deserialize, Serialize};
use std::path::Path;

type BoxErr = Box<dyn std::error::Error + Send + Sync>;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Stat {
    pub value: String,
    pub label: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Shot {
    pub src: String,
    pub caption: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Refine {
    pub name: String,
    pub url: String,
    pub tagline: String,
    pub description: String,
    #[serde(default)]
    pub points: Vec<String>,
    #[serde(default)]
    pub screenshots: Vec<Shot>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Project {
    pub name: String,
    pub period: String,
    pub blurb: String,
    #[serde(default)]
    pub url: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Cover {
    pub title: String,
    pub body: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Consulting {
    pub headline: String,
    pub intro: String,
    #[serde(default)]
    pub covers: Vec<Cover>,
    pub note: String,
    pub cta: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SiteConfig {
    pub name: String,
    pub role_title: String,
    pub tagline: String,
    pub positioning: String,
    pub hero_sub: String,
    pub bio: String,
    pub email: String,
    pub x_handle: String,
    pub x_url: String,
    pub substack_url: String,
    pub github_url: String,
    pub repo_url: String,
    pub scheduler_url: String,
    pub scheduler_label: String,
    pub cal_link: String,
    #[serde(default)]
    pub stats: Vec<Stat>,
    pub refine: Refine,
    pub consulting: Consulting,
    #[serde(default)]
    pub projects: Vec<Project>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct WritingMeta {
    pub slug: String,
    pub title: String,
    pub status: String,
    pub date: String,
    pub blurb: String,
    #[serde(default)]
    pub featured: bool,
    #[serde(default)]
    pub order: i64,
}

#[derive(Debug, Deserialize)]
struct WritingsFile {
    #[serde(default)]
    writing: Vec<WritingMeta>,
}

pub fn load_site(root: &Path) -> Result<SiteConfig, BoxErr> {
    let raw = std::fs::read_to_string(root.join("content/site.toml"))?;
    Ok(toml::from_str(&raw)?)
}

pub fn load_writings(root: &Path) -> Result<Vec<WritingMeta>, BoxErr> {
    let raw = std::fs::read_to_string(root.join("content/writings.toml"))?;
    let parsed: WritingsFile = toml::from_str(&raw)?;
    let mut list = parsed.writing;
    list.sort_by_key(|w| w.order);
    Ok(list)
}

/// A short hash of the CSS + JS contents, appended to asset URLs as `?v=...`.
/// Each deploy with changed assets gets a new value, so Cloudflare's edge cache
/// (which caches static files) serves the new file instead of a stale one.
pub fn asset_version(root: &Path) -> String {
    use std::hash::{Hash, Hasher};
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    for rel in ["static/css/site.css", "static/js/site.js"] {
        if let Ok(bytes) = std::fs::read(root.join(rel)) {
            bytes.hash(&mut hasher);
        }
    }
    format!("{:x}", hasher.finish())
}

/// Build the minijinja environment by loading every `*.html` in `templates/`.
/// Templates are added as owned strings, keeping the environment `'static`.
pub fn build_env(root: &Path) -> Result<minijinja::Environment<'static>, BoxErr> {
    let mut env = minijinja::Environment::new();
    let dir = root.join("templates");
    for entry in std::fs::read_dir(&dir)? {
        let path = entry?.path();
        if path.extension().and_then(|e| e.to_str()) == Some("html") {
            let name = path
                .file_name()
                .and_then(|n| n.to_str())
                .ok_or("bad template name")?
                .to_string();
            let source = std::fs::read_to_string(&path)?;
            env.add_template_owned(name, source)?;
        }
    }
    Ok(env)
}
