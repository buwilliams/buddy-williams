//! Loads structured site content (TOML) and the template environment.
//! Content is data-driven so copy can change without touching Rust.

use serde::{Deserialize, Serialize};
use std::path::Path;

type BoxErr = Box<dyn std::error::Error + Send + Sync>;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Shot {
    pub src: String,
    pub caption: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GrowthLab {
    pub name: String,
    pub url: String,
    pub school_url: String,
    pub tagline: String,
    pub description: String,
    #[serde(default)]
    pub points: Vec<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Video {
    pub name: String,
    pub tagline: String,
    pub youtube_id: String,
    pub youtube_url: String,
    pub essay_slug: String,
    pub description: String,
    #[serde(default)]
    pub points: Vec<String>,
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
    pub growth_lab: GrowthLab,
    pub video: Video,
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
    /// When the essay was first written, `YYYY-MM-DD`.
    pub created: String,
    /// When the essay was last substantively revised, `YYYY-MM-DD`.
    pub updated: String,
    /// Display label for `created`, e.g. "Oct 2025".
    pub date: String,
    /// Display label for `updated` ("Sep 2026") when it falls in a later month
    /// than `created`; empty otherwise.
    pub updated_label: String,
    pub blurb: String,
    #[serde(default)]
    pub featured: bool,
    #[serde(default)]
    pub order: i64,
}

/// Per-essay metadata, parsed from the `---` YAML frontmatter at the top of each
/// `essays/<slug>.md`. The slug is the file stem, so it isn't stored here.
#[derive(Debug, Deserialize)]
struct FrontMatter {
    title: String,
    status: String,
    created: String,
    updated: String,
    blurb: String,
    #[serde(default)]
    featured: bool,
    #[serde(default)]
    order: i64,
}

pub fn load_site(root: &Path) -> Result<SiteConfig, BoxErr> {
    let raw = std::fs::read_to_string(root.join("content/site.toml"))?;
    Ok(toml::from_str(&raw)?)
}

/// Load published writings by scanning `<root>/essays/*.md`. An essay is published
/// iff it opens with a `---` YAML frontmatter block; files without one (raw drafts,
/// working notes) are skipped. Sorted newest to oldest by the month in `created`,
/// with `order` as the tie-breaker for writings created in the same month.
pub fn load_writings(root: &Path) -> Result<Vec<WritingMeta>, BoxErr> {
    let mut list = Vec::new();
    for entry in std::fs::read_dir(root.join("essays"))? {
        let path = entry?.path();
        if path.extension().and_then(|e| e.to_str()) != Some("md") {
            continue;
        }
        let raw = std::fs::read_to_string(&path)?;
        let Some(fm) = crate::markdown::split_frontmatter(&raw).0 else {
            continue; // no frontmatter → unpublished
        };
        let slug = path
            .file_stem()
            .and_then(|s| s.to_str())
            .ok_or("bad essay filename")?
            .to_string();
        let meta: FrontMatter = serde_yaml::from_str(fm)
            .map_err(|e| format!("frontmatter in {}: {e}", path.display()))?;
        list.push(writing_meta(slug, meta)?);
    }
    sort_writings_newest_first(&mut list)?;
    Ok(list)
}

fn writing_meta(slug: String, meta: FrontMatter) -> Result<WritingMeta, BoxErr> {
    let bad = |field: &str, value: &str| -> BoxErr {
        format!("unsupported {field} date {value:?} in {slug}; expected YYYY-MM-DD")
            .into()
    };
    let created = parse_date(&meta.created).ok_or_else(|| bad("created", &meta.created))?;
    let updated = parse_date(&meta.updated).ok_or_else(|| bad("updated", &meta.updated))?;
    if updated < created {
        return Err(format!("updated date precedes created date in {slug}").into());
    }
    let updated_label = if (updated.0, updated.1) > (created.0, created.1) {
        month_label(updated)
    } else {
        String::new()
    };
    Ok(WritingMeta {
        date: month_label(created),
        updated_label,
        created: meta.created,
        updated: meta.updated,
        slug,
        title: meta.title,
        status: meta.status,
        blurb: meta.blurb,
        featured: meta.featured,
        order: meta.order,
    })
}

fn sort_writings_newest_first(list: &mut [WritingMeta]) -> Result<(), BoxErr> {
    for writing in list.iter() {
        if parse_date(&writing.created).is_none() {
            return Err(format!(
                "unsupported created date {:?} in {}; expected YYYY-MM-DD",
                writing.created, writing.slug
            )
            .into());
        }
    }

    list.sort_by(|a, b| {
        created_month(b)
            .cmp(&created_month(a))
            .then_with(|| a.order.cmp(&b.order))
            .then_with(|| a.slug.cmp(&b.slug))
    });
    Ok(())
}

fn created_month(w: &WritingMeta) -> Option<(i32, u8)> {
    parse_date(&w.created).map(|(y, m, _)| (y, m))
}

/// Parse a `YYYY-MM-DD` date into (year, month, day).
fn parse_date(date: &str) -> Option<(i32, u8, u8)> {
    let [year, month, day] = date.split('-').collect::<Vec<_>>()[..] else {
        return None;
    };
    if year.len() != 4 || month.len() != 2 || day.len() != 2 {
        return None;
    }
    let (year, month, day): (i32, u8, u8) =
        (year.parse().ok()?, month.parse().ok()?, day.parse().ok()?);
    if !(1..=12).contains(&month) || !(1..=31).contains(&day) {
        return None;
    }
    Some((year, month, day))
}

fn month_label((year, month, _): (i32, u8, u8)) -> String {
    const MONTHS: [&str; 12] = [
        "Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec",
    ];
    format!("{} {year}", MONTHS[month as usize - 1])
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

#[cfg(test)]
mod tests {
    use super::*;

    fn writing(slug: &str, created: &str, order: i64) -> WritingMeta {
        WritingMeta {
            slug: slug.into(),
            title: slug.into(),
            status: "Draft".into(),
            created: created.into(),
            updated: created.into(),
            date: String::new(),
            updated_label: String::new(),
            blurb: String::new(),
            featured: true,
            order,
        }
    }

    #[test]
    fn writings_sort_newest_first_with_order_as_same_month_tiebreaker() {
        let mut writings = vec![
            writing("october", "2025-10-21", 0),
            writing("june-second", "2026-06-01", 2),
            writing("july", "2026-07-11", 9),
            writing("june-first", "2026-06-03", 1),
        ];

        sort_writings_newest_first(&mut writings).unwrap();

        let slugs: Vec<&str> = writings
            .iter()
            .map(|writing| writing.slug.as_str())
            .collect();
        assert_eq!(slugs, ["july", "june-first", "june-second", "october"]);
    }

    #[test]
    fn writings_reject_dates_that_cannot_be_sorted() {
        let mut writings = vec![writing("undated", "Oct 2025", 0)];

        let error = sort_writings_newest_first(&mut writings).unwrap_err();

        assert!(error.to_string().contains("expected YYYY-MM-DD"));
    }

    fn front(created: &str, updated: &str) -> FrontMatter {
        FrontMatter {
            title: "T".into(),
            status: "Final".into(),
            created: created.into(),
            updated: updated.into(),
            blurb: String::new(),
            featured: false,
            order: 0,
        }
    }

    #[test]
    fn dates_must_be_yyyy_mm_dd() {
        assert_eq!(parse_date("2025-10-08"), Some((2025, 10, 8)));
        for bad in ["2026-03", "2026-13-01", "2026-03-00", "26-03-01", "Mar 2026", "2026-03-1"] {
            assert_eq!(parse_date(bad), None, "{bad}");
        }
    }

    #[test]
    fn labels_show_created_month_and_later_updates_only() {
        let m = writing_meta("a".into(), front("2025-10-21", "2026-09-26")).unwrap();
        assert_eq!((m.date.as_str(), m.updated_label.as_str()), ("Oct 2025", "Sep 2026"));

        let m = writing_meta("b".into(), front("2026-09-01", "2026-09-26")).unwrap();
        assert_eq!((m.date.as_str(), m.updated_label.as_str()), ("Sep 2026", ""));

        assert!(writing_meta("c".into(), front("2026-09-26", "2026-01-01")).is_err());
    }
}
