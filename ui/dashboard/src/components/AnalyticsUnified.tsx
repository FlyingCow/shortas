import React, { useState, useEffect, useCallback } from 'react';
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
import {
  TrendingUp,
  Users,
  MousePointerClick,
  Globe,
  Download,
  RefreshCw,
  BarChart3,
  Monitor,
  Activity,
  Target
} from 'lucide-react';
import { apiService, ClickAnalytics, RoutePerformanceDto } from '../services/api';
import LoadingSpinner from './LoadingSpinner';
import WorldMap from './WorldMap';
import './DesignSystem.css';

const analyticsStyles = `
/* ===== ANALYTICS PAGE STYLES ===== */

/* Page Header */
.an-page-header {
  display: flex;
  align-items: flex-start;
  justify-content: space-between;
  gap: 1.5rem;
  margin-bottom: 2rem;
  flex-wrap: wrap;
}

.an-page-title-section {
  flex: 1;
  min-width: 200px;
}

.an-page-title {
  font-size: 1.5rem;
  font-weight: 700;
  color: var(--text-primary);
  margin: 0 0 0.25rem 0;
  letter-spacing: -0.025em;
}

.an-page-subtitle {
  font-size: 0.875rem;
  color: var(--text-muted);
  margin: 0;
}

.an-page-actions {
  display: flex;
  align-items: center;
  gap: 0.75rem;
  flex-wrap: wrap;
}

/* Stats Grid */
.an-stats-grid {
  display: grid;
  grid-template-columns: repeat(4, 1fr);
  gap: 1rem;
  margin-bottom: 1.5rem;
}

@media (max-width: 1200px) {
  .an-stats-grid {
    grid-template-columns: repeat(2, 1fr);
  }
}

@media (max-width: 600px) {
  .an-stats-grid {
    grid-template-columns: 1fr;
  }
}

.an-stat-card {
  background: var(--bg-elevated);
  border: 1px solid var(--border-primary);
  border-radius: var(--radius-xl);
  padding: 1.25rem;
  display: flex;
  align-items: center;
  gap: 1rem;
  transition: all var(--transition-normal);
}

.an-stat-card:hover {
  box-shadow: var(--shadow-md);
  border-color: var(--border-secondary);
}

.an-stat-icon {
  width: 44px;
  height: 44px;
  border-radius: var(--radius-lg);
  display: flex;
  align-items: center;
  justify-content: center;
  flex-shrink: 0;
}

.an-stat-icon svg {
  width: 22px;
  height: 22px;
  color: #fff;
}

.an-stat-icon.primary { background: var(--color-primary); }
.an-stat-icon.success { background: var(--color-success); }
.an-stat-icon.warning { background: var(--color-warning); }
.an-stat-icon.error { background: var(--color-error); }
.an-stat-icon.info { background: #8b5cf6; }

.an-stat-content {
  flex: 1;
  min-width: 0;
}

.an-stat-value {
  font-size: 1.5rem;
  font-weight: 700;
  color: var(--text-primary);
  line-height: 1.2;
  margin-bottom: 0.125rem;
}

.an-stat-label {
  font-size: 0.75rem;
  color: var(--text-secondary);
  font-weight: 500;
  text-transform: uppercase;
  letter-spacing: 0.03em;
}

/* Toolbar */
.an-toolbar {
  background: var(--bg-elevated);
  border: 1px solid var(--border-primary);
  border-radius: var(--radius-xl);
  padding: 1rem 1.25rem;
  margin-bottom: 1.5rem;
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 1rem;
  flex-wrap: wrap;
}

.an-toolbar-section {
  display: flex;
  align-items: center;
  gap: 0.75rem;
  flex-wrap: wrap;
}

/* Date Range Buttons */
.an-range-group {
  display: inline-flex;
  border: 1px solid var(--border-secondary);
  border-radius: var(--radius-lg);
  overflow: hidden;
  background: var(--bg-primary);
}

.an-range-btn {
  padding: 0.5rem 0.875rem;
  font-size: 0.8125rem;
  font-weight: 600;
  border: none;
  background: transparent;
  color: var(--text-secondary);
  cursor: pointer;
  transition: all var(--transition-fast);
  font-family: inherit;
}

.an-range-btn:not(:last-child) {
  border-right: 1px solid var(--border-secondary);
}

.an-range-btn:hover:not(.active) {
  background: var(--bg-tertiary);
  color: var(--text-primary);
}

.an-range-btn.active {
  background: var(--color-primary);
  color: #ffffff;
}

/* Refreshing indicator */
.an-refreshing {
  position: fixed;
  top: 70px;
  right: 20px;
  background: var(--bg-elevated);
  border: 1px solid var(--border-primary);
  padding: 0.5rem 1rem;
  border-radius: var(--radius-lg);
  font-size: 0.8125rem;
  color: var(--text-secondary);
  box-shadow: var(--shadow-lg);
  z-index: 100;
  display: flex;
  align-items: center;
  gap: 0.5rem;
}

/* Section Grid */
.an-section {
  margin-bottom: 1.5rem;
}

.an-section-row {
  display: grid;
  gap: 1.5rem;
  margin-bottom: 1.5rem;
}

.an-section-row.two-cols {
  grid-template-columns: repeat(2, 1fr);
}

.an-section-row.three-cols {
  grid-template-columns: repeat(3, 1fr);
}

@media (max-width: 1024px) {
  .an-section-row.two-cols,
  .an-section-row.three-cols {
    grid-template-columns: 1fr;
  }
}

/* Chart Card */
.an-chart-card {
  background: var(--bg-elevated);
  border: 1px solid var(--border-primary);
  border-radius: var(--radius-xl);
  overflow: hidden;
}

.an-chart-header {
  padding: 1rem 1.25rem;
  border-bottom: 1px solid var(--border-primary);
  background: var(--bg-secondary);
}

.an-chart-title {
  font-size: 0.9375rem;
  font-weight: 600;
  color: var(--text-primary);
  margin: 0 0 0.25rem 0;
  display: flex;
  align-items: center;
  gap: 0.5rem;
}

.an-chart-title svg {
  color: var(--color-primary);
}

.an-chart-desc {
  font-size: 0.75rem;
  color: var(--text-muted);
  margin: 0;
}

.an-chart-body {
  padding: 1.25rem;
}

/* Tooltip */
.an-tooltip {
  background: var(--bg-elevated);
  border: 1px solid var(--border-primary);
  border-radius: var(--radius-md);
  padding: 0.75rem;
  box-shadow: var(--shadow-lg);
}

.an-tooltip-label {
  font-size: 0.75rem;
  font-weight: 600;
  color: var(--text-primary);
  margin-bottom: 0.5rem;
  padding-bottom: 0.5rem;
  border-bottom: 1px solid var(--border-primary);
}

.an-tooltip-row {
  display: flex;
  align-items: center;
  gap: 0.5rem;
  padding: 0.25rem 0;
  font-size: 0.8125rem;
}

.an-tooltip-dot {
  width: 8px;
  height: 8px;
  border-radius: 50%;
  flex-shrink: 0;
}

.an-tooltip-name {
  color: var(--text-secondary);
  flex: 1;
}

.an-tooltip-value {
  font-weight: 600;
  color: var(--text-primary);
}

/* Pie Legend */
.an-pie-legend {
  display: flex;
  flex-wrap: wrap;
  gap: 0.75rem;
  justify-content: center;
  padding: 0.75rem 1rem;
  border-top: 1px solid var(--border-primary);
  background: var(--bg-secondary);
}

.an-pie-legend-item {
  display: flex;
  align-items: center;
  gap: 0.375rem;
  font-size: 0.75rem;
}

.an-pie-legend-dot {
  width: 8px;
  height: 8px;
  border-radius: 50%;
  flex-shrink: 0;
}

.an-pie-legend-label {
  color: var(--text-secondary);
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
  max-width: 100px;
}

/* Country List */
.an-country-list {
  display: flex;
  flex-direction: column;
  gap: 0.5rem;
}

.an-country-row {
  display: flex;
  align-items: center;
  gap: 0.75rem;
  padding: 0.5rem 0;
}

.an-country-rank {
  width: 20px;
  height: 20px;
  border-radius: 50%;
  background: var(--bg-tertiary);
  display: flex;
  align-items: center;
  justify-content: center;
  font-size: 0.6875rem;
  font-weight: 600;
  color: var(--text-muted);
  flex-shrink: 0;
}

.an-country-name {
  flex: 1;
  font-size: 0.8125rem;
  font-weight: 500;
  color: var(--text-primary);
  min-width: 0;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.an-country-bar-wrap {
  flex: 1;
  max-width: 120px;
}

.an-country-bar {
  height: 6px;
  background: var(--bg-tertiary);
  border-radius: 3px;
  overflow: hidden;
}

.an-country-bar-fill {
  height: 100%;
  background: var(--color-primary);
  border-radius: 3px;
  transition: width var(--transition-normal);
}

.an-country-value {
  font-size: 0.8125rem;
  font-weight: 600;
  color: var(--text-primary);
  min-width: 50px;
  text-align: right;
}

.an-country-pct {
  font-size: 0.75rem;
  color: var(--text-muted);
  min-width: 40px;
  text-align: right;
}

/* Table */
.an-table-container {
  overflow-x: auto;
}

.an-table {
  width: 100%;
  border-collapse: collapse;
  font-size: 0.8125rem;
}

.an-table th {
  text-align: left;
  padding: 0.75rem 1rem;
  font-size: 0.6875rem;
  font-weight: 600;
  text-transform: uppercase;
  letter-spacing: 0.05em;
  color: var(--text-muted);
  background: var(--bg-secondary);
  border-bottom: 1px solid var(--border-primary);
  white-space: nowrap;
}

.an-table td {
  padding: 0.75rem 1rem;
  border-bottom: 1px solid var(--border-primary);
  vertical-align: middle;
}

.an-table tbody tr {
  transition: background var(--transition-fast);
}

.an-table tbody tr:hover {
  background: var(--bg-secondary);
}

.an-table tbody tr:last-child td {
  border-bottom: none;
}

.an-table-route {
  font-family: var(--font-family-mono);
  font-size: 0.75rem;
  color: var(--color-primary);
}

.an-table-value {
  font-weight: 600;
  color: var(--text-primary);
}

.an-table-success {
  color: var(--color-success);
}

.an-table-error {
  color: var(--color-error);
}

/* Empty State */
.an-empty {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  padding: 3rem 2rem;
  text-align: center;
}

.an-empty-icon {
  width: 56px;
  height: 56px;
  border-radius: 50%;
  background: var(--bg-tertiary);
  display: flex;
  align-items: center;
  justify-content: center;
  margin-bottom: 1rem;
}

.an-empty-icon svg {
  width: 28px;
  height: 28px;
  color: var(--text-muted);
}

.an-empty p {
  font-size: 0.875rem;
  color: var(--text-muted);
  margin: 0;
}

/* Error State */
.an-error {
  background: var(--bg-elevated);
  border: 1px solid var(--color-error);
  border-radius: var(--radius-xl);
  padding: 2rem;
  text-align: center;
  margin: 2rem 0;
}

.an-error-title {
  font-size: 1.125rem;
  font-weight: 600;
  color: var(--color-error);
  margin: 0 0 0.5rem 0;
}

.an-error-message {
  font-size: 0.875rem;
  color: var(--text-secondary);
  margin: 0 0 1rem 0;
}

/* Responsive */
@media (max-width: 768px) {
  .an-page-header {
    flex-direction: column;
    gap: 1rem;
  }

  .an-page-actions {
    width: 100%;
    justify-content: space-between;
  }

  .an-toolbar {
    flex-direction: column;
    align-items: stretch;
  }

  .an-toolbar-section {
    width: 100%;
    justify-content: center;
  }
}

/* Spin animation */
@keyframes an-spin {
  from { transform: rotate(0deg); }
  to { transform: rotate(360deg); }
}

.an-icon-spin {
  animation: an-spin 1s linear infinite;
}
`;

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

const formatAxisNumber = (value: number): string => {
  if (value >= 1000000) return `${(value / 1000000).toFixed(1)}M`;
  if (value >= 1000) return `${(value / 1000).toFixed(0)}k`;
  return value.toString();
};

const CustomTooltip = ({ active, payload, label }: any) => {
  if (!active || !payload?.length) return null;
  return (
    <div className="an-tooltip">
      {label != null && label !== '' && (
        <div className="an-tooltip-label">{label}</div>
      )}
      {payload.map((entry: any, i: number) => {
        const percent = entry.payload?.percent;
        return (
          <div key={i} className="an-tooltip-row">
            <span className="an-tooltip-dot" style={{ backgroundColor: entry.color || entry.fill }} />
            <span className="an-tooltip-name">{entry.name}</span>
            <span className="an-tooltip-value">
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
  <div className="an-pie-legend">
    {items.slice(0, 6).map((item, i) => (
      <div key={i} className="an-pie-legend-item">
        <span className="an-pie-legend-dot" style={{ backgroundColor: item.color }} />
        <span className="an-pie-legend-label">{item.name}</span>
      </div>
    ))}
  </div>
);

const Analytics: React.FC = () => {
  const [analytics, setAnalytics] = useState<ClickAnalytics | null>(null);
  const [routePerformance, setRoutePerformance] = useState<RoutePerformanceDto[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const [dateRange, setDateRange] = useState('30d');

  const fetchAnalytics = useCallback(async () => {
    try {
      setLoading(true);
      setError(null);

      const endDate = new Date();
      const startDate = new Date();

      switch (dateRange) {
        case '7d': startDate.setDate(startDate.getDate() - 7); break;
        case '30d': startDate.setDate(startDate.getDate() - 30); break;
        case '90d': startDate.setDate(startDate.getDate() - 90); break;
        case '1y': startDate.setFullYear(startDate.getFullYear() - 1); break;
      }

      const fromDate = startDate.toISOString().split('T')[0];
      const toDate = endDate.toISOString().split('T')[0];

      const [dailyStats, geographicStats, deviceStats, browserStats, routePerf] = await Promise.all([
        apiService.clickstream.getDailyStats({ fromDate, toDate }),
        apiService.clickstream.getGeographicStats({ fromDate, toDate }),
        apiService.clickstream.getDeviceStats({ fromDate, toDate }),
        apiService.clickstream.getBrowserStats({ fromDate, toDate }),
        apiService.clickstream.getRoutePerformance({ fromDate, toDate, limit: 10 }),
      ]);

      const totals = dailyStats.reduce((acc, stat) => ({
        totalClicks: acc.totalClicks + stat.total_clicks,
        uniqueClicks: acc.uniqueClicks + stat.unique_clicks,
      }), { totalClicks: 0, uniqueClicks: 0 });

      const analyticsData: ClickAnalytics = {
        total_clicks: totals.totalClicks,
        unique_clicks: totals.uniqueClicks,
        clicks_by_date: dailyStats.map(stat => ({
          date: new Date(stat.date).toLocaleDateString('en-US', { month: 'short', day: 'numeric' }),
          clicks: stat.total_clicks,
        })),
        clicks_by_device: deviceStats.slice(0, 8).map(stat => ({
          device: `${stat.device_family} (${stat.os_family})`,
          clicks: stat.total_clicks,
        })),
        clicks_by_country: geographicStats.slice(0, 10).map(stat => ({
          country: stat.country,
          clicks: stat.total_clicks,
        })),
        clicks_by_browser: browserStats.slice(0, 8).map(stat => ({
          browser: stat.user_agent_family,
          clicks: stat.total_clicks,
        })),
      };

      setAnalytics(analyticsData);
      setRoutePerformance(routePerf);
    } catch (err: any) {
      console.error('Failed to fetch analytics:', err);
      const errorMessage = err.response?.data?.message || err.message || 'Failed to load analytics data. Please try again.';
      setError(errorMessage);
    } finally {
      setLoading(false);
    }
  }, [dateRange]);

  useEffect(() => {
    fetchAnalytics();
  }, [fetchAnalytics]);

  const exportData = () => {
    alert('Export functionality would be implemented here');
  };

  if (loading && !analytics) {
    return <LoadingSpinner message="Loading analytics..." />;
  }

  if (error && !analytics) {
    return (
      <div className="container" style={{ paddingTop: '2rem' }}>
        <style>{analyticsStyles}</style>
        <div className="an-error">
          <h3 className="an-error-title">Error Loading Analytics</h3>
          <p className="an-error-message">{error}</p>
          <button className="btn btn-primary" onClick={fetchAnalytics}>
            <RefreshCw size={16} />
            Retry
          </button>
        </div>
      </div>
    );
  }

  const uniqueRate = analytics && analytics.total_clicks > 0
    ? ((analytics.unique_clicks / analytics.total_clicks) * 100).toFixed(1)
    : '0';

  const dateRanges = [
    { value: '7d', label: '7D' },
    { value: '30d', label: '30D' },
    { value: '90d', label: '90D' },
    { value: '1y', label: '1Y' },
  ];

  return (
    <>
      <style>{analyticsStyles}</style>
      <div className="container" style={{ paddingTop: '1.5rem', paddingBottom: '2rem' }}>
        {/* Refreshing indicator */}
        {loading && analytics && (
          <div className="an-refreshing">
            <RefreshCw size={14} className="an-icon-spin" />
            Updating...
          </div>
        )}

        {/* Page Header */}
        <div className="an-page-header">
          <div className="an-page-title-section">
            <h1 className="an-page-title">Analytics</h1>
            <p className="an-page-subtitle">In-depth analysis of your link performance</p>
          </div>
          <div className="an-page-actions">
            <button className="btn btn-outline btn-sm" onClick={exportData}>
              <Download size={14} />
              Export
            </button>
            <button
              className="btn btn-outline btn-sm"
              onClick={fetchAnalytics}
              disabled={loading}
            >
              <RefreshCw size={14} className={loading ? 'an-icon-spin' : ''} />
              Refresh
            </button>
          </div>
        </div>

        {/* Toolbar */}
        <div className="an-toolbar">
          <div className="an-toolbar-section">
            <div className="an-range-group">
              {dateRanges.map((r) => (
                <button
                  key={r.value}
                  className={`an-range-btn ${dateRange === r.value ? 'active' : ''}`}
                  onClick={() => setDateRange(r.value)}
                >
                  {r.label}
                </button>
              ))}
            </div>
          </div>
        </div>

        {/* Stats Grid */}
        <div className="an-stats-grid">
          <div className="an-stat-card">
            <div className="an-stat-icon primary">
              <MousePointerClick />
            </div>
            <div className="an-stat-content">
              <div className="an-stat-value">{analytics?.total_clicks.toLocaleString() || 0}</div>
              <div className="an-stat-label">Total Clicks</div>
            </div>
          </div>

          <div className="an-stat-card">
            <div className="an-stat-icon success">
              <Users />
            </div>
            <div className="an-stat-content">
              <div className="an-stat-value">{analytics?.unique_clicks.toLocaleString() || 0}</div>
              <div className="an-stat-label">Unique Clicks</div>
            </div>
          </div>

          <div className="an-stat-card">
            <div className="an-stat-icon warning">
              <TrendingUp />
            </div>
            <div className="an-stat-content">
              <div className="an-stat-value">{uniqueRate}%</div>
              <div className="an-stat-label">Unique Rate</div>
            </div>
          </div>

          <div className="an-stat-card">
            <div className="an-stat-icon info">
              <Globe />
            </div>
            <div className="an-stat-content">
              <div className="an-stat-value">{analytics?.clicks_by_country?.length || 0}</div>
              <div className="an-stat-label">Countries</div>
            </div>
          </div>
        </div>

        {/* Clicks Over Time - Full Width */}
        <div className="an-section">
          <div className="an-chart-card">
            <div className="an-chart-header">
              <h3 className="an-chart-title">
                <BarChart3 size={16} />
                Clicks Over Time
              </h3>
              <p className="an-chart-desc">Daily click trends for the selected period</p>
            </div>
            <div className="an-chart-body" style={{ height: '300px' }}>
              <ResponsiveContainer width="100%" height="100%">
                <AreaChart data={analytics?.clicks_by_date || []}>
                  <defs>
                    <linearGradient id="analyticsClicksGradient" x1="0" y1="0" x2="0" y2="1">
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
                    fill="url(#analyticsClicksGradient)"
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
        <div className="an-section-row two-cols">
          <div className="an-chart-card">
            <div className="an-chart-header">
              <h3 className="an-chart-title">
                <Globe size={16} />
                Geographic Distribution
              </h3>
              <p className="an-chart-desc">Hover to see details</p>
            </div>
            <div className="an-chart-body" style={{ height: '300px' }}>
              <WorldMap
                data={(analytics?.clicks_by_country || []).map(country => {
                  const totalClicks = analytics?.total_clicks || 1;
                  return {
                    name: country.country,
                    clicks: country.clicks,
                    percentage: parseFloat(((country.clicks / totalClicks) * 100).toFixed(2))
                  };
                })}
                height={300}
              />
            </div>
          </div>

          <div className="an-chart-card">
            <div className="an-chart-header">
              <h3 className="an-chart-title">
                <Globe size={16} />
                Top Countries
              </h3>
              <p className="an-chart-desc">By click volume</p>
            </div>
            <div className="an-chart-body">
              {(analytics?.clicks_by_country || []).length > 0 ? (
                <div className="an-country-list">
                  {(analytics?.clicks_by_country || []).map((country: any, index: number) => {
                    const totalClicks = analytics?.total_clicks || 1;
                    const percentage = Math.round((country.clicks / totalClicks) * 100);
                    return (
                      <div key={country.country} className="an-country-row">
                        <span className="an-country-rank">{index + 1}</span>
                        <span className="an-country-name">{country.country}</span>
                        <div className="an-country-bar-wrap">
                          <div className="an-country-bar">
                            <div
                              className="an-country-bar-fill"
                              style={{ width: `${percentage}%` }}
                            />
                          </div>
                        </div>
                        <span className="an-country-value">{country.clicks.toLocaleString()}</span>
                        <span className="an-country-pct">{percentage}%</span>
                      </div>
                    );
                  })}
                </div>
              ) : (
                <div className="an-empty">
                  <div className="an-empty-icon">
                    <Globe />
                  </div>
                  <p>No geographic data available</p>
                </div>
              )}
            </div>
          </div>
        </div>

        {/* Distribution Section */}
        <div className="an-section-row two-cols">
          {/* Device Distribution */}
          <div className="an-chart-card">
            <div className="an-chart-header">
              <h3 className="an-chart-title">
                <Monitor size={16} />
                Devices
              </h3>
              <p className="an-chart-desc">By device type</p>
            </div>
            <div className="an-chart-body" style={{ height: '250px' }}>
              {(analytics?.clicks_by_device || []).length > 0 ? (
                <ResponsiveContainer width="100%" height="100%">
                  <PieChart>
                    <Pie
                      data={analytics?.clicks_by_device || []}
                      cx="50%"
                      cy="50%"
                      outerRadius={80}
                      innerRadius={50}
                      dataKey="clicks"
                      nameKey="device"
                      paddingAngle={2}
                      stroke="var(--bg-primary)"
                      strokeWidth={2}
                    >
                      {(analytics?.clicks_by_device || []).map((_, index) => (
                        <Cell key={`cell-${index}`} fill={CHART_COLORS[index % CHART_COLORS.length]} />
                      ))}
                    </Pie>
                    <Tooltip content={<CustomTooltip />} />
                  </PieChart>
                </ResponsiveContainer>
              ) : (
                <div className="an-empty">
                  <div className="an-empty-icon">
                    <Monitor />
                  </div>
                  <p>No device data available</p>
                </div>
              )}
            </div>
            {(analytics?.clicks_by_device || []).length > 0 && (
              <PieLegend
                items={(analytics?.clicks_by_device || []).map((entry, i) => ({
                  name: entry.device,
                  color: CHART_COLORS[i % CHART_COLORS.length],
                }))}
              />
            )}
          </div>

          {/* Browser Distribution */}
          <div className="an-chart-card">
            <div className="an-chart-header">
              <h3 className="an-chart-title">
                <Activity size={16} />
                Browsers
              </h3>
              <p className="an-chart-desc">By browser</p>
            </div>
            <div className="an-chart-body" style={{ height: '250px' }}>
              {(analytics?.clicks_by_browser || []).length > 0 ? (
                <ResponsiveContainer width="100%" height="100%">
                  <PieChart>
                    <Pie
                      data={analytics?.clicks_by_browser || []}
                      cx="50%"
                      cy="50%"
                      outerRadius={80}
                      innerRadius={50}
                      dataKey="clicks"
                      nameKey="browser"
                      paddingAngle={2}
                      stroke="var(--bg-primary)"
                      strokeWidth={2}
                    >
                      {(analytics?.clicks_by_browser || []).map((_, index) => (
                        <Cell key={`cell-${index}`} fill={CHART_COLORS[index % CHART_COLORS.length]} />
                      ))}
                    </Pie>
                    <Tooltip content={<CustomTooltip />} />
                  </PieChart>
                </ResponsiveContainer>
              ) : (
                <div className="an-empty">
                  <div className="an-empty-icon">
                    <Activity />
                  </div>
                  <p>No browser data available</p>
                </div>
              )}
            </div>
            {(analytics?.clicks_by_browser || []).length > 0 && (
              <PieLegend
                items={(analytics?.clicks_by_browser || []).map((entry, i) => ({
                  name: entry.browser,
                  color: CHART_COLORS[i % CHART_COLORS.length],
                }))}
              />
            )}
          </div>
        </div>

        {/* Route Performance Table */}
        {routePerformance.length > 0 && (
          <div className="an-section">
            <div className="an-chart-card">
              <div className="an-chart-header">
                <h3 className="an-chart-title">
                  <Target size={16} />
                  Top Performing Routes
                </h3>
                <p className="an-chart-desc">Routes ranked by total clicks</p>
              </div>
              <div className="an-table-container">
                <table className="an-table">
                  <thead>
                    <tr>
                      <th>Route</th>
                      <th>Clicks</th>
                      <th>Unique</th>
                      <th>Human</th>
                      <th>Bot</th>
                      <th>Countries</th>
                      <th>Devices</th>
                    </tr>
                  </thead>
                  <tbody>
                    {routePerformance.slice(0, 10).map((route, index) => (
                      <tr key={index}>
                        <td className="an-table-route">
                          {route.route_id.substring(0, 8)}...
                        </td>
                        <td className="an-table-value">{route.total_clicks.toLocaleString()}</td>
                        <td>{route.unique_visitors.toLocaleString()}</td>
                        <td className="an-table-success">{route.human_clicks.toLocaleString()}</td>
                        <td className="an-table-error">{route.bot_clicks.toLocaleString()}</td>
                        <td>{route.countries_reached}</td>
                        <td>{route.device_types}</td>
                      </tr>
                    ))}
                  </tbody>
                </table>
              </div>
            </div>
          </div>
        )}
      </div>
    </>
  );
};

export default Analytics;
