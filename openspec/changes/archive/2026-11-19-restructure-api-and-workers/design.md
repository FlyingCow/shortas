# Design

## Context

See proposal.md (Why) for the motivation. The repo has two Cargo workspaces:

- **Root** `Cargo.toml`: `management-api`, `notification-worker`, `redirect/shortas-common`.
- **`redirect/Cargo.toml`**: the five `click-*` crates plus `cert-bot`, `domain-verifier`, `route-icon-worker`, `route-verifier`.

The four redirect workers take every dependency version from `redirect/Cargo.toml`'s `[workspace.dependencies]`. None of them depend on `shortas-common`. Every `redirect/*` Dockerfile `COPY`s all nine workspace members, because Cargo needs every member present to resolve the workspace.

`notification-worker` depends on `management-api` as a library (`use management_api::...`). Its Dockerfile copies the root workspace.

Nothing is deployed yet (no AWS or Hetzner environment is live).

## Goals / Non-Goals

**Goals:**
- One home for background workers (`workers/`) and one short name for the management service (`api`).
- `cargo build` / `cargo test` stay green in both workspaces; every Docker image builds from the repo root context.
- No leftover reference to the old paths, or to the `management-api` service name, outside the deliberate exceptions below.

**Non-Goals:**
- Renaming the moved workers' images, services or ECR repositories.
- Changing the S3 `notification-templates/` key prefix or RabbitMQ names.
- Changing the OAuth contract (Cognito resource-server identifier, JWT audience).
- Merging the `click-*` crates into the root workspace.

## Decisions

### D1. Moved redirect workers join the root workspace

Root `members` become `api`, `workers/notification-worker`, `workers/cert-bot`, `workers/domain-verifier`, `workers/route-icon-worker`, `workers/route-verifier`, `redirect/shortas-common`.

There are no version conflicts. The root `[workspace.dependencies]` needs these additions from `redirect/Cargo.toml`: `aws-sdk-dynamodb`, `dyn-clone`, `env_logger`, `hickory-resolver`, `instant-acme` (with its feature set), `mongodb`, `rcgen`, `rustls`, `scraper`, `serde_derive`, `tokio-graceful-shutdown`, `tracing-loki`, `urlencoding`, `x509-parser`. `salvo` already matches on version (0.84.0); root's feature set is a superset, so the workers just get extra features. Drop entries from `redirect/Cargo.toml` that no remaining member uses.

*Alternatives:*
- A third `workers/` workspace would mean three lockfiles. `notification-worker` would also need either a path dependency into another workspace or to stay at the root.
- Keeping the workers in the `redirect` workspace through `../workers/*` members would put members outside the workspace root, which Cargo handles poorly.

### D2. Full rename of `management-api` to `api`, except the OAuth contract

- Package `name` and `[[bin]]` name become `api`. The library is referenced as `api::` (`use management_api::` becomes `use api::` in notification-worker).
- Image, ECR repo, ECS service and task family, ALB target group (`${env}-api`, Terraform resource `aws_lb_target_group.api`, output `api_target_group_arn`), compose service `api`, container `shortas-api`, Caddy `reverse_proxy api:8080`.
- **Kept as `management-api`**: Cognito `aws_cognito_resource_server` identifier and name, and `APP__JWT__AUDIENCE`. Changing them would replace the resource server, change scope strings and invalidate tokens, and it gains nothing. Each kept occurrence gets a one-line comment saying it is an OAuth identifier that was left alone on purpose.

*Alternative:* renaming folders only and keeping the service name. The user rejected this in favor of a consistent name.

### D3. Dockerfiles follow the new workspace membership

- **Root-workspace images** (`api`, `notification-worker`, the four moved workers) copy `Cargo.toml`, `Cargo.lock`, `api/`, `workers/` and `redirect/shortas-common/`, then build with `-p <crate>` from `/app`. Binaries come from `/app/target/release/`. Config is copied from `workers/<name>/config/`.
- **Redirect-workspace images** (`click-*`, `redirect/Dockerfile`) stop copying the four moved crates.
- Dockerfiles move with their crates (`workers/<name>/Dockerfile`, `api/Dockerfile`). The build context stays the repo root.

### D4. Templates mount and test path

- `workers/notification-templates/` is mounted at `/templates` by the `notification-templates-sync` compose services. The relative path changes from `../../notification-templates` to `../../workers/notification-templates`.
- The `template_store.rs` test path becomes `{CARGO_MANIFEST_DIR}/../workers/notification-templates/...`.

### D5. Makefile drives both workspaces

Replace the .NET targets. `ROOT_MANIFEST := Cargo.toml` and `REDIRECT_MANIFEST := redirect/Cargo.toml`. `build`, `test`, `check`, `clippy`, `fmt`, `release` and `clean` run against both. `build-%` / `test-%` pick the workspace that contains the package (for example, try root and fall back to redirect, or use an explicit list). Keep the UI and compose targets. This matters because `.github/workflows/rust.yml` runs `make build` / `make test`.

### D6. Moves use `git mv`

`git mv` keeps history readable (`git log --follow`). Delete the stray untracked `api/logs/` before `git mv management-api api`.

## Risks / Trade-offs

- **Missed reference to an old path or name** → finish with a repo-wide grep for `management-api`, `management_api`, `notification-worker/`, `notification-templates` (path uses) and `redirect/(cert-bot|domain-verifier|route-icon-worker|route-verifier)`. The only allowed hits are the D2 exceptions, the S3 prefix and archived OpenSpec changes.
- **Root `Cargo.lock` churn**: adding the workers pulls in mongodb, rustls and others → expected. Regenerate both lockfiles with `cargo check` and commit them.
- **Root workspace builds get slower** (all crates in one `cargo build`) → acceptable, and per-crate `-p` builds in Docker are unaffected.
- **Terraform renames recreate resources** (ECR, ECS service, target group) → no impact while nothing is deployed. `terraform validate` per environment is enough.
- **Crate name `api` is generic** → it is only used as a library by notification-worker, so name collisions are unlikely.

## Migration Plan

A single cutover, since nothing is live: one change lands the moves, renames and config updates together. Rollback is reverting the commit.
