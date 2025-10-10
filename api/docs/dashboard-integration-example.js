/**
 * ClickStream API Integration Example for Dashboard
 * 
 * This file provides examples of how to integrate the ClickStream API
 * into your dashboard application.
 */

// Configuration
const API_BASE_URL = 'http://localhost:5050/api/v1';
const KEYCLOAK_URL = 'http://localhost:8080/realms/shortas-dev';
const CLIENT_ID = 'shortas-api';
const CLIENT_SECRET = 'YOUR_CLIENT_SECRET_HERE'; // Replace with actual secret

/**
 * Authentication Service
 */
class AuthService {
  constructor() {
    this.token = localStorage.getItem('jwt_token');
    this.tokenExpiry = localStorage.getItem('jwt_token_expiry');
  }

  async getToken() {
    // Check if token exists and is not expired
    if (this.token && this.tokenExpiry && new Date() < new Date(this.tokenExpiry)) {
      return this.token;
    }

    // Get new token
    return await this.refreshToken();
  }

  async refreshToken() {
    try {
      const response = await fetch(`${KEYCLOAK_URL}/auth/protocol/openid-connect/token`, {
        method: 'POST',
        headers: {
          'Content-Type': 'application/x-www-form-urlencoded',
        },
        body: new URLSearchParams({
          grant_type: 'password',
          client_id: CLIENT_ID,
          client_secret: CLIENT_SECRET,
          username: 'testuser', // Replace with actual username
          password: 'testpassword' // Replace with actual password
        })
      });

      if (!response.ok) {
        throw new Error(`Authentication failed: ${response.status}`);
      }

      const data = await response.json();
      this.token = data.access_token;
      
      // Store token and expiry
      localStorage.setItem('jwt_token', this.token);
      localStorage.setItem('jwt_token_expiry', new Date(Date.now() + data.expires_in * 1000).toISOString());
      
      return this.token;
    } catch (error) {
      console.error('Authentication error:', error);
      throw error;
    }
  }

  getAuthHeaders() {
    return {
      'Authorization': `Bearer ${this.token}`,
      'Content-Type': 'application/json'
    };
  }
}

/**
 * ClickStream API Service
 */
class ClickStreamAPI {
  constructor(authService) {
    this.auth = authService;
  }

  async makeRequest(endpoint, options = {}) {
    const token = await this.auth.getToken();
    const url = `${API_BASE_URL}${endpoint}`;
    
    const response = await fetch(url, {
      ...options,
      headers: {
        ...this.auth.getAuthHeaders(),
        ...options.headers
      }
    });

    if (!response.ok) {
      const error = await response.json().catch(() => ({ message: 'Unknown error' }));
      throw new Error(`API Error ${response.status}: ${error.message || error.error || 'Unknown error'}`);
    }

    return await response.json();
  }

  // Get all clickstream data
  async getClickStream(params = {}) {
    const queryString = new URLSearchParams(params).toString();
    const endpoint = `/clickstream${queryString ? `?${queryString}` : ''}`;
    return await this.makeRequest(endpoint);
  }

  // Get clickstream data for specific route
  async getClickStreamByRoute(routeId, params = {}) {
    const queryString = new URLSearchParams(params).toString();
    const endpoint = `/clickstream/${routeId}${queryString ? `?${queryString}` : ''}`;
    return await this.makeRequest(endpoint);
  }

  // Get clickstream statistics
  async getStats(params = {}) {
    const queryString = new URLSearchParams(params).toString();
    const endpoint = `/clickstream/stats${queryString ? `?${queryString}` : ''}`;
    return await this.makeRequest(endpoint);
  }
}

/**
 * Dashboard Integration Examples
 */

// Initialize services
const authService = new AuthService();
const clickStreamAPI = new ClickStreamAPI(authService);

// Example 1: Basic usage
async function loadClickStreamData() {
  try {
    console.log('Loading clickstream data...');
    const data = await clickStreamAPI.getClickStream();
    console.log('ClickStream data:', data);
    return data;
  } catch (error) {
    console.error('Error loading clickstream data:', error);
    throw error;
  }
}

// Example 2: Load data with filters
async function loadFilteredClickStreamData(startDate, endDate, routeId = null) {
  try {
    const params = {
      startDate: startDate,
      endDate: endDate
    };
    
    if (routeId) {
      params.routeId = routeId;
    }

    const data = await clickStreamAPI.getClickStream(params);
    console.log('Filtered clickstream data:', data);
    return data;
  } catch (error) {
    console.error('Error loading filtered clickstream data:', error);
    throw error;
  }
}

// Example 3: Load statistics
async function loadClickStreamStats(routeId = null) {
  try {
    const params = routeId ? { routeId } : {};
    const stats = await clickStreamAPI.getStats(params);
    console.log('ClickStream statistics:', stats);
    return stats;
  } catch (error) {
    console.error('Error loading clickstream statistics:', error);
    throw error;
  }
}

// Example 4: Real-time data loading with error handling
async function loadDashboardData() {
  const loadingElement = document.getElementById('loading');
  const errorElement = document.getElementById('error');
  const dataElement = document.getElementById('data');

  try {
    // Show loading state
    if (loadingElement) loadingElement.style.display = 'block';
    if (errorElement) errorElement.style.display = 'none';

    // Load data in parallel
    const [clickData, stats] = await Promise.all([
      clickStreamAPI.getClickStream(),
      clickStreamAPI.getStats()
    ]);

    // Update UI
    if (dataElement) {
      dataElement.innerHTML = `
        <h3>Click Stream Data (${clickData.length} records)</h3>
        <div>Total Clicks: ${stats.totalClicks || 'N/A'}</div>
        <div>Unique Clicks: ${stats.uniqueClicks || 'N/A'}</div>
        <div>Bot Clicks: ${stats.botClicks || 'N/A'}</div>
      `;
    }

    console.log('Dashboard data loaded successfully');
    return { clickData, stats };

  } catch (error) {
    console.error('Error loading dashboard data:', error);
    
    // Show error state
    if (errorElement) {
      errorElement.innerHTML = `Error: ${error.message}`;
      errorElement.style.display = 'block';
    }
    
    throw error;
  } finally {
    // Hide loading state
    if (loadingElement) loadingElement.style.display = 'none';
  }
}

// Example 5: React Hook (if using React)
function useClickStreamData(routeId = null, startDate = null, endDate = null) {
  const [data, setData] = useState(null);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState(null);

  useEffect(() => {
    const loadData = async () => {
      setLoading(true);
      setError(null);
      
      try {
        const params = {};
        if (routeId) params.routeId = routeId;
        if (startDate) params.startDate = startDate;
        if (endDate) params.endDate = endDate;

        const result = await clickStreamAPI.getClickStream(params);
        setData(result);
      } catch (err) {
        setError(err.message);
      } finally {
        setLoading(false);
      }
    };

    loadData();
  }, [routeId, startDate, endDate]);

  return { data, loading, error };
}

// Example 6: Chart.js integration
async function createClickStreamChart(canvasId) {
  try {
    const stats = await clickStreamAPI.getStats();
    const ctx = document.getElementById(canvasId).getContext('2d');
    
    new Chart(ctx, {
      type: 'bar',
      data: {
        labels: ['Total Clicks', 'Unique Clicks', 'Bot Clicks'],
        datasets: [{
          label: 'Click Statistics',
          data: [stats.totalClicks, stats.uniqueClicks, stats.botClicks],
          backgroundColor: ['#3498db', '#2ecc71', '#e74c3c']
        }]
      },
      options: {
        responsive: true,
        scales: {
          y: {
            beginAtZero: true
          }
        }
      }
    });
  } catch (error) {
    console.error('Error creating chart:', error);
  }
}

// Export for use in other modules
if (typeof module !== 'undefined' && module.exports) {
  module.exports = {
    AuthService,
    ClickStreamAPI,
    loadClickStreamData,
    loadFilteredClickStreamData,
    loadClickStreamStats,
    loadDashboardData,
    useClickStreamData,
    createClickStreamChart
  };
}

// Auto-load dashboard data when DOM is ready
document.addEventListener('DOMContentLoaded', () => {
  console.log('ClickStream API integration loaded');
  
  // Uncomment to auto-load data
  // loadDashboardData();
});
