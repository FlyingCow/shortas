describe('development config flags', () => {
  const originalEnv = { ...process.env };

  afterEach(() => {
    process.env = { ...originalEnv };
    jest.resetModules();
  });

  it('isDevelopment is true when NODE_ENV is development', () => {
    jest.resetModules();
    process.env.NODE_ENV = 'development';
    const { isDevelopment } = require('./development');
    expect(isDevelopment).toBe(true);
  });

  it('isDevelopment is false when NODE_ENV is production', () => {
    jest.resetModules();
    process.env.NODE_ENV = 'production';
    const { isDevelopment } = require('./development');
    expect(isDevelopment).toBe(false);
  });

  it('useMockData is true only when REACT_APP_USE_MOCK_DATA is the string "true"', () => {
    jest.resetModules();
    process.env.REACT_APP_USE_MOCK_DATA = 'true';
    expect(require('./development').useMockData).toBe(true);

    jest.resetModules();
    process.env.REACT_APP_USE_MOCK_DATA = 'false';
    expect(require('./development').useMockData).toBe(false);

    jest.resetModules();
    delete process.env.REACT_APP_USE_MOCK_DATA;
    expect(require('./development').useMockData).toBe(false);
  });
});

describe('mock fixtures', () => {
  const { mockAnalytics, mockRoutes, mockUserSettings } = require('./development');

  it('mockAnalytics totals are internally consistent with its breakdowns', () => {
    const sumByCountry = mockAnalytics.clicks_by_country.reduce((sum: number, c: any) => sum + c.clicks, 0);
    expect(sumByCountry).toBeLessThanOrEqual(mockAnalytics.total_clicks);
    expect(mockAnalytics.clicks_by_date).toHaveLength(7);
  });

  it('mockRoutes each have a non-empty id and switch', () => {
    expect(mockRoutes.length).toBeGreaterThan(0);
    mockRoutes.forEach((route: any) => {
      expect(route.id).toBeTruthy();
      expect(route.switch).toBeTruthy();
      expect(route.properties).toBeDefined();
    });
  });

  it('mockUserSettings has the fields the settings UI expects', () => {
    expect(mockUserSettings.email).toContain('@');
    expect(Array.isArray(mockUserSettings.skip_tracking)).toBe(true);
  });
});
