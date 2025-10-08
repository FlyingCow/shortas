import React, { useState, useEffect } from 'react';
import { 
  BarChart, 
  Bar, 
  XAxis, 
  YAxis, 
  Tooltip, 
  ResponsiveContainer,
  LineChart,
  Line,
  PieChart,
  Pie,
  Cell
} from 'recharts';
import { 
  TrendingUp, 
  Users, 
  MousePointer, 
  Globe,
  Download,
} from 'lucide-react';
// Removed Bootstrap Dropdown import - using unified controls
import { apiService, ClickAnalytics } from '../services/api';
import LoadingSpinner from './LoadingSpinner';
import './DesignSystem.css';

const Analytics: React.FC = () => {
  const [analytics, setAnalytics] = useState<ClickAnalytics | null>(null);
  const [topRoutes, setTopRoutes] = useState<any[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const [dateRange, setDateRange] = useState('30d');

  useEffect(() => {
    fetchAnalytics();
  }, [dateRange]);

  const fetchAnalytics = async () => {
    try {
      setLoading(true);
      setError(null);

      // Calculate date range
      const endDate = new Date();
      const startDate = new Date();
      
      switch (dateRange) {
        case '7d':
          startDate.setDate(startDate.getDate() - 7);
          break;
        case '30d':
          startDate.setDate(startDate.getDate() - 30);
          break;
        case '90d':
          startDate.setDate(startDate.getDate() - 90);
          break;
        case '1y':
          startDate.setFullYear(startDate.getFullYear() - 1);
          break;
      }

      const dateRangeParams = {
        start: startDate.toISOString(),
        end: endDate.toISOString(),
      };

      // Fetch analytics data
      const [analyticsData, topRoutesData] = await Promise.all([
        apiService.analytics.getOverview(dateRangeParams),
        apiService.analytics.getTopRoutes(10),
      ]);

      setAnalytics(analyticsData);
      setTopRoutes(topRoutesData);
    } catch (err) {
      console.error('Failed to fetch analytics:', err);
      setError('Failed to load analytics data. Please try again.');
    } finally {
      setLoading(false);
    }
  };

  const exportData = () => {
    // This would implement data export functionality
    alert('Export functionality would be implemented here');
  };

  if (loading) {
    return <LoadingSpinner message="Loading analytics..." />;
  }

  if (error) {
    return (
      <div className="alert alert-error">
        <h3>Error Loading Analytics</h3>
        <p>{error}</p>
        <button className="btn btn-primary" onClick={fetchAnalytics}>
          Retry
        </button>
      </div>
    );
  }

  const COLORS = ['var(--primary-500)', 'var(--success-500)', 'var(--warning-500)', 'var(--error-500)', 'var(--primary-600)'];

  return (
    <div className="container">
      {/* Page Header */}
      <div className="page-header">
        <h1 className="page-title">Analytics</h1>
        <p className="page-subtitle">Detailed insights into your URL performance</p>
      </div>

      {/* Date Range Selector */}
      <div className="card mb-lg">
        <div className="card-body">
          <div className="flex items-center justify-between">
            <div className="control-group">
              <select 
                className="control-dropdown"
                value={dateRange}
                onChange={(e) => setDateRange(e.target.value)}
              >
                <option value="7d">7 Days</option>
                <option value="30d">30 Days</option>
                <option value="90d">90 Days</option>
                <option value="1y">1 Year</option>
              </select>
            </div>
            <button className="btn btn-secondary" onClick={exportData}>
              <Download size={16} />
              Export
            </button>
          </div>
        </div>
      </div>

      {/* Key Metrics */}
      <div className="stats-grid">
        <div className="stats-card">
          <div className="stats-icon">
            <MousePointer size={24} />
          </div>
          <div className="stats-content">
            <div className="stats-value">{analytics?.total_clicks.toLocaleString() || '0'}</div>
            <div className="stats-label">Total Clicks</div>
            <div className="stats-change">+12.5% from last period</div>
          </div>
        </div>

        <div className="stats-card">
          <div className="stats-icon">
            <Users size={24} />
          </div>
          <div className="stats-content">
            <div className="stats-value">{analytics?.unique_clicks.toLocaleString() || '0'}</div>
            <div className="stats-label">Unique Visitors</div>
            <div className="stats-change">+8.3% from last period</div>
          </div>
        </div>

        <div className="stats-card">
          <div className="stats-icon">
            <TrendingUp size={24} />
          </div>
          <div className="stats-content">
            <div className="stats-value">
              {analytics ? ((analytics.unique_clicks / analytics.total_clicks) * 100).toFixed(1) : '0'}%
            </div>
            <div className="stats-label">Unique Rate</div>
            <div className="stats-change">-2.1% from last period</div>
          </div>
        </div>

        <div className="stats-card">
          <div className="stats-icon">
            <Globe size={24} />
          </div>
          <div className="stats-content">
            <div className="stats-value">{analytics?.clicks_by_country?.length || '0'}</div>
            <div className="stats-label">Countries</div>
            <div className="stats-change">+3 new countries</div>
          </div>
        </div>
      </div>

      {/* Charts Grid */}
      <div className="grid grid-cols-1 lg:grid-cols-2 gap-lg mb-2xl">
        {/* Clicks Over Time */}
        <div className="card">
          <div className="card-header">
            <h3 className="card-title">Clicks Over Time</h3>
            <p className="card-subtitle">Daily click trends</p>
          </div>
          <div className="card-body">
            <div style={{ height: '300px' }}>
              <ResponsiveContainer width="100%" height="100%">
                <LineChart data={analytics?.clicks_by_date || []}>
                  <XAxis 
                    dataKey="date" 
                    stroke="var(--text-muted)"
                    fontSize={12}
                  />
                  <YAxis 
                    stroke="var(--text-muted)"
                    fontSize={12}
                  />
                  <Tooltip 
                    contentStyle={{
                      backgroundColor: 'var(--bg-primary)',
                      border: '1px solid var(--border-primary)',
                      borderRadius: '0px',
                      color: 'var(--text-primary)'
                    }}
                  />
                  <Line 
                    type="monotone" 
                    dataKey="clicks" 
                    stroke="var(--primary-500)" 
                    strokeWidth={3}
                    dot={{ fill: 'var(--primary-500)', strokeWidth: 2, r: 5 }}
                  />
                </LineChart>
              </ResponsiveContainer>
            </div>
          </div>
        </div>

        {/* Geographic Distribution */}
        <div className="card">
          <div className="card-header">
            <h3 className="card-title">Top Countries</h3>
            <p className="card-subtitle">Geographic distribution</p>
          </div>
          <div className="card-body">
            <div style={{ height: '300px' }}>
              <ResponsiveContainer width="100%" height="100%">
                <BarChart data={analytics?.clicks_by_country?.slice(0, 8) || []}>
                  <XAxis 
                    dataKey="country" 
                    stroke="var(--text-muted)"
                    fontSize={12}
                  />
                  <YAxis 
                    stroke="var(--text-muted)"
                    fontSize={12}
                  />
                  <Tooltip 
                    contentStyle={{
                      backgroundColor: 'var(--bg-primary)',
                      border: '1px solid var(--border-primary)',
                      borderRadius: '0px',
                      color: 'var(--text-primary)'
                    }}
                  />
                  <Bar dataKey="clicks" fill="var(--primary-500)" radius={[0, 0, 0, 0]} />
                </BarChart>
              </ResponsiveContainer>
            </div>
          </div>
        </div>

        {/* Device Distribution */}
        <div className="card">
          <div className="card-header">
            <h3 className="card-title">Device Types</h3>
            <p className="card-subtitle">Click sources by device</p>
          </div>
          <div className="card-body">
            <div style={{ height: '300px' }}>
              <ResponsiveContainer width="100%" height="100%">
                <PieChart>
                  <Pie
                    data={analytics?.clicks_by_device || []}
                    cx="50%"
                    cy="50%"
                    labelLine={false}
                    label={({ device, percent }) => percent > 0.05 ? `${device} ${(percent * 100).toFixed(0)}%` : ''}
                    outerRadius={120}
                    innerRadius={60}
                    fill="var(--primary-400)"
                    dataKey="clicks"
                    paddingAngle={2}
                    stroke="var(--bg-primary)"
                    strokeWidth={2}
                  >
                    {(analytics?.clicks_by_device || []).map((entry, index) => (
                      <Cell key={`cell-${index}`} fill={COLORS[index % COLORS.length]} />
                    ))}
                  </Pie>
                  <Tooltip 
                    contentStyle={{
                      backgroundColor: 'var(--bg-primary)',
                      border: '1px solid var(--border-primary)',
                      borderRadius: '0px',
                      color: 'var(--text-primary)'
                    }}
                    formatter={(value: any, name: string, props: any) => [
                      `${value} clicks (${((props.payload.percent || 0) * 100).toFixed(1)}%)`,
                      props.payload.device
                    ]}
                  />
                </PieChart>
              </ResponsiveContainer>
            </div>
          </div>
        </div>

        {/* Browser Distribution */}
        <div className="card">
          <div className="card-header">
            <h3 className="card-title">Browser Distribution</h3>
            <p className="card-subtitle">Click sources by browser</p>
          </div>
          <div className="card-body">
            <div style={{ height: '300px' }}>
              <ResponsiveContainer width="100%" height="100%">
                <PieChart>
                  <Pie
                    data={analytics?.clicks_by_browser || []}
                    cx="50%"
                    cy="50%"
                    labelLine={false}
                    label={({ browser, percent }) => percent > 0.05 ? `${browser} ${(percent * 100).toFixed(0)}%` : ''}
                    outerRadius={120}
                    innerRadius={60}
                    fill="var(--primary-400)"
                    dataKey="clicks"
                    paddingAngle={2}
                    stroke="var(--bg-primary)"
                    strokeWidth={2}
                  >
                    {(analytics?.clicks_by_browser || []).map((entry, index) => (
                      <Cell key={`cell-${index}`} fill={COLORS[index % COLORS.length]} />
                    ))}
                  </Pie>
                  <Tooltip 
                    contentStyle={{
                      backgroundColor: 'var(--bg-primary)',
                      border: '1px solid var(--border-primary)',
                      borderRadius: '0px',
                      color: 'var(--text-primary)'
                    }}
                    formatter={(value: any, name: string, props: any) => [
                      `${value} clicks (${((props.payload.percent || 0) * 100).toFixed(1)}%)`,
                      props.payload.browser
                    ]}
                  />
                </PieChart>
              </ResponsiveContainer>
            </div>
          </div>
        </div>
      </div>

      {/* Top Routes Table */}
      <div className="card">
        <div className="card-header">
          <h3 className="card-title">Top Performing Routes</h3>
          <p className="card-subtitle">Your most clicked shortened URLs</p>
        </div>
        <div className="card-body p-0">
          <div className="table-container">
            <div className="table-wrapper">
              <table className="unified-table">
                <thead>
                  <tr>
                    <th>Route</th>
                    <th>Destination</th>
                    <th>Clicks</th>
                    <th>Unique Clicks</th>
                    <th>CTR</th>
                  </tr>
                </thead>
                <tbody>
                  {topRoutes.map((route, index) => (
                    <tr key={index}>
                      <td>
                        <div className="table-cell-text">
                          <span className="table-url">{route.link}</span>
                        </div>
                      </td>
                      <td>
                        <div className="table-cell-text">
                          <span className="table-cell-primary" title={route.destination}>
                            {route.destination?.length > 40 
                              ? `${route.destination.substring(0, 40)}...` 
                              : route.destination}
                          </span>
                        </div>
                      </td>
                      <td>
                        <span className="table-metric table-metric-large">
                          {route.total_clicks?.toLocaleString() || '0'}
                        </span>
                      </td>
                      <td>
                        <span className="table-metric table-metric-large">
                          {route.unique_clicks?.toLocaleString() || '0'}
                        </span>
                      </td>
                      <td>
                        <span className="table-metric table-response-fast">
                          {route.total_clicks ? ((route.unique_clicks / route.total_clicks) * 100).toFixed(1) : '0'}%
                        </span>
                      </td>
                    </tr>
                  ))}
                </tbody>
              </table>
              
              {topRoutes.length === 0 && (
                <div className="table-empty">
                  <div className="table-empty-icon">
                    <TrendingUp size={48} />
                  </div>
                  <div className="table-empty-title">No routes found</div>
                  <div className="table-empty-description">
                    No route data available for the selected time period.
                  </div>
                </div>
              )}
            </div>
          </div>
        </div>
      </div>
    </div>
  );
};

export default Analytics;
