import {
  getRouteImagesBaseUrl,
  getKeycloakUrl,
  getKeycloakClientId,
  getProxyApiUrl,
} from './runtimeEnv';

describe('runtimeEnv', () => {
  const originalEnv = { ...process.env };

  afterEach(() => {
    window.__APP_ENV__ = undefined;
    process.env = { ...originalEnv };
  });

  it('prefers window.__APP_ENV__ over process.env', () => {
    window.__APP_ENV__ = { REACT_APP_ROUTE_IMAGES_BASE_URL: 'https://runtime.example.com' };
    process.env.REACT_APP_ROUTE_IMAGES_BASE_URL = 'https://buildtime.example.com';
    expect(getRouteImagesBaseUrl()).toBe('https://runtime.example.com');
  });

  it('falls back to process.env when window.__APP_ENV__ is not set', () => {
    window.__APP_ENV__ = undefined;
    process.env.REACT_APP_ROUTE_IMAGES_BASE_URL = 'https://buildtime.example.com';
    expect(getRouteImagesBaseUrl()).toBe('https://buildtime.example.com');
  });

  it('returns an empty string when neither source has a value', () => {
    window.__APP_ENV__ = undefined;
    delete process.env.REACT_APP_ROUTE_IMAGES_BASE_URL;
    expect(getRouteImagesBaseUrl()).toBe('');
  });

  it('getKeycloakUrl falls back to localhost default when unset', () => {
    window.__APP_ENV__ = undefined;
    delete process.env.REACT_APP_KEYCLOAK_URL;
    expect(getKeycloakUrl()).toBe('http://localhost:8080');
  });

  it('getKeycloakClientId falls back to shortas-dashboard default when unset', () => {
    window.__APP_ENV__ = undefined;
    delete process.env.REACT_APP_KEYCLOAK_CLIENT_ID;
    expect(getKeycloakClientId()).toBe('shortas-dashboard');
  });

  it('getProxyApiUrl falls back to localhost:8090 default when unset', () => {
    window.__APP_ENV__ = undefined;
    delete process.env.REACT_APP_PROXY_API_URL;
    expect(getProxyApiUrl()).toBe('http://localhost:8090');
  });

  it('getKeycloakUrl prefers an explicit runtime value over the default', () => {
    window.__APP_ENV__ = { REACT_APP_KEYCLOAK_URL: 'https://auth.example.com' };
    expect(getKeycloakUrl()).toBe('https://auth.example.com');
  });
});
