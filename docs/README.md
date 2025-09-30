# Shortas Documentation

This directory contains the documentation for the Shortas project, built with Jekyll and styled with a Vector.dev inspired theme.

## 🎨 Vector.dev Inspired Theme

The documentation uses a custom theme inspired by [Vector.dev](https://vector.dev), featuring:

- **Teal Color Scheme**: Vector's signature `#00d4aa` primary color
- **Modern Cards**: Enhanced cards with hover effects and accent bars
- **Gradient Buttons**: Animated buttons with shimmer effects
- **Professional Typography**: Clean, modern font hierarchy with Inter font
- **Smooth Animations**: Vector-style transitions and hover states
- **Dark Theme**: Deep background with high contrast text
- **Responsive Design**: Works perfectly on all devices

## 🚀 Local Development

### Prerequisites

- Ruby 3.1+
- Bundler
- Jekyll 4.3+

### Setup

```bash
# Install dependencies
bundle install

# Serve locally
bundle exec jekyll serve

# Or use the custom script
./serve.sh
```

The documentation will be available at `http://localhost:4000`.

## 📁 Structure

```
docs/
├── _config.yml          # Jekyll configuration
├── _layouts/            # Custom layouts
│   └── vector-theme.html # Main Vector.dev inspired layout
├── index.md             # Homepage
├── getting-started/     # Getting started guides
├── architecture/        # Architecture documentation
├── api/                 # API reference
├── deployment/          # Deployment guides
├── development/         # Development guides
└── about.md            # About page
```

## 🎯 Features

- **GitHub Pages Compatible**: Ready for automatic deployment
- **Vector.dev Styling**: Professional, modern design inspired by Vector.dev
- **Responsive**: Works on all devices
- **Fast Loading**: Optimized for performance
- **SEO Friendly**: Proper meta tags and sitemap
- **Search Engine**: Built-in search functionality

## 🚀 Deployment

The documentation is automatically deployed to GitHub Pages when changes are pushed to the main branch.

### Manual Deployment

```bash
# Build the site
bundle exec jekyll build

# Serve locally
bundle exec jekyll serve
```

## 📝 Contributing

To contribute to the documentation:

1. Make your changes in the `docs/` directory
2. Test locally with `bundle exec jekyll serve`
3. Commit and push your changes
4. The documentation will be automatically updated

## 🎨 Customization

The Vector.dev inspired theme can be customized by modifying:

- `_layouts/vector-theme.html` - Main layout and CSS
- `_config.yml` - Site configuration and theme settings
- Individual page files for content

## 📄 License

This documentation is part of the Shortas project and is licensed under the MIT License.
