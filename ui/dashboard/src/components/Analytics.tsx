import React, { useState, useEffect } from 'react';
import { Container, Row, Col, Card, Button, ButtonGroup, Alert } from 'react-bootstrap';
import { 
  BarChart, 
  Bar, 
  XAxis, 
  YAxis, 
  CartesianGrid, 
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
  Calendar,
  Download
} from 'lucide-react';
import { apiService, ClickAnalytics } from '../services/api';
import LoadingSpinner from './LoadingSpinner';
import './Analytics.css';
import './UnifiedTable.css';

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
      <div className="error-message">
        <p>{error}</p>
        <button onClick={fetchAnalytics} className="btn">
          Retry
        </button>
      </div>
    );
  }

  const COLORS = ['#00d4aa', '#ff6b6b', '#ffa726', '#42a5f5', '#ab47bc'];

  return (
    <div className="analytics-page">
      {/* Header */}
      <div className="analytics-header">
        <div className="analytics-title">
          <h2>Analytics Dashboard</h2>
          <p>Detailed insights into your URL performance</p>
        </div>
        <div className="analytics-actions">
          <div className="date-range-selector">
            {[
              { value: '7d', label: '7 Days' },
              { value: '30d', label: '30 Days' },
              { value: '90d', label: '90 Days' },
              { value: '1y', label: '1 Year' },
            ].map((option) => (
              <button
                key={option.value}
                onClick={() => setDateRange(option.value)}
                className={`date-range-button ${dateRange === option.value ? 'active' : ''}`}
              >
                {option.label}
              </button>
            ))}
          </div>
          <button onClick={exportData} className="btn btn-secondary">
            <Download className="btn-icon" />
            Export
          </button>
        </div>
      </div>

      {/* Key Metrics */}
      <div className="metrics-grid">
        <div className="metric-card">
          <div className="metric-icon">
            <MousePointer />
          </div>
          <div className="metric-content">
            <div className="metric-value">{analytics?.total_clicks.toLocaleString() || '0'}</div>
            <div className="metric-label">Total Clicks</div>
            <div className="metric-change">+12.5% from last period</div>
          </div>
        </div>

        <div className="metric-card">
          <div className="metric-icon">
            <Users />
          </div>
          <div className="metric-content">
            <div className="metric-value">{analytics?.unique_clicks.toLocaleString() || '0'}</div>
            <div className="metric-label">Unique Visitors</div>
            <div className="metric-change">+8.3% from last period</div>
          </div>
        </div>

        <div className="metric-card">
          <div className="metric-icon">
            <TrendingUp />
          </div>
          <div className="metric-content">
            <div className="metric-value">
              {analytics ? ((analytics.unique_clicks / analytics.total_clicks) * 100).toFixed(1) : '0'}%
            </div>
            <div className="metric-label">Unique Rate</div>
            <div className="metric-change">-2.1% from last period</div>
          </div>
        </div>

        <div className="metric-card">
          <div className="metric-icon">
            <Globe />
          </div>
          <div className="metric-content">
            <div className="metric-value">{analytics?.clicks_by_country?.length || '0'}</div>
            <div className="metric-label">Countries</div>
            <div className="metric-change">+3 new countries</div>
          </div>
        </div>
      </div>

      {/* Charts Section */}
      <div className="charts-section">
        {/* Clicks Over Time */}
        <div className="chart-card full-width">
          <div className="chart-header">
            <h3>Clicks Over Time</h3>
            <Calendar className="chart-icon" />
          </div>
          <div className="chart-container large">
            <ResponsiveContainer width="100%" height={400}>
              <LineChart data={analytics?.clicks_by_date || []}>
                <CartesianGrid strokeDasharray="3 3" stroke="var(--border-color)" />
                <XAxis 
                  dataKey="date" 
                  stroke="var(--text-secondary)"
                  fontSize={12}
                />
                <YAxis 
                  stroke="var(--text-secondary)"
                  fontSize={12}
                />
                <Tooltip 
                  contentStyle={{
                    backgroundColor: 'var(--bg-secondary)',
                    border: '1px solid var(--border-color)',
                    borderRadius: '8px',
                    color: 'var(--text-primary)'
                  }}
                />
                <Line 
                  type="monotone" 
                  dataKey="clicks" 
                  stroke="var(--primary-color)" 
                  strokeWidth={3}
                  dot={{ fill: 'var(--primary-color)', strokeWidth: 2, r: 5 }}
                />
              </LineChart>
            </ResponsiveContainer>
          </div>
        </div>

        {/* Geographic Distribution */}
        <div className="chart-card">
          <div className="chart-header">
            <h3>Top Countries</h3>
            <Globe className="chart-icon" />
          </div>
          <div className="chart-container">
            <ResponsiveContainer width="100%" height={300}>
              <BarChart data={analytics?.clicks_by_country?.slice(0, 8) || []}>
                <CartesianGrid strokeDasharray="3 3" stroke="var(--border-color)" />
                <XAxis 
                  dataKey="country" 
                  stroke="var(--text-secondary)"
                  fontSize={12}
                />
                <YAxis 
                  stroke="var(--text-secondary)"
                  fontSize={12}
                />
                <Tooltip 
                  contentStyle={{
                    backgroundColor: 'var(--bg-secondary)',
                    border: '1px solid var(--border-color)',
                    borderRadius: '8px',
                    color: 'var(--text-primary)'
                  }}
                />
                <Bar dataKey="clicks" fill="var(--primary-color)" />
              </BarChart>
            </ResponsiveContainer>
          </div>
        </div>

        {/* Device Distribution */}
        <div className="chart-card">
          <div className="chart-header">
            <h3>Device Types</h3>
            <TrendingUp className="chart-icon" />
          </div>
          <div className="chart-container">
            <ResponsiveContainer width="100%" height={300}>
              <PieChart>
                <Pie
                  data={analytics?.clicks_by_device || []}
                  cx="50%"
                  cy="50%"
                  labelLine={false}
                  label={({ device, percent }) => `${device} ${(percent * 100).toFixed(0)}%`}
                  outerRadius={80}
                  fill="#8884d8"
                  dataKey="clicks"
                >
                  {(analytics?.clicks_by_device || []).map((entry, index) => (
                    <Cell key={`cell-${index}`} fill={COLORS[index % COLORS.length]} />
                  ))}
                </Pie>
                <Tooltip 
                  contentStyle={{
                    backgroundColor: 'var(--bg-secondary)',
                    border: '1px solid var(--border-color)',
                    borderRadius: '8px',
                    color: 'var(--text-primary)'
                  }}
                />
              </PieChart>
            </ResponsiveContainer>
          </div>
        </div>
      </div>

      {/* Top Routes Table */}
      <div className="top-routes-section">
        <div className="section-header">
          <h3>Top Performing Routes</h3>
          <p>Your most clicked shortened URLs</p>
        </div>
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
  );
};

export default Analytics;
