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

                        <#if displayMessage && message?has_content>
                            <div class="keycloak-login-alert keycloak-login-alert-${message.type}">
                                <span class="keycloak-login-alert-text">${message.summary}</span>
                            </div>
                        </#if>

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

