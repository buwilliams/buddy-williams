# Buddy Williams — personal site

A polished personal-branding site for technology founders, built as a small
**Rust + Axum** server. Show-don't-tell: it features [Refine](https://getrefine.dev)
(in production), $2.3M+ in closed engagements, four companies founded, and the
full body of writing — essays rendered on-site from Markdown.

## Run locally

```bash
BW_WRITINGS_ROOT=.. cargo run            # serves on http://localhost:3000
BW_WRITINGS_ROOT=.. PORT=8080 cargo run  # override the port
```

The server reads two roots:

- **`BW_SITE_ROOT`** (default `.`) — the site app files: `templates/`, `content/`,
  and `static/`, all under `website/`.
- **`BW_WRITINGS_ROOT`** (default = `BW_SITE_ROOT`) — the repo root that holds the
  canonical `essays/` and `assets/`. Running from inside `website/`, that's the
  parent directory, so set `BW_WRITINGS_ROOT=..`.

The essays are **not duplicated** into the site. The Markdown under the repo's
`essays/` is the single source of truth ("data"); the backend renders it into the
styled site at request time — dropping the `# H1`, stripping the hand-written TOC
and generating the on-page one, rewriting relative links/images, and applying the
site CSS.

## Editing content

No Rust changes are needed for copy edits:

- **`content/site.toml`** — bio, tagline, stats, social links, the Refine
  showcase, consulting offer, and selected projects.
- **`../essays/*.md`** — the essays themselves, the single source of truth. Each
  published essay opens with a `+++` TOML frontmatter block (`title`, `status`,
  `date`, `blurb`, optional `featured`, `order`); the slug is the filename. The
  site scans `essays/` at startup and publishes every file that has frontmatter —
  set `featured = true` to surface on the home page, `order` to sort (lowest
  first), and delete the frontmatter to unpublish. Files without frontmatter (raw
  drafts, working notes) are ignored. Essays render on GitHub *and* on the site
  from this one copy; the site rewrites their relative links/images at render
  time. Images live in `../assets/` and are served at `/assets/...`.

### Booking

`scheduler_url` in `site.toml` is the "Book a free intro call" link. Drop in your
Cal.com / Calendly URL. Paid tiers later = a paid Cal.com event type swapped in
here, plus a pricing block — no rearchitecting.

## Routes

| Path               | Purpose                          |
| ------------------ | -------------------------------- |
| `/`                | Home                             |
| `/writings`        | Essay index (status badges)      |
| `/writings/:slug`  | Essay reader (Markdown → HTML)   |
| `/consulting`      | The offer + booking              |
| `/resume`          | Résumé PDF                       |
| `/healthz`         | Health check (for Fly)           |
| `/static/*`        | CSS, JS, images                  |

## Deploy to Fly.io

```bash
fly launch --no-deploy   # first time (or: fly apps create buddy-williams)
fly deploy
```

`fly.toml` runs one `shared-cpu-1x` / 256MB machine in `dfw`, listens on `:8080`,
forces HTTPS, auto-stops when idle, and health-checks `/healthz`.

The Docker **build context is the repo root** (so the image can include `essays/`
and `assets/` alongside the site app). Deploy from the repo root, pointing at the
config and Dockerfile under `website/`:

```bash
flyctl deploy . --config website/fly.toml --dockerfile website/Dockerfile
```

CI (`.github/workflows/deploy.yml`) does this automatically on pushes to `main`
that touch `website/**`, `essays/**`, or `assets/**`.
