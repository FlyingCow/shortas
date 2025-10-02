# Shortas Landing Page

A modern, responsive React landing page for the Shortas URL shortening service.

## Features

- 🚀 **Modern Design**: Clean, professional design with gradient backgrounds and smooth animations
- 📱 **Fully Responsive**: Optimized for desktop, tablet, and mobile devices
- ⚡ **Fast Performance**: Built with React and optimized for speed
- 🎨 **Beautiful UI**: Modern CSS with hover effects and smooth transitions
- 📊 **Interactive Elements**: Engaging components and call-to-action sections

## Components

- **Header**: Navigation with mobile menu support
- **Hero**: Eye-catching hero section with URL shortening demo
- **Features**: Showcase of key platform features
- **How It Works**: Step-by-step process explanation
- **Stats**: Trust indicators and user statistics
- **Footer**: Complete footer with links and social media

## Getting Started

### Prerequisites

- Node.js (v14 or higher)
- npm or yarn

### Installation

1. Navigate to the project directory:
   ```bash
   cd shortas-landing
   ```

2. Install dependencies:
   ```bash
   npm install
   ```

3. Start the development server:
   ```bash
   npm start
   ```

4. Open [http://localhost:3000](http://localhost:3000) to view it in the browser.

### Building for Production

```bash
npm run build
```

This builds the app for production to the `build` folder.

## Deployment

The built files can be deployed to any static hosting service:

- **Netlify**: Drag and drop the `build` folder
- **Vercel**: Connect your GitHub repository
- **AWS S3**: Upload the `build` folder contents
- **GitHub Pages**: Use the `gh-pages` package

## Customization

### Colors

The main brand colors are defined in CSS custom properties:
- Primary: `#667eea` to `#764ba2` (gradient)
- Secondary: `#ffd89b` to `#19547b` (gradient)

### Content

Update the content in each component file:
- `src/components/Hero.tsx` - Main headline and description
- `src/components/Features.tsx` - Feature list and descriptions
- `src/components/Stats.tsx` - Statistics and numbers

### Styling

Each component has its own CSS file for easy customization:
- Global styles: `src/App.css`
- Component styles: `src/components/[Component].css`

## Performance

- Optimized images and assets
- Minimal bundle size
- Lazy loading ready
- SEO-friendly structure

## Browser Support

- Chrome (latest)
- Firefox (latest)
- Safari (latest)
- Edge (latest)

## License

MIT License - see LICENSE file for details.