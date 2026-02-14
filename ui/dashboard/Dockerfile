# Build stage
FROM node:20-alpine AS builder

WORKDIR /app

# Copy package files
COPY package*.json ./

# Install dependencies (npm install so lock file need not match Node 20 resolution exactly; use npm ci in CI if lock file is generated with Node 20)
RUN npm install

# Copy source code
COPY . .

# Build (no build-time env; dashboard reads config at runtime from env.js injected by entrypoint)
RUN npm run build

# Production stage
FROM nginx:alpine

# Copy custom nginx config
COPY nginx.conf /etc/nginx/conf.d/default.conf

# Copy built assets from builder stage
COPY --from=builder /app/build /usr/share/nginx/html

# Entrypoint injects Docker Compose env into env.js at runtime
COPY entrypoint.sh /entrypoint.sh
RUN chmod +x /entrypoint.sh

# Expose port 80
EXPOSE 80

ENTRYPOINT ["/entrypoint.sh"]
