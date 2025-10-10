import React, { useState, useEffect, useCallback } from 'react';
import { Plus, Edit, Trash2, Search, BarChart3, MousePointer, Users, Clock, Activity } from 'lucide-react';
import { BarChart, Bar, XAxis, YAxis, Tooltip, ResponsiveContainer, AreaChart, Area } from 'recharts';
import { apiService, RouteDto } from '../services/api';
import LoadingSpinner from './LoadingSpinner';
import WorldMap from './WorldMap';
import './DesignSystem.css';

const RoutesWithSidebar: React.FC = () => {
  const [routes, setRoutes] = useState<RouteDto[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const [searchTerm, setSearchTerm] = useState('');
  const [statusFilter, setStatusFilter] = useState('all');
  const [selectedRoute, setSelectedRoute] = useState<RouteDto | null>(null);
  const [editingRoute, setEditingRoute] = useState<RouteDto | null>(null);
  const [editFormData, setEditFormData] = useState<any>(null);
  const [analytics, setAnalytics] = useState<any>(null);
  const [analyticsLoading, setAnalyticsLoading] = useState(false);
  const [timeRange, setTimeRange] = useState('7d');

  useEffect(() => {
    fetchRoutes();
  }, []);

  const fetchRoutes = async () => {
    try {
      setLoading(true);
      setError(null);
      const response = await apiService.routes.list({ page: 1, pageSize: 100 });
      setRoutes(response.data);
    } catch (err) {
      console.error('Failed to fetch routes:', err);
      setError('Failed to load routes. Please try again.');
    } finally {
      setLoading(false);
    }
  };

  // Helper function to parse domain and path from link
  const parseLinkParts = (link: string): { domain: string; path: string } => {
    const parts = link.split('/');
    return {
      domain: parts[0] || '',
      path: parts.slice(1).join('/') || ''
    };
  };

  const handleDeleteRoute = async (route: RouteDto) => {
    if (!window.confirm(`Are you sure you want to delete the route "${route.link}"?`)) {
      return;
    }

    try {
      const { domain, path } = parseLinkParts(route.link);
      await apiService.routes.delete(domain, path);
      await fetchRoutes();
      if (selectedRoute?.link === route.link) {
        setSelectedRoute(null);
      }
    } catch (err) {
      console.error('Failed to delete route:', err);
      alert('Failed to delete route. Please try again.');
    }
  };

  const handleEditRoute = (route: RouteDto) => {
    setEditingRoute(route);
    setEditFormData({
      link: route.link,
      dest: route.dest,
      code: route.code,
      status: route.status,
      switch: route.switch,
      properties: { ...route.properties }
    });
  };

  const handleSaveRoute = async () => {
    try {
      if (editingRoute) {
        const { domain, path } = parseLinkParts(editingRoute.link);
        await apiService.routes.update(domain, path, editFormData);
      } else {
        await apiService.routes.create(editFormData);
      }
      await fetchRoutes();
      setEditingRoute(null);
      setEditFormData(null);
    } catch (err) {
      console.error('Failed to save route:', err);
      alert('Failed to save route. Please try again.');
    }
  };

  const handleCancelEdit = () => {
    setEditingRoute(null);
    setEditFormData(null);
  };

  const handleCreateRoute = () => {
    setEditingRoute(null);
    setEditFormData({
      link: '',
      dest: '',
      code: 302,
      status: 'active',
      switch: 'default',
      properties: { domain_id: 'default' }
    });
  };


  const fetchAnalytics = useCallback(async (route: RouteDto) => {
    try {
      setAnalyticsLoading(true);
      // Mock analytics data - replace with real API call
      const generateMockAnalytics = (route: RouteDto) => {
        const days = timeRange === '24h' ? 1 : timeRange === '7d' ? 7 : timeRange === '30d' ? 30 : 7;
        const clicks = Math.floor(Math.random() * 1000) + 100;
        const conversions = Math.floor(clicks * (0.1 + Math.random() * 0.2));
        
        return {
          totalClicks: clicks,
          totalConversions: conversions,
          conversionRate: ((conversions / clicks) * 100).toFixed(1),
          uniqueVisitors: Math.floor(clicks * 0.7),
          avgTimeOnPage: Math.floor(Math.random() * 300) + 30,
          bounceRate: (Math.random() * 40 + 20).toFixed(1),
          topCountries: [
            { name: 'United States', clicks: Math.floor(clicks * 0.4), percentage: 40 },
            { name: 'United Kingdom', clicks: Math.floor(clicks * 0.25), percentage: 25 },
            { name: 'Canada', clicks: Math.floor(clicks * 0.15), percentage: 15 },
            { name: 'Germany', clicks: Math.floor(clicks * 0.1), percentage: 10 },
            { name: 'France', clicks: Math.floor(clicks * 0.1), percentage: 10 },
            { name: 'Japan', clicks: 0, percentage: 0 },
            { name: 'Australia', clicks: 0, percentage: 0 },
            { name: 'Brazil', clicks: 0, percentage: 0 },
            { name: 'India', clicks: 0, percentage: 0 },
            { name: 'China', clicks: 0, percentage: 0 }
          ],
          topBrowsers: [
            { name: 'Chrome', clicks: Math.floor(clicks * 0.6), percentage: 60 },
            { name: 'Safari', clicks: Math.floor(clicks * 0.2), percentage: 20 },
            { name: 'Firefox', clicks: Math.floor(clicks * 0.1), percentage: 10 },
            { name: 'Edge', clicks: Math.floor(clicks * 0.1), percentage: 10 }
          ],
          dailyClicks: Array.from({ length: days }, (_, i) => ({
            date: new Date(Date.now() - (days - 1 - i) * 24 * 60 * 60 * 1000).toISOString().split('T')[0],
            clicks: Math.floor(Math.random() * 50) + 10,
            conversions: Math.floor(Math.random() * 10) + 2
          })),
          hourlyClicks: Array.from({ length: 24 }, (_, i) => ({
            hour: i,
            clicks: Math.floor(Math.random() * 20) + 5
          }))
        };
      };
      
      const mockAnalytics = generateMockAnalytics(route);
      setAnalytics(mockAnalytics);
    } catch (err) {
      console.error('Failed to fetch analytics:', err);
    } finally {
      setAnalyticsLoading(false);
    }
  }, [timeRange]);

  const handleSelectRoute = (route: RouteDto) => {
    setSelectedRoute(route);
    fetchAnalytics(route);
  };

  const getCountryColor = (index: number, clicks: number) => {
    if (clicks === 0) {
      return '#f1f5f9'; // Light gray for countries with no traffic
    }
    const colors = ['#667eea', '#764ba2', '#4facfe', '#00f2fe', '#fa709a'];
    return colors[index % colors.length];
  };


  const filteredRoutes = routes.filter(route => {
    const matchesSearch = route.link.toLowerCase().includes(searchTerm.toLowerCase()) ||
                         route.dest.toLowerCase().includes(searchTerm.toLowerCase());
    const matchesStatus = statusFilter === 'all' || route.status.toLowerCase() === statusFilter.toLowerCase();
    return matchesSearch && matchesStatus;
  });

  if (loading) {
    return <LoadingSpinner />;
  }

  if (error) {
    return (
      <div className="alert alert-error">
        <h3>Error Loading Routes</h3>
        <p>{error}</p>
        <button className="btn btn-primary" onClick={fetchRoutes}>
          Retry
        </button>
      </div>
    );
  }

  return (
    <div className={`routes-with-sidebar ${editFormData ? 'editing' : ''}`}>
      {/* Sidebar */}
      <div className="routes-sidebar">
        <div className="sidebar-content">
          {/* Search and Filters */}
          <div className="sidebar-controls">
            <div className="search-box">
              <Search size={16} />
              <input
                type="text"
                placeholder="Search routes..."
                value={searchTerm}
                onChange={(e) => setSearchTerm(e.target.value)}
              />
            </div>
            
            <div className="control-group">
              <select 
                className="control-dropdown"
                value={statusFilter}
                onChange={(e) => setStatusFilter(e.target.value)}
              >
                <option value="all">All Status</option>
                <option value="active">Active</option>
                <option value="inactive">Inactive</option>
              </select>
            </div>
          </div>

          {/* Routes List */}
          <div className="routes-list">
            {filteredRoutes.map((route, index) => (
              <div
                key={`${route.link}-${index}`}
                className={`route-item ${selectedRoute?.link === route.link ? 'selected' : ''}`}
                onClick={() => handleSelectRoute(route)}
              >
                <div className="route-info">
                  <div className="route-link">{route.link}</div>
                  <div className="route-destination">
                    {route.dest.length > 30 ? `${route.dest.substring(0, 30)}...` : route.dest}
                  </div>
                  <div className="route-meta">
                    <span className={`route-status ${route.status.toLowerCase()}`}>
                      {route.status}
                    </span>
                    <span className="route-code">{route.code}</span>
                  </div>
                </div>
                <div className="route-actions">
                  <button
                    className="action-btn"
                    onClick={(e) => {
                      e.stopPropagation();
                      handleEditRoute(route);
                    }}
                    title="Edit route"
                  >
                    <Edit size={14} />
                  </button>
                  <button
                    className="action-btn"
                    onClick={(e) => {
                      e.stopPropagation();
                      handleDeleteRoute(route);
                    }}
                    title="Delete route"
                  >
                    <Trash2 size={14} />
                  </button>
                </div>
              </div>
            ))}

            {filteredRoutes.length === 0 && (
              <div className="empty-state">
                <BarChart3 size={32} />
                <p>No routes found</p>
                <small>
                  {searchTerm ? 'No routes match your search criteria.' : 'Create your first route to get started.'}
                </small>
              </div>
            )}
          </div>
        </div>

        <div className="sidebar-footer">
          <div className="footer-stats">
            <div className="stat-item">
              <span className="stat-label">Total Routes</span>
              <span className="stat-value">{routes.length}</span>
            </div>
            <div className="stat-item">
              <span className="stat-label">Active</span>
              <span className="stat-value">{routes.filter(r => r.status === 'active').length}</span>
            </div>
          </div>
          <button className="btn btn-primary w-100" onClick={handleCreateRoute}>
            <Plus size={16} />
            Create Route
          </button>
        </div>
      </div>

      {/* Main Content */}
      <div className="main-content">
        {editFormData ? (
          <div className="route-edit-content">
            <div className="edit-header">
              <h2>{editingRoute ? 'Edit Route' : 'Create New Route'}</h2>
            </div>

            <div className="edit-form">
              <div className="form-section">
                <h3>Basic Information</h3>
                <div className="form-grid">
                  <div className="form-group">
                    <label>Link</label>
                    <input
                      type="text"
                      value={editFormData.link}
                      onChange={(e) => setEditFormData({...editFormData, link: e.target.value})}
                      placeholder="Enter route link"
                    />
                  </div>
                  <div className="form-group">
                    <label>Destination</label>
                    <input
                      type="text"
                      value={editFormData.dest}
                      onChange={(e) => setEditFormData({...editFormData, dest: e.target.value})}
                      placeholder="Enter destination URL"
                    />
                  </div>
                  <div className="form-group">
                    <label>Status Code</label>
                    <select 
                      className="form-dropdown"
                      value={editFormData.code}
                      onChange={(e) => setEditFormData({...editFormData, code: parseInt(e.target.value)})}
                    >
                      <option value={301}>301 - Permanent Redirect</option>
                      <option value={302}>302 - Temporary Redirect</option>
                      <option value={307}>307 - Temporary Redirect (Preserve Method)</option>
                      <option value={308}>308 - Permanent Redirect (Preserve Method)</option>
                    </select>
                  </div>
                  <div className="form-group">
                    <label>Status</label>
                    <select 
                      className="form-dropdown"
                      value={editFormData.status}
                      onChange={(e) => setEditFormData({...editFormData, status: e.target.value})}
                    >
                      <option value="active">Active</option>
                      <option value="inactive">Inactive</option>
                    </select>
                  </div>
                </div>
              </div>
            </div>
            
            {/* Form Actions */}
            <div className="edit-actions">
              <button 
                className="btn btn-outline"
                onClick={handleCancelEdit}
              >
                Cancel
              </button>
              <button 
                className="btn btn-primary"
                onClick={handleSaveRoute}
              >
                Save Route
              </button>
            </div>
          </div>
        ) : selectedRoute ? (
          <div className="analytics-content">
            <div className="analytics-header">
              <div className="route-info">
                <h2>{selectedRoute.link}</h2>
                <p className="route-destination">{selectedRoute.dest}</p>
                <div className="route-actions-header">
                  <button 
                    className="btn btn-outline btn-sm"
                    onClick={() => handleEditRoute(selectedRoute)}
                  >
                    <Edit size={16} />
                    Edit Route
                  </button>
                  
                  {/* Time Range Selector */}
                  <div className="time-range-selector">
                    <button 
                      className={`time-range-btn ${timeRange === '24h' ? 'active' : ''}`}
                      onClick={() => setTimeRange('24h')}
                    >
                      24h
                    </button>
                    <button 
                      className={`time-range-btn ${timeRange === '7d' ? 'active' : ''}`}
                      onClick={() => setTimeRange('7d')}
                    >
                      7d
                    </button>
                    <button 
                      className={`time-range-btn ${timeRange === '30d' ? 'active' : ''}`}
                      onClick={() => setTimeRange('30d')}
                    >
                      30d
                    </button>
                  </div>
                </div>
              </div>
            </div>

            {analyticsLoading ? (
              <div className="welcome-content">
                <LoadingSpinner />
                <p>Loading analytics...</p>
              </div>
            ) : analytics ? (
              <div className="analytics-content">
                {/* Metrics Grid */}
                <div className="metrics-grid">
                  <div className="metric-card">
                    <div className="metric-icon">
                      <MousePointer size={24} />
                    </div>
                    <div className="metric-content">
                      <div className="metric-value">{analytics.totalClicks.toLocaleString()}</div>
                      <div className="metric-label">Total Clicks</div>
                    </div>
                  </div>
                  
                  <div className="metric-card">
                    <div className="metric-icon">
                      <Activity size={24} />
                    </div>
                    <div className="metric-content">
                      <div className="metric-value">{analytics.totalConversions.toLocaleString()}</div>
                      <div className="metric-label">Conversions</div>
                    </div>
                  </div>
                  
                  <div className="metric-card">
                    <div className="metric-icon">
                      <Users size={24} />
                    </div>
                    <div className="metric-content">
                      <div className="metric-value">{analytics.uniqueVisitors.toLocaleString()}</div>
                      <div className="metric-label">Unique Visitors</div>
                    </div>
                  </div>
                  
                  <div className="metric-card">
                    <div className="metric-icon">
                      <Clock size={24} />
                    </div>
                    <div className="metric-content">
                      <div className="metric-value">{analytics.avgTimeOnPage}s</div>
                      <div className="metric-label">Avg. Time</div>
                    </div>
                  </div>
                </div>

                {/* Charts Grid */}
                <div className="charts-grid">
                  {/* Daily Clicks Chart */}
                  <div className="routes-chart-card">
                    <h4>Daily Clicks</h4>
                    <div className="chart-container">
                      <ResponsiveContainer width="100%" height={350}>
                        <AreaChart data={analytics.dailyClicks}>
                          <XAxis dataKey="date" />
                          <YAxis />
                          <Tooltip 
                            contentStyle={{
                              backgroundColor: 'var(--bg-elevated)',
                              border: '1px solid var(--border-primary)',
                              borderRadius: '0px'
                            }}
                          />
                          <Area 
                            type="monotone" 
                            dataKey="clicks" 
                            stroke="#667eea" 
                            fill="#667eea" 
                            fillOpacity={0.1}
                            strokeWidth={2}
                          />
                        </AreaChart>
                      </ResponsiveContainer>
                    </div>
                  </div>

                  {/* Top Browsers Chart */}
                  <div className="routes-chart-card">
                    <h4>Top Browsers</h4>
                    <div className="chart-container">
                      <ResponsiveContainer width="100%" height={350}>
                        <BarChart data={analytics.topBrowsers}>
                          <XAxis dataKey="name" />
                          <YAxis />
                          <Tooltip 
                            contentStyle={{
                              backgroundColor: 'var(--bg-elevated)',
                              border: '1px solid var(--border-primary)',
                              borderRadius: '0px'
                            }}
                          />
                          <Bar dataKey="clicks" fill="#4facfe" radius={[0, 0, 0, 0]} />
                        </BarChart>
                      </ResponsiveContainer>
                    </div>
                  </div>
                </div>

                {/* Map and Countries Row */}
                <div className="map-countries-row">
                  {/* World Map - Half Width */}
                  <div className="routes-chart-card map-half">
                    <h4>Traffic by Country</h4>
                    <div className="chart-container">
                      <WorldMap data={analytics.topCountries} height={450} />
                    </div>
                  </div>

                  {/* Top Countries List - Half Width */}
                  <div className="routes-chart-card countries-half">
                    <h4>Top Countries</h4>
                    <div className="countries-list">
                      {analytics.topCountries.map((country: any, index: number) => (
                        <div key={country.name} className="country-item">
                          <div className="country-info">
                            <div className="country-name">{country.name}</div>
                            <div className="country-clicks">{country.clicks || 0} clicks</div>
                          </div>
                          <div className="country-stats">
                            <div className="country-percentage">{country.percentage || 0}%</div>
                            <div className="country-bar">
                              <div 
                                className="country-bar-fill"
                                style={{ 
                                  width: `${country.percentage || 0}%`,
                                  backgroundColor: getCountryColor(index, country.clicks || 0)
                                }}
                              />
                            </div>
                          </div>
                        </div>
                      ))}
                    </div>
                  </div>
                </div>

              </div>
            ) : (
              <div className="welcome-content">
                <div className="welcome-icon">
                  <BarChart3 size={64} />
                </div>
                <h2>Route Analytics</h2>
                <p>Analytics for {selectedRoute.link} will be displayed here.</p>
                <div className="welcome-features">
                  <div className="feature-item">
                    <BarChart3 size={20} />
                    <span>Detailed Analytics</span>
                  </div>
                  <div className="feature-item">
                    <Edit size={20} />
                    <span>Performance Metrics</span>
                  </div>
                  <div className="feature-item">
                    <Search size={20} />
                    <span>Traffic Analysis</span>
                  </div>
                </div>
              </div>
            )}
          </div>
        ) : (
          <div className="welcome-content">
            <div className="welcome-icon">
              <BarChart3 size={64} />
            </div>
            <h2>Select a Route</h2>
            <p>Choose a route from the sidebar to view its analytics and performance metrics.</p>
            <div className="welcome-features">
              <div className="feature-item">
                <BarChart3 size={20} />
                <span>Detailed Analytics</span>
              </div>
              <div className="feature-item">
                <Edit size={20} />
                <span>Performance Metrics</span>
              </div>
              <div className="feature-item">
                <Search size={20} />
                <span>Traffic Analysis</span>
              </div>
            </div>
          </div>
        )}
      </div>

    </div>
  );
};

export default RoutesWithSidebar;
