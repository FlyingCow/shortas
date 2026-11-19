# Shortas

A self-hostable URL shortener with custom domains, conditional routing and real-time click analytics. The backend is Rust and the dashboard is React.

```
 visitor --> Click Router --> Click Tracker --> Click Aggregator --> ClickHouse
                 ^                                                      |
                 | routes                                     analytics |
                 |                                                      v
            Click Router API <---- Management API <---- Dashboard <--- user
```

- Short links on your own domains, with automatic Let's Encrypt certificates
- Conditional routing by country, device, browser, OS or language
- Click analytics streamed into ClickHouse, visible within seconds
- QR codes, Safe Browsing checks, and team workspaces

## Quick start

```bash
git clone --recurse-submodules https://github.com/FlyingCow/shortas.git
cd shortas
cp redirect/route-verifier/.env.example redirect/route-verifier/.env   # set GSB_API_KEY
cp redirect/cert-bot/.env.example redirect/cert-bot/.env
cd infra/custom && docker compose up -d
```

Open <http://localhost:3000> and sign in as `testuser` / `testpassword`.

## Documentation

Full docs are at **[shortas.work](https://shortas.work/)**:

- [Getting Started](https://shortas.work/getting-started/): local stack, sample data, URLs
- [Architecture](https://shortas.work/architecture/): how the pieces fit
- [Deployment](https://shortas.work/deployment/): Hetzner and AWS
- [Development](https://shortas.work/development/): repo layout, build, test
- [API](https://shortas.work/api/): Management API

## License

[MIT](LICENSE)
