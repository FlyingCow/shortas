import React, { useState, useEffect, useCallback } from 'react';
import {
  Plus,
  Edit,
  Trash2,
  Search,
  BarChart3,
  MousePointer,
  Users,
  Activity,
  Bot,
  Globe,
  RefreshCw
} from 'lucide-react';
import {
  BarChart,
  Bar,
  XAxis,
  YAxis,
  Tooltip,
  ResponsiveContainer,
  AreaChart,
  Area,
  PieChart,
  Pie,
  Cell,
  CartesianGrid
} from 'recharts';
import { apiService, RouteDto, RoutingPolicy, DomainDto, RouteSearchResult } from '../services/api';
import LoadingSpinner from './LoadingSpinner';
import WorldMap from './WorldMap';
import RouteForm from './RouteForm';
import './DesignSystem.css';

// Shared chart constants and helpers (matching DashboardUnified)

const CHART_COLORS = [
  'var(--primary-500)',
  'var(--success-500)',
  'var(--warning-500)',
  'var(--error-500)',
  'var(--primary-400)',
  'var(--success-400)',
  'var(--warning-400)',
  'var(--primary-600)',
];

const TRAFFIC_COLORS = ['var(--success-500)', 'var(--error-500)'];

const formatAxisNumber = (value: number): string => {
  if (value >= 1000000) return `${(value / 1000000).toFixed(1)}M`;
  if (value >= 1000) return `${(value / 1000).toFixed(0)}k`;
  return value.toString();
};

const CustomTooltip = ({ active, payload, label }: any) => {
  if (!active || !payload?.length) return null;
  return (
    <div className="dashboard-tooltip">
      {label != null && label !== '' && (
        <div className="dashboard-tooltip-label">{label}</div>
      )}
      {payload.map((entry: any, i: number) => {
        const percent = entry.payload?.percent;
        return (
          <div key={i} className="dashboard-tooltip-row">
            <span className="dashboard-tooltip-dot" style={{ backgroundColor: entry.color || entry.fill }} />
            <span className="dashboard-tooltip-name">{entry.name}</span>
            <span className="dashboard-tooltip-value">
              {typeof entry.value === 'number' ? entry.value.toLocaleString() : entry.value}
              {percent != null && ` (${(percent * 100).toFixed(1)}%)`}
            </span>
          </div>
        );
      })}
    </div>
  );
};

const PieLegend: React.FC<{ items: { name: string; color: string }[] }> = ({ items }) => (
  <div className="dashboard-pie-legend">
    {items.slice(0, 6).map((item, i) => (
      <div key={i} className="dashboard-pie-legend-item">
        <span className="dashboard-pie-legend-dot" style={{ backgroundColor: item.color }} />
        <span className="dashboard-pie-legend-label">{item.name}</span>
      </div>
    ))}
  </div>
);

const RoutesWithSidebar: React.FC = () => {
  const [routes, setRoutes] = useState<RouteDto[]>([]);
  const [domains, setDomains] = useState<DomainDto[]>([]);
  const [workspaces, setWorkspaces] = useState<any[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const [searchTerm, setSearchTerm] = useState('');
  const [statusFilter, setStatusFilter] = useState('all');
  const [workspaceFilter, setWorkspaceFilter] = useState('all');
  const [selectedRoute, setSelectedRoute] = useState<RouteDto | null>(null);
  const [isEditing, setIsEditing] = useState(false);
  const [editingRoute, setEditingRoute] = useState<RouteDto | null>(null);
  const [analytics, setAnalytics] = useState<any>(null);
  const [analyticsLoading, setAnalyticsLoading] = useState(false);
  const [timeRange, setTimeRange] = useState('7d');
  const [searchResults, setSearchResults] = useState<RouteDto[] | null>(null);
  const [searchLoading, setSearchLoading] = useState(false);
  const searchDebounceRef = React.useRef<ReturnType<typeof setTimeout> | null>(null);

  useEffect(() => {
    fetchRoutes();
    fetchDomains();
    fetchWorkspaces();
  }, []);

  useEffect(() => {
    fetchRoutes();
  }, [workspaceFilter]);

  // Debounced Elasticsearch search
  useEffect(() => {
    if (searchDebounceRef.current) {
      clearTimeout(searchDebounceRef.current);
    }

    if (!searchTerm.trim()) {
      setSearchResults(null);
      setSearchLoading(false);
      return;
    }

    setSearchLoading(true);
    searchDebounceRef.current = setTimeout(async () => {
      try {
        const wsId = workspaceFilter !== 'all' ? workspaceFilter : undefined;
        const response = await apiService.routes.search({
          q: searchTerm.trim(),
          page: 1,
          pageSize: 100,
          workspaceId: wsId,
        });
        // Map search results to RouteDto-compatible objects
        const mapped: RouteDto[] = response.data.map((r: RouteSearchResult) => ({
          id: r.id,
          switch: r.switch,
          link: r.link,
          dest: r.dest || '',
          destFormat: 'Http',
          code: 0,
          ttl: 0,
          status: r.status,
          terminal: 'External',
          domain: r.domainName ? { id: '', name: r.domainName, ownerId: '' } : undefined,
        }));
        setSearchResults(mapped);
      } catch (err: any) {
        console.error('Search failed:', err);
        // Fall back to showing no results rather than error
        setSearchResults([]);
      } finally {
        setSearchLoading(false);
      }
    }, 300);

    return () => {
      if (searchDebounceRef.current) {
        clearTimeout(searchDebounceRef.current);
      }
    };
  }, [searchTerm, workspaceFilter]);

  const fetchRoutes = async () => {
    try {
      setLoading(true);
      setError(null);
      const params: any = { page: 1, pageSize: 100 };
      if (workspaceFilter && workspaceFilter !== 'all') {
        params.workspaceId = workspaceFilter;
      }
      const response = await apiService.routes.list(params);
      setRoutes(response.data);
    } catch (err: any) {
      console.error('Failed to fetch routes:', err);
      setError('Failed to load routes. Please try again.');
    } finally {
      setLoading(false);
    }
  };

  const fetchDomains = async () => {
    try {
      const response = await apiService.domains.list({ page: 1, pageSize: 100 });
      setDomains(response.data);
    } catch (err: any) {
      console.error('Failed to fetch domains:', err);
    }
  };

  const fetchWorkspaces = async () => {
    try {
      const data = await apiService.workspaces.list();
      setWorkspaces(data);
    } catch (err: any) {
      console.error('Failed to fetch workspaces:', err);
    }
  };

  const handleDeleteRoute = async (route: RouteDto) => {
    if (!window.confirm(`Are you sure you want to delete the route "${route.link}"?`)) {
      return;
    }

    if (!route.id) {
      alert('Cannot delete route: missing ID');
      return;
    }

    try {
      await apiService.routes.delete(route.id);
      await fetchRoutes();
      if (selectedRoute?.id === route.id) {
        setSelectedRoute(null);
      }
    } catch (err: any) {
      console.error('Failed to delete route:', err);
      alert('Failed to delete route. Please try again.');
    }
  };

  const handleEditRoute = (route: RouteDto) => {
    setEditingRoute(route);
    setIsEditing(true);
  };

  const handleCancelEdit = () => {
    setEditingRoute(null);
    setIsEditing(false);
  };

  const handleCreateRoute = () => {
    setEditingRoute(null);
    setIsEditing(true);
  };

  const fetchAnalytics = useCallback(async (route: RouteDto) => {
    try {
      setAnalyticsLoading(true);

      const endDate = new Date();
      const startDate = new Date();

      switch (timeRange) {
        case '24h': startDate.setHours(startDate.getHours() - 24); break;
        case '7d': startDate.setDate(startDate.getDate() - 7); break;
        case '30d': startDate.setDate(startDate.getDate() - 30); break;
        case '90d': startDate.setDate(startDate.getDate() - 90); break;
      }

      const fromDate = startDate.toISOString().split('T')[0];
      const toDate = endDate.toISOString().split('T')[0];
      const fromHour = startDate.toISOString().replace('T', ' ').substring(0, 13);
      const toHour = endDate.toISOString().replace('T', ' ').substring(0, 13);

      const [dailyStats, geographicStats, deviceStats, browserStats, trafficStats] = await Promise.all([
        apiService.clickstream.getDailyStats({ routeId: route.id, fromDate, toDate }),
        apiService.clickstream.getGeographicStats({ routeId: route.id, fromDate, toDate }),
        apiService.clickstream.getDeviceStats({ routeId: route.id, fromDate, toDate }),
        apiService.clickstream.getBrowserStats({ routeId: route.id, fromDate, toDate }),
        apiService.clickstream.getTrafficTypeStats({ routeId: route.id, fromHour, toHour }),
      ]);

      const totals = dailyStats.reduce((acc, stat) => ({
        totalClicks: acc.totalClicks + stat.total_clicks,
        uniqueClicks: acc.uniqueClicks + stat.unique_clicks,
        botClicks: acc.botClicks + stat.bot_clicks,
        humanClicks: acc.humanClicks + stat.human_clicks,
      }), { totalClicks: 0, uniqueClicks: 0, botClicks: 0, humanClicks: 0 });

      const humanStat = trafficStats.find(stat => !stat.is_bot);
      const botStat = trafficStats.find(stat => stat.is_bot);

      const analyticsData = {
        totalClicks: totals.totalClicks,
        uniqueVisitors: totals.uniqueClicks,
        humanClicks: totals.humanClicks,
        botClicks: totals.botClicks,
        topCountries: geographicStats.slice(0, 10).map(stat => {
          const percentage = totals.totalClicks > 0
            ? (stat.total_clicks / totals.totalClicks) * 100
            : 0;
          return {
            name: stat.country,
            clicks: stat.total_clicks,
            percentage: Math.round(percentage)
          };
        }),
        topBrowsers: browserStats.slice(0, 8).map(stat => ({
          name: stat.user_agent_family,
          clicks: stat.total_clicks,
        })),
        topDevices: deviceStats.slice(0, 8).map(stat => ({
          device: `${stat.device_family} (${stat.os_family})`,
          clicks: stat.total_clicks,
        })),
        trafficType: [
          { name: 'Human', value: humanStat?.total_clicks || 0 },
          { name: 'Bot', value: botStat?.total_clicks || 0 },
        ],
        dailyClicks: dailyStats.map(stat => ({
          date: new Date(stat.date).toLocaleDateString('en-US', { month: 'short', day: 'numeric' }),
          clicks: stat.total_clicks,
        })),
      };

      setAnalytics(analyticsData);
    } catch (err: any) {
      console.error('Failed to fetch analytics:', err);
    } finally {
      setAnalyticsLoading(false);
    }
  }, [timeRange]);

  // Refetch analytics when route or time range changes
  useEffect(() => {
    if (selectedRoute) {
      fetchAnalytics(selectedRoute);
    }
  }, [selectedRoute, fetchAnalytics]);

  const handleSelectRoute = (route: RouteDto) => {
    setSelectedRoute(route);
  };

  const getPolicyType = (policy?: RoutingPolicy): string => {
    if (!policy || policy === 'Basic') return 'Basic';
    if (policy === 'Mirroring') return 'Mirroring';
    if (typeof policy === 'object') {
      if ('Conditional' in policy) return 'Conditional';
      if ('Challenge' in policy) return 'Challenge';
      if ('File' in policy) return 'File';
    }
    return 'Basic';
  };

  const getPolicyBadgeClass = (policyType: string): string => {
    switch (policyType) {
      case 'Conditional': return 'table-status-info';
      case 'Challenge': return 'table-status-warning';
      case 'File': return 'table-status-secondary';
      case 'Mirroring': return 'table-status-info';
      default: return 'table-status-secondary';
    }
  };

  // Use ES search results when search is active, otherwise use local routes with status filter
  const baseRoutes = searchResults !== null ? searchResults : routes;
  const filteredRoutes = baseRoutes.filter(route => {
    const matchesStatus = statusFilter === 'all' || route.status.toLowerCase() === statusFilter.toLowerCase();
    return matchesStatus;
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

  const humanRate = analytics && analytics.totalClicks > 0
    ? `${((analytics.humanClicks / analytics.totalClicks) * 100).toFixed(0)}%`
    : null;
  const botRate = analytics && analytics.totalClicks > 0
    ? `${((analytics.botClicks / analytics.totalClicks) * 100).toFixed(0)}%`
    : null;

  return (
    <div className={`routes-with-sidebar ${isEditing ? 'editing' : ''}`}>
      {/* Sidebar */}
      <div className="routes-sidebar">
        <div className="sidebar-content">
          {/* Search and Filters */}
          <div className="sidebar-controls">
            <div className="search-box">
              {searchLoading ? <RefreshCw size={16} className="icon-spin" /> : <Search size={16} />}
              <input
                type="text"
                placeholder="Search by link, domain, destination..."
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

            <div className="control-group">
              <select
                className="control-dropdown"
                value={workspaceFilter}
                onChange={(e) => setWorkspaceFilter(e.target.value)}
              >
                <option value="all">All Workspaces</option>
                {workspaces.map((workspace) => (
                  <option key={workspace.id} value={workspace.id}>
                    {workspace.name}
                  </option>
                ))}
              </select>
            </div>
          </div>

          {/* Routes List */}
          <div className="routes-list">
            {filteredRoutes.map((route) => (
              <div
                key={route.id || route.link}
                className={`route-item ${selectedRoute?.id === route.id ? 'selected' : ''}`}
                onClick={() => handleSelectRoute(route)}
              >
                <div className="route-info">
                  <div className="route-link">{route.link}</div>
                  {route.domain?.name && (
                    <div className="route-domain-name" style={{ fontSize: '0.75rem', color: 'var(--text-muted)', marginTop: '2px' }}>
                      {route.domain.name}
                    </div>
                  )}
                  <div className="route-destination">
                    {route.dest.length > 30 ? `${route.dest.substring(0, 30)}...` : route.dest}
                  </div>
                  <div className="route-meta">
                    <span className={`route-status ${route.status.toLowerCase()}`}>
                      {route.status}
                    </span>
                    {route.policy && (
                      <span className={`table-status-badge ${getPolicyBadgeClass(getPolicyType(route.policy))}`}>
                        {getPolicyType(route.policy)}
                      </span>
                    )}
                    {route.code > 0 && <span className="route-code">{route.code}</span>}
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
              <span className="stat-label">{searchResults !== null ? 'Results' : 'Total Routes'}</span>
              <span className="stat-value">{filteredRoutes.length}</span>
            </div>
            <div className="stat-item">
              <span className="stat-label">Active</span>
              <span className="stat-value">{filteredRoutes.filter(r => r.status.toLowerCase() === 'active').length}</span>
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
        {isEditing ? (
          <div className="route-edit-content" style={{ padding: '1.5rem' }}>
            <div className="edit-header">
              <h2>{editingRoute ? 'Edit Route' : 'Create New Route'}</h2>
            </div>
            <RouteForm
              route={editingRoute}
              domains={domains}
              workspaces={workspaces}
              showWorkspace
              onSave={async (data) => {
                if (editingRoute?.id) {
                  await apiService.routes.update(editingRoute.id, data);
                } else {
                  await apiService.routes.create(data);
                }
                await fetchRoutes();
                setEditingRoute(null);
                setIsEditing(false);
              }}
              onCancel={handleCancelEdit}
            />
          </div>
        ) : selectedRoute ? (
          <div className="analytics-content">
            {/* Analytics Header */}
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

                  <div className="dashboard-date-range">
                    {[
                      { value: '24h', label: '24h' },
                      { value: '7d', label: '7d' },
                      { value: '30d', label: '30d' },
                      { value: '90d', label: '90d' },
                    ].map((option) => (
                      <button
                        key={option.value}
                        className={`dashboard-date-btn ${timeRange === option.value ? 'active' : ''}`}
                        onClick={() => setTimeRange(option.value)}
                      >
                        {option.label}
                      </button>
                    ))}
                  </div>

                  <button
                    className="btn btn-outline btn-sm"
                    onClick={() => selectedRoute && fetchAnalytics(selectedRoute)}
                    disabled={analyticsLoading}
                    title="Refresh analytics"
                  >
                    <RefreshCw size={14} className={analyticsLoading ? 'icon-spin' : ''} />
                  </button>
                </div>
              </div>
            </div>

            {/* Analytics Body */}
            {analyticsLoading && !analytics ? (
              <div className="welcome-content">
                <LoadingSpinner />
                <p>Loading analytics...</p>
              </div>
            ) : analytics ? (
              <div className="route-analytics-body">
                {/* Refreshing indicator */}
                {analyticsLoading && (
                  <div className="dashboard-refreshing">Updating...</div>
                )}

                {/* Stats Cards */}
                <div className="route-analytics-stats">
                  {[
                    { icon: MousePointer, value: analytics.totalClicks, label: 'Total Clicks', color: 'var(--primary-500)' },
                    { icon: Users, value: analytics.uniqueVisitors, label: 'Unique Visitors', color: 'var(--success-500)' },
                    { icon: Activity, value: analytics.humanClicks, label: humanRate ? `Human (${humanRate})` : 'Human', color: 'var(--success-600)' },
                    { icon: Bot, value: analytics.botClicks, label: botRate ? `Bot (${botRate})` : 'Bot', color: 'var(--error-500)' },
                  ].map((stat, idx) => (
                    <div key={idx} className="card dashboard-stat-card">
                      <div className="dashboard-stat-icon" style={{ color: stat.color }}>
                        <stat.icon size={20} />
                      </div>
                      <div>
                        <div className="dashboard-stat-value">
                          {typeof stat.value === 'number' ? stat.value.toLocaleString() : stat.value || '0'}
                        </div>
                        <div className="dashboard-stat-label">{stat.label}</div>
                      </div>
                    </div>
                  ))}
                </div>

                {/* Daily Clicks - Full Width Area Chart */}
                <div className="dashboard-section">
                  <div className="card dashboard-chart-card">
                    <h3 className="dashboard-chart-title">Clicks Over Time</h3>
                    <p className="dashboard-chart-desc">Daily click trends for this route</p>
                    <div style={{ height: '280px' }}>
                      <ResponsiveContainer width="100%" height="100%">
                        <AreaChart data={analytics.dailyClicks}>
                          <defs>
                            <linearGradient id="routeClicksGradient" x1="0" y1="0" x2="0" y2="1">
                              <stop offset="0%" stopColor="#3b82f6" stopOpacity={0.15} />
                              <stop offset="100%" stopColor="#3b82f6" stopOpacity={0} />
                            </linearGradient>
                          </defs>
                          <CartesianGrid strokeDasharray="3 3" stroke="var(--border-secondary)" vertical={false} />
                          <XAxis
                            dataKey="date"
                            stroke="var(--text-muted)"
                            fontSize={11}
                            tickLine={false}
                            axisLine={false}
                          />
                          <YAxis
                            stroke="var(--text-muted)"
                            fontSize={11}
                            tickLine={false}
                            axisLine={false}
                            width={45}
                            tickFormatter={formatAxisNumber}
                          />
                          <Tooltip content={<CustomTooltip />} />
                          <Area
                            type="monotone"
                            dataKey="clicks"
                            name="Clicks"
                            stroke="var(--primary-500)"
                            fill="url(#routeClicksGradient)"
                            strokeWidth={2}
                            dot={false}
                            activeDot={{ r: 4, fill: 'var(--primary-500)', stroke: 'var(--bg-primary)', strokeWidth: 2 }}
                          />
                        </AreaChart>
                      </ResponsiveContainer>
                    </div>
                  </div>
                </div>

                {/* Geographic Section */}
                <div className="dashboard-section route-analytics-geo">
                  <div className="card dashboard-chart-card">
                    <h3 className="dashboard-chart-title">
                      <Globe size={16} style={{ color: 'var(--primary-500)' }} />
                      Geographic Distribution
                    </h3>
                    <p className="dashboard-chart-desc">Hover to see details</p>
                    <div style={{ height: '300px' }}>
                      <WorldMap data={analytics.topCountries} height={300} />
                    </div>
                  </div>
                  <div className="card dashboard-chart-card">
                    <h3 className="dashboard-chart-title">Top Countries</h3>
                    <p className="dashboard-chart-desc">By click volume</p>
                    <div className="route-analytics-countries">
                      {analytics.topCountries.length > 0 ? (
                        analytics.topCountries.map((country: any, index: number) => (
                          <div key={country.name} className="ra-country-row">
                            <span className="ra-country-rank">{index + 1}</span>
                            <span className="ra-country-name">{country.name}</span>
                            <div className="ra-country-bar-wrap">
                              <div className="ra-country-bar">
                                <div
                                  className="ra-country-bar-fill"
                                  style={{ width: `${country.percentage || 0}%` }}
                                />
                              </div>
                            </div>
                            <span className="ra-country-value">{(country.clicks || 0).toLocaleString()}</span>
                            <span className="ra-country-pct">{country.percentage || 0}%</span>
                          </div>
                        ))
                      ) : (
                        <div className="dashboard-empty-state" style={{ padding: '2rem 0' }}>
                          <Globe size={28} style={{ opacity: 0.4 }} />
                          <p>No geographic data</p>
                        </div>
                      )}
                    </div>
                  </div>
                </div>

                {/* Distribution Section */}
                <div className="dashboard-section route-analytics-distribution">
                  {/* Browser Distribution - Pie Chart */}
                  <div className="card dashboard-chart-card">
                    <h3 className="dashboard-chart-title">Browsers</h3>
                    <p className="dashboard-chart-desc">By browser</p>
                    <div style={{ height: '200px' }}>
                      {analytics.topBrowsers.length > 0 ? (
                        <ResponsiveContainer width="100%" height="100%">
                          <PieChart>
                            <Pie
                              data={analytics.topBrowsers}
                              cx="50%"
                              cy="50%"
                              outerRadius={70}
                              innerRadius={45}
                              dataKey="clicks"
                              nameKey="name"
                              paddingAngle={2}
                              stroke="var(--bg-primary)"
                              strokeWidth={2}
                            >
                              {analytics.topBrowsers.map((_: any, index: number) => (
                                <Cell key={`cell-${index}`} fill={CHART_COLORS[index % CHART_COLORS.length]} />
                              ))}
                            </Pie>
                            <Tooltip content={<CustomTooltip />} />
                          </PieChart>
                        </ResponsiveContainer>
                      ) : (
                        <div className="dashboard-empty-state">
                          <Activity size={28} style={{ opacity: 0.4 }} />
                          <p>No browser data</p>
                        </div>
                      )}
                    </div>
                    {analytics.topBrowsers.length > 0 && (
                      <PieLegend
                        items={analytics.topBrowsers.map((entry: any, i: number) => ({
                          name: entry.name,
                          color: CHART_COLORS[i % CHART_COLORS.length],
                        }))}
                      />
                    )}
                  </div>

                  {/* Device Distribution - Pie Chart */}
                  <div className="card dashboard-chart-card">
                    <h3 className="dashboard-chart-title">Devices</h3>
                    <p className="dashboard-chart-desc">By device type</p>
                    <div style={{ height: '200px' }}>
                      {analytics.topDevices.length > 0 ? (
                        <ResponsiveContainer width="100%" height="100%">
                          <PieChart>
                            <Pie
                              data={analytics.topDevices}
                              cx="50%"
                              cy="50%"
                              outerRadius={70}
                              innerRadius={45}
                              dataKey="clicks"
                              nameKey="device"
                              paddingAngle={2}
                              stroke="var(--bg-primary)"
                              strokeWidth={2}
                            >
                              {analytics.topDevices.map((_: any, index: number) => (
                                <Cell key={`cell-${index}`} fill={CHART_COLORS[index % CHART_COLORS.length]} />
                              ))}
                            </Pie>
                            <Tooltip content={<CustomTooltip />} />
                          </PieChart>
                        </ResponsiveContainer>
                      ) : (
                        <div className="dashboard-empty-state">
                          <Activity size={28} style={{ opacity: 0.4 }} />
                          <p>No device data</p>
                        </div>
                      )}
                    </div>
                    {analytics.topDevices.length > 0 && (
                      <PieLegend
                        items={analytics.topDevices.map((entry: any, i: number) => ({
                          name: entry.device,
                          color: CHART_COLORS[i % CHART_COLORS.length],
                        }))}
                      />
                    )}
                  </div>

                  {/* Traffic Type - Pie Chart */}
                  <div className="card dashboard-chart-card">
                    <h3 className="dashboard-chart-title">Traffic Type</h3>
                    <p className="dashboard-chart-desc">Bot vs Human</p>
                    <div style={{ height: '200px' }}>
                      {analytics.trafficType.some((t: any) => t.value > 0) ? (
                        <ResponsiveContainer width="100%" height="100%">
                          <PieChart>
                            <Pie
                              data={analytics.trafficType}
                              cx="50%"
                              cy="50%"
                              labelLine={false}
                              label={({ name, percent }) => percent > 0.05 ? `${name} ${(percent * 100).toFixed(0)}%` : ''}
                              outerRadius={70}
                              innerRadius={45}
                              dataKey="value"
                              paddingAngle={3}
                              stroke="var(--bg-primary)"
                              strokeWidth={2}
                            >
                              <Cell key="cell-0" fill={TRAFFIC_COLORS[0]} />
                              <Cell key="cell-1" fill={TRAFFIC_COLORS[1]} />
                            </Pie>
                            <Tooltip content={<CustomTooltip />} />
                          </PieChart>
                        </ResponsiveContainer>
                      ) : (
                        <div className="dashboard-empty-state">
                          <Activity size={28} style={{ opacity: 0.4 }} />
                          <p>No traffic data</p>
                        </div>
                      )}
                    </div>
                    {analytics.trafficType.some((t: any) => t.value > 0) && (
                      <PieLegend
                        items={analytics.trafficType.map((entry: any, i: number) => ({
                          name: entry.name,
                          color: TRAFFIC_COLORS[i],
                        }))}
                      />
                    )}
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
