// Development configuration
export const isDevelopment = process.env.NODE_ENV === 'development';

// Mock data flag - set to true to use mock data instead of real API calls
export const useMockData = process.env.REACT_APP_USE_MOCK_DATA === 'true' || false;

// Mock analytics data for development
export const mockAnalytics = {
  total_clicks: 15420,
  unique_clicks: 8765,
  clicks_by_date: [
    { date: '2024-01-01', clicks: 1200 },
    { date: '2024-01-02', clicks: 1350 },
    { date: '2024-01-03', clicks: 980 },
    { date: '2024-01-04', clicks: 1500 },
    { date: '2024-01-05', clicks: 1100 },
    { date: '2024-01-06', clicks: 1300 },
    { date: '2024-01-07', clicks: 1400 },
  ],
  clicks_by_country: [
    { country: 'US', clicks: 5500 },
    { country: 'UK', clicks: 2100 },
    { country: 'CA', clicks: 1800 },
    { country: 'DE', clicks: 1200 },
    { country: 'FR', clicks: 900 },
  ],
  clicks_by_device: [
    { device: 'Desktop', clicks: 8500 },
    { device: 'Mobile', clicks: 5200 },
    { device: 'Tablet', clicks: 1720 },
  ],
  clicks_by_browser: [
    { browser: 'Chrome', clicks: 9200 },
    { browser: 'Firefox', clicks: 3100 },
    { browser: 'Safari', clicks: 2400 },
    { browser: 'Edge', clicks: 720 },
  ],
};

// Mock routes data
export const mockRoutes = [
  {
    switch: 'main',
    link: 'example.com/example1',
    dest: 'https://example.com/page1',
    destFormat: 'Http',
    code: 301,
    ttl: 3600,
    status: 'Active',
    terminal: 'External',
    properties: {
      routeId: 'route-1',
      domainId: 'example.com',
      ownerId: 'user-1',
      scripts: [],
      tags: ['demo'],
      custom: {},
      opengraph: true,
      allowDebug: false,
    },
  },
  {
    switch: 'main',
    link: 'example.com/example2',
    dest: 'https://example.com/page2',
    destFormat: 'Http',
    code: 302,
    ttl: 1800,
    status: 'Active',
    terminal: 'External',
    properties: {
      routeId: 'route-2',
      domainId: 'example.com',
      ownerId: 'user-1',
      scripts: [],
      tags: ['demo'],
      custom: {},
      opengraph: false,
      allowDebug: true,
    },
  },
  {
    switch: 'main',
    link: 'example.com/promo',
    dest: 'https://example.com/special-offer',
    destFormat: 'Http',
    code: 307,
    ttl: 7200,
    status: 'Active',
    terminal: 'External',
    properties: {
      routeId: 'route-3',
      domainId: 'example.com',
      ownerId: 'user-1',
      scripts: [],
      tags: ['promo', 'marketing'],
      custom: {},
      opengraph: true,
      allowDebug: false,
    },
  },
];

// Mock user settings
export const mockUserSettings = {
  email: 'demo@shortas.com',
  status: 'active',
  debug: false,
  overflow: true,
  skip_tracking: ['utm_source', 'utm_medium'],
  allowed_request_params: ['utm_source', 'utm_medium', 'utm_campaign'],
  allowed_destination_params: ['redirect', 'target', 'next'],
};

