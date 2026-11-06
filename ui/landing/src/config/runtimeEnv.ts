/**
 * Runtime config from env.js (injected by Docker entrypoint from Compose env).
 * Falls back to process.env (build-time) when not set at runtime.
 */
declare global {
  interface Window {
    __APP_ENV__?: {
      REACT_APP_DASHBOARD_URL?: string;
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

export const getDashboardUrl = (): string =>
  getRuntimeEnv('REACT_APP_DASHBOARD_URL') || 'http://localhost:3000';
