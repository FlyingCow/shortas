<#import "template.ftl" as layout>
<@layout.registrationLayout displayMessage=true; section>
    <#if section = "form">
        <div class="shortas-login-container">
            <div class="shortas-login-wrapper">
                <!-- Left — Branding -->
                <div class="shortas-login-branding">
                    <div class="shortas-login-branding-inner">
                        <div>
                            <svg class="shortas-login-logo" width="56" height="56" viewBox="0 0 24 24" fill="none" stroke="white" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                                <polygon points="13 2 3 14 12 14 11 22 21 10 12 10 13 2" />
                            </svg>
                            <h1 class="shortas-login-title">SHORTAS</h1>
                        </div>

                        <h2 class="shortas-login-subtitle">Welcome to Shortas</h2>
                        <p class="shortas-login-lead">
                            Manage your shortened URLs with powerful analytics,
                            real-time tracking, and enterprise-grade security.
                        </p>

                        <div class="shortas-login-features">
                            <div class="shortas-login-feature">
                                <svg class="shortas-login-feature-icon" width="22" height="22" viewBox="0 0 24 24" fill="none" stroke="white" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                                    <path d="M12 22s8-4 8-10V5l-8-3-8 3v7c0 6 8 10 8 10z"/>
                                </svg>
                                <div>
                                    <h5>Secure Authentication</h5>
                                    <small>Protected by Keycloak OpenID Connect</small>
                                </div>
                            </div>

                            <div class="shortas-login-feature">
                                <svg class="shortas-login-feature-icon" width="22" height="22" viewBox="0 0 24 24" fill="none" stroke="white" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                                    <path d="M15 3h4a2 2 0 0 1 2 2v14a2 2 0 0 1-2 2h-4"/>
                                    <polyline points="10,17 15,12 10,7"/>
                                    <line x1="15" y1="12" x2="3" y2="12"/>
                                </svg>
                                <div>
                                    <h5>Single Sign-On</h5>
                                    <small>Seamless authentication experience</small>
                                </div>
                            </div>

                            <div class="shortas-login-feature">
                                <svg class="shortas-login-feature-icon" width="22" height="22" viewBox="0 0 24 24" fill="none" stroke="white" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                                    <polyline points="22 12 18 12 15 21 9 3 6 12 2 12"/>
                                </svg>
                                <div>
                                    <h5>Real-Time Analytics</h5>
                                    <small>Track clicks, conversions, and traffic</small>
                                </div>
                            </div>
                        </div>
                    </div>
                </div>

                <!-- Right — Form -->
                <div class="shortas-login-form-panel">
                    <div class="shortas-login-card">
                        <div class="shortas-login-card-body">
                            <div class="shortas-login-header">
                                <svg class="shortas-login-form-logo" width="36" height="36" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                                    <polygon points="13 2 3 14 12 14 11 22 21 10 12 10 13 2" />
                                </svg>
                                <h2 class="shortas-login-form-title">Sign In</h2>
                                <p class="shortas-login-form-subtitle">Access your Shortas dashboard</p>
                            </div>

                            <div class="shortas-login-form-desc">
                                <p>Sign in with your credentials to continue.</p>
                            </div>

                            <#if message?has_content && message.type?has_content>
                                <div class="shortas-alert shortas-alert-${message.type}">
                                    ${message.summary!}
                                </div>
                            </#if>

                            <form id="kc-form-login" action="${url.loginAction}" method="post">
                                <div class="shortas-form-group">
                                    <label for="username" class="shortas-form-label">
                                        <#if !realm.loginWithEmailAllowed>
                                            ${msg("username")}
                                        <#elseif !realm.registrationEmailAsUsername>
                                            ${msg("usernameOrEmail")}
                                        <#else>
                                            ${msg("email")}
                                        </#if>
                                    </label>
                                    <input tabindex="1"
                                           id="username"
                                           class="shortas-form-input"
                                           name="username"
                                           value="${(login.username!'')}"
                                           type="text"
                                           autofocus
                                           autocomplete="off"
                                           placeholder="<#if realm.loginWithEmailAllowed>Enter your email<#else>Enter your username</#if>" />
                                </div>

                                <div class="shortas-form-group">
                                    <label for="password" class="shortas-form-label">${msg("password")}</label>
                                    <input tabindex="2"
                                           id="password"
                                           class="shortas-form-input"
                                           name="password"
                                           type="password"
                                           autocomplete="off"
                                           placeholder="Enter your password" />
                                </div>

                                <#if realm.rememberMe && !usernameEditDisabled??>
                                    <div class="shortas-form-check">
                                        <input tabindex="3" id="rememberMe" name="rememberMe" type="checkbox" <#if login.rememberMe??>checked</#if> />
                                        <label for="rememberMe">${msg("rememberMe")}</label>
                                    </div>
                                </#if>

                                <div class="shortas-form-actions">
                                    <input type="hidden" id="id-hidden-input" name="credentialId" <#if auth?has_content && auth.selectedCredential?has_content>value="${auth.selectedCredential}"</#if>/>
                                    <button tabindex="4" name="login" id="kc-login" type="submit" class="shortas-btn shortas-btn-primary">
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
                                <div class="shortas-form-footer">
                                    <span style="font-size:0.875rem;color:var(--text-tertiary)">Don't have an account?</span>
                                    <a href="${url.registrationUrl}" class="shortas-link">${msg("doRegister")}</a>
                                </div>
                            </#if>

                            <#if realm.resetPasswordAllowed>
                                <div class="shortas-form-footer">
                                    <a href="${url.loginResetCredentialsUrl}" class="shortas-link">${msg("doForgotPassword")}</a>
                                </div>
                            </#if>

                            <div class="shortas-badges">
                                <span class="shortas-badge shortas-badge-success">
                                    <svg width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                                        <path d="M12 22s8-4 8-10V5l-8-3-8 3v7c0 6 8 10 8 10z"/>
                                    </svg>
                                    Secure
                                </span>
                                <span class="shortas-badge shortas-badge-info">
                                    <svg width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
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
        </div>
    </#if>
</@layout.registrationLayout>
