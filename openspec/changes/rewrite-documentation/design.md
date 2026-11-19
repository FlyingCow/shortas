# Design

## Context

See proposal.md (Why) for the motivation. Current state that shapes the approach:

- The site is Jekyll on GitHub Pages: `docs/`, `CNAME` = `shortas.work`, theme `minima`, plus a custom `professional-theme.html` layout. The top navigation is driven by `site.navigation` in `_config.yml`. No CI workflow builds the docs; Pages serves `docs/` directly.
- The site has 7 pages, all using `professional-theme`. `vector-theme.html` is not referenced. The `guides`/`api` collections in `_config.yml` have no content.
- Inbound links that must keep resolving: `/`, `/architecture/`, `/getting-started/`, `/development/`, `/deployment/`, `/api/`, `/about/` (from `ui/landing` and the READMEs).
- The infra READMEs (`infra/hetzner`, `infra/hetzner-budget`, `infra/aws/terraform`) are operational runbooks with real commands that people follow. They are the least bloated per sentence, but the largest by volume.

## Goals / Non-Goals

**Goals:**
- A reader can understand what Shortas is and how it works from the README plus the site's home and architecture pages in about 5 minutes.
- Each fact has exactly one home (see Decision 2). Duplicates are replaced by links.
- Every command and port in the rewritten docs is checked against the repo (Makefile, compose files, config) as it stands now.

**Non-Goals:**
- New visual design or theme work. The existing `professional-theme` layout is reused as is.
- Exhaustive reference material (endpoint-by-endpoint API docs, RabbitMQ topology, outbox internals). Code and inline comments are the source for those.
- Docs CI (link checking, a Jekyll build workflow).

## Decisions

### 1. Site map: keep all 7 URLs, shrink the content

| URL | Purpose | Budget |
|---|---|---|
| `/` | Pitch, capability list, one diagram, links | ~300 words |
| `/architecture/` | Components and how a click flows; data stores; TLS/custom domains in one section | ~900 words |
| `/getting-started/` | Run the full stack locally (compose), seed data, open the dashboard | ~400 words |
| `/deployment/` | Comparison table of the 4 targets + a short section each, linking to the `infra/*` runbooks | ~600 words |
| `/development/` | Repo layout, build/test/lint per workspace, adding a notification template | ~500 words |
| `/api/` | Endpoint *groups* of the Management API (one table), auth, base URL; internal APIs mentioned in one line | ~300 words |
| `/about/` | Goals, author, license, links | ~150 words |

*Why keep `/api/` instead of folding it into architecture?* The landing page links to it, and a small stable page costs little. *Alternative:* merging pages and adding redirects (needs the `jekyll-redirect-from` plugin). Rejected because it adds a plugin to save one short page.

Budgets are targets, not hard limits. The total site should land around 3k words, down from ~6.5k.

### 2. Where each fact lives

| Fact | Single home | Everywhere else |
|---|---|---|
| Architecture diagram + component roles | `/architecture/` | README has one simplified diagram, linked |
| Service port table | `/getting-started/` | Removed from README |
| Project tree | `/development/` | README gets a 1-line pointer |
| Make / cargo commands | `/development/` | Component READMEs show only their own commands |
| Deployment steps | `infra/<target>/README.md` | Site summarizes and links; README links to the site |
| Tech stack table | `/architecture/` | Removed from README |
| Per-service config/env vars | That service's README | Site does not repeat them |

### 3. Writing rules (apply to every rewritten file)

- Lines wrap naturally in source. Paragraphs are at most ~4 sentences, and one idea per paragraph.
- Describe what the system **is**, in present tense. No history ("previously", "was rewritten"), no "not yet wired", no "for future use", no explanations of bug fixes.
- Prefer tables and lists over prose for inventories. Prefer prose over lists for explaining a flow.
- Diagrams use plain ASCII box-drawing that renders reliably in GitHub and in the Jekyll `<pre>`. One diagram per concept, never the same diagram twice.
- Internal names (`PublishNotificationEvent`, `CryptoCache`) appear only when the reader needs them to find code.

### 4. Component README template

```
# <name>
One sentence: what it does and where it sits (link to /architecture/#<anchor>).
## Run       – the 1–3 commands to build/run/test it
## Config    – env vars / config file table (only if it has its own)
```

Target 10–40 lines. Infra READMEs are the exception: they keep their runbook steps, but get trimmed to prerequisites → steps → verify → operate (backups, upgrades), with rationale cut to one line per decision.

### 5. Legacy `api/`

All mentions are removed from the README and the site. `api/README.md` becomes a 3-line note pointing to `management-api/`. The directory itself is untouched. This is an assumed default and easy to revisit.

### 6. Jekyll config cleanup

- `url: "https://shortas.work"`. Remove `email`, `twitter_username`, the unused `collections` and their defaults, and the `jekyll-feed` plugin (there is no blog). Keep `jekyll-sitemap` and `jekyll-seo-tag`.
- Delete `_layouts/vector-theme.html`.
- Update `navigation` only if the page titles change. The URLs stay the same.

### 7. Rewrite from scratch, verify against code

Each page is written fresh from the code and config, not by editing the old text. The old text is used only as a checklist so nothing important is lost. Each command, port, and path is verified by reading the Makefile, the compose files, or the source before it goes in.

## Risks / Trade-offs

- [Dropping detail that someone relied on, e.g. the RabbitMQ topology or the notification pipeline steps] → Before deleting, check that the information is recoverable from code or comments. Where it isn't, keep a compact version in the relevant component README.
- [Infra runbooks lose a necessary step during trimming] → Cut only rationale and prose. Never remove a command or step without confirming it is obsolete.
- [Facts drift again over time] → Mitigated by the single-home table (Decision 2). Not enforced by tooling (non-goal).
- [Live site changes on merge to main] → It is docs only, and git revert rolls it back. Preview locally with `bundle exec jekyll serve` before merging.

## Migration Plan

1. Rewrite on a branch. Preview the site locally.
2. Merge to main. GitHub Pages republishes from `docs/`.
3. Spot-check the inbound URLs from the landing page on shortas.work.

Rollback: revert the merge commit.
