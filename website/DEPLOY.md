# Deploying buddy-williams.com

The site is a Rust/Axum app deployed to **Fly.io**, with the domain on
**Cloudflare**, and **GitHub Actions** auto-deploying every push to `main` that
touches `website/`.

Do these once, in order. All `fly`/`gh` commands assume you're in `website/`.

---

## 1. First deploy to Fly.io (one time)

```bash
fly auth login                  # opens a browser
fly apps create buddy-williams  # name must match fly.toml
fly deploy                      # builds the Dockerfile remotely and boots a machine
```

Confirm it's live:

```bash
fly open                        # opens https://buddy-williams.fly.dev
fly status
```

---

## 2. GitHub Actions auto-deploy (one time)

The workflow lives at `.github/workflows/deploy.yml` (repo root). It needs a Fly
deploy token stored as the `FLY_API_TOKEN` GitHub secret.

Create the token and push it to GitHub in one step (requires the `gh` CLI, authed):

```bash
fly tokens create deploy -x 8760h | gh secret set FLY_API_TOKEN --repo buwilliams/buddy-williams
```

Or manually: run `fly tokens create deploy -x 8760h`, copy the output, and add it
at **GitHub → repo Settings → Secrets and variables → Actions → New repository
secret**, name `FLY_API_TOKEN`.

From then on, any push to `main` under `website/**` deploys automatically. You can
also trigger it from the Actions tab (workflow_dispatch).

---

## 3. Custom domain — Fly certificate

Allocate IPs (IPv6 is free and dedicated; the shared IPv4 is free):

```bash
fly ips allocate-v6
fly ips allocate-v4 --shared
fly ips list                    # note the addresses
```

Request certificates for the apex and www:

```bash
fly certs add buddy-williams.com
fly certs add www.buddy-williams.com
fly certs show buddy-williams.com   # prints the exact DNS records / validation target
```

Keep `fly certs show` output handy — it tells you the `_acme-challenge` target
needed for validation.

---

## 4. Cloudflare DNS + TLS

In the Cloudflare dashboard for **buddy-williams.com**:

**SSL/TLS → Overview:** set encryption mode to **Full (strict)**.

**DNS → Records:** add

| Type  | Name              | Content                                   | Proxy        |
| ----- | ----------------- | ----------------------------------------- | ------------ |
| A     | `@`               | _Fly shared IPv4_ (from `fly ips list`)   | see below    |
| AAAA  | `@`               | _Fly dedicated IPv6_ (from `fly ips list`)| see below    |
| CNAME | `www`             | `buddy-williams.fly.dev`                  | see below    |
| CNAME | `_acme-challenge` | _target from `fly certs show`_            | **DNS only** |

The `_acme-challenge` record **must be DNS-only (grey cloud)** so Fly's
Let's Encrypt validation succeeds.

**Proxy choice for the A/AAAA/www records:**

- **Recommended — DNS only (grey cloud) first.** Simplest; Fly handles TLS and
  issues the cert without interference. Once `fly certs show buddy-williams.com`
  reports the cert is **Ready**, you may flip A/AAAA/www to **Proxied (orange)**
  to get Cloudflare's CDN + DDoS protection. With proxy on, keep SSL mode at
  **Full (strict)** and leave `_acme-challenge` grey.

Verify end to end:

```bash
fly certs show buddy-williams.com     # status should be "Ready"
curl -I https://buddy-williams.com/healthz
```

---

## Day-to-day

- **Deploy:** push to `main` (touching `website/`) — Actions does the rest.
- **Manual deploy:** `fly deploy` from `website/`.
- **Logs:** `fly logs`. **Console:** `fly ssh console`.
- **New essay:** add `essays/<slug>.md` with a `+++` frontmatter block at the top
  (`title`, `status`, `date`, `blurb`, optional `featured`/`order`), commit.
