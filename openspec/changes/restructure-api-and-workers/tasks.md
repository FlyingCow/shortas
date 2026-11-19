# Tasks

## 1. Moves and workspace

- [x] 1.1 Delete the stray untracked `api/logs/` directory and verify `api/` no longer exists
- [x] 1.2 `git mv management-api api`; create `workers/`; `git mv` `notification-worker`, `notification-templates`, `redirect/cert-bot`, `redirect/domain-verifier`, `redirect/route-icon-worker`, `redirect/route-verifier` into `workers/`; verify with `git status` that all of them show as renames
- [x] 1.3 Rename the package and `[[bin]]` in `api/Cargo.toml` to `api`. Update notification-worker's path dependencies (`../../api`) and `use management_api::` to `use api::` in `workers/notification-worker/src`. Verify `cargo check -p notification-worker` passes
- [x] 1.4 Update root `Cargo.toml` members (`api`, `workers/*` crates, `redirect/shortas-common`) and add the missing `[workspace.dependencies]` from design D1. Verify `cargo check --workspace` passes at the root
- [x] 1.5 Remove the four workers from `redirect/Cargo.toml` members and prune unused workspace deps. Verify `cargo check --workspace --manifest-path redirect/Cargo.toml` passes
- [x] 1.6 Fix the template test path in `api/src/infrastructure/storage/template_store.rs` (`../workers/notification-templates`). Verify `cargo test --workspace` (root) and `cargo test --workspace --manifest-path redirect/Cargo.toml` both pass
- [x] 1.7 Commit the regenerated root and `redirect/` `Cargo.lock` files

## 2. Makefile

- [x] 2.1 Rewrite `Makefile` per design D5: drop the .NET targets, run build/test/check/clippy/fmt/release/clean against both manifests, and route `build-%`/`test-%` to the right workspace. Verify `make build`, `make test` and `make build-cert-bot` succeed

## 3. Docker images

- [x] 3.1 Rewrite `api/Dockerfile` and `workers/notification-worker/Dockerfile` for the new paths and `api` binary. Verify `docker build -f api/Dockerfile .` and `docker build -f workers/notification-worker/Dockerfile .` succeed
- [x] 3.2 Rewrite the four moved worker Dockerfiles to build from the root workspace (design D3). Verify `docker build -f workers/<name>/Dockerfile .` succeeds for each
- [x] 3.3 Remove the moved crates from the `COPY` lists in every `redirect/*/Dockerfile` and `redirect/Dockerfile`. Verify `docker build` succeeds for each `click-*` image
- [x] 3.4 Update `.github/workflows/docker-publish.yml` (service `api`, new Dockerfile paths). Verify every matrix `dockerfile:` path exists on disk

## 4. Compose and Hetzner

- [x] 4.1 Update `redirect/docker-compose.yml` build paths for the moved workers and any `management-api` hostnames. Verify `docker compose -f redirect/docker-compose.yml config` succeeds
- [x] 4.2 Update `api/docker-compose.yml` and `infra/custom/docker-compose.yml` (include path, service names). Verify `docker compose -f infra/custom/docker-compose.yml config` succeeds
- [x] 4.3 Update `infra/hetzner` (compose app/data/platform, `Caddyfile.app`, `.env.*.example`, `deploy.sh`, README): service and image `api`, container `shortas-api`, templates mount `../../workers/notification-templates`. Verify `docker compose ... config` succeeds for each compose file
- [x] 4.4 Update `infra/hetzner-budget` (compose, Caddyfile, `.env.example`, postgres-init comment, README) the same way. Verify `docker compose -f infra/hetzner-budget/docker-compose.yml config` succeeds

## 5. AWS

- [x] 5.1 Update `infra/aws/scripts/build-push-images.sh` and `deploy-services.sh` (service `api`, new Dockerfile paths). Verify `bash -n` passes on both and each referenced Dockerfile exists
- [x] 5.2 Rename `management-api` to `api` in the Terraform ECR/ECS/ALB modules and the three environments (target group resource and output `api`, ECS service name). Keep the Cognito identifier and `APP__JWT__AUDIENCE` as `management-api`, each with a comment explaining why (design D2). Verify `terraform fmt -check` and `terraform validate` pass in each environment
- [x] 5.3 Update `infra/aws/terraform/README.md` and output descriptions. Verify they contain no remaining `management-api` service references

## 6. Docs and references

- [x] 6.1 Update `README.md`, `docs/**`, `redirect/readme.md`, `api/README.md`, the worker READMEs, `.claude/context.yaml`, code comments, and `infra/custom/seed-dashboard-data.py` / `ui/dashboard/src/services/api.ts` comments for the new paths and the `api` name. Verify the docs render (Jekyll links point at existing paths)

## 7. Final verification

- [x] 7.1 Repo-wide grep (excluding `target/`, `node_modules/`, `openspec/changes/archive/`) for `management-api`, `management_api`, `redirect/(cert-bot|domain-verifier|route-icon-worker|route-verifier)`, and root-level `notification-worker/` or `notification-templates/` paths. Verify the only hits are the design D2 exceptions and the S3 `notification-templates/` prefix
- [x] 7.2 Bring up the local stack (`infra/custom`) and verify `api`, `notification-worker` and the four moved workers start and pass health checks

## Workflow follow-up

- Archive the change once implementation is merged.
