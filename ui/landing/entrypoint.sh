#!/bin/sh
set -e

# Inject Docker Compose environment into env.js so the landing page reads it at runtime
HTML_DIR="/usr/share/nginx/html"
ENV_JS="${HTML_DIR}/env.js"

# Escape for use inside a JavaScript string (backslash and double-quote)
escape_js() {
  printf '%s' "$1" | sed "s/\\\\/\\\\\\\\/g; s/\"/\\\\\"/g"
}

DASHBOARD_URL="${REACT_APP_DASHBOARD_URL:-}"

cat > "$ENV_JS" << EOF
window.__APP_ENV__=window.__APP_ENV__||{};
window.__APP_ENV__.REACT_APP_DASHBOARD_URL="$(escape_js "$DASHBOARD_URL")";
EOF

exec nginx -g "daemon off;"
