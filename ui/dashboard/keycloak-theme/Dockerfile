FROM quay.io/keycloak/keycloak:latest

# Copy custom theme
COPY . /opt/keycloak/themes/shortas/

# Set environment variables
ENV KC_DB=postgres
ENV KC_DB_URL=jdbc:postgresql://postgres:5432/keycloak
ENV KC_DB_USERNAME=keycloak
ENV KC_DB_PASSWORD=password
ENV KC_HOSTNAME_STRICT=false
ENV KC_HOSTNAME_STRICT_HTTPS=false

# Build Keycloak with custom theme
RUN /opt/keycloak/bin/kc.sh build

# Expose port
EXPOSE 8080

# Start Keycloak
CMD ["/opt/keycloak/bin/kc.sh", "start-dev"]

