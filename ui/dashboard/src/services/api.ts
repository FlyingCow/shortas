import axios, { AxiosInstance, AxiosRequestConfig } from 'axios';
import { getToken, updateToken, isAuthenticated, isInitialized } from '../config/keycloak';
import { useMockData, mockAnalytics, mockRoutes, mockUserSettings } from '../config/development';

// API base URLs
const PROXY_API_URL = process.env.REACT_APP_PROXY_API_URL || 'http://localhost:8090';
const ROUTER_API_URL = `${PROXY_API_URL}/api/v1`;
const AGGREGATOR_API_URL = `${PROXY_API_URL}/api/aggregator/v1`;

// Create axios instances
const createApiInstance = (baseURL: string): AxiosInstance => {
  const instance = axios.create({
    baseURL,
    timeout: 10000,
    headers: {
      'Content-Type': 'application/json',
    },
  });

  // Request interceptor to add auth token
  instance.interceptors.request.use(
    async (config) => {
      try {
        // Check if Keycloak is initialized and user is authenticated
        if (!isInitialized()) {
          return Promise.reject(new Error('Keycloak not initialized'));
        }

        if (!isAuthenticated()) {
          return Promise.reject(new Error('User not authenticated'));
        }

        const token = getToken();
        if (!token) {
          return Promise.reject(new Error('No authentication token available'));
        }

        // Try to refresh the token, but fall back to the current token if refresh fails
        try {
          await updateToken(30);
        } catch {
          // Token refresh failed (e.g. Keycloak unreachable) — use existing token
        }
        const currentToken = getToken();
        if (currentToken) {
          config.headers.Authorization = `Bearer ${currentToken}`;
        }
      } catch (error) {
        console.error('Failed to set auth token:', error);
        return Promise.reject(error);
      }
      return config;
    },
    (error) => {
      return Promise.reject(error);
    }
  );

  // Response interceptor for error handling
  instance.interceptors.response.use(
    (response) => response,
    (error) => {
      if (error.response?.status === 401) {
        // Token expired or invalid, redirect to logged out page
        window.location.href = '/logged-out';
      }
      return Promise.reject(error);
    }
  );

  return instance;
};

// API instances
export const routerApi = createApiInstance(ROUTER_API_URL);
export const aggregatorApi = createApiInstance(AGGREGATOR_API_URL);

// Conditional Routing Types
export interface Expression {
  default_operator?: 'And' | 'Or';
  ua?: StringCondition;
  os?: StringCondition;
  device?: StringCondition;
  lang?: StringCondition;
  country?: StringCondition;
  date?: DateCondition;
  rnd?: NumericCondition;
  day_of_week?: NumericCondition;
  day_of_month?: NumericCondition;
  month?: NumericCondition;
  and?: Expression[];
  or?: Expression[];
}

export interface StringCondition {
  eq?: string;
  starts?: string;
  ends?: string;
  in?: string[];
}

export interface NumericCondition {
  eq?: number;
  gt?: number;
  lt?: number;
  in?: number[];
}

export interface DateCondition {
  eq?: string;
  gt?: string;
  lt?: string;
  in?: string[];
}

export interface ConditionalRouting {
  key: string;
  condition: Expression;
  dest?: string;  // Destination URL stored inline in the policy
}

export interface ConditionRouteDto {
  dest: string;           // Destination URL for this condition
  condition: Expression;  // Matching conditions
}

export interface ChallengeRouting {
  type?: string;
  title?: string;
  message?: string;
}

export interface FileRouting {
  path?: string;
  mime_type?: string;
}

export type RoutingPolicy =
  | 'Basic'
  | 'Mirroring'
  | { Conditional: ConditionalRouting[] }
  | { Challenge: ChallengeRouting }
  | { File: FileRouting };

// API Types
export interface RouteDto {
  id?: string;
  switch: string;
  link: string;
  dest: string;
  destFormat: string;
  code: number;
  ttl: number;
  status: string;
  terminal: string;
  policy?: RoutingPolicy;
  domainId?: string;
  domain?: DomainDto;
  conditions?: ConditionRouteDto[];  // Conditional routes for master/child pattern
  properties?: {
    routeId: string;
    domainId: string;
    ownerId: string;
    creatorId?: string;
    workspaceId?: string;
    scripts: string[];
    tags: string[];
    custom: Record<string, any>;
    native?: Record<string, any>;
    bundling?: Record<string, any>;
    opengraph: boolean;
    allowDebug: boolean;
  };
}

export interface PaginatedResponse<T> {
  data: T[];
  pagination: {
    page: number;
    pageSize: number;
    totalCount: number;
    totalPages: number;
  };
}

export interface ClickAnalytics {
  total_clicks: number;
  unique_clicks: number;
  clicks_by_date: Array<{
    date: string;
    clicks: number;
  }>;
  clicks_by_country: Array<{
    country: string;
    clicks: number;
  }>;
  clicks_by_device: Array<{
    device: string;
    clicks: number;
  }>;
  clicks_by_browser: Array<{
    browser: string;
    clicks: number;
  }>;
}

export interface ClickStreamEvent {
  id: string;
  ownerId: string;
  creatorId: string;
  routeId: string;
  routeName?: string | null;
  routeDomainName?: string | null;
  workspaceId: string;
  created: string;
  dest: string;
  ip: string;
  continent: string | null;
  country: string | null;
  location: string | null;
  osFamily: string | null;
  osVersion: string | null;
  userAgentFamily: string | null;
  userAgentVersion: string | null;
  deviceBrand: string | null;
  deviceFamily: string | null;
  deviceModel: string | null;
  sessionFirst: string;
  sessionClicks: number;
  isUnique: boolean;
  isBot: boolean;
}

export interface ClickStreamStats {
  totalClicks: number;
  uniqueClicks: number;
  botClicks: number;
  topCountries: Array<{
    country: string;
    count: number;
  }>;
  topDevices: Array<{
    device: string;
    count: number;
  }>;
  clickTrends: Array<{
    date: string;
    clicks: number;
  }>;
}

// Statistics DTOs (from materialized views)
// Using snake_case to match API response
export interface DailyStatsDto {
  date: string;
  total_clicks: number;
  unique_clicks: number;
  bot_clicks: number;
  human_clicks: number;
  unique_ips: number;
}

export interface HourlyStatsDto {
  hour: string;
  total_clicks: number;
  unique_clicks: number;
  bot_clicks: number;
  human_clicks: number;
  unique_ips: number;
}

export interface GeographicStatsDto {
  continent?: string;
  country: string;
  location?: string;
  total_clicks: number;
  unique_clicks: number;
  unique_ips: number;
}

export interface DeviceStatsDto {
  device_family: string;
  os_family: string;
  total_clicks: number;
  unique_clicks: number;
}

export interface BrowserStatsDto {
  user_agent_family: string;
  user_agent_version?: string;
  total_clicks: number;
  unique_clicks: number;
}

export interface RoutePerformanceDto {
  route_id: string;
  route_name?: string | null;
  route_domain_name?: string | null;
  total_clicks: number;
  unique_visitors: number;
  bot_clicks: number;
  human_clicks: number;
  countries_reached: number;
  device_types: number;
}

export interface TopDestinationDto {
  dest: string;
  total_clicks: number;
  unique_visitors: number;
}

export interface TrafficTypeStatsDto {
  is_bot: boolean;
  total_clicks: number;
  unique_ips: number;
}

// Route Search Types (Elasticsearch)
export interface RouteSearchResult {
  id: string;
  link: string;
  switch: string;
  dest?: string;
  domainName?: string;
  status: string;
  ownerId?: string;
  workspaceId?: string;
}

export interface SearchPaginatedResponse<T> {
  data: T[];
  pagination: {
    page: number;
    pageSize: number;
    totalCount: number;
    totalPages: number;
  };
}

// Domain Types
export type DomainVerificationStatus = 'Pending' | 'Verified' | 'Failed';

export interface DomainDto {
  id: string;
  name: string;
  ownerId: string;
  verificationStatus: DomainVerificationStatus;
  verificationReason: string;
  lastVerificationCheck?: string;
  nextVerificationCheck?: string;
}

export interface DnsConfigDto {
  txtRecordName: string;
  allowedIpv4: string[];
  allowedIpv6: string[];
}

export interface CreateDomainDto {
  name: string;
}

export interface UpdateDomainDto {
  name: string;
}

// Certificate Types
export interface CertificateDto {
  id: string;
  key: string;
  cert: string;
  ocspResp?: string;
  ownerId: string;
  domainId: string;
  domain?: DomainDto;
}

// Workspace Types
export interface WorkspaceDto {
  id: string;
  name: string;
  description: string;
  type: string;  // "System" or "User"
  createdAt: string;
  updatedAt: string;
  userRole?: string;
  members?: UserWorkspaceDto[];
  isSystem?: boolean;
}

export interface UserWorkspaceDto {
  id: string;
  userId: string;
  workspaceId: string;
  role: string;
  joinedAt: string;
}

export interface CreateWorkspaceDto {
  name: string;
  description: string;
}

export interface UpdateWorkspaceDto {
  name?: string;
  description?: string;
}

export interface InitializationResponse {
  workspace: WorkspaceDto | null;
  userSettings: any | null;
  message: string;
}

export interface InitializationStatusResponse {
  needsInitialization: boolean;
  hasWorkspaces: boolean;
  hasDomains: boolean;
  hasUserSettings: boolean;
}

// Helper function to simulate API delay
const delay = (ms: number) => new Promise(resolve => setTimeout(resolve, ms));

// API Methods
export const apiService = {
  // Routes API
  routes: {
    list: async (params?: { page?: number; pageSize?: number; search?: string; status?: string; workspaceId?: string }): Promise<PaginatedResponse<RouteDto>> => {
      if (useMockData) {
        await delay(500); // Simulate network delay
        const page = params?.page || 1;
        const pageSize = params?.pageSize || 20;
        const start = (page - 1) * pageSize;
        const end = start + pageSize;
        const paginatedRoutes = mockRoutes.slice(start, end);

        return {
          data: paginatedRoutes,
          pagination: {
            page,
            pageSize,
            totalCount: mockRoutes.length,
            totalPages: Math.ceil(mockRoutes.length / pageSize),
          },
        };
      }
      const response = await routerApi.get('/routes', { params });
      return response.data;
    },

    get: async (id: string) => {
      const response = await routerApi.get(`/routes/${id}`);
      return response.data;
    },

    create: async (route: Partial<RouteDto>) => {
      const response = await routerApi.post('/routes', route);
      return response.data;
    },

    update: async (id: string, route: Partial<RouteDto>) => {
      const response = await routerApi.put(`/routes/${id}`, route);
      return response.data;
    },

    delete: async (id: string) => {
      const response = await routerApi.delete(`/routes/${id}`);
      return response.data;
    },

    bulkCreate: async (routes: Partial<RouteDto>[]) => {
      const response = await routerApi.post('/routes/bulk', routes);
      return response.data;
    },

    bulkUpdate: async (routes: Partial<RouteDto>[]) => {
      const response = await routerApi.put('/routes/bulk', routes);
      return response.data;
    },

    bulkDelete: async (routeIds: string[]) => {
      const response = await routerApi.delete('/routes/bulk', { data: routeIds });
      return response.data;
    },

    suggestLink: async (domainId: string): Promise<{ link: string }> => {
      const response = await routerApi.get('/routes/suggest-link', { params: { domainId } });
      return response.data;
    },

    search: async (params: { q: string; page?: number; pageSize?: number; workspaceId?: string }): Promise<SearchPaginatedResponse<RouteSearchResult>> => {
      const response = await routerApi.get('/routes/search', { params });
      return response.data;
    },

    reindex: async (): Promise<{ message: string; count: number }> => {
      const response = await routerApi.post('/routes/search/reindex');
      return response.data;
    },
  },

  // Analytics API
  analytics: {
    getOverview: async (dateRange?: { start: string; end: string }): Promise<ClickAnalytics> => {
      if (useMockData) {
        await delay(800); // Simulate network delay
        return mockAnalytics;
      }
      const response = await aggregatorApi.get('/analytics/overview', {
        params: dateRange,
      });
      return response.data;
    },
    
    getRouteAnalytics: async (routeId: string, dateRange?: { start: string; end: string }) => {
      const response = await aggregatorApi.get(`/analytics/routes/${routeId}`, {
        params: dateRange,
      });
      return response.data;
    },
    
    getTopRoutes: async (limit = 10) => {
      if (useMockData) {
        await delay(600);
        return mockRoutes.slice(0, limit).map((route, index) => ({
          ...route,
          total_clicks: Math.floor(Math.random() * 5000) + 1000,
          unique_clicks: Math.floor(Math.random() * 3000) + 500,
        }));
      }
      const response = await aggregatorApi.get('/analytics/top-routes', {
        params: { limit },
      });
      return response.data;
    },
  },

  // User Settings API
  userSettings: {
    get: async (userId: string) => {
      if (useMockData) {
        await delay(400);
        return mockUserSettings;
      }
      const response = await routerApi.get(`/user-settings/${userId}`);
      return response.data;
    },

    update: async (userId: string, settings: any) => {
      const response = await routerApi.put(`/user-settings/${userId}`, settings);
      return response.data;
    },
  },

  // ClickStream API
  clickstream: {
    getAll: async (params?: { routeId?: string; startDate?: string; endDate?: string }): Promise<ClickStreamEvent[]> => {
      const response = await routerApi.get('/clickstream', { params });
      return response.data;
    },

    getByRoute: async (routeId: string, params?: { startDate?: string; endDate?: string }): Promise<ClickStreamEvent[]> => {
      const response = await routerApi.get(`/clickstream/${routeId}`, { params });
      return response.data;
    },

    getStats: async (params?: { routeId?: string; startDate?: string; endDate?: string }): Promise<ClickStreamStats> => {
      const response = await routerApi.get('/clickstream/stats', { params });
      return response.data;
    },

    // Materialized view statistics endpoints
    getDailyStats: async (params?: { routeId?: string; fromDate?: string; toDate?: string }): Promise<DailyStatsDto[]> => {
      const response = await routerApi.get('/clickstream/stats/daily', { params });
      return response.data;
    },

    getHourlyStats: async (params?: { routeId?: string; fromHour?: string; toHour?: string }): Promise<HourlyStatsDto[]> => {
      const response = await routerApi.get('/clickstream/stats/hourly', { params });
      return response.data;
    },

    getGeographicStats: async (params?: { routeId?: string; fromDate?: string; toDate?: string }): Promise<GeographicStatsDto[]> => {
      const response = await routerApi.get('/clickstream/stats/geographic', { params });
      return response.data;
    },

    getDeviceStats: async (params?: { routeId?: string; fromDate?: string; toDate?: string }): Promise<DeviceStatsDto[]> => {
      const response = await routerApi.get('/clickstream/stats/devices', { params });
      return response.data;
    },

    getBrowserStats: async (params?: { routeId?: string; fromDate?: string; toDate?: string }): Promise<BrowserStatsDto[]> => {
      const response = await routerApi.get('/clickstream/stats/browsers', { params });
      return response.data;
    },

    getRoutePerformance: async (params?: { fromDate?: string; toDate?: string; limit?: number }): Promise<RoutePerformanceDto[]> => {
      const response = await routerApi.get('/clickstream/stats/route-performance', { params });
      return response.data;
    },

    getTopDestinations: async (params?: { routeId?: string; fromDate?: string; toDate?: string; limit?: number }): Promise<TopDestinationDto[]> => {
      const response = await routerApi.get('/clickstream/stats/top-destinations', { params });
      return response.data;
    },

    getTrafficTypeStats: async (params?: { routeId?: string; fromHour?: string; toHour?: string }): Promise<TrafficTypeStatsDto[]> => {
      const response = await routerApi.get('/clickstream/stats/traffic-types', { params });
      return response.data;
    },
  },

  // Domains API
  domains: {
    list: async (params?: { page?: number; pageSize?: number; search?: string }): Promise<PaginatedResponse<DomainDto>> => {
      const response = await routerApi.get('/domains', { params });
      return response.data;
    },

    get: async (id: string): Promise<DomainDto> => {
      const response = await routerApi.get(`/domains/${id}`);
      return response.data;
    },

    getByName: async (name: string): Promise<DomainDto> => {
      const response = await routerApi.get(`/domains/by-name/${name}`);
      return response.data;
    },

    create: async (domain: CreateDomainDto): Promise<DomainDto> => {
      const response = await routerApi.post('/domains', domain);
      return response.data;
    },

    update: async (id: string, domain: UpdateDomainDto): Promise<DomainDto> => {
      const response = await routerApi.put(`/domains/${id}`, domain);
      return response.data;
    },

    delete: async (id: string): Promise<void> => {
      const response = await routerApi.delete(`/domains/${id}`);
      return response.data;
    },

    triggerVerification: async (id: string): Promise<DomainDto> => {
      const response = await routerApi.post(`/domains/${id}/verify`);
      return response.data;
    },

    getDnsConfig: async (): Promise<DnsConfigDto> => {
      const response = await routerApi.get('/domains/dns-config');
      return response.data;
    },
  },

  // Certificates API
  certificates: {
    list: async (params?: { page?: number; pageSize?: number; search?: string; domainId?: string }): Promise<PaginatedResponse<CertificateDto>> => {
      const response = await routerApi.get('/certificates', { params });
      return response.data;
    },

    getByDomain: async (domainId: string): Promise<CertificateDto> => {
      const response = await routerApi.get(`/certificates/by-domain/${domainId}`);
      return response.data;
    },

    create: async (certificate: Partial<CertificateDto>): Promise<CertificateDto> => {
      const response = await routerApi.post('/certificates', certificate);
      return response.data;
    },

    update: async (id: string, certificate: Partial<CertificateDto>): Promise<CertificateDto> => {
      const response = await routerApi.put(`/certificates/${id}`, certificate);
      return response.data;
    },

    delete: async (id: string): Promise<void> => {
      const response = await routerApi.delete(`/certificates/${id}`);
      return response.data;
    },
  },

  // Workspaces API
  workspaces: {
    list: async (): Promise<WorkspaceDto[]> => {
      const response = await routerApi.get('/workspaces');
      return response.data;
    },

    get: async (id: string): Promise<WorkspaceDto> => {
      const response = await routerApi.get(`/workspaces/${id}`);
      return response.data;
    },

    create: async (workspace: CreateWorkspaceDto): Promise<WorkspaceDto> => {
      const response = await routerApi.post('/workspaces', workspace);
      return response.data;
    },

    update: async (id: string, workspace: UpdateWorkspaceDto): Promise<WorkspaceDto> => {
      const response = await routerApi.put(`/workspaces/${id}`, workspace);
      return response.data;
    },

    delete: async (id: string): Promise<void> => {
      await routerApi.delete(`/workspaces/${id}`);
    },

    getMembers: async (id: string): Promise<UserWorkspaceDto[]> => {
      const response = await routerApi.get(`/workspaces/${id}/members`);
      return response.data;
    },

    addMember: async (id: string, userId: string, role: string = 'Member'): Promise<void> => {
      await routerApi.post(`/workspaces/${id}/members`, { userId, role });
    },

    removeMember: async (id: string, userId: string): Promise<void> => {
      await routerApi.delete(`/workspaces/${id}/members/${userId}`);
    },

    updateMemberRole: async (id: string, userId: string, role: string): Promise<void> => {
      await routerApi.put(`/workspaces/${id}/members/${userId}`, { role });
    },
  },

  // User initialization
  user: {
    initialize: async (): Promise<InitializationResponse> => {
      if (useMockData) {
        await delay(500);
        return {
          workspace: {
            id: '00000000-0000-0000-0000-000000000000',
            name: 'My Workspace',
            description: 'Default workspace for organizing your routes',
            type: 'User',
            createdAt: new Date().toISOString(),
            updatedAt: new Date().toISOString(),
            userRole: 'Owner',
          },
          userSettings: mockUserSettings,
          message: 'User initialization completed successfully',
        };
      }
      const response = await routerApi.post('/user/initialize');
      return response.data;
    },

    getInitializationStatus: async (): Promise<InitializationStatusResponse> => {
      if (useMockData) {
        await delay(300);
        return {
          needsInitialization: false,
          hasWorkspaces: true,
          hasDomains: true,
          hasUserSettings: true,
        };
      }
      const response = await routerApi.get('/user/initialization-status');
      return response.data;
    },
  },
};
