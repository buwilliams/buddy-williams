# Buddy Williams — personal site

A polished personal-branding site for technology founders, built as a small
**Rust + Axum** server. Show-don't-tell: it features [Refine](https://getrefine.dev)
(in production), $2.3M+ in closed engagements, four companies founded, and the
full body of writing — essays rendered on-site from Markdown.

## Run locally

```bash
cargo run            # serves on http://localhost:3000
PORT=8080 cargo run  # override the port
```

The server reads `templates/`, `content/`, and `static/` relative to the current
directory (override with `BW_SITE_ROOT`).

## Editing content

No Rust changes are needed for copy edits:

- **`content/site.toml`** — bio, tagline, stats, social links, the Refine
  showcase, consulting offer, and selected projects.
- **`content/writings.toml`** — the published-essay manifest. `slug` maps to
  `content/essays/<slug>.md`. Set `featured = true` to surface on the home page;
  `order` controls sort (lowest first). Remove an entry to unpublish it.
- **`content/essays/*.md`** — the essays themselves (copied from the repo's
  `essays/`). The leading `# H1` is dropped automatically (the title comes from
  the manifest).

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
forces HTTPS, auto-stops when idle, and health-checks `/healthz`. The multi-stage
`Dockerfile` builds a release binary and ships it with `templates/`, `content/`,
and `static/`.
