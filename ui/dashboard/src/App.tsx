import React, { useState, useEffect } from 'react';
import { BrowserRouter as Router, Routes, Route, Navigate } from 'react-router-dom';
import keycloak, { keycloakInitOptions, initializeKeycloak } from './config/keycloak';
import { useMockData } from './config/development';
import { ThemeProvider } from './contexts/ThemeContext';
import Dashboard from './components/DashboardUnified';
import RoutesPage from './components/RoutesWithAnalytics';
import Analytics from './components/AnalyticsUnified';
import Settings from './components/SettingsUnified';
import Clickstream from './components/ClickstreamUnified';
import Layout from './components/LayoutUnified';
import LoadingSpinner from './components/LoadingSpinner';
import Login from './components/Login';
import KeycloakError from './components/KeycloakError';
import './App.css';
import './components/DesignSystem.css';

interface AppState {
  keycloakInitialized: boolean;
  authenticated: boolean;
  loading: boolean;
  error: string | null;
}

const App: React.FC = () => {
  const [state, setState] = useState<AppState>({
    keycloakInitialized: false,
    authenticated: false,
    loading: true,
    error: null,
  });

  useEffect(() => {
    const initKeycloak = async () => {
      try {
        // Skip Keycloak initialization in mock data mode
        if (useMockData) {
          console.log('Mock data mode enabled - skipping Keycloak initialization');
          setState({
            keycloakInitialized: true,
            authenticated: true, // Mock authenticated state
            loading: false,
            error: null,
          });
          return;
        }

        // Initialize Keycloak using the protected initialization function
        const authenticated = await initializeKeycloak(keycloakInitOptions);
        
        setState({
          keycloakInitialized: true,
          authenticated,
          loading: false,
          error: null,
        });

        // Set up token refresh
        if (authenticated) {
          setInterval(() => {
            keycloak.updateToken(70).catch(() => {
              console.error('Failed to refresh token');
            });
          }, 60000); // Refresh every minute
        }
      } catch (error) {
        console.error('Keycloak initialization failed:', error);
        setState({
          keycloakInitialized: false,
          authenticated: false,
          loading: false,
          error: 'Failed to initialize authentication. Please try again.',
        });
      }
    };

    initKeycloak();
  }, []);

  if (state.loading) {
    return <LoadingSpinner message="Initializing authentication..." />;
  }

  if (state.error) {
    return (
      <KeycloakError 
        error={state.error}
        onRetry={() => window.location.reload()}
        onEnableMockData={() => {
          // Create or update .env.local with mock data enabled
          alert('Please add REACT_APP_USE_MOCK_DATA=true to your .env.local file and restart the app.');
        }}
      />
    );
  }


  return (
    <ThemeProvider>
      <Router>
        <Routes>
        {/* Public Routes */}
        <Route path="/login" element={
          state.authenticated ? (
            <Navigate to="/dashboard" replace />
          ) : (
            <Login 
              onLogin={() => keycloak.login()}
              error={state.error}
            />
          )
        } />
        
        {/* Protected Routes */}
        <Route path="/*" element={
          state.authenticated ? (
            <Layout>
              <Routes>
                <Route path="/" element={<Navigate to="/dashboard" replace />} />
                <Route path="/dashboard" element={<Dashboard />} />
                <Route path="/routes" element={<RoutesPage />} />
                <Route path="/analytics" element={<Analytics />} />
                <Route path="/clickstream" element={<Clickstream />} />
                <Route path="/settings" element={<Settings />} />
                <Route path="*" element={<Navigate to="/dashboard" replace />} />
              </Routes>
            </Layout>
          ) : (
            <Navigate to="/login" replace />
          )
        } />
      </Routes>
    </Router>
    </ThemeProvider>
  );
};

export default App;
