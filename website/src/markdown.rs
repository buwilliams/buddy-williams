//! Markdown rendering for essays. Mirrors Refine's pulldown-cmark approach:
//! render to HTML, inject heading anchors, and collect a table of contents.

use pulldown_cmark::{html, Event, HeadingLevel, Options, Parser, Tag, TagEnd};
use serde::Serialize;
use std::collections::HashSet;

/// Context for rewriting relative Markdown links and images so a single source
/// of truth (GitHub-native relative paths) renders correctly on the site.
///
/// The essays are authored with relative links — `[x](other-essay.md)`,
/// `[y](../fragments/z.md)`, `![img](../assets/a/b.png)` — which resolve on
/// GitHub. On the site those same paths would 404, so we rewrite them:
///
/// - a link to a *published* essay  → `/writings/<slug>`
/// - a link to anything else (`.md`) → its canonical GitHub blob URL
/// - a relative image                → root-absolute `/assets/...` (served)
///
/// Absolute URLs, `mailto:`, in-page `#anchors`, and already-root-absolute
/// paths are left untouched.
pub struct RewriteCtx {
    /// Slugs that have a `/writings/<slug>` route (from the writings manifest).
    pub published: HashSet<String>,
    /// GitHub blob base for unpublished targets, e.g.
    /// `https://github.com/<owner>/<repo>/blob/main`.
    pub repo_base: String,
}

#[derive(Debug, Serialize)]
pub struct TocItem {
    pub level: u8,
    pub text: String,
    pub id: String,
}

#[derive(Debug, Serialize)]
pub struct Rendered {
    pub html: String,
    pub toc: Vec<TocItem>,
}

/// Split optional `---`-fenced YAML frontmatter from the top of a markdown file.
/// Returns `(frontmatter, body)`. The file must open with a `---` line; the block
/// ends at the next line that is exactly `---`. Any later `---` (a horizontal
/// rule in the prose) is left untouched. If the file does not open with a fence,
/// the frontmatter is `None` and the whole input is returned as the body.
pub fn split_frontmatter(md: &str) -> (Option<&str>, &str) {
    let Some(after_open) = md
        .strip_prefix("---\n")
        .or_else(|| md.strip_prefix("---\r\n"))
    else {
        return (None, md);
    };
    // Scan for a line that is exactly `---` (the closing fence).
    let mut from = 0;
    while let Some(rel) = after_open[from..].find("\n---") {
        let nl = from + rel; // the '\n' that begins the candidate line
        let fence = nl + 1; // first '-' of the candidate
        let line_end = after_open[fence..]
            .find('\n')
            .map(|i| fence + i)
            .unwrap_or(after_open.len());
        if after_open[fence..line_end].trim_end() == "---" {
            let fm = &after_open[..nl];
            let after = &after_open[line_end..];
            let body = after.strip_prefix('\n').unwrap_or(after);
            return (Some(fm), body);
        }
        from = fence;
    }
    // Unterminated fence: treat the whole file as body.
    (None, md)
}

/// Remove the leading `# H1` line (the title is shown from the manifest, so we
/// avoid rendering it twice).
pub fn strip_first_h1(md: &str) -> String {
    let mut lines = md.lines();
    let mut out = String::with_capacity(md.len());
    let mut dropped = false;
    // skip any leading blank lines, then drop the first H1 if present
    let mut pending_blanks = String::new();
    for line in lines.by_ref() {
        if !dropped {
            if line.trim().is_empty() {
                pending_blanks.push('\n');
                continue;
            }
            if line.trim_start().starts_with("# ") {
                dropped = true;
                continue;
            }
            // first non-blank line isn't an H1: keep everything as-is
            out.push_str(&pending_blanks);
            out.push_str(line);
            out.push('\n');
            dropped = true;
            continue;
        }
        out.push_str(line);
        out.push('\n');
    }
    out
}

/// Remove a hand-written "Contents" / "Table of Contents" section so it doesn't
/// duplicate the styled TOC the reader generates. Spans from the TOC heading up
/// to (but not including) the next heading. The Markdown source is left intact —
/// this only affects on-site rendering, so GitHub still shows the raw TOC.
pub fn strip_toc(md: &str) -> String {
    let lines: Vec<&str> = md.lines().collect();
    let mut out = String::with_capacity(md.len());
    let mut i = 0;
    while i < lines.len() {
        if is_toc_heading(lines[i]) {
            i += 1;
            while i < lines.len() && !is_heading(lines[i]) {
                i += 1;
            }
            continue;
        }
        out.push_str(lines[i]);
        out.push('\n');
        i += 1;
    }
    out
}

fn is_toc_heading(line: &str) -> bool {
    let t = line.trim();
    if !t.starts_with('#') {
        return false;
    }
    let rest = t.trim_start_matches('#').trim().to_ascii_lowercase();
    rest == "contents" || rest == "table of contents"
}

fn is_heading(line: &str) -> bool {
    let t = line.trim_start();
    let hashes = t.bytes().take_while(|&b| b == b'#').count();
    (1..=6).contains(&hashes) && t[hashes..].starts_with(' ')
}

/// Resolve a relative path against a base directory, collapsing `.` and `..`,
/// and return a repo-root-relative path. Essays live in `essays/`, so a bare
/// `layers.md` resolves to `essays/layers.md` and `../fragments/z.md` to
/// `fragments/z.md`.
fn resolve_repo_path(path: &str, base_dir: &str) -> String {
    let mut parts: Vec<&str> = base_dir.split('/').filter(|s| !s.is_empty()).collect();
    for seg in path.split('/') {
        match seg {
            "" | "." => {}
            ".." => {
                parts.pop();
            }
            other => parts.push(other),
        }
    }
    parts.join("/")
}

/// Rewrite a relative link URL for on-site rendering. Returns `None` when the
/// URL should be left untouched (absolute, anchor, mailto, root-absolute, or a
/// non-`.md` relative link, which images handle separately).
fn rewrite_link(url: &str, ctx: &RewriteCtx) -> Option<String> {
    if url.is_empty()
        || url.starts_with('#')
        || url.starts_with('/')
        || url.starts_with("mailto:")
        || url.contains("://")
    {
        return None;
    }
    let (path, frag) = match url.split_once('#') {
        Some((p, f)) => (p, Some(f)),
        None => (url, None),
    };
    if !path.ends_with(".md") {
        return None;
    }
    let repo_path = resolve_repo_path(path, "essays");
    let target = match repo_path.strip_prefix("essays/") {
        Some(file) if ctx.published.contains(file.trim_end_matches(".md")) => {
            format!("/writings/{}", file.trim_end_matches(".md"))
        }
        _ => format!("{}/{}", ctx.repo_base, repo_path),
    };
    Some(match frag {
        Some(f) => format!("{target}#{f}"),
        None => target,
    })
}

/// Rewrite a relative image URL to a root-absolute path served from `/assets`
/// (or wherever it lives in the repo). Absolute and root-absolute URLs are
/// left untouched.
fn rewrite_image(url: &str) -> Option<String> {
    if url.is_empty()
        || url.starts_with('/')
        || url.starts_with("data:")
        || url.contains("://")
    {
        return None;
    }
    Some(format!("/{}", resolve_repo_path(url, "essays")))
}

pub fn render(md: &str, ctx: &RewriteCtx) -> Rendered {
    let mut opts = Options::empty();
    opts.insert(Options::ENABLE_TABLES);
    opts.insert(Options::ENABLE_STRIKETHROUGH);
    opts.insert(Options::ENABLE_FOOTNOTES);
    opts.insert(Options::ENABLE_SMART_PUNCTUATION);

    let mut events: Vec<Event> = Parser::new_ext(md, opts).collect();
    let mut toc = Vec::new();

    // Rewrite relative links and images so the GitHub-native source resolves
    // correctly on the site. See `RewriteCtx`.
    for ev in events.iter_mut() {
        match ev {
            Event::Start(Tag::Link { dest_url, .. }) => {
                if let Some(new) = rewrite_link(dest_url, ctx) {
                    *dest_url = new.into();
                }
            }
            Event::Start(Tag::Image { dest_url, .. }) => {
                if let Some(new) = rewrite_image(dest_url) {
                    *dest_url = new.into();
                }
            }
            _ => {}
        }
    }

    let mut i = 0;
    while i < events.len() {
        if let Event::Start(Tag::Heading { level, .. }) = &events[i] {
            let lvl = *level;
            // gather text content of the heading
            let mut text = String::new();
            let mut j = i + 1;
            while j < events.len() {
                match &events[j] {
                    Event::Text(t) | Event::Code(t) => text.push_str(t),
                    Event::End(TagEnd::Heading(_)) => break,
                    _ => {}
                }
                j += 1;
            }
            let id = slugify(&text);
            if matches!(lvl, HeadingLevel::H2 | HeadingLevel::H3) && !text.is_empty() {
                toc.push(TocItem {
                    level: heading_num(lvl),
                    text: text.clone(),
                    id: id.clone(),
                });
            }
            if let Event::Start(Tag::Heading { id: slot, .. }) = &mut events[i] {
                *slot = Some(id.into());
            }
        }
        i += 1;
    }

    let mut out = String::new();
    html::push_html(&mut out, events.into_iter());
    Rendered { html: out, toc }
}

fn heading_num(level: HeadingLevel) -> u8 {
    match level {
        HeadingLevel::H1 => 1,
        HeadingLevel::H2 => 2,
        HeadingLevel::H3 => 3,
        HeadingLevel::H4 => 4,
        HeadingLevel::H5 => 5,
        HeadingLevel::H6 => 6,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ctx() -> RewriteCtx {
        RewriteCtx {
            published: ["computer-people", "layers", "framework-of-personhood"]
                .into_iter()
                .map(String::from)
                .collect(),
            repo_base: "https://github.com/owner/repo/blob/main".into(),
        }
    }

    #[test]
    fn published_essay_links_go_to_writings() {
        assert_eq!(rewrite_link("layers.md", &ctx()).unwrap(), "/writings/layers");
        // anchors are preserved
        assert_eq!(
            rewrite_link("computer-people.md#glossary", &ctx()).unwrap(),
            "/writings/computer-people#glossary"
        );
    }

    #[test]
    fn unpublished_targets_go_to_github_blob() {
        assert_eq!(
            rewrite_link("../bridges/mfc-guide.md", &ctx()).unwrap(),
            "https://github.com/owner/repo/blob/main/bridges/mfc-guide.md"
        );
        // an essay not in the manifest still goes to GitHub, not a dead /writings route
        assert_eq!(
            rewrite_link("cyclic-rationality.md", &ctx()).unwrap(),
            "https://github.com/owner/repo/blob/main/essays/cyclic-rationality.md"
        );
    }

    #[test]
    fn absolute_anchor_and_mailto_links_untouched() {
        assert!(rewrite_link("https://example.com", &ctx()).is_none());
        assert!(rewrite_link("#section", &ctx()).is_none());
        assert!(rewrite_link("mailto:hi@x.com", &ctx()).is_none());
        assert!(rewrite_link("/writings/layers", &ctx()).is_none());
        // non-markdown relative links are left for image handling / pass-through
        assert!(rewrite_link("notes.txt", &ctx()).is_none());
    }

    #[test]
    fn relative_images_become_root_absolute() {
        assert_eq!(
            rewrite_image("../assets/state-of-ai/x.png").unwrap(),
            "/assets/state-of-ai/x.png"
        );
        assert!(rewrite_image("https://cdn.example.com/x.png").is_none());
        assert!(rewrite_image("/assets/x.png").is_none());
    }
}

fn slugify(text: &str) -> String {
    let mut slug = String::with_capacity(text.len());
    let mut last_dash = false;
    for ch in text.chars() {
        if ch.is_ascii_alphanumeric() {
            slug.push(ch.to_ascii_lowercase());
            last_dash = false;
        } else if !last_dash && !slug.is_empty() {
            slug.push('-');
            last_dash = true;
        }
    }
    while slug.ends_with('-') {
        slug.pop();
    }
    if slug.is_empty() {
        slug.push_str("section");
    }
    slug
}
