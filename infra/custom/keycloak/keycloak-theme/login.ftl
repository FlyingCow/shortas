<#import "template.ftl" as layout>
<@layout.registrationLayout displayMessage=true; section>
    <#if section = "header">
        Sign In
    <#elseif section = "form">
        <p class="shortas-login-form-subtitle">Access your Shortas dashboard</p>
        <div class="shortas-login-form-desc">
            <p>Sign in with your credentials to continue.</p>
        </div>

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
    </#if>
</@layout.registrationLayout>
