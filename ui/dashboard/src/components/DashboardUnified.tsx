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
  Activity,
  Bot,
  Target,
  RefreshCw,
  BarChart3,
  Monitor,
  Smartphone,
} from 'lucide-react';
import { apiService, ClickAnalytics, DailyStatsDto, GeographicStatsDto, DeviceStatsDto, BrowserStatsDto, TrafficTypeStatsDto, RoutePerformanceDto } from '../services/api';
import { getCountryDisplayName } from '../utils/countries';
import LoadingSpinner from './LoadingSpinner';
import WorldMap from './WorldMap';
import './DesignSystem.css';

const dashboardStyles = `
/* ===== DASHBOARD PAGE STYLES ===== */

/* Page Header */
.db-page-header {
  display: flex;
  align-items: flex-start;
  justify-content: space-between;
  gap: 1.5rem;
  margin-bottom: 2rem;
  flex-wrap: wrap;
}

.db-page-title-section {
  flex: 1;
  min-width: 200px;
}

.db-page-title {
  font-size: 1.5rem;
  font-weight: 700;
  color: var(--text-primary);
  margin: 0 0 0.25rem 0;
  letter-spacing: -0.025em;
}

.db-page-subtitle {
  font-size: 0.875rem;
  color: var(--text-muted);
  margin: 0;
}

.db-page-actions {
  display: flex;
  align-items: center;
  gap: 0.75rem;
  flex-wrap: wrap;
}

/* Stats Grid */
.db-stats-grid {
  display: grid;
  grid-template-columns: repeat(6, 1fr);
  gap: 1rem;
  margin-bottom: 1.5rem;
}

@media (max-width: 1400px) {
  .db-stats-grid {
    grid-template-columns: repeat(3, 1fr);
  }
}

@media (max-width: 900px) {
  .db-stats-grid {
    grid-template-columns: repeat(2, 1fr);
  }
}

@media (max-width: 600px) {
  .db-stats-grid {
    grid-template-columns: 1fr;
  }
}

.db-stat-card {
  background: var(--bg-elevated);
  border: 1px solid var(--border-primary);
  border-radius: var(--radius-xl);
  padding: 1.25rem;
  display: flex;
  align-items: center;
  gap: 1rem;
  transition: all var(--transition-normal);
}

.db-stat-card:hover {
  box-shadow: var(--shadow-md);
  border-color: var(--border-secondary);
}

.db-stat-icon {
  width: 44px;
  height: 44px;
  border-radius: var(--radius-lg);
  display: flex;
  align-items: center;
  justify-content: center;
  flex-shrink: 0;
}

.db-stat-icon svg {
  width: 22px;
  height: 22px;
  color: #fff;
}

.db-stat-icon.primary { background: var(--color-primary); }
.db-stat-icon.success { background: var(--color-success); }
.db-stat-icon.warning { background: var(--color-warning); }
.db-stat-icon.error { background: var(--color-error); }
.db-stat-icon.info { background: #8b5cf6; }

.db-stat-content {
  flex: 1;
  min-width: 0;
}

.db-stat-value {
  font-size: 1.5rem;
  font-weight: 700;
  color: var(--text-primary);
  line-height: 1.2;
  margin-bottom: 0.125rem;
}

.db-stat-label {
  font-size: 0.75rem;
  color: var(--text-secondary);
  font-weight: 500;
  text-transform: uppercase;
  letter-spacing: 0.03em;
}

/* Toolbar */
.db-toolbar {
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

.db-toolbar-section {
  display: flex;
  align-items: center;
  gap: 0.75rem;
  flex-wrap: wrap;
}

/* Date Range Buttons */
.db-range-group {
  display: inline-flex;
  border: 1px solid var(--border-secondary);
  border-radius: var(--radius-lg);
  overflow: hidden;
  background: var(--bg-primary);
}

.db-range-btn {
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

.db-range-btn:not(:last-child) {
  border-right: 1px solid var(--border-secondary);
}

.db-range-btn:hover:not(.active) {
  background: var(--bg-tertiary);
  color: var(--text-primary);
}

.db-range-btn.active {
  background: var(--color-primary);
  color: #ffffff;
}

/* Refreshing indicator */
.db-refreshing {
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
.db-section {
  margin-bottom: 1.5rem;
}

.db-section-row {
  display: grid;
  gap: 1.5rem;
  margin-bottom: 1.5rem;
}

.db-section-row.two-cols {
  grid-template-columns: repeat(2, 1fr);
}

.db-section-row.three-cols {
  grid-template-columns: repeat(3, 1fr);
}

@media (max-width: 1024px) {
  .db-section-row.two-cols,
  .db-section-row.three-cols {
    grid-template-columns: 1fr;
  }
}

/* Chart Card */
.db-chart-card {
  background: var(--bg-elevated);
  border: 1px solid var(--border-primary);
  border-radius: var(--radius-xl);
  overflow: hidden;
}

.db-chart-header {
  padding: 1rem 1.25rem;
  border-bottom: 1px solid var(--border-primary);
  background: var(--bg-secondary);
}

.db-chart-title {
  font-size: 0.9375rem;
  font-weight: 600;
  color: var(--text-primary);
  margin: 0 0 0.25rem 0;
  display: flex;
  align-items: center;
  gap: 0.5rem;
}

.db-chart-title svg {
  color: var(--color-primary);
}

.db-chart-desc {
  font-size: 0.75rem;
  color: var(--text-muted);
  margin: 0;
}

.db-chart-body {
  padding: 1.25rem;
}

/* Tooltip */
.db-tooltip {
  background: var(--bg-elevated);
  border: 1px solid var(--border-primary);
  border-radius: var(--radius-md);
  padding: 0.75rem;
  box-shadow: var(--shadow-lg);
}

.db-tooltip-label {
  font-size: 0.75rem;
  font-weight: 600;
  color: var(--text-primary);
  margin-bottom: 0.5rem;
  padding-bottom: 0.5rem;
  border-bottom: 1px solid var(--border-primary);
}

.db-tooltip-row {
  display: flex;
  align-items: center;
  gap: 0.5rem;
  padding: 0.25rem 0;
  font-size: 0.8125rem;
}

.db-tooltip-dot {
  width: 8px;
  height: 8px;
  border-radius: 50%;
  flex-shrink: 0;
}

.db-tooltip-name {
  color: var(--text-secondary);
  flex: 1;
}

.db-tooltip-value {
  font-weight: 600;
  color: var(--text-primary);
}

/* Pie Legend */
.db-pie-legend {
  display: flex;
  flex-wrap: wrap;
  gap: 0.75rem;
  justify-content: center;
  padding: 0.75rem 1rem;
  border-top: 1px solid var(--border-primary);
  background: var(--bg-secondary);
}

.db-pie-legend-item {
  display: flex;
  align-items: center;
  gap: 0.375rem;
  font-size: 0.75rem;
}

.db-pie-legend-dot {
  width: 8px;
  height: 8px;
  border-radius: 50%;
  flex-shrink: 0;
}

.db-pie-legend-label {
  color: var(--text-secondary);
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
  max-width: 100px;
}

/* Country List */
.db-country-list {
  display: flex;
  flex-direction: column;
  gap: 0.5rem;
}

.db-country-row {
  display: flex;
  align-items: center;
  gap: 0.75rem;
  padding: 0.5rem 0;
}

.db-country-rank {
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

.db-country-name {
  flex: 1;
  font-size: 0.8125rem;
  font-weight: 500;
  color: var(--text-primary);
  min-width: 0;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.db-country-bar-wrap {
  flex: 1;
  max-width: 120px;
}

.db-country-bar {
  height: 6px;
  background: var(--bg-tertiary);
  border-radius: 3px;
  overflow: hidden;
}

.db-country-bar-fill {
  height: 100%;
  background: var(--color-primary);
  border-radius: 3px;
  transition: width var(--transition-normal);
}

.db-country-value {
  font-size: 0.8125rem;
  font-weight: 600;
  color: var(--text-primary);
  min-width: 50px;
  text-align: right;
}

.db-country-pct {
  font-size: 0.75rem;
  color: var(--text-muted);
  min-width: 40px;
  text-align: right;
}

/* Table */
.db-table-container {
  overflow-x: auto;
}

.db-table {
  width: 100%;
  border-collapse: collapse;
  font-size: 0.8125rem;
}

.db-table th {
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

.db-table td {
  padding: 0.75rem 1rem;
  border-bottom: 1px solid var(--border-primary);
  vertical-align: middle;
}

.db-table tbody tr {
  transition: background var(--transition-fast);
}

.db-table tbody tr:hover {
  background: var(--bg-secondary);
}

.db-table tbody tr:last-child td {
  border-bottom: none;
}

.db-table-route {
  font-family: var(--font-family-mono);
  font-size: 0.75rem;
  color: var(--color-primary);
}

.db-table-value {
  font-weight: 600;
  color: var(--text-primary);
}

.db-table-success {
  color: var(--color-success);
}

.db-table-error {
  color: var(--color-error);
}

/* Empty State */
.db-empty {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  padding: 3rem 2rem;
  text-align: center;
}

.db-empty-icon {
  width: 56px;
  height: 56px;
  border-radius: 50%;
  background: var(--bg-tertiary);
  display: flex;
  align-items: center;
  justify-content: center;
  margin-bottom: 1rem;
}

.db-empty-icon svg {
  width: 28px;
  height: 28px;
  color: var(--text-muted);
}

.db-empty p {
  font-size: 0.875rem;
  color: var(--text-muted);
  margin: 0;
}

/* Error State */
.db-error {
  background: var(--bg-elevated);
  border: 1px solid var(--color-error);
  border-radius: var(--radius-xl);
  padding: 2rem;
  text-align: center;
  margin: 2rem 0;
}

.db-error-title {
  font-size: 1.125rem;
  font-weight: 600;
  color: var(--color-error);
  margin: 0 0 0.5rem 0;
}

.db-error-message {
  font-size: 0.875rem;
  color: var(--text-secondary);
  margin: 0 0 1rem 0;
}

/* Responsive */
@media (max-width: 768px) {
  .db-page-header {
    flex-direction: column;
    gap: 1rem;
  }

  .db-page-actions {
    width: 100%;
    justify-content: space-between;
  }

  .db-toolbar {
    flex-direction: column;
    align-items: stretch;
  }

  .db-toolbar-section {
    width: 100%;
    justify-content: center;
  }
}

/* Spin animation for refresh icon */
@keyframes db-spin {
  from { transform: rotate(0deg); }
  to { transform: rotate(360deg); }
}

.db-icon-spin {
  animation: db-spin 1s linear infinite;
}
`;

interface DashboardStats {
  totalClicks: number;
  uniqueClicks: number;
  botClicks: number;
  humanClicks: number;
  totalRoutes: number;
  activeRoutes: number;
}

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
    <div className="db-tooltip">
      {label != null && label !== '' && (
        <div className="db-tooltip-label">{label}</div>
      )}
      {payload.map((entry: any, i: number) => {
        const percent = entry.payload?.percent;
        return (
          <div key={i} className="db-tooltip-row">
            <span className="db-tooltip-dot" style={{ backgroundColor: entry.color || entry.fill }} />
            <span className="db-tooltip-name">{entry.name}</span>
            <span className="db-tooltip-value">
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
  <div className="db-pie-legend">
    {items.slice(0, 6).map((item, i) => (
      <div key={i} className="db-pie-legend-item">
        <span className="db-pie-legend-dot" style={{ backgroundColor: item.color }} />
        <span className="db-pie-legend-label">{item.name}</span>
      </div>
    ))}
  </div>
);

const Dashboard: React.FC = () => {
  const [analytics, setAnalytics] = useState<ClickAnalytics | null>(null);
  const [stats, setStats] = useState<DashboardStats | null>(null);
  const [trafficTypeStats, setTrafficTypeStats] = useState<TrafficTypeStatsDto[]>([]);
  const [routePerformance, setRoutePerformance] = useState<RoutePerformanceDto[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const [dateRange, setDateRange] = useState('7d');

  const fetchDashboardData = useCallback(async () => {
    try {
      setLoading(true);
      setError(null);

      const endDate = new Date();
      const startDate = new Date();

      switch (dateRange) {
        case '24h': startDate.setHours(startDate.getHours() - 24); break;
        case '7d': startDate.setDate(startDate.getDate() - 7); break;
        case '30d': startDate.setDate(startDate.getDate() - 30); break;
        case '90d': startDate.setDate(startDate.getDate() - 90); break;
      }

      const fromDate = startDate.toISOString().split('T')[0];
      const toDate = endDate.toISOString().split('T')[0];
      const fromHour = startDate.toISOString().replace('T', ' ').substring(0, 13);
      const toHour = endDate.toISOString().replace('T', ' ').substring(0, 13);

      const [dailyStats, geographicStats, deviceStats, browserStats, trafficStats, routePerf, routesResponse] = await Promise.all([
        apiService.clickstream.getDailyStats({ fromDate, toDate }),
        apiService.clickstream.getGeographicStats({ fromDate, toDate }),
        apiService.clickstream.getDeviceStats({ fromDate, toDate }),
        apiService.clickstream.getBrowserStats({ fromDate, toDate }),
        apiService.clickstream.getTrafficTypeStats({ fromHour, toHour }),
        apiService.clickstream.getRoutePerformance({ fromDate, toDate, limit: 10 }),
        apiService.routes.list({ page: 1, pageSize: 1000 }),
      ]);

      const totals = dailyStats.reduce((acc, stat) => ({
        totalClicks: acc.totalClicks + stat.total_clicks,
        uniqueClicks: acc.uniqueClicks + stat.unique_clicks,
        botClicks: acc.botClicks + stat.bot_clicks,
        humanClicks: acc.humanClicks + stat.human_clicks,
      }), { totalClicks: 0, uniqueClicks: 0, botClicks: 0, humanClicks: 0 });

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
          country: getCountryDisplayName(stat.country) || stat.country,
          clicks: stat.total_clicks,
        })),
        clicks_by_browser: browserStats.slice(0, 8).map(stat => ({
          browser: stat.user_agent_family,
          clicks: stat.total_clicks,
        })),
      };

      setAnalytics(analyticsData);
      setTrafficTypeStats(trafficStats);
      setRoutePerformance(routePerf);

      const activeRoutes = routesResponse.data.filter((route: any) => route.status === 'Active').length;

      setStats({
        totalClicks: totals.totalClicks,
        uniqueClicks: totals.uniqueClicks,
        botClicks: totals.botClicks,
        humanClicks: totals.humanClicks,
        totalRoutes: routesResponse.pagination.totalCount,
        activeRoutes,
      });
    } catch (err: any) {
      console.error('Failed to fetch dashboard data:', err);
      const errorMessage = err.response?.data?.message || err.message || 'Failed to load dashboard data. Please try again.';
      setError(errorMessage);
    } finally {
      setLoading(false);
    }
  }, [dateRange]);

  useEffect(() => {
    fetchDashboardData();
  }, [fetchDashboardData]);

  useEffect(() => {
    const isInputFocused = () => {
      const el = document.activeElement as HTMLElement | null;
      if (!el || el === document.body) return false;
      const tag = el.tagName;
      return tag === 'INPUT' || tag === 'TEXTAREA' || tag === 'SELECT' || el.isContentEditable;
    };
    const onKey = (e: KeyboardEvent) => {
      if (e.altKey && e.key.toLowerCase() === 'r' && !isInputFocused()) {
        fetchDashboardData();
        e.preventDefault();
      }
    };
    document.addEventListener('keydown', onKey);
    return () => document.removeEventListener('keydown', onKey);
  }, [fetchDashboardData]);

  if (loading && !analytics) {
    return <LoadingSpinner message="Loading dashboard..." />;
  }

  if (error && !analytics) {
    return (
      <div className="container" style={{ paddingTop: '2rem' }}>
        <style>{dashboardStyles}</style>
        <div className="db-error">
          <h3 className="db-error-title">Error Loading Dashboard</h3>
          <p className="db-error-message">{error}</p>
          <button className="btn btn-primary" onClick={fetchDashboardData}>
            <RefreshCw size={16} />
            Retry
          </button>
        </div>
      </div>
    );
  }

  const humanRate = stats && stats.totalClicks > 0
    ? ((stats.humanClicks / stats.totalClicks) * 100).toFixed(0)
    : '0';
  const botRate = stats && stats.totalClicks > 0
    ? ((stats.botClicks / stats.totalClicks) * 100).toFixed(0)
    : '0';
  const uniqueRate = stats && stats.totalClicks > 0
    ? ((stats.uniqueClicks / stats.totalClicks) * 100).toFixed(1)
    : '0';

  const trafficData = (() => {
    const humanStat = trafficTypeStats.find(stat => !stat.is_bot);
    const botStat = trafficTypeStats.find(stat => stat.is_bot);
    return [
      { name: 'Human', value: humanStat?.total_clicks || 0 },
      { name: 'Bot', value: botStat?.total_clicks || 0 },
    ];
  })();

  const dateRanges = [
    { value: '24h', label: '24H' },
    { value: '7d', label: '7D' },
    { value: '30d', label: '30D' },
    { value: '90d', label: '90D' },
  ];

  return (
    <>
      <style>{dashboardStyles}</style>
      <div className="container" style={{ paddingTop: '1.5rem', paddingBottom: '2rem' }}>
        {/* Refreshing indicator */}
        {loading && analytics && (
          <div className="db-refreshing">
            <RefreshCw size={14} className="db-icon-spin" />
            Updating...
          </div>
        )}

        {/* Page Header */}
        <div className="db-page-header">
          <div className="db-page-title-section">
            <h1 className="db-page-title">Dashboard</h1>
            <p className="db-page-subtitle">Overview of your link performance and analytics</p>
          </div>
          <div className="db-page-actions">
            <button
              className="btn btn-outline btn-sm"
              onClick={fetchDashboardData}
              disabled={loading}
            >
              <RefreshCw size={14} className={loading ? 'db-icon-spin' : ''} />
              Refresh
            </button>
          </div>
        </div>

        {/* Toolbar */}
        <div className="db-toolbar">
          <div className="db-toolbar-section">
            <div className="db-range-group">
              {dateRanges.map((r) => (
                <button
                  key={r.value}
                  className={`db-range-btn ${dateRange === r.value ? 'active' : ''}`}
                  onClick={() => setDateRange(r.value)}
                >
                  {r.label}
                </button>
              ))}
            </div>
          </div>
        </div>

        {/* Stats Grid */}
        <div className="db-stats-grid">
          <div className="db-stat-card">
            <div className="db-stat-icon primary">
              <MousePointerClick />
            </div>
            <div className="db-stat-content">
              <div className="db-stat-value">{stats?.totalClicks.toLocaleString() || 0}</div>
              <div className="db-stat-label">Total Clicks</div>
            </div>
          </div>

          <div className="db-stat-card">
            <div className="db-stat-icon success">
              <Users />
            </div>
            <div className="db-stat-content">
              <div className="db-stat-value">{stats?.uniqueClicks.toLocaleString() || 0}</div>
              <div className="db-stat-label">Unique Clicks</div>
            </div>
          </div>

          <div className="db-stat-card">
            <div className="db-stat-icon info">
              <Activity />
            </div>
            <div className="db-stat-content">
              <div className="db-stat-value">{stats?.humanClicks.toLocaleString() || 0}</div>
              <div className="db-stat-label">Human ({humanRate}%)</div>
            </div>
          </div>

          <div className="db-stat-card">
            <div className="db-stat-icon error">
              <Bot />
            </div>
            <div className="db-stat-content">
              <div className="db-stat-value">{stats?.botClicks.toLocaleString() || 0}</div>
              <div className="db-stat-label">Bot ({botRate}%)</div>
            </div>
          </div>

          <div className="db-stat-card">
            <div className="db-stat-icon warning">
              <TrendingUp />
            </div>
            <div className="db-stat-content">
              <div className="db-stat-value">{uniqueRate}%</div>
              <div className="db-stat-label">Unique Rate</div>
            </div>
          </div>

          <div className="db-stat-card">
            <div className="db-stat-icon primary">
              <Globe />
            </div>
            <div className="db-stat-content">
              <div className="db-stat-value">{stats?.activeRoutes || 0}</div>
              <div className="db-stat-label">Active Routes</div>
            </div>
          </div>
        </div>

        {/* Clicks Over Time - Full Width */}
        <div className="db-section">
          <div className="db-chart-card">
            <div className="db-chart-header">
              <h3 className="db-chart-title">
                <BarChart3 size={16} />
                Clicks Over Time
              </h3>
              <p className="db-chart-desc">Daily click trends for the selected period</p>
            </div>
            <div className="db-chart-body" style={{ height: '300px' }}>
              <ResponsiveContainer width="100%" height="100%">
                <AreaChart data={analytics?.clicks_by_date || []}>
                  <defs>
                    <linearGradient id="clicksGradient" x1="0" y1="0" x2="0" y2="1">
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
                    fill="url(#clicksGradient)"
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
        <div className="db-section-row two-cols">
          <div className="db-chart-card">
            <div className="db-chart-header">
              <h3 className="db-chart-title">
                <Globe size={16} />
                Geographic Distribution
              </h3>
              <p className="db-chart-desc">Hover to see details</p>
            </div>
            <div className="db-chart-body" style={{ height: '300px' }}>
              <WorldMap
                data={(analytics?.clicks_by_country || []).map(country => {
                  const totalClicks = stats?.totalClicks || 1;
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

          <div className="db-chart-card">
            <div className="db-chart-header">
              <h3 className="db-chart-title">
                <Globe size={16} />
                Top Countries
              </h3>
              <p className="db-chart-desc">By click volume</p>
            </div>
            <div className="db-chart-body">
              {(analytics?.clicks_by_country || []).length > 0 ? (
                <div className="db-country-list">
                  {(analytics?.clicks_by_country || []).map((country: any, index: number) => {
                    const totalClicks = stats?.totalClicks || 1;
                    const percentage = Math.round((country.clicks / totalClicks) * 100);
                    return (
                      <div key={country.country} className="db-country-row">
                        <span className="db-country-rank">{index + 1}</span>
                        <span className="db-country-name">{country.country}</span>
                        <div className="db-country-bar-wrap">
                          <div className="db-country-bar">
                            <div
                              className="db-country-bar-fill"
                              style={{ width: `${percentage}%` }}
                            />
                          </div>
                        </div>
                        <span className="db-country-value">{country.clicks.toLocaleString()}</span>
                        <span className="db-country-pct">{percentage}%</span>
                      </div>
                    );
                  })}
                </div>
              ) : (
                <div className="db-empty">
                  <div className="db-empty-icon">
                    <Globe />
                  </div>
                  <p>No geographic data available</p>
                </div>
              )}
            </div>
          </div>
        </div>

        {/* Distribution Section */}
        <div className="db-section-row three-cols">
          {/* Device Distribution */}
          <div className="db-chart-card">
            <div className="db-chart-header">
              <h3 className="db-chart-title">
                <Monitor size={16} />
                Devices
              </h3>
              <p className="db-chart-desc">By device type</p>
            </div>
            <div className="db-chart-body" style={{ height: '200px' }}>
              {(analytics?.clicks_by_device || []).length > 0 ? (
                <>
                  <ResponsiveContainer width="100%" height="100%">
                    <PieChart>
                      <Pie
                        data={analytics?.clicks_by_device || []}
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
                        {(analytics?.clicks_by_device || []).map((_, index) => (
                          <Cell key={`cell-${index}`} fill={CHART_COLORS[index % CHART_COLORS.length]} />
                        ))}
                      </Pie>
                      <Tooltip content={<CustomTooltip />} />
                    </PieChart>
                  </ResponsiveContainer>
                  <PieLegend
                    items={(analytics?.clicks_by_device || []).map((entry, i) => ({
                      name: entry.device,
                      color: CHART_COLORS[i % CHART_COLORS.length],
                    }))}
                  />
                </>
              ) : (
                <div className="db-empty">
                  <div className="db-empty-icon">
                    <Monitor />
                  </div>
                  <p>No device data</p>
                </div>
              )}
            </div>
          </div>

          {/* Browser Distribution */}
          <div className="db-chart-card">
            <div className="db-chart-header">
              <h3 className="db-chart-title">
                <Globe size={16} />
                Browsers
              </h3>
              <p className="db-chart-desc">By browser</p>
            </div>
            <div className="db-chart-body" style={{ height: '200px' }}>
              {(analytics?.clicks_by_browser || []).length > 0 ? (
                <>
                  <ResponsiveContainer width="100%" height="100%">
                    <PieChart>
                      <Pie
                        data={analytics?.clicks_by_browser || []}
                        cx="50%"
                        cy="50%"
                        outerRadius={70}
                        innerRadius={45}
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
                  <PieLegend
                    items={(analytics?.clicks_by_browser || []).map((entry, i) => ({
                      name: entry.browser,
                      color: CHART_COLORS[i % CHART_COLORS.length],
                    }))}
                  />
                </>
              ) : (
                <div className="db-empty">
                  <div className="db-empty-icon">
                    <Globe />
                  </div>
                  <p>No browser data</p>
                </div>
              )}
            </div>
          </div>

          {/* Traffic Type */}
          <div className="db-chart-card">
            <div className="db-chart-header">
              <h3 className="db-chart-title">
                <Activity size={16} />
                Traffic Type
              </h3>
              <p className="db-chart-desc">Bot vs Human</p>
            </div>
            <div className="db-chart-body" style={{ height: '200px' }}>
              {trafficTypeStats.length > 0 ? (
                <ResponsiveContainer width="100%" height="100%">
                  <PieChart>
                    <Pie
                      data={trafficData}
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
                <div className="db-empty">
                  <div className="db-empty-icon">
                    <Activity />
                  </div>
                  <p>No data available</p>
                </div>
              )}
            </div>
            {trafficTypeStats.length > 0 && (
              <PieLegend
                items={trafficData.map((entry, i) => ({
                  name: entry.name,
                  color: TRAFFIC_COLORS[i],
                }))}
              />
            )}
          </div>
        </div>

        {/* Top Performing Routes */}
        <div className="db-section">
          <div className="db-chart-card">
            <div className="db-chart-header">
              <h3 className="db-chart-title">
                <Target size={16} />
                Top Performing Routes
              </h3>
              <p className="db-chart-desc">Routes ranked by total clicks</p>
            </div>
            <div className="db-table-container">
              {routePerformance.length > 0 ? (
                <table className="db-table">
                  <thead>
                    <tr>
                      <th>Domain</th>
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
                    {routePerformance.slice(0, 5).map((route, index) => (
                      <tr key={index}>
                        <td>
                          <span className="db-table-route">{route.route_domain_name ?? '—'}</span>
                        </td>
                        <td>
                          <span className="db-table-route">{route.route_name ?? '—'}</span>
                        </td>
                        <td className="db-table-value">{route.total_clicks.toLocaleString()}</td>
                        <td>{route.unique_visitors.toLocaleString()}</td>
                        <td className="db-table-success">{route.human_clicks.toLocaleString()}</td>
                        <td className="db-table-error">{route.bot_clicks.toLocaleString()}</td>
                        <td>{route.countries_reached}</td>
                        <td>{route.device_types}</td>
                      </tr>
                    ))}
                  </tbody>
                </table>
              ) : (
                <div className="db-empty">
                  <div className="db-empty-icon">
                    <Target />
                  </div>
                  <p>No route performance data for the selected period</p>
                </div>
              )}
            </div>
          </div>
        </div>
      </div>
    </>
  );
};

export default Dashboard;
