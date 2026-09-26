# AGENTS.md

Guidance for AI agents working in this repository.

## What This Repository Is

Essays by Buddy Williams, written in Markdown, and the website that publishes them at [buddy-williams.com](https://buddy-williams.com). Topics span AI and personhood, economics, epistemology, and what it means to be human in an age of thinking machines.

## Repository Structure

- `essays/` — All writing lives here, one Markdown file per essay. The file name is the URL slug (`/writings/<slug>`).
- `style-guide.md` — **Read this before any writing or editing task.** Defines voice, brevity, structure, criticism standards, and priorities.
- `website/` — Rust/Axum site that renders `essays/` directly (no copied content). Essay images live in `website/static/img/essays/`. See `website/README.md` and `website/DEPLOY.md`.
- `resume/` — Résumés, cover letters, `about.md`, and `my-work.md` (a work inventory whose essay table mirrors essay frontmatter).

## Essay Frontmatter and Status

An essay is published on the site if and only if it begins with a YAML frontmatter block:

```yaml
---
title: "Thinking About Thinking"
status: "Final"        # Draft | Working | Final
created: "2026-09-01"  # YYYY-MM-DD when first written; use the 1st if the day is unknown
updated: "2026-09-26"  # YYYY-MM-DD of the last substantive revision
order: 0               # tie-breaker within a month, lower first; the index sorts newest first by created month
featured: true         # optional
blurb: "One or two sentences for the index."
---
```

Status: **Draft** (AI-assisted sketch, not ready for public criticism) → **Working** (rewritten by hand) → **Final** (complete, no further updates intended). Files without frontmatter are unpublished drafts.

When you substantively revise an essay, set `updated` to today. Leave `created` alone. Essay pages show "Updated <Mon YYYY>" when `updated` falls in a later month than `created`.

## Mechanics

- Link other essays with relative paths (`other-essay.md`, `other-essay.md#anchor`), never `/writings/...`. The site rewrites `.md` links, and relative links also work on GitHub.
- Link images as `../website/static/img/essays/<file>`; the site serves them at `/static/img/essays/<file>`.
- Site tests: `cd website && cargo test`. Every push to `main` that touches `essays/`, `website/`, or the workflow auto-deploys the site, so publishing happens on merge to `main`.

## How to Assist with Writing

`style-guide.md` is the authoritative reference. Key points:

- **Model essay**: `essays/thinking-about-thinking.md`. Match its plain language, brevity, and structure.
- **Brevity**: Every sentence earns its place. Cut restated theses, recaps, slogan stacks, and filler, but never a step the argument depends on.
- **Voice**: Clear, serious, exploratory, intellectually alive. Thomas Sowell's clarity meets David Deutsch's explanatory ambition. Not academic, not performative.
- **Structure**: Introduction → Table of Contents → numbered sections → Conclusion → Further Reading / Appendices. No "Abstract", no byline or subtitle under the title, no horizontal rules between sections.
- **Terminological consistency**: When a term is defined, use it precisely throughout. Don't swap in near-synonyms that blur distinctions.
- **Criticism**: Identify unsupported claims, logical gaps, missing transitions, unstated assumptions, and factual errors. Distinguish preventable confusion from fundamental disagreement.
- **Anticipate objections**: Flag claims stated too strongly or too early. Add bridging clauses, soften until earned, or reorder.
- **Priority order**: explanatory clarity > conceptual accuracy > originality > readability > elegance.
- **Avoid**: empty academic language, jargon, false certainty, summarizing instead of explaining, softening bold ideas to seem safer.

## Intellectual Context

Rooted in critical rationalism (Popper, Deutsch): knowledge grows by conjecture and criticism, and people, human or artificial, are universal explainers. Also systems thinking and economics as coordination under uncertainty. Influenced by Hayek, Sowell, Sagan, Gödel, Turing, Haidt. The central question: how to design systems that remain humane as intelligence becomes abundant.
