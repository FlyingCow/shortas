<#--
  Shared chrome for every Keycloak login-theme page (login, register,
  forgot-password, update-password, error, info, ...). Pages we don't
  override (anything besides login.ftl) come from Keycloak's base theme
  and only use the "header"/"form"/"info" nested sections — see
  https://github.com/keycloak/keycloak/tree/23.0.6/.../theme/base/login
  for the originals this wraps. Deliberately NOT replicating base's
  locale dropdown / "try another way" / social-provider slots since this
  realm doesn't use those flows; add them here if that changes.
-->
<#macro registrationLayout bodyClass="" displayInfo=false displayMessage=true displayRequiredFields=false displayWide=false showAnotherWayIfPresent=true>
<!DOCTYPE html>
<html lang="${(locale.currentLanguageTag)!'en'}">
<head>
    <meta charset="utf-8">
    <meta name="viewport" content="width=device-width, initial-scale=1">
    <meta name="robots" content="noindex, nofollow">
    <title>${msg("loginTitle",(realm.displayName!''))}</title>
    <link rel="stylesheet" href="${url.resourcesPath}/css/login.css">
</head>
<body>
    <div class="shortas-login-container">
        <div class="shortas-login-wrapper">
            <!-- Left — Branding (shared across every page) -->
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

            <!-- Right — Page content -->
            <div class="shortas-login-form-panel">
                <div class="shortas-login-card">
                    <div class="shortas-login-card-body">
                        <div class="shortas-login-header">
                            <svg class="shortas-login-form-logo" width="36" height="36" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                                <polygon points="13 2 3 14 12 14 11 22 21 10 12 10 13 2" />
                            </svg>
                            <h2 class="shortas-login-form-title"><#nested "header"></h2>
                        </div>

                        <#if displayRequiredFields>
                            <p class="shortas-login-form-desc"><small>* ${msg("requiredFields")}</small></p>
                        </#if>

                        <#if displayMessage && message?has_content && message.type?has_content>
                            <div class="shortas-alert shortas-alert-${message.type}">
                                ${kcSanitize(message.summary)?no_esc}
                            </div>
                        </#if>

                        <#nested "form">

                        <#if displayInfo>
                            <div class="shortas-login-form-desc">
                                <#nested "info">
                            </div>
                        </#if>
                    </div>
                </div>
            </div>
        </div>
    </div>
    <script src="${url.resourcesPath}/js/login.js"></script>
</body>
</html>
</#macro>
