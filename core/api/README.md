# Redirect Core: API

Redirect management api, used to create/update/delete links and user setttings.

# How it works

# Installing

To install needed components:
```
make install
```

To start local environment:
```
make deploy_local
```

```bash
cargo build
```

# Database setup

# Running

define the environment on which we're running by adding `ENV=<env>`, which will use the `.env.<env>` file

```bash
ENV=dev cargo run
```

# Code quality & security

Used in CI/CD

```bash
cargo fmt --all -- --check
cargo clippy --all-targets
cargo audit
cargo outdated
```

# Testing

```bash
ENV=test cargo test
```

# API Documentation

TODO: https://github.com/paperclip-rs/paperclip