# Keycloak Theme Integration Guide

This guide explains how to integrate the custom Shortas login page layout into Keycloak as a custom theme.

## Overview

We'll create a custom Keycloak theme that uses the Shortas login page design. This involves creating theme files, configuring Keycloak, and deploying the theme.

## Step 1: Create Keycloak Theme Directory Structure

Create the following directory structure in your Keycloak installation:

```
keycloak/themes/shortas/
├── login/
│   ├── theme.properties
│   ├── login.ftl
│   ├── resources/
│   │   ├── css/
│   │   │   ├── login.css
│   │   │   └── common.css
│   │   ├── js/
│   │   │   └── login.js
│   │   └── img/
│   │       └── logo.svg
│   └── messages/
│       └── messages_en.properties
├── account/
│   ├── theme.properties
│   └── account.ftl
└── admin/
    ├── theme.properties
    └── admin.ftl
```

## Step 2: Theme Configuration Files

### theme.properties (login/)
```properties
# Keycloak theme configuration
parent=keycloak
import=common/keycloak

# Theme metadata
name=Shortas
description=Shortas custom theme for Keycloak
version=1.0.0
```

### theme.properties (account/)
```properties
parent=keycloak
import=common/keycloak
```

### theme.properties (admin/)
```properties
parent=keycloak
import=common/keycloak
```

## Step 3: Login Template (login.ftl)

Create `login.ftl` with the following content:

```html
<#import "template.ftl" as layout>
<@layout.registrationLayout displayMessage=!messagesPerField.existsError('username','password') displayInfo=realm.password && realm.registrationAllowed && !registrationDisabled??; section>
    <#if section = "header">
        <div class="keycloak-login-container">
            <div class="keycloak-login-wrapper">
                <!-- Left Side - Branding -->
                <div class="keycloak-login-branding">
                    <div class="keycloak-login-content">
                        <div class="keycloak-login-logo-section">
                            <svg class="keycloak-login-logo" width="64" height="64" viewBox="0 0 24 24" fill="none" stroke="white" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                                <polygon points="13 2 3 14 12 14 11 22 21 10 12 10 13 2" />
                            </svg>
                            <h1 class="keycloak-login-title">SHORTAS</h1>
                        </div>
                        
                        <div class="keycloak-login-description">
                            <h2 class="keycloak-login-subtitle">Welcome to Shortas Dashboard</h2>
                            <p class="keycloak-login-lead">
                                Manage your shortened URLs with powerful analytics, 
                                real-time tracking, and enterprise-grade security.
                            </p>
                        </div>

                        <div class="keycloak-login-features">
                            <div class="keycloak-login-feature">
                                <svg class="keycloak-login-feature-icon" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="white" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                                    <path d="M12 22s8-4 8-10V5l-8-3-8 3v7c0 6 8 10 8 10z"/>
                                </svg>
                                <div class="keycloak-login-feature-content">
                                    <h5>Secure Authentication</h5>
                                    <small>Protected by Keycloak OpenID Connect</small>
                                </div>
                            </div>
                            
                            <div class="keycloak-login-feature">
                                <svg class="keycloak-login-feature-icon" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="white" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                                    <path d="M15 3h4a2 2 0 0 1 2 2v14a2 2 0 0 1-2 2h-4"/>
                                    <polyline points="10,17 15,12 10,7"/>
                                    <line x1="15" y1="12" x2="3" y2="12"/>
                                </svg>
                                <div class="keycloak-login-feature-content">
                                    <h5>Single Sign-On</h5>
                                    <small>Seamless authentication experience</small>
                                </div>
                            </div>
                            
                            <div class="keycloak-login-feature">
                                <svg class="keycloak-login-feature-icon" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="white" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                                    <path d="M9 12l2 2 4-4"/>
                                    <path d="M21 12c-1 0-3-1-3-3s2-3 3-3 3 1 3 3-2 3-3 3"/>
                                    <path d="M3 12c1 0 3-1 3-3s-2-3-3-3-3 1-3 3 2 3 3 3"/>
                                    <path d="M12 3c0 1-1 3-3 3s-3-2-3-3 1-3 3-3 3 2 3 3"/>
                                    <path d="M12 21c0-1 1-3 3-3s3 2 3 3-1 3-3 3-3-2-3-3"/>
                                </svg>
                                <div class="keycloak-login-feature-content">
                                    <h5>Enterprise Security</h5>
                                    <small>Advanced security features and compliance</small>
                                </div>
                            </div>
                        </div>
                    </div>
                </div>

                <!-- Right Side - Login Form -->
                <div class="keycloak-login-form">
                    <div class="keycloak-login-form-container">
                        <div class="keycloak-login-form-header">
                            <svg class="keycloak-login-form-logo" width="40" height="40" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                                <polygon points="13 2 3 14 12 14 11 22 21 10 12 10 13 2" />
                            </svg>
                            <h2 class="keycloak-login-form-title">Sign In</h2>
                            <p class="keycloak-login-form-subtitle">Access your Shortas dashboard</p>
                        </div>

                        <div class="keycloak-login-form-description">
                            <p>Use your organization credentials to sign in securely through our Keycloak authentication system.</p>
                        </div>

                        <form id="kc-form-login" onsubmit="login.disabled = true; return true;" action="${url.loginAction}" method="post">
                            <#if !realm.loginWithEmailAllowed>
                                <div class="keycloak-login-form-group">
                                    <label for="username" class="keycloak-login-form-label">
                                        <#if !realm.loginWithEmailAllowed>
                                            ${msg("username")}
                                        <#else>
                                            ${msg("usernameOrEmail")}
                                        </#if>
                                    </label>
                                    <#if usernameEditDisabled??>
                                        <input tabindex="1" id="username" class="keycloak-login-form-input" name="username" value="${(login.username!'')}" type="text" disabled />
                                    <#else>
                                        <input tabindex="1" id="username" class="keycloak-login-form-input" name="username" value="${(login.username!'')}" type="text" autofocus autocomplete="off" />
                                    </#if>
                                </div>
                            </#if>

                            <div class="keycloak-login-form-group">
                                <label for="password" class="keycloak-login-form-label">${msg("password")}</label>
                                <input tabindex="2" id="password" class="keycloak-login-form-input" name="password" type="password" autocomplete="off" />
                            </div>

                            <#if realm.rememberMe && !usernameEditDisabled??>
                                <div class="keycloak-login-form-checkbox">
                                    <input tabindex="3" id="rememberMe" name="rememberMe" type="checkbox" <#if login.rememberMe??>checked</#if> />
                                    <label for="rememberMe">${msg("rememberMe")}</label>
                                </div>
                            </#if>

                            <div class="keycloak-login-form-actions">
                                <input type="hidden" id="id-hidden-input" name="credentialId" <#if auth.selectedCredential?has_content>value="${auth.selectedCredential}"</#if>/>
                                <button tabindex="4" name="login" id="kc-login" type="submit" class="keycloak-login-button keycloak-login-button-primary">
                                    <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                                        <path d="M15 3h4a2 2 0 0 1 2 2v14a2 2 0 0 1-2 2h-4"/>
                                        <polyline points="10,17 15,12 10,7"/>
                                        <line x1="15" y1="12" x2="3" y2="12"/>
                                    </svg>
                                    ${msg("doLogIn")}
                                </button>
                            </div>
                        </form>

                        <#if realm.password && realm.registrationAllowed && !registrationDisabled??>
                            <div class="keycloak-login-form-footer">
                                <a href="${url.registrationUrl}" class="keycloak-login-link">${msg("doRegister")}</a>
                            </div>
                        </#if>

                        <#if realm.resetPasswordAllowed>
                            <div class="keycloak-login-form-footer">
                                <a href="${url.loginResetCredentialsUrl}" class="keycloak-login-link">${msg("doForgotPassword")}</a>
                            </div>
                        </#if>

                        <div class="keycloak-login-badges">
                            <span class="keycloak-login-badge keycloak-login-badge-success">
                                <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                                    <path d="M12 22s8-4 8-10V5l-8-3-8 3v7c0 6 8 10 8 10z"/>
                                </svg>
                                Secure
                            </span>
                            <span class="keycloak-login-badge keycloak-login-badge-info">
                                <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                                    <path d="M15 3h4a2 2 0 0 1 2 2v14a2 2 0 0 1-2 2h-4"/>
                                    <polyline points="10,17 15,12 10,7"/>
                                    <line x1="15" y1="12" x2="3" y2="12"/>
                                </svg>
                                SSO
                            </span>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    </#if>
</@layout.registrationLayout>
```

## Step 4: CSS Styling (login.css)

Create `resources/css/login.css`:

```css
/* Keycloak Login Theme Styles */
.keycloak-login-container {
    min-height: 100vh;
    background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
    display: flex;
    align-items: center;
    justify-content: center;
    padding: 0;
    margin: 0;
}

.keycloak-login-wrapper {
    display: flex;
    width: 100%;
    max-width: 1200px;
    min-height: 100vh;
    box-shadow: 0 20px 40px rgba(0, 0, 0, 0.1);
}

.keycloak-login-branding {
    flex: 1;
    background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
    color: white;
    position: relative;
    overflow: hidden;
    display: flex;
    align-items: center;
    justify-content: center;
    padding: 3rem;
}

.keycloak-login-branding::before {
    content: '';
    position: absolute;
    top: 0;
    left: 0;
    right: 0;
    bottom: 0;
    background: url('data:image/svg+xml,<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 100 100"><defs><pattern id="grain" width="100" height="100" patternUnits="userSpaceOnUse"><circle cx="25" cy="25" r="1" fill="white" opacity="0.1"/><circle cx="75" cy="75" r="1" fill="white" opacity="0.1"/><circle cx="50" cy="10" r="0.5" fill="white" opacity="0.05"/><circle cx="10" cy="60" r="0.5" fill="white" opacity="0.05"/><circle cx="90" cy="40" r="0.5" fill="white" opacity="0.05"/></pattern></defs><rect width="100" height="100" fill="url(%23grain)"/></svg>');
    opacity: 0.3;
}

.keycloak-login-content {
    position: relative;
    z-index: 1;
    text-align: center;
    max-width: 500px;
}

.keycloak-login-logo-section {
    margin-bottom: 2rem;
}

.keycloak-login-logo {
    margin-bottom: 1rem;
}

.keycloak-login-title {
    font-size: 3.5rem;
    font-weight: 800;
    letter-spacing: -0.02em;
    margin: 0;
    background: linear-gradient(135deg, #ffffff 0%, #f8f9fa 100%);
    -webkit-background-clip: text;
    -webkit-text-fill-color: transparent;
    background-clip: text;
}

.keycloak-login-description {
    margin-bottom: 3rem;
}

.keycloak-login-subtitle {
    font-size: 1.5rem;
    font-weight: 600;
    margin-bottom: 1.5rem;
    opacity: 0.9;
}

.keycloak-login-lead {
    font-size: 1.1rem;
    line-height: 1.6;
    opacity: 0.8;
    margin: 0;
}

.keycloak-login-features {
    text-align: left;
}

.keycloak-login-feature {
    display: flex;
    align-items: center;
    margin-bottom: 1.5rem;
    padding: 0.75rem;
    border-radius: 8px;
    background: rgba(255, 255, 255, 0.1);
    backdrop-filter: blur(10px);
    transition: all 0.3s ease;
}

.keycloak-login-feature:hover {
    background: rgba(255, 255, 255, 0.15);
    transform: translateX(5px);
}

.keycloak-login-feature-icon {
    width: 24px;
    height: 24px;
    margin-right: 1rem;
    flex-shrink: 0;
}

.keycloak-login-feature-content h5 {
    font-size: 1.1rem;
    font-weight: 600;
    margin: 0 0 0.25rem 0;
    color: white;
}

.keycloak-login-feature-content small {
    font-size: 0.9rem;
    opacity: 0.8;
    color: rgba(255, 255, 255, 0.9);
}

.keycloak-login-form {
    flex: 1;
    background: white;
    display: flex;
    align-items: center;
    justify-content: center;
    padding: 3rem;
}

.keycloak-login-form-container {
    width: 100%;
    max-width: 400px;
}

.keycloak-login-form-header {
    text-align: center;
    margin-bottom: 2rem;
}

.keycloak-login-form-logo {
    margin-bottom: 1rem;
    color: #6b7280;
}

.keycloak-login-form-title {
    font-size: 1.75rem;
    font-weight: 700;
    color: #1a202c;
    margin: 0 0 0.5rem 0;
}

.keycloak-login-form-subtitle {
    color: #4a5568;
    margin: 0 0 2rem 0;
}

.keycloak-login-form-description {
    text-align: center;
    margin-bottom: 2rem;
}

.keycloak-login-form-description p {
    color: #6b7280;
    margin: 0;
}

.keycloak-login-form-group {
    margin-bottom: 1.5rem;
}

.keycloak-login-form-label {
    display: block;
    font-weight: 600;
    color: #374151;
    margin-bottom: 0.5rem;
}

.keycloak-login-form-input {
    width: 100%;
    padding: 0.75rem 1rem;
    border: 2px solid #e5e7eb;
    border-radius: 8px;
    font-size: 1rem;
    transition: all 0.3s ease;
    box-sizing: border-box;
}

.keycloak-login-form-input:focus {
    outline: none;
    border-color: #667eea;
    box-shadow: 0 0 0 3px rgba(102, 126, 234, 0.1);
}

.keycloak-login-form-checkbox {
    display: flex;
    align-items: center;
    margin-bottom: 1.5rem;
}

.keycloak-login-form-checkbox input {
    margin-right: 0.5rem;
}

.keycloak-login-form-actions {
    margin-bottom: 2rem;
}

.keycloak-login-button {
    width: 100%;
    padding: 0.75rem 1.5rem;
    border: none;
    border-radius: 12px;
    font-weight: 600;
    font-size: 1rem;
    cursor: pointer;
    transition: all 0.3s ease;
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 0.5rem;
    position: relative;
    overflow: hidden;
}

.keycloak-login-button::before {
    content: '';
    position: absolute;
    top: 0;
    left: -100%;
    width: 100%;
    height: 100%;
    background: linear-gradient(90deg, transparent, rgba(255, 255, 255, 0.2), transparent);
    transition: left 0.5s;
}

.keycloak-login-button:hover::before {
    left: 100%;
}

.keycloak-login-button-primary {
    background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
    color: white;
    box-shadow: 0 4px 15px rgba(102, 126, 234, 0.4);
}

.keycloak-login-button-primary:hover {
    transform: translateY(-2px);
    box-shadow: 0 8px 25px rgba(102, 126, 234, 0.6);
    color: white;
}

.keycloak-login-form-footer {
    text-align: center;
    margin-bottom: 1rem;
}

.keycloak-login-link {
    color: #667eea;
    text-decoration: none;
    font-weight: 500;
    transition: color 0.3s ease;
}

.keycloak-login-link:hover {
    color: #5a67d8;
    text-decoration: underline;
}

.keycloak-login-badges {
    display: flex;
    justify-content: center;
    gap: 1rem;
    flex-wrap: wrap;
    margin-top: 2rem;
}

.keycloak-login-badge {
    display: inline-flex;
    align-items: center;
    padding: 0.5rem 1rem;
    border-radius: 20px;
    font-size: 0.875rem;
    font-weight: 600;
    gap: 0.5rem;
}

.keycloak-login-badge-success {
    background: linear-gradient(135deg, #48bb78 0%, #38a169 100%);
    color: white;
}

.keycloak-login-badge-info {
    background: linear-gradient(135deg, #4299e1 0%, #3182ce 100%);
    color: white;
}

/* Responsive Design */
@media (max-width: 768px) {
    .keycloak-login-wrapper {
        flex-direction: column;
    }
    
    .keycloak-login-branding {
        display: none;
    }
    
    .keycloak-login-form {
        padding: 2rem 1rem;
    }
    
    .keycloak-login-form-logo {
        display: block;
    }
}

@media (max-width: 480px) {
    .keycloak-login-form {
        padding: 1.5rem 1rem;
    }
    
    .keycloak-login-form-title {
        font-size: 1.5rem;
    }
}
```

## Step 5: JavaScript (login.js)

Create `resources/js/login.js`:

```javascript
// Keycloak Login Theme JavaScript
document.addEventListener('DOMContentLoaded', function() {
    // Add loading state to login button
    const loginForm = document.getElementById('kc-form-login');
    const loginButton = document.getElementById('kc-login');
    
    if (loginForm && loginButton) {
        loginForm.addEventListener('submit', function() {
            loginButton.disabled = true;
            loginButton.innerHTML = '<svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><circle cx="12" cy="12" r="10"/><path d="M8 12l2 2 4-4"/></svg> Signing In...';
        });
    }
    
    // Add focus management
    const usernameInput = document.getElementById('username');
    const passwordInput = document.getElementById('password');
    
    if (usernameInput && !usernameInput.value) {
        usernameInput.focus();
    } else if (passwordInput) {
        passwordInput.focus();
    }
    
    // Add form validation
    const inputs = document.querySelectorAll('.keycloak-login-form-input');
    inputs.forEach(input => {
        input.addEventListener('blur', function() {
            if (this.value.trim() === '') {
                this.style.borderColor = '#ef4444';
            } else {
                this.style.borderColor = '#e5e7eb';
            }
        });
    });
});
```

## Step 6: Keycloak Configuration

### 1. Enable Custom Theme

Add to your Keycloak configuration:

```properties
# keycloak.conf
kc.spi-theme-static-max-age=-1
kc.spi-theme-cache-themes=false
kc.spi-theme-cache-templates=false
```

### 2. Set Theme in Admin Console

1. Go to Keycloak Admin Console
2. Navigate to **Realm Settings** → **Themes**
3. Set **Login theme** to `shortas`
4. Set **Account theme** to `shortas`
5. Set **Admin theme** to `shortas`
6. Click **Save**

### 3. Alternative: Set via CLI

```bash
# Set theme for realm
kcadm.sh update realms/shortas-dev -s loginTheme=shortas
kcadm.sh update realms/shortas-dev -s accountTheme=shortas
kcadm.sh update realms/shortas-dev -s adminTheme=shortas
```

## Step 7: Docker Integration

### Dockerfile for Custom Theme

```dockerfile
FROM quay.io/keycloak/keycloak:latest

# Copy custom theme
COPY themes/ /opt/keycloak/themes/

# Set theme as default
ENV KC_DB=postgres
ENV KC_DB_URL=jdbc:postgresql://postgres:5432/keycloak
ENV KC_DB_USERNAME=keycloak
ENV KC_DB_PASSWORD=password
ENV KC_HOSTNAME_STRICT=false
ENV KC_HOSTNAME_STRICT_HTTPS=false

# Build and start
RUN /opt/keycloak/bin/kc.sh build
CMD ["/opt/keycloak/bin/kc.sh", "start-dev"]
```

### Docker Compose

```yaml
version: '3.8'
services:
  keycloak:
    build: .
    ports:
      - "8080:8080"
    environment:
      KC_DB: postgres
      KC_DB_URL: jdbc:postgresql://postgres:5432/keycloak
      KC_DB_USERNAME: keycloak
      KC_DB_PASSWORD: password
      KC_HOSTNAME_STRICT: false
      KC_HOSTNAME_STRICT_HTTPS: false
    depends_on:
      - postgres
    volumes:
      - ./themes:/opt/keycloak/themes

  postgres:
    image: postgres:13
    environment:
      POSTGRES_DB: keycloak
      POSTGRES_USER: keycloak
      POSTGRES_PASSWORD: password
    volumes:
      - postgres_data:/var/lib/postgresql/data

volumes:
  postgres_data:
```

## Step 8: Testing

### 1. Start Keycloak with Custom Theme

```bash
# Start Keycloak
./bin/kc.sh start-dev --spi-theme-static-max-age=-1
```

### 2. Access Login Page

Navigate to: `http://localhost:8080/auth/realms/shortas-dev/protocol/openid-connect/auth`

### 3. Verify Theme

- Check that the custom Shortas design is displayed
- Verify responsive behavior on mobile
- Test login functionality
- Check error message styling

## Step 9: Troubleshooting

### Common Issues

1. **Theme not loading**: Check file permissions and paths
2. **CSS not applied**: Verify CSS file is in correct location
3. **JavaScript errors**: Check browser console for errors
4. **Responsive issues**: Test on different screen sizes

### Debug Steps

1. **Check Keycloak logs**:
   ```bash
   tail -f /opt/keycloak/data/log/keycloak.log
   ```

2. **Verify theme files**:
   ```bash
   ls -la /opt/keycloak/themes/shortas/login/
   ```

3. **Test theme loading**:
   ```bash
   curl -I http://localhost:8080/auth/realms/shortas-dev/protocol/openid-connect/auth
   ```

## Step 10: Production Deployment

### 1. Build Production Theme

```bash
# Build Keycloak with custom theme
./bin/kc.sh build --spi-theme-static-max-age=31536000
```

### 2. Deploy to Production

```bash
# Start in production mode
./bin/kc.sh start --optimized
```

### 3. Set Production Theme

```bash
# Set theme via CLI
kcadm.sh config credentials --server http://localhost:8080 --realm master --user admin
kcadm.sh update realms/shortas-dev -s loginTheme=shortas
```

## Benefits

- **✅ Consistent Branding**: Matches dashboard design
- **✅ Professional Look**: Enterprise-grade appearance
- **✅ Responsive Design**: Works on all devices
- **✅ Security Focused**: Emphasizes authentication security
- **✅ Customizable**: Easy to modify colors and content
- **✅ Performance**: Optimized for speed and efficiency

This integration provides a seamless login experience that matches your Shortas dashboard design while maintaining all Keycloak security features.

