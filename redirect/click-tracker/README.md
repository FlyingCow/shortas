# click-tracker

Consumes raw clicks from `hit-stream-main` and enriches each one with geolocation (MaxMind), parsed user agent, and a Redis-backed session. It publishes the result to `click-aggs-main`. See [Architecture → Click path](https://shortas.work/architecture/#click-path).

## Run

```bash
make build-click-tracker
make test-click-tracker
```

It has no HTTP port.

## Config

| Section | Purpose |
|---|---|
| `fluvio.hit_stream`, `fluvio.click_aggs` | Input and output topics (`kafka.*` is an alternative) |
| `pipeline` | Parallelism |
| `geo_ip` | MaxMind database path |
| `uaparser` | Regex file from the `redirect/data/ua-parser` submodule |
| `redis` | Session store |
