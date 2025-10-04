// Debug utilities for development
export const debugLog = (message: string, data?: any) => {
  if (process.env.NODE_ENV === 'development') {
    console.log(`[Shortas Debug] ${message}`, data || '');
  }
};

export const debugWarn = (message: string, data?: any) => {
  if (process.env.NODE_ENV === 'development') {
    console.warn(`[Shortas Warning] ${message}`, data || '');
  }
};

export const debugError = (message: string, error?: any) => {
  if (process.env.NODE_ENV === 'development') {
    console.error(`[Shortas Error] ${message}`, error || '');
  }
};
