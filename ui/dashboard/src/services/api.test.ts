import MockAdapter from 'axios-mock-adapter';

jest.mock('../config/keycloak', () => ({
  getToken: jest.fn(),
  updateToken: jest.fn(),
  isAuthenticated: jest.fn(),
  isInitialized: jest.fn(),
}));

describe('api instance interceptors', () => {
  let keycloak: typeof import('../config/keycloak');
  let routerApi: typeof import('./api').routerApi;
  let publicApi: typeof import('./api').publicApi;
  let mock: MockAdapter;
  let publicMock: MockAdapter;

  beforeEach(() => {
    jest.resetModules();
    // eslint-disable-next-line @typescript-eslint/no-var-requires
    keycloak = require('../config/keycloak');
    // eslint-disable-next-line @typescript-eslint/no-var-requires
    const api = require('./api');
    routerApi = api.routerApi;
    publicApi = api.publicApi;

    (keycloak.isInitialized as jest.Mock).mockReturnValue(true);
    (keycloak.isAuthenticated as jest.Mock).mockReturnValue(true);
    (keycloak.getToken as jest.Mock).mockReturnValue('access-token');
    (keycloak.updateToken as jest.Mock).mockResolvedValue(true);

    mock = new MockAdapter(routerApi);
    publicMock = new MockAdapter(publicApi);
  });

  afterEach(() => {
    mock.restore();
    publicMock.restore();
  });

  it('attaches a bearer token to authenticated requests', async () => {
    mock.onGet('/ping').reply((config) => {
      expect(config.headers?.Authorization).toBe('Bearer access-token');
      return [200, { ok: true }];
    });

    await routerApi.get('/ping');
  });

  it('refreshes the token before attaching it, falling back to the current token if refresh fails', async () => {
    (keycloak.updateToken as jest.Mock).mockRejectedValue(new Error('refresh failed'));
    mock.onGet('/ping').reply((config) => {
      expect(config.headers?.Authorization).toBe('Bearer access-token');
      return [200, {}];
    });

    await routerApi.get('/ping');
    expect(keycloak.updateToken).toHaveBeenCalledWith(30);
  });

  it('rejects the request when keycloak is not initialized', async () => {
    (keycloak.isInitialized as jest.Mock).mockReturnValue(false);
    mock.onGet('/ping').reply(200, {});

    await expect(routerApi.get('/ping')).rejects.toThrow(/not initialized/);
  });

  it('rejects the request when the user is not authenticated', async () => {
    (keycloak.isAuthenticated as jest.Mock).mockReturnValue(false);
    mock.onGet('/ping').reply(200, {});

    await expect(routerApi.get('/ping')).rejects.toThrow(/not authenticated/);
  });

  it('rejects the request when no token is available', async () => {
    (keycloak.getToken as jest.Mock).mockReturnValue(undefined);
    mock.onGet('/ping').reply(200, {});

    await expect(routerApi.get('/ping')).rejects.toThrow(/No authentication token/);
  });

  it('redirects to /logged-out on a 401 response', async () => {
    const original = window.location;
    // @ts-expect-error - jsdom location isn't directly reassignable
    delete window.location;
    window.location = { ...original, href: '' } as Location;

    mock.onGet('/secure').reply(401, { message: 'unauthorized' });

    await expect(routerApi.get('/secure')).rejects.toBeDefined();
    expect(window.location.href).toBe('/logged-out');

    window.location = original;
  });

  it('publicApi attaches a token when available but never rejects without one', async () => {
    publicMock.onGet('/public-thing').reply((config) => {
      expect(config.headers?.Authorization).toBe('Bearer access-token');
      return [200, {}];
    });
    await publicApi.get('/public-thing');

    (keycloak.isAuthenticated as jest.Mock).mockReturnValue(false);
    publicMock.onGet('/public-thing-2').reply((config) => {
      expect(config.headers?.Authorization).toBeUndefined();
      return [200, {}];
    });
    await expect(publicApi.get('/public-thing-2')).resolves.toBeDefined();
  });
});

describe('apiService (real network path)', () => {
  let keycloak: typeof import('../config/keycloak');
  let api: typeof import('./api');
  let mock: MockAdapter;

  beforeEach(() => {
    jest.resetModules();
    process.env.REACT_APP_USE_MOCK_DATA = 'false';
    // eslint-disable-next-line @typescript-eslint/no-var-requires
    keycloak = require('../config/keycloak');
    // eslint-disable-next-line @typescript-eslint/no-var-requires
    api = require('./api');

    (keycloak.isInitialized as jest.Mock).mockReturnValue(true);
    (keycloak.isAuthenticated as jest.Mock).mockReturnValue(true);
    (keycloak.getToken as jest.Mock).mockReturnValue('access-token');
    (keycloak.updateToken as jest.Mock).mockResolvedValue(true);

    mock = new MockAdapter(api.routerApi);
  });

  afterEach(() => {
    mock.restore();
    delete process.env.REACT_APP_USE_MOCK_DATA;
  });

  it('routes.list forwards params and returns the server pagination envelope', async () => {
    mock.onGet('/routes').reply((config) => {
      expect(config.params).toEqual({ page: 2, pageSize: 10 });
      return [200, { data: [{ id: 'r1' }], pagination: { page: 2, pageSize: 10, totalCount: 11, totalPages: 2 } }];
    });

    const result = await api.apiService.routes.list({ page: 2, pageSize: 10 });
    expect(result.data).toEqual([{ id: 'r1' }]);
    expect(result.pagination.totalPages).toBe(2);
  });

  it('routes.create POSTs the route body and returns the created route', async () => {
    mock.onPost('/routes').reply((config) => {
      expect(JSON.parse(config.data)).toEqual({ switch: 'main', link: 'abc' });
      return [201, { id: 'r1', switch: 'main', link: 'abc' }];
    });

    const result = await api.apiService.routes.create({ switch: 'main', link: 'abc' } as any);
    expect(result.id).toBe('r1');
  });

  it('routes.list propagates a server error', async () => {
    mock.onGet('/routes').reply(500, { message: 'db down' });
    await expect(api.apiService.routes.list()).rejects.toBeDefined();
  });

  it('analytics.getOverview maps country codes to display names on the real path', async () => {
    const aggregatorMock = new MockAdapter(api.aggregatorApi);
    aggregatorMock.onGet('/analytics/overview').reply(200, {
      total_clicks: 10,
      unique_clicks: 8,
      clicks_by_date: [],
      clicks_by_country: [{ country: 'us', clicks: 5 }],
    });

    const result = await api.apiService.analytics.getOverview();
    expect(result.clicks_by_country[0].country).toBe('United States');
    aggregatorMock.restore();
  });
});

describe('apiService (mock-data path)', () => {
  let api: typeof import('./api');

  beforeEach(() => {
    jest.resetModules();
    process.env.REACT_APP_USE_MOCK_DATA = 'true';
    // eslint-disable-next-line @typescript-eslint/no-var-requires
    api = require('./api');
  });

  afterEach(() => {
    delete process.env.REACT_APP_USE_MOCK_DATA;
  });

  it('routes.list paginates the in-memory mock routes instead of calling the network', async () => {
    const result = await api.apiService.routes.list({ page: 1, pageSize: 2 });
    expect(result.data.length).toBeLessThanOrEqual(2);
    expect(result.pagination.totalCount).toBeGreaterThan(0);
  });

  it('analytics.getOverview returns the static mock analytics payload', async () => {
    const result = await api.apiService.analytics.getOverview();
    expect(result.total_clicks).toBeGreaterThan(0);
    expect(Array.isArray(result.clicks_by_country)).toBe(true);
  });
});
