# Tasks

Each rewritten file follows the writing rules in design.md (Decision 3) and is verified against the repo (Decision 7). "Verify" means checking every command, port and path in the file against the Makefile, compose files, or source.

## 1. Site scaffolding

- [x] 1.1 Clean up `docs/_config.yml` per design Decision 6 (set url to shortas.work, remove the placeholder email/twitter, collections, collection defaults and jekyll-feed). Verify that `bundle exec jekyll build` in `docs/` succeeds with no warnings about missing collections.
- [x] 1.2 Delete `docs/_layouts/vector-theme.html`. Verify with `grep -r vector-theme docs/` that nothing references it.
- [x] 1.3 Rewrite `docs/README.md` as a ≤20-line "how to preview the site" note. Verify the documented `bundle exec jekyll serve` command serves the site.

## 2. Site pages (rewrite from scratch, URLs unchanged)

- [x] 2.1 Rewrite `/architecture/` (`docs/architecture/index.md`, ~900 words): component table, click-flow diagram, management plane, data stores, custom domains & TLS, tech stack. Make sure it has the anchors the README links to. Verify the component list against `Cargo.toml` workspaces and `redirect/docker-compose.yml`.
- [x] 2.2 Rewrite `/` (`docs/index.md`, ~300 words): pitch, capabilities, one simplified diagram, page links. Verify it introduces no diagram or table that duplicates `/architecture/`.
- [x] 2.3 Rewrite `/getting-started/` (~400 words): prerequisites, clone with submodules, `infra/custom` compose up, seed script, login, port table. Verify the ports against `infra/custom/docker-compose.yml` and the included compose files, and the seed flags against `seed-dashboard-data.py --help`.
- [x] 2.4 Rewrite `/deployment/` (~600 words): a comparison table of local / Hetzner budget / Hetzner multi-node / AWS (cost, HA, effort), a short section per target linking to its `infra/*/README.md`, and notification-template sync. Verify that every linked runbook path exists.
- [x] 2.5 Rewrite `/development/` (~500 words): repo layout tree, build/test/lint for the `redirect/` workspace (Makefile) and the root workspace (cargo), dashboard dev, adding a notification template. Verify every make target with `make -n <target>` and every cargo `-p` name against `Cargo.toml`.
- [x] 2.6 Rewrite `/api/` (~300 words): Management API base URL, auth, a table of endpoint groups, and one line on the internal Click Router/Aggregator APIs. Verify the groups against the router definitions in `management-api/src`.
- [x] 2.7 Rewrite `/about/` (~150 words) and check `404.md`. Verify there are no history or legacy mentions.
- [x] 2.8 Check the site as a whole: run `bundle exec jekyll serve`, open every nav link and every inbound URL (`/`, `/architecture/`, `/getting-started/`, `/development/`, `/deployment/`, `/api/`, `/about/`), and confirm the total word count across `docs/**/*.md` is about 3k (`wc -w`).

## 3. Root README

- [x] 3.1 Rewrite `README.md` (~200–250 words): one-paragraph pitch, simplified diagram, 3–5 line quickstart, links to the site pages, license. Verify it contains no port table, tech-stack table, project tree, or `api/` mention, and that every link resolves.

## 4. Component READMEs (template from design Decision 4)

- [x] 4.1 Rewrite `redirect/readme.md` as the redirect-workspace entry point (what's in it, make targets, link to `/architecture/`). Verify the targets with `make -n`.
- [x] 4.2 Rewrite `redirect/click-router`, `click-router-api`, `click-tracker`, `click-aggregator-api`, and `route-verifier` READMEs to the template (10–40 lines each). Verify each config table against that service's config files and source.
- [x] 4.3 Trim `redirect/monitoring/README.md` to: what's included, URLs/ports, how to query logs. Verify the ports against the monitoring compose file.
- [x] 4.4 Add short READMEs for `management-api/` and `notification-worker/` (they have none, and they are the homes for their own config per design Decision 2). Verify against their `config/` directories.
- [x] 4.5 Rewrite `ui/dashboard/README.md` and `ui/landing/README.md` to the template. Verify the npm scripts against `package.json`.
- [x] 4.6 Replace `api/README.md` with a 3-line "superseded by `management-api/`" note. Verify with `grep -ri "api/" README.md docs/` that no legacy mention remains outside it.

## 5. Infra runbooks (trim, keep every step)

- [x] 5.1 Trim `infra/hetzner-budget/README.md` to prerequisites → steps → verify → operate, with rationale cut to one line per decision. Verify that every command block from the old version is present or explicitly obsolete (diff the extracted code blocks before and after).
- [x] 5.2 Trim `infra/hetzner/README.md` the same way. Verify with the same before/after code-block diff.
- [x] 5.3 Trim `infra/aws/terraform/README.md` and `infra/aws/terraform/modules/cognito/README.md` the same way. Verify with the same before/after code-block diff, and check that the script names exist under `infra/aws/terraform/scripts/`.

## 6. Final consistency check

- [x] 6.1 Duplication check: confirm that each fact in design Decision 2 appears in only its home file (grep for distinctive strings such as port numbers `5810`/`5820` and `├── management-api`). Confirm there are no broken relative links in the READMEs (check every `](...)` target exists).

## Workflow follow-up

- Preview, merge to main, and spot-check the shortas.work URLs linked from the landing page.
- Archive the change after merge.
