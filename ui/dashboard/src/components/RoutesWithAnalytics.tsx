import React, { useState, useEffect } from 'react';
import { 
  Plus, 
  Edit, 
  Trash2, 
  ExternalLink, 
  Copy,
  BarChart3,
  Search,
  Users,
  MousePointer,
  Clock,
  Activity,
  Globe
} from 'lucide-react';
// Removed Bootstrap Dropdown import - using unified controls
import { 
  BarChart, 
  Bar, 
  XAxis, 
  YAxis, 
  CartesianGrid, 
  Tooltip, 
  ResponsiveContainer,
  PieChart,
  Pie,
  Cell,
  AreaChart,
  Area
} from 'recharts';
import { apiService, RouteDto } from '../services/api';
import LoadingSpinner from './LoadingSpinner';
import RouteEditModal from './RouteEditModal';
import './DesignSystem.css';

const RoutesWithAnalytics: React.FC = () => {
  const [routes, setRoutes] = useState<RouteDto[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const [searchTerm, setSearchTerm] = useState('');
  const [statusFilter, setStatusFilter] = useState('all');
  const [selectedRoute, setSelectedRoute] = useState<RouteDto | null>(null);
  const [analytics, setAnalytics] = useState<any>(null);
  const [analyticsLoading, setAnalyticsLoading] = useState(false);
  const [timeRange, setTimeRange] = useState<'24h' | '7d' | '30d'>('7d');
  const [editingRoute, setEditingRoute] = useState<RouteDto | null>(null);
  const [showEditModal, setShowEditModal] = useState(false);

  useEffect(() => {
    fetchRoutes();
  }, []);

  useEffect(() => {
    if (selectedRoute) {
      fetchAnalytics();
    }
  }, [selectedRoute, timeRange]);

  const fetchRoutes = async () => {
    try {
      setLoading(true);
      setError(null);
      const data = await apiService.routes.list({ limit: 100 });
      setRoutes(data);
    } catch (err) {
      console.error('Failed to fetch routes:', err);
      setError('Failed to load routes. Please try again.');
    } finally {
      setLoading(false);
    }
  };

  const fetchAnalytics = async () => {
    if (!selectedRoute) return;
    
    setAnalyticsLoading(true);
    try {
      // Simulate API call
      await new Promise(resolve => setTimeout(resolve, 1000));
      const mockData = generateMockAnalytics(selectedRoute);
      setAnalytics(mockData);
    } catch (err) {
      console.error('Failed to fetch analytics:', err);
    } finally {
      setAnalyticsLoading(false);
    }
  };

  const generateMockAnalytics = (route: RouteDto) => {
    const baseClicks = Math.floor(Math.random() * 1000) + 100;
    const uniqueVisitors = Math.floor(baseClicks * 0.7);
    
    return {
      totalClicks: baseClicks,
      uniqueVisitors,
      avgResponseTime: Math.floor(Math.random() * 200) + 50,
      errorRate: Math.random() * 5,
      topCountries: [
        { country: 'United States', clicks: Math.floor(baseClicks * 0.35), percentage: 35 },
        { country: 'United Kingdom', clicks: Math.floor(baseClicks * 0.20), percentage: 20 },
        { country: 'Germany', clicks: Math.floor(baseClicks * 0.15), percentage: 15 },
        { country: 'France', clicks: Math.floor(baseClicks * 0.12), percentage: 12 },
        { country: 'Canada', clicks: Math.floor(baseClicks * 0.10), percentage: 10 },
        { country: 'Other', clicks: Math.floor(baseClicks * 0.08), percentage: 8 }
      ],
      deviceBreakdown: [
        { device: 'Mobile', clicks: Math.floor(baseClicks * 0.55), percentage: 55 },
        { device: 'Desktop', clicks: Math.floor(baseClicks * 0.35), percentage: 35 },
        { device: 'Tablet', clicks: Math.floor(baseClicks * 0.10), percentage: 10 }
      ],
      browserBreakdown: [
        { browser: 'Chrome', clicks: Math.floor(baseClicks * 0.45), percentage: 45 },
        { browser: 'Safari', clicks: Math.floor(baseClicks * 0.25), percentage: 25 },
        { browser: 'Firefox', clicks: Math.floor(baseClicks * 0.15), percentage: 15 },
        { browser: 'Edge', clicks: Math.floor(baseClicks * 0.10), percentage: 10 },
        { browser: 'Other', clicks: Math.floor(baseClicks * 0.05), percentage: 5 }
      ],
      hourlyClicks: Array.from({ length: 24 }, (_, i) => ({
        hour: `${i.toString().padStart(2, '0')}:00`,
        clicks: Math.floor(Math.random() * 20) + 5
      })),
      dailyClicks: Array.from({ length: 7 }, (_, i) => {
        const date = new Date();
        date.setDate(date.getDate() - (6 - i));
        return {
          date: date.toLocaleDateString('en-US', { month: 'short', day: 'numeric' }),
          clicks: Math.floor(Math.random() * 100) + 20
        };
      }),
      referrers: [
        { referrer: 'google.com', clicks: Math.floor(baseClicks * 0.40), percentage: 40 },
        { referrer: 'facebook.com', clicks: Math.floor(baseClicks * 0.25), percentage: 25 },
        { referrer: 'twitter.com', clicks: Math.floor(baseClicks * 0.15), percentage: 15 },
        { referrer: 'linkedin.com', clicks: Math.floor(baseClicks * 0.10), percentage: 10 },
        { referrer: 'Direct', clicks: Math.floor(baseClicks * 0.10), percentage: 10 }
      ],
      responseTimeDistribution: [
        { range: '0-100ms', count: Math.floor(baseClicks * 0.60) },
        { range: '100-200ms', count: Math.floor(baseClicks * 0.25) },
        { range: '200-500ms', count: Math.floor(baseClicks * 0.10) },
        { range: '500ms+', count: Math.floor(baseClicks * 0.05) }
      ]
    };
  };

  const handleDeleteRoute = async (route: RouteDto) => {
    if (!window.confirm(`Are you sure you want to delete the route "${route.link}"?`)) {
      return;
    }

    try {
      await apiService.routes.delete(route.switch, route.properties.domain_id, route.link);
      await fetchRoutes();
      if (selectedRoute?.link === route.link) {
        setSelectedRoute(null);
        setAnalytics(null);
      }
    } catch (err) {
      console.error('Failed to delete route:', err);
      alert('Failed to delete route. Please try again.');
    }
  };

  const handleEditRoute = (route: RouteDto) => {
    setEditingRoute(route);
    setShowEditModal(true);
  };

  const handleSaveRoute = async (routeData: any) => {
    try {
      if (editingRoute) {
        await apiService.routes.update(editingRoute.switch, editingRoute.properties.domain_id, editingRoute.link, routeData);
      } else {
        await apiService.routes.create(routeData);
      }
      await fetchRoutes();
      setShowEditModal(false);
      setEditingRoute(null);
    } catch (err) {
      console.error('Failed to save route:', err);
      alert('Failed to save route. Please try again.');
    }
  };

  const handleCreateRoute = () => {
    setEditingRoute(null);
    setShowEditModal(true);
  };

  const handleSelectRoute = (route: RouteDto) => {
    setSelectedRoute(route);
  };

  // Removed unused copyToClipboard function

  const filteredRoutes = routes.filter(route => {
    const matchesSearch = route.link.toLowerCase().includes(searchTerm.toLowerCase()) ||
                         route.dest.toLowerCase().includes(searchTerm.toLowerCase());
    const matchesStatus = statusFilter === 'all' || route.status.toLowerCase() === statusFilter.toLowerCase();
    return matchesSearch && matchesStatus;
  });

  const getCountryColor = (index: number) => {
    const colors = ['var(--primary-500)', 'var(--success-500)', 'var(--warning-500)', 'var(--error-500)', 'var(--primary-600)', 'var(--primary-400)'];
    return colors[index % colors.length];
  };

  const getBrowserColor = (browser: string) => {
    switch (browser.toLowerCase()) {
      case 'chrome':
        return 'var(--primary-500)';
      case 'safari':
        return 'var(--success-500)';
      case 'firefox':
        return 'var(--warning-500)';
      case 'edge':
        return 'var(--primary-600)';
      default:
        return 'var(--text-muted)';
    }
  };

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
    <div className="routes-with-analytics">
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
              <div className="control-select">
                <select
                  value={statusFilter}
                  onChange={(e) => setStatusFilter(e.target.value)}
                >
                  <option value="all">All Status</option>
                  <option value="active">Active</option>
                  <option value="inactive">Inactive</option>
                </select>
              </div>
            </div>
          </div>

          {/* Routes List */}
          <div className="routes-list">
            {filteredRoutes.map((route) => (
              <div
                key={`${route.switch}-${route.properties.domain_id}-${route.link}`}
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

      {/* Main Content - Analytics */}
      <div className="main-content">
        {selectedRoute && analytics ? (
          <div className="analytics-content">
            <div className="analytics-header">
              <div className="route-info">
                <h2>{selectedRoute.link}</h2>
                <p className="route-destination">{selectedRoute.dest}</p>
              </div>
              <div className="time-range-selector">
                <div className="btn-group" role="group">
                  {(['24h', '7d', '30d'] as const).map((range) => (
                    <button
                      key={range}
                      className={`btn ${timeRange === range ? 'btn-primary' : 'btn-outline-primary'}`}
                      onClick={() => setTimeRange(range)}
                    >
                      {range}
                    </button>
                  ))}
                </div>
              </div>
            </div>

            {/* Key Metrics */}
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
                  <div className="metric-value">{analytics.avgResponseTime}ms</div>
                  <div className="metric-label">Avg Response</div>
                </div>
              </div>
              <div className="metric-card">
                <div className="metric-icon">
                  <Activity size={24} />
                </div>
                <div className="metric-content">
                  <div className="metric-value">{analytics.errorRate.toFixed(1)}%</div>
                  <div className="metric-label">Error Rate</div>
                </div>
              </div>
            </div>

            {/* Charts */}
            <div className="charts-grid">
              <div className="chart-card">
                <h4>Clicks Over Time</h4>
                <ResponsiveContainer width="100%" height={300}>
                  <AreaChart data={analytics.dailyClicks}>
                    <CartesianGrid strokeDasharray="3 3" />
                    <XAxis dataKey="date" />
                    <YAxis />
                    <Tooltip />
                    <Area 
                      type="monotone" 
                      dataKey="clicks" 
                      stroke="var(--primary-500)" 
                      fill="var(--primary-500)" 
                      fillOpacity={0.3}
                    />
                  </AreaChart>
                </ResponsiveContainer>
              </div>

              <div className="chart-card">
                <h4>Top Countries</h4>
                <ResponsiveContainer width="100%" height={300}>
                  <BarChart data={analytics.topCountries} layout="horizontal">
                    <CartesianGrid strokeDasharray="3 3" />
                    <XAxis type="number" />
                    <YAxis dataKey="country" type="category" width={100} />
                    <Tooltip />
                    <Bar dataKey="clicks" fill="var(--primary-500)" />
                  </BarChart>
                </ResponsiveContainer>
              </div>

              <div className="chart-card">
                <h4>Device Breakdown</h4>
                <ResponsiveContainer width="100%" height={300}>
                  <PieChart>
                    <Pie
                      data={analytics.deviceBreakdown}
                      cx="50%"
                      cy="50%"
                      labelLine={false}
                      label={({ device, percentage }) => `${device} (${percentage}%)`}
                      outerRadius={80}
                      fill="var(--primary-400)"
                      dataKey="clicks"
                    >
                      {analytics.deviceBreakdown.map((entry: any, index: number) => (
                        <Cell key={`cell-${index}`} fill={getCountryColor(index)} />
                      ))}
                    </Pie>
                    <Tooltip />
                  </PieChart>
                </ResponsiveContainer>
              </div>

              <div className="chart-card">
                <h4>Browser Breakdown</h4>
                <ResponsiveContainer width="100%" height={300}>
                  <PieChart>
                    <Pie
                      data={analytics.browserBreakdown}
                      cx="50%"
                      cy="50%"
                      labelLine={false}
                      label={({ browser, percentage }) => `${browser} (${percentage}%)`}
                      outerRadius={80}
                      fill="var(--primary-400)"
                      dataKey="clicks"
                    >
                      {analytics.browserBreakdown.map((entry: any, index: number) => (
                        <Cell key={`cell-${index}`} fill={getBrowserColor(entry.browser)} />
                      ))}
                    </Pie>
                    <Tooltip />
                  </PieChart>
                </ResponsiveContainer>
              </div>
            </div>

            {/* Top Referrers */}
            <div className="referrers-section">
              <h4>Top Referrers</h4>
              <div className="referrers-list">
                {analytics.referrers.map((referrer: any, index: number) => (
                  <div key={index} className="referrer-item">
                    <div className="referrer-info">
                      <Globe size={16} />
                      <span className="referrer-name">{referrer.referrer}</span>
                    </div>
                    <div className="referrer-stats">
                      <span className="referrer-clicks">{referrer.clicks.toLocaleString()}</span>
                      <div className="referrer-bar">
                        <div 
                          className="referrer-progress" 
                          style={{ width: `${referrer.percentage}%` }}
                        ></div>
                      </div>
                      <span className="referrer-percentage">{referrer.percentage}%</span>
                    </div>
                  </div>
                ))}
              </div>
            </div>
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
                <ExternalLink size={20} />
                <span>Performance Metrics</span>
              </div>
              <div className="feature-item">
                <Copy size={20} />
                <span>Traffic Analysis</span>
              </div>
            </div>
          </div>
        )}
      </div>

      {/* Route Edit Modal */}
      <RouteEditModal
        show={showEditModal}
        onHide={() => {
          setShowEditModal(false);
          setEditingRoute(null);
        }}
        route={editingRoute}
        onSave={handleSaveRoute}
      />
    </div>
  );
};

export default RoutesWithAnalytics;
