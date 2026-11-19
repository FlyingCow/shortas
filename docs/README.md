# Docs site

Source for [shortas.work](https://shortas.work/). It is a Jekyll site that GitHub Pages serves from this directory.

## Preview locally

```bash
cd docs
bundle install
bundle exec jekyll serve
```

Then open <http://localhost:4000>.

## Layout

- One page per directory (`architecture/index.md` → `/architecture/`).
- `_layouts/professional-theme.html` is the only layout.
- Top navigation comes from `navigation` in `_config.yml`.
