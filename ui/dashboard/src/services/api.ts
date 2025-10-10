import axios, { AxiosInstance, AxiosRequestConfig } from 'axios';
import { getToken, updateToken, isAuthenticated, isInitialized } from '../config/keycloak';
import { useMockData, mockAnalytics, mockRoutes, mockUserSettings } from '../config/development';

// API base URLs
const PROXY_API_URL = process.env.REACT_APP_PROXY_API_URL || 'http://localhost:5050';
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
        
        // Ensure token is valid (refresh if needed)
        await updateToken(30);
        const refreshedToken = getToken();
        if (refreshedToken) {
          config.headers.Authorization = `Bearer ${refreshedToken}`;
        }
      } catch (error) {
        console.error('Failed to update token:', error);
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

// API Types
export interface RouteDto {
  switch: string;
  link: string;
  dest: string;
  destFormat: string;
  code: number;
  ttl: number;
  status: string;
  terminal: string;
  properties?: {
    routeId: string;
    domainId: string;
    ownerId: string;
    scripts: string[];
    tags: string[];
    custom: Record<string, any>;
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
  workspaceId: string;
  created: string;
  dest: string;
  ip: string;
  continent: string;
  country: string;
  location: string;
  osFamily: string;
  osVersion: string;
  userAgentFamily: string;
  userAgentVersion: string;
  deviceBrand: string;
  deviceFamily: string;
  deviceModel: string;
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

// Helper function to simulate API delay
const delay = (ms: number) => new Promise(resolve => setTimeout(resolve, ms));

// API Methods
export const apiService = {
  // Routes API
  routes: {
    list: async (params?: { page?: number; pageSize?: number; search?: string; status?: string }): Promise<PaginatedResponse<RouteDto>> => {
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

    get: async (domain: string, path: string) => {
      const response = await routerApi.get(`/routes/${domain}/${path}`);
      return response.data;
    },

    create: async (route: Partial<RouteDto>) => {
      const response = await routerApi.post('/routes', route);
      return response.data;
    },

    update: async (domain: string, path: string, route: Partial<RouteDto>) => {
      const response = await routerApi.put(`/routes/${domain}/${path}`, route);
      return response.data;
    },

    delete: async (domain: string, path: string) => {
      const response = await routerApi.delete(`/routes/${domain}/${path}`);
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
  },
};
