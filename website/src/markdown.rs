//! Markdown rendering for essays. Mirrors Refine's pulldown-cmark approach:
//! render to HTML, inject heading anchors, and collect a table of contents.

use pulldown_cmark::{html, Event, HeadingLevel, Options, Parser, Tag, TagEnd};
use serde::Serialize;

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

pub fn render(md: &str) -> Rendered {
    let mut opts = Options::empty();
    opts.insert(Options::ENABLE_TABLES);
    opts.insert(Options::ENABLE_STRIKETHROUGH);
    opts.insert(Options::ENABLE_FOOTNOTES);
    opts.insert(Options::ENABLE_SMART_PUNCTUATION);

    let mut events: Vec<Event> = Parser::new_ext(md, opts).collect();
    let mut toc = Vec::new();

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
