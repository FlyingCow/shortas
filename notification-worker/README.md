# notification-worker

Consumes notification events from RabbitMQ, renders the matching Handlebars templates from object storage, and delivers them in-app (a PostgreSQL row the dashboard shows) and by email. It has no HTTP port. It uses `management-api` as a library for repositories, email and storage.

Templates live in [`notification-templates/`](../notification-templates). See [Development → Notification templates](https://shortas.work/development/#notification-templates) for their format.

## Delivery and retries

```
notifications (topic exchange, key = notification id)
   -> notifications queue -> worker --ok--> ack
                               |
                             fail -> notifications.retry (waits retry_delay)
                                        -> back to the exchange
                                        -> after max_retries: notifications.dead
```

Messages in `notifications.dead` need manual inspection.

## Run

```bash
cargo run  -p notification-worker
cargo test -p notification-worker
```

## Config

`config/default.toml` → `config/<APP_RUN_MODE>.toml` → `config/local.toml` → `APP__SECTION__KEY` environment variables (same loader as `management-api`).

| Section | Purpose |
|---|---|
| `database` | PostgreSQL (shared with `management-api`) |
| `minio` | Bucket the templates are read from |
| `rabbitmq` | Queue names, `notifications_retry_delay_ms` (30 s), `notifications_max_retries` (5) |
| `email` | SES or SMTP sender |
