# Keycloak Login Page Layout

This document describes the custom Keycloak login page layout component that matches the Shortas dashboard design.

## Overview

The `KeycloakLogin` component provides a custom login page layout that can be used with Keycloak's theme customization features. It maintains the same visual design as the logged-out page but is specifically tailored for the login flow.

## Components

### KeycloakLogin.tsx
- **Purpose**: Custom login page layout component
- **Props**: 
  - `onLogin`: Function to handle login action
  - `onGoToLanding`: Function to handle landing page navigation
- **Features**:
  - Responsive design (desktop/tablet/mobile)
  - Custom Shortas branding
  - Security-focused messaging
  - Professional styling

### KeycloakLogin.css
- **Purpose**: Styling for the Keycloak login page
- **Features**:
  - Gradient background matching dashboard theme
  - Glass morphism effects
  - Responsive breakpoints
  - Professional button styling
  - Security badges

## Design Features

### Visual Elements
- **Logo**: Custom Shortas polygon logo (64px in branding, 40px in form)
- **Background**: Purple-to-blue gradient matching dashboard theme
- **Typography**: Professional, security-focused messaging
- **Icons**: Lucide React icons for visual consistency

### Layout Structure
```
┌─────────────────────────────────────────────────────────┐
│                    Full Height Container                │
├─────────────────────┬───────────────────────────────────┤
│   Left Side         │        Right Side                │
│   (Branding)        │        (Login Form)              │
│                     │                                   │
│   • Logo            │   • Sign In Form                 │
│   • Title           │   • Keycloak Button              │
│   • Description     │   • Landing Page Button          │
│   • Features        │   • Security Badges              │
│                     │                                   │
└─────────────────────┴───────────────────────────────────┘
```

### Responsive Behavior
- **Desktop (lg+)**: Two-column layout with branding and form
- **Tablet/Mobile**: Single column with form only
- **Branding**: Hidden on smaller screens, shown on desktop

## Usage

### Basic Implementation
```tsx
import KeycloakLogin from './components/KeycloakLogin';

const handleLogin = () => {
  // Redirect to Keycloak login
  window.location.href = '/auth/realms/shortas-dev/protocol/openid-connect/auth';
};

const handleGoToLanding = () => {
  // Navigate to landing page
  window.location.href = 'https://shortas.com';
};

<KeycloakLogin 
  onLogin={handleLogin}
  onGoToLanding={handleGoToLanding}
/>
```

### With Keycloak Integration
```tsx
import keycloak, { keycloakLoginOptions } from '../config/keycloak';

const handleKeycloakLogin = async () => {
  try {
    await keycloak.login(keycloakLoginOptions);
  } catch (error) {
    console.error('Login failed:', error);
  }
};

<KeycloakLogin 
  onLogin={handleKeycloakLogin}
  onGoToLanding={handleGoToLanding}
/>
```

## Keycloak Theme Integration

### Custom Theme Setup
To use this layout as a Keycloak theme:

1. **Create Theme Directory**:
   ```
   keycloak/themes/shortas/
   ├── login/
   │   ├── theme.properties
   │   ├── resources/
   │   │   ├── css/
   │   │   │   └── login.css
   │   │   └── js/
   │   │       └── login.js
   │   └── login.ftl
   ```

2. **Theme Properties**:
   ```properties
   # theme.properties
   parent=keycloak
   import=common/keycloak
   ```

3. **Login Template**:
   Use the HTML structure from `KeycloakLogin.tsx` in `login.ftl`

4. **Styling**:
   Copy CSS from `KeycloakLogin.css` to `resources/css/login.css`

### Theme Configuration
```properties
# Enable custom theme
keycloak.theme.shortas.enabled=true
keycloak.theme.shortas.name=Shortas
keycloak.theme.shortas.parent=keycloak
```

## Customization

### Colors
```css
/* Primary gradient */
background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);

/* Button colors */
.keycloak-login-button-primary {
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
}
```

### Logo
```tsx
<Logo size={64} className="mb-3" color="white" />
```

### Text Content
- **Title**: "SHORTAS"
- **Subtitle**: "Welcome to Shortas Dashboard"
- **Description**: Customizable messaging
- **Features**: Security-focused feature list

## Security Features

### Visual Indicators
- **Shield Icon**: Security authentication
- **SSO Badge**: Single Sign-On indicator
- **Secure Badge**: Security assurance

### Messaging
- **Authentication**: "Protected by Keycloak OpenID Connect"
- **SSO**: "Seamless authentication experience"
- **Enterprise**: "Advanced security features and compliance"

## Browser Support

- **Modern Browsers**: Full support with all features
- **CSS Features**: 
  - CSS Grid and Flexbox
  - CSS Custom Properties
  - Backdrop filters
  - Gradients and shadows

## Performance

- **Bundle Size**: Minimal impact on application size
- **CSS**: Optimized for performance
- **Images**: SVG-based logo for scalability
- **Responsive**: Mobile-first design approach

## Accessibility

- **Semantic HTML**: Proper heading structure
- **ARIA Labels**: Screen reader support
- **Keyboard Navigation**: Full keyboard accessibility
- **Color Contrast**: WCAG compliant color ratios
- **Focus Management**: Clear focus indicators

## Integration Examples

### React Router
```tsx
<Route path="/login" element={<KeycloakLogin onLogin={handleLogin} onGoToLanding={handleLanding} />} />
```

### Next.js
```tsx
export default function LoginPage() {
  return (
    <KeycloakLogin 
      onLogin={() => router.push('/auth/keycloak')}
      onGoToLanding={() => router.push('/')}
    />
  );
}
```

### Standalone HTML
```html
<!DOCTYPE html>
<html>
<head>
  <link rel="stylesheet" href="keycloak-login.css">
</head>
<body>
  <div id="keycloak-login-root"></div>
  <script src="keycloak-login.js"></script>
</body>
</html>
```

## Troubleshooting

### Common Issues
1. **Logo not displaying**: Check Logo component import
2. **Styling issues**: Verify CSS file is loaded
3. **Responsive problems**: Check viewport meta tag
4. **Button not working**: Verify event handlers

### Debug Steps
1. Check browser console for errors
2. Verify component props are passed correctly
3. Test responsive breakpoints
4. Validate CSS is loading properly

## Future Enhancements

- **Multi-language support**: i18n integration
- **Dark mode**: Theme switching capability
- **Animation**: Enhanced transitions and effects
- **Accessibility**: Additional ARIA improvements
- **Performance**: Further optimization opportunities

