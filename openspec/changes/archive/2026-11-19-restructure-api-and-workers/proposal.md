# Proposal

## Why

Background workers are spread across the repo. `notification-worker` and `notification-templates` sit at the root, while `cert-bot`, `domain-verifier`, `route-icon-worker` and `route-verifier` live inside `redirect/` even though they are not part of the redirect hot path. The management service is called `management-api`, which is longer than it needs to be. Nothing is deployed yet, so this is the cheapest moment to settle the layout and names.

## What Changes

- **BREAKING (naming)**: Rename `management-api/` to `api/`. Rename the crate, binary, Docker image, ECR repository, ECS service, ALB target group, compose service and container name, and Caddy upstream from `management-api` to `api`.
  - **Kept**: the Cognito resource-server identifier and `APP__JWT__AUDIENCE` stay `management-api`. They are an OAuth contract, not a service name.
- Create `workers/` and move into it:
  - `notification-worker/` and `notification-templates/`
  - `redirect/cert-bot/`, `redirect/domain-verifier/`, `redirect/route-icon-worker/` and `redirect/route-verifier/`
- Move the four redirect workers from the `redirect/` Cargo workspace into the root workspace. Root members become `api`, `workers/*` and `redirect/shortas-common`. `redirect/` keeps only the `click-*` crates.
- Moved workers keep their image, service and ECR names. Only their source paths change.
- Update every Dockerfile (all `redirect/*` Dockerfiles `COPY` the full workspace), compose files, the CI build matrix, AWS build/deploy scripts, Terraform and docs to match.
- Delete the stray untracked `api/logs/` directory, which would block the rename.
- Replace the stale `Makefile` (it targets a removed .NET `api.sln`) with targets for both Rust workspaces. Rust CI runs `make build` / `make test`.

## Capabilities

### New Capabilities

None.

### Modified Capabilities

None. This is a pure restructure with no change in runtime behavior. The change sets `skip_specs: true`.

## Impact

- **Code**: `api/` crate (renamed), `workers/notification-worker` (`use management_api::` becomes `use api::`), the template-path test in `template_store.rs`, root and `redirect/` `Cargo.toml`, both `Cargo.lock` files.
- **Build**: all Rust Dockerfiles, `.github/workflows/docker-publish.yml`, `Makefile` (used by `.github/workflows/rust.yml`).
- **Infra**: `redirect/docker-compose.yml`, `api/docker-compose.yml`, `infra/custom`, `infra/hetzner`, `infra/hetzner-budget` (compose, Caddyfiles, env examples), `infra/aws/scripts/*`, `infra/aws/terraform` (ECR, ECS, ALB modules and the three environments).
- **Docs**: `README.md`, `docs/**`, `redirect/readme.md`, service READMEs, `.claude/context.yaml`.
- **Unchanged**: S3 key prefix `notification-templates/`, RabbitMQ exchange and queue names, JWT audience, Cognito identifier.
