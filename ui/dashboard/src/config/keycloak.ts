import Keycloak from 'keycloak-js';
import { debugLog, debugError } from '../utils/debug';

// Keycloak configuration for shortas-dev realm
const keycloakConfig = {
  url: process.env.REACT_APP_KEYCLOAK_URL || 'http://localhost:8080',
  realm: 'shortas-dev',
  clientId: process.env.REACT_APP_KEYCLOAK_CLIENT_ID || 'shortas-dashboard',
};

// Initialize Keycloak instance with singleton pattern
let keycloakInstance: Keycloak | null = null;

const getKeycloakInstance = (): Keycloak => {
  if (!keycloakInstance) {
    debugLog('Creating new Keycloak instance', keycloakConfig);
    keycloakInstance = new Keycloak(keycloakConfig);
  }
  return keycloakInstance;
};

const keycloak = getKeycloakInstance();

export default keycloak;

// Keycloak initialization options
export const keycloakInitOptions = {
  onLoad: 'check-sso' as const, // Check if user is already logged in, don't force login
  checkLoginIframe: false,
  pkceMethod: 'S256' as const,
};

// Login options for when user clicks login
export const keycloakLoginOptions = {
  redirectUri: window.location.origin,
};

// Helper functions for token management
export const getToken = (): string | undefined => {
  return keycloak.token;
};

export const getRefreshToken = (): string | undefined => {
  return keycloak.refreshToken;
};

export const getUserInfo = () => {
  return {
    username: keycloak.tokenParsed?.preferred_username,
    email: keycloak.tokenParsed?.email,
    name: keycloak.tokenParsed?.name,
    roles: keycloak.tokenParsed?.realm_access?.roles || [],
    sub: keycloak.tokenParsed?.sub,
  };
};

export const hasRole = (role: string): boolean => {
  return keycloak.hasRealmRole(role);
};

export const logout = () => {
  keycloak.logout({
    redirectUri: window.location.origin + '/login',
  });
};

export const updateToken = (minValidity = 30): Promise<boolean> => {
  return keycloak.updateToken(minValidity);
};

// Check if Keycloak is initialized and authenticated
export const isAuthenticated = (): boolean => {
  return keycloak.authenticated || false;
};

// Track initialization state
let initializationPromise: Promise<boolean> | null = null;
let isKeycloakInitialized = false;

// Initialize Keycloak with guard against multiple initialization
export const initializeKeycloak = async (options: any): Promise<boolean> => {
  // If already initialized, return the result
  if (isKeycloakInitialized) {
    debugLog('Keycloak already initialized, returning cached result');
    return keycloak.authenticated || false;
  }
  
  // If initialization is in progress, wait for it
  if (initializationPromise) {
    debugLog('Keycloak initialization in progress, waiting...');
    return initializationPromise;
  }
  
  debugLog('Starting Keycloak initialization', options);
  
  // Start initialization
  initializationPromise = keycloak.init(options).then((authenticated) => {
    debugLog('Keycloak initialization successful', { authenticated });
    isKeycloakInitialized = true;
    return authenticated;
  }).catch((error) => {
    debugError('Keycloak initialization failed', error);
    isKeycloakInitialized = false;
    initializationPromise = null; // Reset on error to allow retry
    
    // Provide helpful error messages based on error type
    let helpfulMessage = 'Failed to initialize authentication.';
    
    if (error.message?.includes('Failed to fetch') || error.message?.includes('NetworkError')) {
      helpfulMessage = `Keycloak server is not reachable at ${keycloakConfig.url}. Please start Keycloak server or enable mock data mode (REACT_APP_USE_MOCK_DATA=true).`;
    } else if (error.message?.includes('404') || error.message?.includes('Not Found')) {
      helpfulMessage = `Keycloak realm '${keycloakConfig.realm}' not found. Please create the realm or enable mock data mode.`;
    } else if (error.message?.includes('401') || error.message?.includes('Unauthorized')) {
      helpfulMessage = `Keycloak client '${keycloakConfig.clientId}' is not properly configured. Please check client settings or enable mock data mode.`;
    }
    
    // Create enhanced error with helpful message
    const enhancedError = new Error(helpfulMessage);
    enhancedError.stack = error.stack;
    throw enhancedError;
  });
  
  return initializationPromise;
};

// Check if Keycloak is initialized
export const isInitialized = (): boolean => {
  return isKeycloakInitialized;
};
