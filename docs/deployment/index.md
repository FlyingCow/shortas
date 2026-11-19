---
title: Deployment
permalink: /deployment/
---

# Deployment

Shortas runs as a set of Docker containers. There are four ways to run it, from a laptop to a highly available cloud setup. The same service images are used everywhere; what changes is how many hosts there are and who runs the databases.

| Target | Hosts | Databases | HA | Rough cost | Runbook |
|---|---|---|---|---|---|
| Local | 1 (your machine) | in containers | no | free | [Getting Started](/getting-started/) |
| Hetzner budget | 1 VM | in containers | no | ~€20 / month | [`infra/hetzner-budget`][hb] |
| Hetzner | 4 VMs + 2 load balancers | in containers on a data node | app tier | ~€220–250 / month | [`infra/hetzner`][h] |
| AWS | ECS Fargate | managed (RDS, DynamoDB, ElastiCache, Amazon MQ) | yes (prod) | varies | [`infra/aws/terraform`][aws] |

[hb]: https://github.com/FlyingCow/shortas/tree/main/infra/hetzner-budget
[h]: https://github.com/FlyingCow/shortas/tree/main/infra/hetzner
[aws]: https://github.com/FlyingCow/shortas/tree/main/infra/aws/terraform

## Hetzner budget

Everything runs on one CX43 VM (8 vCPU, 16 GB) with Docker Compose, using prebuilt images from GHCR. Caddy handles TLS for the dashboard and API. Click Router terminates TLS for short-link domains on a second IP. This is good for demos and low traffic. When the VM is down, everything is down.

## Hetzner

The same compose services are split across four hosts on a private network:

- **Two app nodes** behind load balancers run the stateless services.
- **A platform node** runs Keycloak and monitoring.
- **A data node** with an attached volume runs every database.

Workers that must run only once (Click Tracker, Click Aggregator, Cert Bot) run on the first app node only. Moving up from the budget tier means adding nodes, not changing the setup.

## AWS

Terraform provisions a VPC, ECS Fargate services and managed data stores. MongoDB is replaced by DynamoDB and Keycloak by Cognito. The `dev` and `prod` environments differ mainly in redundancy: NAT per AZ, Multi-AZ RDS, Redis replicas, no Spot in prod. Deployment uses three scripts:

```bash
infra/aws/scripts/deploy-infra.sh dev apply
infra/aws/scripts/build-push-images.sh dev
infra/aws/scripts/deploy-services.sh dev
```

## Before going live

These apply to every target:

- **Certificates**: Cert Bot defaults to the Let's Encrypt *staging* directory. Once issuance works, switch to production with `CERTBOT__ACME__DIRECTORY_URL` (or `directory_url` in `redirect/cert-bot/config/production.toml`).
- **Email**: configure real SMTP or SES credentials. MailHog exists only in the local stack.
- **Safe Browsing**: use your own Google Safe Browsing API key.
- **Notification templates**: the deploy syncs `notification-templates/` to object storage. On AWS, `deploy-services.sh` does this whenever `notification-worker` is deployed.
