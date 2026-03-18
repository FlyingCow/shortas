FROM nginx:alpine

# Copy custom nginx config
COPY redirect/infra/pages/nginx/nginx.conf /etc/nginx/conf.d/default.conf

# Copy static HTML pages
COPY redirect/infra/pages/html/ /usr/share/nginx/html/

# Expose port
EXPOSE 5801

# Health check
HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
    CMD wget --no-verbose --tries=1 --spider http://127.0.0.1:5801/health || exit 1
