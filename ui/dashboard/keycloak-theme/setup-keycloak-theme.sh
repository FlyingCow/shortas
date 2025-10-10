#!/bin/bash

# Keycloak Theme Setup Script
# This script sets up the Shortas custom theme in Keycloak

set -e

echo "🚀 Setting up Shortas Keycloak Theme..."

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Function to print colored output
print_status() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

print_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

print_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Check if Docker is installed
if ! command -v docker &> /dev/null; then
    print_error "Docker is not installed. Please install Docker first."
    exit 1
fi

# Check if Docker Compose is installed
if ! command -v docker-compose &> /dev/null; then
    print_error "Docker Compose is not installed. Please install Docker Compose first."
    exit 1
fi

# Create necessary directories
print_status "Creating theme directory structure..."
mkdir -p keycloak/themes/shortas/login/resources/{css,js,img}
mkdir -p keycloak/themes/shortas/account
mkdir -p keycloak/themes/shortas/admin

# Copy theme files
print_status "Copying theme files..."

# Copy login template
cp login.ftl keycloak/themes/shortas/login/
cp theme.properties keycloak/themes/shortas/login/

# Copy CSS
cp resources/css/login.css keycloak/themes/shortas/login/resources/css/

# Copy JavaScript
cp resources/js/login.js keycloak/themes/shortas/login/resources/js/

# Copy theme properties for other themes
cp theme.properties keycloak/themes/shortas/account/
cp theme.properties keycloak/themes/shortas/admin/

# Create basic templates for other themes
cat > keycloak/themes/shortas/account/account.ftl << 'EOF'
<#import "template.ftl" as layout>
<@layout.registrationLayout; section>
    <#if section = "header">
        <h1>Account Management</h1>
    </#if>
</@layout.registrationLayout>
EOF

cat > keycloak/themes/shortas/admin/admin.ftl << 'EOF'
<#import "template.ftl" as layout>
<@layout.registrationLayout; section>
    <#if section = "header">
        <h1>Admin Console</h1>
    </#if>
</@layout.registrationLayout>
EOF

print_success "Theme files copied successfully!"

# Start Keycloak with Docker Compose
print_status "Starting Keycloak with custom theme..."

# Check if containers are already running
if docker-compose ps | grep -q "Up"; then
    print_warning "Keycloak is already running. Stopping existing containers..."
    docker-compose down
fi

# Start services
docker-compose up -d

# Wait for Keycloak to be ready
print_status "Waiting for Keycloak to start..."
sleep 30

# Check if Keycloak is accessible
max_attempts=30
attempt=1

while [ $attempt -le $max_attempts ]; do
    if curl -s -f http://localhost:8080/auth/realms/master > /dev/null 2>&1; then
        print_success "Keycloak is ready!"
        break
    else
        print_status "Waiting for Keycloak... (attempt $attempt/$max_attempts)"
        sleep 10
        ((attempt++))
    fi
done

if [ $attempt -gt $max_attempts ]; then
    print_error "Keycloak failed to start within the expected time."
    exit 1
fi

# Set up realm and theme
print_status "Setting up realm and theme..."

# Create realm
docker-compose exec keycloak /opt/keycloak/bin/kcadm.sh create realms -s realm=shortas-dev -s enabled=true -s displayName="Shortas Development" -s loginTheme=shortas -s accountTheme=shortas -s adminTheme=shortas

# Create client
docker-compose exec keycloak /opt/keycloak/bin/kcadm.sh create clients -r shortas-dev -s clientId=shortas-api -s enabled=true -s publicClient=true -s redirectUris='["http://localhost:3000/*","http://localhost:5050/*"]' -s webOrigins='["http://localhost:3000","http://localhost:5050"]'

# Create user
docker-compose exec keycloak /opt/keycloak/bin/kcadm.sh create users -r shortas-dev -s username=admin -s enabled=true -s email=admin@shortas.com -s firstName=Admin -s lastName=User

# Set password
docker-compose exec keycloak /opt/keycloak/bin/kcadm.sh set-password -r shortas-dev -u admin --new-password admin123

print_success "Realm and client created successfully!"

# Display access information
echo ""
echo "🎉 Keycloak with Shortas theme is now running!"
echo ""
echo "📋 Access Information:"
echo "  • Keycloak Admin Console: http://localhost:8080/auth/admin"
echo "  • Login Page: http://localhost:8080/auth/realms/shortas-dev/protocol/openid-connect/auth"
echo "  • Realm: shortas-dev"
echo "  • Client: shortas-api"
echo "  • Admin User: admin / admin123"
echo ""
echo "🔧 Theme Configuration:"
echo "  • Login Theme: shortas"
echo "  • Account Theme: shortas"
echo "  • Admin Theme: shortas"
echo ""
echo "📁 Theme Files Location:"
echo "  • Local: ./keycloak/themes/shortas/"
echo "  • Container: /opt/keycloak/themes/shortas/"
echo ""
echo "🛠️  Useful Commands:"
echo "  • View logs: docker-compose logs -f keycloak"
echo "  • Stop services: docker-compose down"
echo "  • Restart services: docker-compose restart"
echo "  • Update theme: Copy files to ./keycloak/themes/shortas/ and restart"
echo ""
echo "✨ Your custom Shortas Keycloak theme is ready to use!"

