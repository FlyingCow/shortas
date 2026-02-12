# Route Icon Worker – Solution Proposal

## Goal

Extract the favicon/icon from a route’s **destination URL** when the route is created or updated, store the image in **MinIO**, and show it in the **dashboard** (e.g. routes sidebar and list).

- **Worker**: Rust, triggered by RabbitMQ.
- **Reuse**: Reuse the existing route-related RabbitMQ events where possible; extend or add messages only as needed.
- **Storage**: MinIO (already in `redirect/docker-compose.yml`).
- **UI**: Dashboard and routes sidebar display the icon; fallback when no icon is available.

---

## 1. Events and messages

**Current state**

- **click-router-api** publishes `RouteChangedMessage { switch, link, action }` to `cache.invalidation.routes` on route create/update/delete (used for cache invalidation by click-router).
- The message does **not** include `dest` or `route_id`, which the icon worker needs.

**Options**

**Option A – Extend the existing message (recommended)**

- Add optional fields to `RouteChangedMessage` (so existing consumers stay valid):
  - `dest: Option<String>` – destination URL to scrape.
  - `route_id: Option<String>` – from `route.properties.route_id`, used as MinIO object key and for API lookups.
- When **action** is `Created` or `Updated` and `dest` is present and non-empty, the icon worker runs (fetch page → extract icon → upload to MinIO).
- When **action** is `Deleted`, the worker can delete the object from MinIO (optional).

**Option B – Separate exchange + message**

- New exchange, e.g. `route.icon.request`.
- New message, e.g. `RouteIconRequest { switch, link, dest, route_id }`.
- click-router-api publishes to this exchange in addition to `cache.invalidation.routes` on create/update.
- Worker only consumes from `route.icon.request`.

**Recommendation:** **Option A** – one message shape, one exchange. Existing cache-invalidation consumers ignore the new fields. Worker only processes when `dest` and `route_id` are present and action is Created/Updated (and optionally on Deleted).

---

## 2. Rust worker (new crate)

**Placement**

- New crate under `redirect/`, e.g. **`route-icon-worker`** (or `icon-worker`).
- Add to workspace in `redirect/Cargo.toml`.

**Responsibilities**

1. **Consume** from the same exchange `cache.invalidation.routes` (or from a dedicated queue bound to it), same as click-router’s cache invalidation, but with its own queue name so messages are not shared.
2. **Filter** messages: only handle `Created` / `Updated` with non-empty `dest` and `route_id`; optionally handle `Deleted`.
3. **Fetch** `dest` URL (HTTP GET), parse HTML, resolve favicon:
   - `<link rel="icon">`, `<link rel="shortcut icon">`, `<link rel="apple-touch-icon">`, etc.
   - Resolve relative URLs against the destination origin.
   - Fallback: `https://<origin>/favicon.ico`.
4. **Download** the icon (follow redirects, limit size, e.g. 1–2 MB).
5. **Normalize** (optional): resize to a fixed size (e.g. 32×32 or 64×64), convert to PNG if desired.
6. **Upload** to MinIO:
   - Bucket: e.g. `route-icons` (create in MinIO setup).
   - Object key: e.g. `{route_id}` or `{route_id}.png` (one icon per route_id).
7. **On Delete** (optional): delete object `{route_id}` from MinIO.

**Dependencies (Rust)**

- `lapin` – RabbitMQ consumer (already in workspace).
- `reqwest` – HTTP client (already in workspace).
- `scraper` or `select` – HTML parsing for `<link rel="icon">`.
- `url` – URL resolution.
- MinIO/S3: `aws-sdk-s3` with custom endpoint (MinIO is S3-compatible), or `minio-rs` / `rust-s3` if you prefer. Workspace already has `aws-config` and `aws-sdk-*`; using the same SDK with `endpoint_url` for MinIO is consistent.
- Optional: `image` (workspace) for resize/convert.

**Configuration**

- RabbitMQ: URI, exchange name (same as today), queue name (e.g. `route-icon-worker`).
- MinIO: endpoint (e.g. `http://minio:9000`), bucket (`route-icons`), access/secret (from env).
- Optional: timeouts, max body size, icon dimensions.

**Deployment**

- New binary in the same repo; run as a separate process/container (e.g. in docker-compose alongside click-router, click-tracker).

---

## 3. MinIO

**Current**

- MinIO is already defined in `redirect/docker-compose.yml` (ports 9002:9000, 9001:9001).
- `minio-setup` creates the `clickhouse` bucket.

**Changes**

- In the same `minio-setup` (or a separate init container), create a bucket, e.g. **`route-icons`**.
- Optionally set a bucket policy so that objects are readable by the dashboard (if the dashboard will hit MinIO directly) or leave private and serve via API (see below).

---

## 4. How the dashboard gets the icon

**Option 1 – API serves icon URL**

- **Store icon URL on the route:** After the worker uploads to MinIO, it could call click-router-api (or write to the same DB) to set e.g. `properties.icon_url = "https://..."`. That requires the worker to know the public URL of the object (e.g. MinIO public URL or a gateway) and an API to update a single property. More moving parts.
- **Or derive URL by convention:** No stored field. API and frontend agree that the icon for route_id `R` is at a fixed path.

**Option 2 – API endpoint that proxies or redirects to MinIO (recommended)**

- New endpoint in **click-router-api** (or a small gateway):  
  **`GET /api/v1/routes/{route_id}/icon`**
- Handler:
  - If MinIO has an object for `route_id`, respond with **302 Redirect** to a pre-signed MinIO URL, or **stream the object** from MinIO (same API server).
  - If no object (worker not run yet or failed), return **204 No Content** or **404**.
- Dashboard always uses: `GET /api/v1/routes/{route_id}/icon` (or the same path under your existing API prefix). No need to store `icon_url` on the route; only `route_id` is required (which you already have).

**Option 3 – Direct MinIO URL**

- MinIO bucket `route-icons` is public-read; object key = `{route_id}`.
- Dashboard uses e.g. `https://minio.example.com/route-icons/{route_id}`. Simpler, but exposes MinIO and requires public bucket. Can be combined with Option 2 later (API redirects to this URL).

**Recommendation:** **Option 2** – `GET /routes/{route_id}/icon` in click-router-api that returns the image from MinIO (stream or redirect). No schema change for the route. Dashboard uses this URL for `route.properties.route_id` (or equivalent id).

---

## 5. Dashboard and routes list/sidebar

**Data**

- **RouteDto** already has `properties.route_id`. No new field is strictly required if the icon URL is always derived as `/api/.../routes/{route_id}/icon`.
- Optionally add **`iconUrl`** (or `icon_url`) to the API response for convenience (e.g. pre-signed or public URL), so the frontend does not need to build the URL. Then the backend would set it when building the DTO (e.g. if MinIO has an object for this route_id, set `iconUrl`, else omit or null).

**UI**

- **Routes sidebar** (e.g. `RoutesWithSidebar.tsx`): For each route in the list, show a small image (e.g. 20×20 or 24×24) next to the route link:
  - `src={route.iconUrl ?? `/api/v1/routes/${route.properties?.routeId}/icon`}` with fallback to a default icon or placeholder when the request 404s.
- **Routes list** (same or other views): Same pattern – icon next to the route when available.
- Use `<img>` with `onError` to fall back to a default icon so broken/missing icons do not leave a broken image.

---

## 6. End-to-end flow

1. User creates or updates a route in the dashboard (destination URL set).
2. **click-router-api** persists the route and publishes **RouteChangedMessage** to `cache.invalidation.routes` with `switch`, `link`, `action`, and (new) `dest`, `route_id`.
3. **click-router** (existing) consumes the same exchange for cache invalidation; it ignores the new fields.
4. **route-icon-worker** (new) consumes from the same exchange (its own queue):
   - On **Created/Updated** with `dest` and `route_id`: fetch `dest` → parse HTML → resolve favicon URL → download → optional resize → upload to MinIO `route-icons/{route_id}`.
   - On **Deleted** (optional): delete `route-icons/{route_id}`.
5. **click-router-api** exposes **GET /routes/{route_id}/icon** → stream or redirect to MinIO object for `route_id`.
6. **Dashboard** uses that URL (or an optional `iconUrl` from the route DTO) and shows the icon in the routes sidebar and list, with a fallback when there is no icon.

---

## 7. Implementation checklist

| # | Item | Owner / place |
|---|------|----------------|
| 1 | Extend `RouteChangedMessage` with `dest`, `route_id` (optional) in click-router-api + click-router (messages.rs) | redirect/click-router-api, redirect/click-router |
| 2 | Publish `dest` and `route_id` when creating/updating routes (and optionally on delete) | click-router-api routes_controller |
| 3 | Add MinIO bucket `route-icons` in docker-compose minio-setup | redirect/docker-compose.yml |
| 4 | New crate `route-icon-worker`: config, RabbitMQ consumer, HTTP fetch, HTML parse, MinIO upload | redirect/route-icon-worker |
| 5 | Add GET /routes/{route_id}/icon in click-router-api (stream or redirect from MinIO) | click-router-api |
| 6 | Dashboard: use route icon in sidebar and list with fallback | ui/dashboard RoutesWithSidebar (and any other route list) |

---

## 8. Worker crate layout (suggested)

```
redirect/route-icon-worker/
├── Cargo.toml
├── config/
│   └── default.toml       # rabbitmq, minio, timeouts
└── src/
    ├── main.rs            # load config, run consumer loop
    ├── config.rs          # settings
    ├── consumer.rs        # lapin consumer, deserialize RouteChangedMessage
    ├── scraper.rs         # fetch URL, parse HTML, resolve favicon URL
    ├── minio.rs           # upload (and delete) object by route_id
    └── error.rs            # error types
```

You can reuse the same RabbitMQ connection pattern as in click-tracker/click-router (reconnect loop, declare exchange, declare queue, bind, consume). The worker does not need to invalidate any cache; it only reacts to route changes and updates MinIO.

---

## 9. Security and robustness

- **Worker**: Validate `dest` scheme (only `http`/`https`), limit response size and timeouts to avoid abuse.
- **MinIO**: Use env-based credentials; in production, use IAM or MinIO policies and keep the bucket private; serve via API (Option 2).
- **API endpoint** `/routes/{route_id}/icon`: Ensure the caller is authorized to see that route (same auth as other route endpoints) so icon access is not a data leak.

This keeps the existing RabbitMQ usage for cache invalidation, adds a small extension to the message and a dedicated Rust worker for icon extraction and MinIO storage, and uses the existing MinIO setup and dashboard route list/sidebar to show the icon.
