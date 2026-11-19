# monitoring

Prometheus, Loki and Grafana for the local stack. `redirect/docker-compose.yml` includes this compose file, so the stack starts with everything else.

| Component | URL | Notes |
|---|---|---|
| Grafana | <http://localhost:3001> | `admin` / `admin`; Prometheus and Loki datasources and dashboards are provisioned |
| Prometheus | <http://localhost:9091> | Scrapes `click-router` (:9092) and `click-aggregator` (:9090); see `prometheus/prometheus.yml` |
| Loki | <http://localhost:3100> | Stores chunks in MinIO (`loki` bucket); see `loki/config.yml` |

## Logs

The `redirect/` services ship only warnings and errors to Loki (`RUST_LOG=warn`, `LOKI_URL`), labelled by `service` and `level`. A service shows up in Loki only after its first warning. To query, open Grafana → Explore → Loki:

```logql
{service="click-router"}
{service=~".+"} |= "ERROR"
{service=~".+"} |~ "(?i)timeout|connection"
```

To list the services Loki has seen so far:

```bash
curl http://localhost:3100/loki/api/v1/label/service/values
```
