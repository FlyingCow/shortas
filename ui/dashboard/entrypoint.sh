#!/bin/sh
set -e

# Inject Docker Compose environment into env.js so the dashboard reads them at runtime
HTML_DIR="/usr/share/nginx/html"
ENV_JS="${HTML_DIR}/env.js"

# Escape for use inside a JavaScript string (backslash and double-quote)
escape_js() {
  printf '%s' "$1" | sed "s/\\\\/\\\\\\\\/g; s/\"/\\\\\"/g"
}

ROUTE_IMAGES_BASE_URL="${REACT_APP_ROUTE_IMAGES_BASE_URL:-}"
KEYCLOAK_URL="${REACT_APP_KEYCLOAK_URL:-}"
KEYCLOAK_CLIENT_ID="${REACT_APP_KEYCLOAK_CLIENT_ID:-}"
PROXY_API_URL="${REACT_APP_PROXY_API_URL:-}"

cat > "$ENV_JS" << EOF
window.__APP_ENV__=window.__APP_ENV__||{};
window.__APP_ENV__.REACT_APP_ROUTE_IMAGES_BASE_URL="$(escape_js "$ROUTE_IMAGES_BASE_URL")";
window.__APP_ENV__.REACT_APP_KEYCLOAK_URL="$(escape_js "$KEYCLOAK_URL")";
window.__APP_ENV__.REACT_APP_KEYCLOAK_CLIENT_ID="$(escape_js "$KEYCLOAK_CLIENT_ID")";
window.__APP_ENV__.REACT_APP_PROXY_API_URL="$(escape_js "$PROXY_API_URL")";
EOF

exec nginx -g "daemon off;"
