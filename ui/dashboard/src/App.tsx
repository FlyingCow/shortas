import React, { useState, useEffect } from 'react';
import { BrowserRouter as Router, Routes, Route, Navigate } from 'react-router-dom';
import keycloak, { keycloakInitOptions, initializeKeycloak, keycloakLoginOptions } from './config/keycloak';
import { useMockData } from './config/development';
import { apiService } from './services/api';
import { ThemeProvider } from './contexts/ThemeContext';
import Dashboard from './components/DashboardUnified';
import RoutesPage from './components/RoutesWithSidebar';
import Domains from './components/Domains';
import Workspaces from './components/Workspaces';
import Analytics from './components/AnalyticsUnified';
import Settings from './components/SettingsUnified';
import Clickstream from './components/ClickstreamUnified';
import Layout from './components/LayoutUnified';
import LoadingSpinner from './components/LoadingSpinner';
import LoggedOut from './components/LoggedOut';
import KeycloakError from './components/KeycloakError';
import InitializationWizard from './components/InitializationWizard';
import './App.css';
import './components/DesignSystem.css';

interface AppState {
  keycloakInitialized: boolean;
  authenticated: boolean;
  loading: boolean;
  error: string | null;
  needsInitialization: boolean;
  checkingInitialization: boolean;
}

const App: React.FC = () => {
  const [state, setState] = useState<AppState>({
    keycloakInitialized: false,
    authenticated: false,
    loading: true,
    error: null,
    needsInitialization: false,
    checkingInitialization: true,
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
            needsInitialization: false,
            checkingInitialization: false,
          });
          return;
        }

        // Initialize Keycloak using the protected initialization function
        const authenticated = await initializeKeycloak(keycloakInitOptions);

        if (!authenticated) {
          setState({
            keycloakInitialized: true,
            authenticated: false,
            loading: false,
            error: null,
            needsInitialization: false,
            checkingInitialization: false,
          });
          return;
        }

        // Set up token refresh
        setInterval(() => {
          keycloak.updateToken(70).catch(() => {
            console.error('Failed to refresh token');
          });
        }, 60000); // Refresh every minute

        // Check if user needs to go through setup wizard
        try {
          console.log('Checking if user needs initialization...');

          // Check if user has any workspaces
          const workspaces = await apiService.workspaces.list();

          // Check if user has any domains
          const domains = await apiService.domains.list({ page: 1, pageSize: 1 });

          const needsSetup = workspaces.length === 0 || domains.data.length === 0;

          if (needsSetup) {
            console.log('User needs initialization - redirecting to setup wizard');
          } else {
            console.log('User already initialized');
          }

          setState({
            keycloakInitialized: true,
            authenticated: true,
            loading: false,
            error: null,
            needsInitialization: needsSetup,
            checkingInitialization: false,
          });
        } catch (error) {
          console.error('Failed to check initialization status:', error);
          // On error, assume user needs initialization to be safe
          setState({
            keycloakInitialized: true,
            authenticated: true,
            loading: false,
            error: null,
            needsInitialization: true,
            checkingInitialization: false,
          });
        }
      } catch (error) {
        console.error('Keycloak initialization failed:', error);
        setState({
          keycloakInitialized: false,
          authenticated: false,
          loading: false,
          error: 'Failed to initialize authentication. Please try again.',
          needsInitialization: false,
          checkingInitialization: false,
        });
      }
    };

    initKeycloak();
  }, []);

  if (state.loading || state.checkingInitialization) {
    return <LoadingSpinner message={state.loading ? "Initializing authentication..." : "Checking setup status..."} />;
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
        <Route path="/logged-out" element={<LoggedOut onLogin={() => keycloak.login(keycloakLoginOptions)} />} />

        {/* Setup Wizard - Protected but outside Layout */}
        <Route path="/setup" element={
          state.authenticated ? (
            <InitializationWizard onComplete={() => {
              // Refresh the page or update state to reflect completion
              setState(prev => ({ ...prev, needsInitialization: false }));
            }} />
          ) : (
            <Navigate to="/logged-out" replace />
          )
        } />

        {/* Protected Routes */}
        <Route path="/*" element={
          !state.authenticated ? (
            <Navigate to="/logged-out" replace />
          ) : state.needsInitialization ? (
            <Navigate to="/setup" replace />
          ) : (
            <Layout>
              <Routes>
                <Route path="/" element={<Navigate to="/dashboard" replace />} />
                <Route path="/dashboard" element={<Dashboard />} />
                <Route path="/routes" element={<RoutesPage />} />
                <Route path="/domains" element={<Domains />} />
                <Route path="/workspaces" element={<Workspaces />} />
                <Route path="/analytics" element={<Analytics />} />
                <Route path="/clickstream" element={<Clickstream />} />
                <Route path="/settings" element={<Settings />} />
                <Route path="*" element={<Navigate to="/dashboard" replace />} />
              </Routes>
            </Layout>
          )
        } />
      </Routes>
    </Router>
    </ThemeProvider>
  );
};

export default App;
