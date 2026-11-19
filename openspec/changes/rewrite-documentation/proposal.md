# Proposal

## Why

The project's documentation (~14k words across the root README, the shortas.work Jekyll site, and per-directory READMEs) has become bloated and hard to read. The same facts (architecture diagram, ports, project tree, make targets, AWS steps) are repeated in up to five places and have drifted apart. Paragraphs read like commit messages, full of implementation rationale and edge cases. The published site documents only AWS deployment, even though the repo also maintains two Hetzner deployments. The docs are being rewritten from scratch, not patched, because nearly every page has the same habits.

## What Changes

- **Audience reset**: the published site (shortas.work) becomes a concise **portfolio/overview** site: what Shortas is, how it works at a glance, how to run it, and how to deploy it. Deep internals (step-by-step pipelines, the RabbitMQ topology, full endpoint catalogs) leave the site.
- **One home per fact**: each fact is written in exactly one place, and every other mention links to it:
  - Root `README.md`: pitch, one diagram, a short quickstart, and links to the site. No duplicated tables.
  - shortas.work: the single explained narrative.
  - Component and infra READMEs: only "how to build/run/configure *this directory*".
- **Rewrite the docs site pages** (`docs/`): home, architecture, getting started (local), deployment, development, about. The page set is consolidated and every page is rewritten to a word budget.
- **Deployment coverage for all four targets**: local Docker Compose, Hetzner multi-node, Hetzner budget (single VM), and AWS Terraform. The site gives a short overview and a comparison of the targets. The step-by-step runbooks stay in the trimmed `infra/*/README.md` files, and the site links to them.
- **API reference reduced**: the hand-maintained endpoint catalog is replaced by a short summary of endpoint groups.
- **Legacy `api/` (.NET) dropped from the docs**: every mention is removed from the README and the site. `api/README.md` shrinks to a short "superseded by `management-api/`" note. (Assumed default: the user did not choose an option for this.)
- **Rewrite or trim every first-party README**: `redirect/`, the per-service READMEs, `redirect/monitoring`, `infra/hetzner`, `infra/hetzner-budget`, `infra/aws/terraform` (+ the cognito module), `ui/dashboard`, `ui/landing`, `docs/README.md`.
- **Jekyll cleanup**: fix `_config.yml` (the `url` should match CNAME `shortas.work`; remove placeholder email and Twitter, unused collections and plugins), and remove the unused `vector-theme.html` layout.
- **URLs kept stable**: existing paths that are linked from the landing page and READMEs (`/`, `/architecture/`, `/getting-started/`, `/deployment/`, `/development/`, `/api/`, `/about/`) keep resolving.

Out of scope: changing code, deleting the `api/` directory, the vendored `redirect/data/ua-parser` submodule docs, redesigning the site's visual theme, and auto-generated API docs (OpenAPI).

## Capabilities

### New Capabilities

None. This change affects documentation only and changes no system behavior, so it opts out of specs (`skip_specs: true`).

### Modified Capabilities

None.

## Impact

- `README.md`, `docs/**` (pages, `_config.yml`, `_layouts/`), and first-party `README.md` files under `redirect/`, `infra/`, `ui/`, and `api/`.
- The live site at shortas.work changes when it is next published from `docs/`.
- Inbound links from `ui/landing` (`/architecture/`, `/development/`, `/getting-started/`, `/`) must keep working. The `#notification-worker` anchor link in `README.md` gets rewritten along with the README.
- No code, API, or dependency changes.
