/**
 * Runtime config from env.js (injected by Docker entrypoint from Compose env).
 * Falls back to process.env (build-time) when not set at runtime.
 */
declare global {
  interface Window {
    __APP_ENV__?: {
      REACT_APP_ROUTE_IMAGES_BASE_URL?: string;
      REACT_APP_KEYCLOAK_URL?: string;
      REACT_APP_KEYCLOAK_CLIENT_ID?: string;
      REACT_APP_PROXY_API_URL?: string;
    };
  }
}

function getRuntimeEnv(key: keyof NonNullable<Window['__APP_ENV__']>): string {
  if (typeof window !== 'undefined' && window.__APP_ENV__?.[key] != null) {
    const v = window.__APP_ENV__[key];
    if (typeof v === 'string') return v;
  }
  const buildEnv = process.env[key];
  return typeof buildEnv === 'string' ? buildEnv : '';
}

export const getRouteImagesBaseUrl = (): string =>
  getRuntimeEnv('REACT_APP_ROUTE_IMAGES_BASE_URL');

export const getKeycloakUrl = (): string =>
  getRuntimeEnv('REACT_APP_KEYCLOAK_URL') || 'http://localhost:8080';

export const getKeycloakClientId = (): string =>
  getRuntimeEnv('REACT_APP_KEYCLOAK_CLIENT_ID') || 'shortas-dashboard';

export const getProxyApiUrl = (): string =>
  getRuntimeEnv('REACT_APP_PROXY_API_URL') || 'http://localhost:8090';
