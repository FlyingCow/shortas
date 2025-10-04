# Shortas Dashboard

A modern React dashboard for managing Shortas URL shortener with Bootstrap UI, Keycloak authentication, and real-time analytics.

## 🚀 Features

### ✅ Completed Features
- **Bootstrap UI Framework**: Modern, responsive design using React-Bootstrap
- **Keycloak Authentication**: Secure OpenID Connect authentication with PKCE
- **Mock Data Mode**: Development mode that bypasses authentication
- **Dashboard Analytics**: Real-time charts and statistics
- **Route Management**: Create, edit, and manage shortened URLs
- **Settings Panel**: User configuration and preferences
- **Error Handling**: Comprehensive error messages and troubleshooting

### 🎨 UI Components
- **Responsive Navbar**: Bootstrap navigation with user dropdown
- **Dashboard Cards**: Statistics cards with icons and charts
- **Data Tables**: Responsive tables with actions and filters
- **Modal Dialogs**: Bootstrap modals for forms and confirmations
- **Professional Login**: Split-screen login with branding
- **Error Pages**: Contextual error handling with solutions

### 🔐 Authentication
- **Keycloak Integration**: Full OpenID Connect flow
- **Token Management**: Automatic token refresh
- **Role-based Access**: Support for user roles and permissions
- **Development Mode**: Mock authentication for development

## 📋 Prerequisites

- Node.js 16+ and npm
- (Optional) Keycloak server for authentication
- (Optional) Shortas backend APIs

## 🛠️ Quick Start

### 1. Install Dependencies
```bash
npm install
```

### 2. Configure Environment
```bash
# Copy the example environment file
cp .env.example .env.local

# Edit .env.local with your settings
# For development with mock data:
REACT_APP_USE_MOCK_DATA=true
```

### 3. Start Development Server
```bash
npm start
```

The dashboard will open at `http://localhost:3000`

## ⚙️ Configuration

### Environment Variables

| Variable | Description | Default | Required |
|----------|-------------|---------|----------|
| `REACT_APP_KEYCLOAK_URL` | Keycloak server URL | `http://localhost:8080` | No |
| `REACT_APP_KEYCLOAK_CLIENT_ID` | Keycloak client ID | `shortas-dashboard` | No |
| `REACT_APP_API_BASE_URL` | Backend API base URL | `http://localhost:8080` | No |
| `REACT_APP_USE_MOCK_DATA` | Use mock data mode | `false` | No |
| `NODE_ENV` | Environment mode | `development` | No |

### Development vs Production

#### Development Mode (Recommended for UI work)
```env
REACT_APP_USE_MOCK_DATA=true
```
- ✅ No external dependencies
- ✅ Full UI functionality
- ✅ Mock data for all components
- ✅ No authentication required

#### Production Mode
```env
REACT_APP_USE_MOCK_DATA=false
REACT_APP_KEYCLOAK_URL=https://your-keycloak.com
REACT_APP_API_BASE_URL=https://your-api.com
```
- ✅ Real Keycloak authentication
- ✅ Live API integration
- ❌ Requires Keycloak server setup
- ❌ Requires backend APIs

## 🔧 Keycloak Setup

### Option 1: Use Mock Data (Recommended for Development)
Set `REACT_APP_USE_MOCK_DATA=true` in `.env.local` - no Keycloak needed!

### Option 2: Real Keycloak Setup

1. **Start Keycloak Server**
   ```bash
   # Download and start Keycloak
   wget https://github.com/keycloak/keycloak/releases/download/23.0.3/keycloak-23.0.3.zip
   unzip keycloak-23.0.3.zip
   cd keycloak-23.0.3
   bin/kc.sh start-dev --http-port=8080
   ```

2. **Create Realm**
   - Go to `http://localhost:8080/admin`
   - Create realm named `shortas-dev`

3. **Create Client**
   - Client ID: `shortas-dashboard`
   - Client Type: `OpenID Connect`
   - Client authentication: `Off` (public client)
   - Valid redirect URIs: `http://localhost:3000/*`
   - Web origins: `http://localhost:3000`

4. **Test Setup**
   ```bash
   # Test Keycloak connectivity
   ./test-keycloak.sh
   ```

## 🧪 Testing Keycloak

Use the included test script to verify your Keycloak setup:

```bash
# Make script executable
chmod +x test-keycloak.sh

# Run connectivity test
./test-keycloak.sh
```

Expected output:
```
🔍 Testing Keycloak connectivity...
==================================
1. ✅ Keycloak server is running at http://localhost:8080
2. ✅ Realm 'shortas-dev' exists and is configured
3. ✅ Admin console is accessible at http://localhost:8080/admin

🎉 Keycloak is properly configured!
```

## 🎨 UI Framework

### Bootstrap 5.3.2
- **Components**: Cards, Tables, Forms, Modals, Navbar
- **Grid System**: Responsive 12-column grid
- **Utilities**: Spacing, colors, typography
- **Icons**: Lucide React icons

### Key Bootstrap Components Used
- `Container`, `Row`, `Col` - Layout grid
- `Card` - Content containers
- `Table` - Data tables with responsive design
- `Button`, `ButtonGroup` - Actions and controls
- `Form`, `InputGroup` - Form inputs
- `Modal` - Dialog boxes
- `Alert` - Messages and notifications
- `Badge` - Status indicators
- `Navbar` - Navigation header

## 📁 Project Structure

```
src/
├── components/          # React components
│   ├── Dashboard.tsx    # Main dashboard with charts
│   ├── Routes.tsx       # URL route management
│   ├── Analytics.tsx    # Analytics and reports
│   ├── Settings.tsx     # User settings
│   ├── Login.tsx        # Authentication UI
│   ├── Layout.tsx       # App layout with navigation
│   └── LoadingSpinner.tsx
├── config/             # Configuration
│   ├── keycloak.ts     # Keycloak setup and helpers
│   └── development.ts  # Mock data and dev settings
├── services/           # API services
│   └── api.ts          # API client and data models
└── utils/              # Utilities
    └── debug.ts        # Debug logging
```

## 🚨 Troubleshooting

### Common Issues

#### 1. Keycloak 401 Error
**Problem**: Browser shows 401 when accessing `/token` URL

**Solutions**:
- Use mock data mode: `REACT_APP_USE_MOCK_DATA=true`
- Start Keycloak server on port 8080
- Create `shortas-dev` realm and `shortas-dashboard` client
- Run `./test-keycloak.sh` to verify setup

#### 2. "Keycloak instance can only be initialized once"
**Problem**: Multiple initialization attempts

**Solution**: The app now includes singleton pattern and initialization guards. If you still see this error, refresh the browser.

#### 3. Network Errors
**Problem**: Failed to fetch from APIs

**Solutions**:
- Use mock data mode: `REACT_APP_USE_MOCK_DATA=true`
- Verify backend APIs are running
- Check CORS settings on backend

#### 4. Styling Issues
**Problem**: Components not styled correctly

**Solution**: Bootstrap CSS is imported automatically. Custom styles are in component CSS files.

### Debug Mode

Enable debug logging:
```env
REACT_APP_LOG_LEVEL=debug
```

Check browser console for detailed logs:
```
[Shortas Debug] Creating new Keycloak instance
[Shortas Debug] Starting Keycloak initialization
[Shortas Debug] Mock data mode enabled
```

## 🔄 Development Workflow

### Recommended Development Setup
1. **Start with Mock Data**
   ```env
   REACT_APP_USE_MOCK_DATA=true
   ```
   - Fastest development experience
   - No external dependencies
   - Full UI functionality

2. **Add Real Authentication Later**
   ```env
   REACT_APP_USE_MOCK_DATA=false
   ```
   - Set up Keycloak when needed
   - Test real authentication flow

### Available Scripts

```bash
# Development server
npm start

# Production build
npm run build

# Run tests
npm test

# Test Keycloak connectivity
./test-keycloak.sh
```

## 🌟 Features in Detail

### Dashboard
- **Statistics Cards**: Total clicks, unique visitors, routes, etc.
- **Time Range Selector**: 24h, 7d, 30d, 90d
- **Charts**: Line charts for trends, bar charts for comparisons, pie charts for distributions
- **Real-time Data**: Auto-refresh capabilities

### Routes Management
- **CRUD Operations**: Create, read, update, delete routes
- **Search & Filter**: Find routes by URL, status, etc.
- **Bulk Actions**: Select multiple routes for batch operations
- **Status Management**: Active, inactive, paused states

### Analytics
- **Traffic Analysis**: Click patterns, geographic distribution
- **Performance Metrics**: Response times, success rates
- **Comparative Reports**: Time-based comparisons
- **Export Features**: Download reports as CSV/PDF

### Settings
- **User Profile**: Update personal information
- **Preferences**: Theme, language, notifications
- **API Keys**: Manage authentication tokens
- **Security**: Password change, 2FA settings

## 🎯 Next Steps

### Potential Enhancements
- [ ] Dark/light theme toggle
- [ ] Advanced filtering and sorting
- [ ] Bulk route operations
- [ ] Export functionality
- [ ] Real-time notifications
- [ ] Mobile app view
- [ ] Advanced analytics
- [ ] Custom dashboards

### Backend Integration
- [ ] Connect to real Shortas APIs
- [ ] Implement WebSocket for real-time updates
- [ ] Add file upload for bulk route creation
- [ ] Implement caching strategies

## 📄 License

This project is part of the Shortas URL shortener system.

## 🤝 Contributing

1. Fork the repository
2. Create a feature branch
3. Make changes with Bootstrap components
4. Test in both mock and real modes
5. Submit a pull request

---

**Happy coding!** 🚀

For questions or issues, check the troubleshooting section or create an issue in the repository.