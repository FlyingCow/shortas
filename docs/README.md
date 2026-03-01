# Shortas Documentation

This directory contains the Jekyll-based documentation site for Shortas, hosted at [shortas.work](https://shortas.work/). The site uses a professional theme (slate blue palette, Inter typography, top navigation).

## Local Preview

```bash
cd docs
bundle install
bundle exec jekyll serve
```

Then open `http://localhost:4000`.

## Structure

```
docs/
├── _config.yml              Jekyll configuration
├── _layouts/                 HTML layouts
├── index.md                  Home page
├── getting-started/index.md  Setup guide
├── architecture/index.md     System architecture
├── api/index.md              API reference
├── deployment/index.md       Deployment guide
├── development/index.md      Development guide
├── about.md                  About page
├── 404.md                    404 page
├── Gemfile                   Ruby dependencies
└── CNAME                     Custom domain config
```
