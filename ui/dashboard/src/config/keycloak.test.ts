// Keycloak.ts keeps module-level singleton state (the Keycloak instance,
// plus init caching/dedup flags), so every test resets modules and
// re-requires it fresh rather than importing it once at the top of the file.

jest.mock('keycloak-js', () => {
  return jest.fn().mockImplementation(() => ({
    init: jest.fn(),
    updateToken: jest.fn(),
    logout: jest.fn(),
    hasRealmRole: jest.fn(),
    token: undefined,
    refreshToken: undefined,
    authenticated: undefined,
    tokenParsed: undefined,
  }));
});

function freshKeycloakModule() {
  jest.resetModules();
  // eslint-disable-next-line @typescript-eslint/no-var-requires
  const KeycloakCtor = require('keycloak-js');
  // eslint-disable-next-line @typescript-eslint/no-var-requires
  const keycloakModule = require('./keycloak');
  const mockInstance = KeycloakCtor.mock.results[KeycloakCtor.mock.results.length - 1].value;
  return { keycloakModule, mockInstance, KeycloakCtor };
}

describe('keycloak helpers', () => {
  it('getToken/getRefreshToken delegate to the keycloak instance', () => {
    const { keycloakModule, mockInstance } = freshKeycloakModule();
    mockInstance.token = 'access-token';
    mockInstance.refreshToken = 'refresh-token';

    expect(keycloakModule.getToken()).toBe('access-token');
    expect(keycloakModule.getRefreshToken()).toBe('refresh-token');
  });

  it('getUserInfo extracts fields from the parsed token, defaulting roles to []', () => {
    const { keycloakModule, mockInstance } = freshKeycloakModule();
    mockInstance.tokenParsed = {
      preferred_username: 'jane',
      email: 'jane@example.com',
      name: 'Jane Doe',
      sub: 'user-1',
    };

    const info = keycloakModule.getUserInfo();
    expect(info).toEqual({
      username: 'jane',
      email: 'jane@example.com',
      name: 'Jane Doe',
      roles: [],
      sub: 'user-1',
    });
  });

  it('getUserInfo surfaces realm roles when present', () => {
    const { keycloakModule, mockInstance } = freshKeycloakModule();
    mockInstance.tokenParsed = { realm_access: { roles: ['admin', 'user'] } };

    expect(keycloakModule.getUserInfo().roles).toEqual(['admin', 'user']);
  });

  it('hasRole delegates to keycloak.hasRealmRole', () => {
    const { keycloakModule, mockInstance } = freshKeycloakModule();
    mockInstance.hasRealmRole.mockReturnValue(true);

    expect(keycloakModule.hasRole('admin')).toBe(true);
    expect(mockInstance.hasRealmRole).toHaveBeenCalledWith('admin');
  });

  it('isAuthenticated defaults to false when keycloak.authenticated is undefined', () => {
    const { keycloakModule } = freshKeycloakModule();
    expect(keycloakModule.isAuthenticated()).toBe(false);
  });

  it('isAuthenticated reflects a truthy keycloak.authenticated', () => {
    const { keycloakModule, mockInstance } = freshKeycloakModule();
    mockInstance.authenticated = true;
    expect(keycloakModule.isAuthenticated()).toBe(true);
  });

  it('logout calls keycloak.logout with the logged-out redirect URI', () => {
    const { keycloakModule, mockInstance } = freshKeycloakModule();
    keycloakModule.logout();
    expect(mockInstance.logout).toHaveBeenCalledWith({
      redirectUri: `${window.location.origin}/logged-out`,
    });
  });

  it('updateToken delegates to keycloak.updateToken with the given minValidity', () => {
    const { keycloakModule, mockInstance } = freshKeycloakModule();
    mockInstance.updateToken.mockResolvedValue(true);

    keycloakModule.updateToken(60);
    expect(mockInstance.updateToken).toHaveBeenCalledWith(60);
  });

  describe('initializeKeycloak', () => {
    it('initializes once and reports authenticated on success', async () => {
      const { keycloakModule, mockInstance } = freshKeycloakModule();
      mockInstance.init.mockResolvedValue(true);

      const result = await keycloakModule.initializeKeycloak({ onLoad: 'check-sso' });

      expect(result).toBe(true);
      expect(keycloakModule.isInitialized()).toBe(true);
      expect(mockInstance.init).toHaveBeenCalledTimes(1);
    });

    it('returns the cached result on subsequent calls without re-initializing', async () => {
      const { keycloakModule, mockInstance } = freshKeycloakModule();
      mockInstance.init.mockResolvedValue(true);
      mockInstance.authenticated = true;

      await keycloakModule.initializeKeycloak({});
      await keycloakModule.initializeKeycloak({});

      expect(mockInstance.init).toHaveBeenCalledTimes(1);
    });

    it('dedupes concurrent calls into a single in-flight init', async () => {
      const { keycloakModule, mockInstance } = freshKeycloakModule();
      let resolveInit: (value: boolean) => void;
      mockInstance.init.mockReturnValue(new Promise((resolve) => { resolveInit = resolve; }));

      const first = keycloakModule.initializeKeycloak({});
      const second = keycloakModule.initializeKeycloak({});

      resolveInit!(true);
      const [a, b] = await Promise.all([first, second]);

      expect(a).toBe(true);
      expect(b).toBe(true);
      expect(mockInstance.init).toHaveBeenCalledTimes(1);
    });

    it('resets state on failure (allowing a retry) and maps network errors to a helpful message', async () => {
      const { keycloakModule, mockInstance } = freshKeycloakModule();
      mockInstance.init.mockRejectedValueOnce(new Error('Failed to fetch'));

      await expect(keycloakModule.initializeKeycloak({})).rejects.toThrow(/not reachable/);
      expect(keycloakModule.isInitialized()).toBe(false);

      // A second attempt should call init() again, not reuse a rejected promise.
      mockInstance.init.mockResolvedValueOnce(true);
      const result = await keycloakModule.initializeKeycloak({});
      expect(result).toBe(true);
      expect(mockInstance.init).toHaveBeenCalledTimes(2);
    });

    it('maps a 404 error to a realm-not-found message', async () => {
      const { keycloakModule, mockInstance } = freshKeycloakModule();
      mockInstance.init.mockRejectedValueOnce(new Error('404 Not Found'));

      await expect(keycloakModule.initializeKeycloak({})).rejects.toThrow(/realm 'shortas-dev' not found/);
    });

    it('maps a 401 error to a client-configuration message', async () => {
      const { keycloakModule, mockInstance } = freshKeycloakModule();
      mockInstance.init.mockRejectedValueOnce(new Error('401 Unauthorized'));

      await expect(keycloakModule.initializeKeycloak({})).rejects.toThrow(/not properly configured/);
    });
  });
});
