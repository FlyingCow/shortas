import React, { useState, useEffect, useCallback } from 'react';
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
  Cell,
  CartesianGrid,
  AreaChart,
  Area
} from 'recharts';
import {
  TrendingUp,
  Users,
  MousePointerClick,
  Globe,
  Monitor,
  Activity,
  Target,
  Bot,
  RefreshCw,
  BarChart3,
} from 'lucide-react';
import {
  apiService,
  DailyStatsDto,
  GeographicStatsDto,
  DeviceStatsDto,
  BrowserStatsDto,
  RoutePerformanceDto,
  TrafficTypeStatsDto
} from '../services/api';
import { getCountryDisplayName } from '../utils/countries';
import LoadingSpinner from './LoadingSpinner';
import './DesignSystem.css';

const statsStyles = `
/* ===== STATS ANALYTICS PAGE STYLES ===== */

/* Page Header */
.sa-page-header {
  display: flex;
  align-items: flex-start;
  justify-content: space-between;
  gap: 1.5rem;
  margin-bottom: 2rem;
  flex-wrap: wrap;
}

.sa-page-title-section {
  flex: 1;
  min-width: 200px;
}

.sa-page-title {
  font-size: 1.5rem;
  font-weight: 700;
  color: var(--text-primary);
  margin: 0 0 0.25rem 0;
  letter-spacing: -0.025em;
}

.sa-page-subtitle {
  font-size: 0.875rem;
  color: var(--text-muted);
  margin: 0;
}

.sa-page-actions {
  display: flex;
  align-items: center;
  gap: 0.75rem;
  flex-wrap: wrap;
}

/* Stats Grid */
.sa-stats-grid {
  display: grid;
  grid-template-columns: repeat(4, 1fr);
  gap: 1rem;
  margin-bottom: 1.5rem;
}

@media (max-width: 1200px) {
  .sa-stats-grid {
    grid-template-columns: repeat(2, 1fr);
  }
}

@media (max-width: 600px) {
  .sa-stats-grid {
    grid-template-columns: 1fr;
  }
}

.sa-stat-card {
  background: var(--bg-elevated);
  border: 1px solid var(--border-primary);
  border-radius: var(--radius-xl);
  padding: 1.25rem;
  display: flex;
  align-items: center;
  gap: 1rem;
  transition: all var(--transition-normal);
}

.sa-stat-card:hover {
  box-shadow: var(--shadow-md);
  border-color: var(--border-secondary);
}

.sa-stat-icon {
  width: 48px;
  height: 48px;
  border-radius: var(--radius-lg);
  display: flex;
  align-items: center;
  justify-content: center;
  flex-shrink: 0;
}

.sa-stat-icon svg {
  width: 24px;
  height: 24px;
  color: #fff;
}

.sa-stat-icon.primary { background: var(--color-primary); }
.sa-stat-icon.success { background: var(--color-success); }
.sa-stat-icon.warning { background: var(--color-warning); }
.sa-stat-icon.info { background: #8b5cf6; }

.sa-stat-content {
  flex: 1;
  min-width: 0;
}

.sa-stat-value {
  font-size: 1.75rem;
  font-weight: 700;
  color: var(--text-primary);
  line-height: 1.2;
  margin-bottom: 0.125rem;
}

.sa-stat-label {
  font-size: 0.8125rem;
  color: var(--text-secondary);
  font-weight: 500;
  text-transform: uppercase;
  letter-spacing: 0.03em;
}

/* Toolbar */
.sa-toolbar {
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

.sa-toolbar-section {
  display: flex;
  align-items: center;
  gap: 0.75rem;
  flex-wrap: wrap;
}

/* Date Range Buttons */
.sa-range-group {
  display: inline-flex;
  border: 1px solid var(--border-secondary);
  border-radius: var(--radius-lg);
  overflow: hidden;
  background: var(--bg-primary);
}

.sa-range-btn {
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

.sa-range-btn:not(:last-child) {
  border-right: 1px solid var(--border-secondary);
}

.sa-range-btn:hover:not(.active) {
  background: var(--bg-tertiary);
  color: var(--text-primary);
}

.sa-range-btn.active {
  background: var(--color-primary);
  color: #ffffff;
}

/* Section Grid */
.sa-section {
  margin-bottom: 1.5rem;
}

.sa-section-row {
  display: grid;
  gap: 1.5rem;
  margin-bottom: 1.5rem;
}

.sa-section-row.two-cols {
  grid-template-columns: repeat(2, 1fr);
}

@media (max-width: 1024px) {
  .sa-section-row.two-cols {
    grid-template-columns: 1fr;
  }
}

/* Chart Card */
.sa-chart-card {
  background: var(--bg-elevated);
  border: 1px solid var(--border-primary);
  border-radius: var(--radius-xl);
  overflow: hidden;
}

.sa-chart-header {
  padding: 1rem 1.25rem;
  border-bottom: 1px solid var(--border-primary);
  background: var(--bg-secondary);
}

.sa-chart-title {
  font-size: 0.9375rem;
  font-weight: 600;
  color: var(--text-primary);
  margin: 0 0 0.25rem 0;
  display: flex;
  align-items: center;
  gap: 0.5rem;
}

.sa-chart-title svg {
  color: var(--color-primary);
}

.sa-chart-desc {
  font-size: 0.75rem;
  color: var(--text-muted);
  margin: 0;
}

.sa-chart-body {
  padding: 1.25rem;
}

/* Tooltip */
.sa-tooltip {
  background: var(--bg-elevated);
  border: 1px solid var(--border-primary);
  border-radius: var(--radius-md);
  padding: 0.75rem;
  box-shadow: var(--shadow-lg);
}

.sa-tooltip-label {
  font-size: 0.75rem;
  font-weight: 600;
  color: var(--text-primary);
  margin-bottom: 0.5rem;
  padding-bottom: 0.5rem;
  border-bottom: 1px solid var(--border-primary);
}

.sa-tooltip-row {
  display: flex;
  align-items: center;
  gap: 0.5rem;
  padding: 0.25rem 0;
  font-size: 0.8125rem;
}

.sa-tooltip-dot {
  width: 8px;
  height: 8px;
  border-radius: 50%;
  flex-shrink: 0;
}

.sa-tooltip-name {
  color: var(--text-secondary);
  flex: 1;
}

.sa-tooltip-value {
  font-weight: 600;
  color: var(--text-primary);
}

/* Pie Legend */
.sa-pie-legend {
  display: flex;
  flex-wrap: wrap;
  gap: 0.75rem;
  justify-content: center;
  padding: 0.75rem 1rem;
  border-top: 1px solid var(--border-primary);
  background: var(--bg-secondary);
}

.sa-pie-legend-item {
  display: flex;
  align-items: center;
  gap: 0.375rem;
  font-size: 0.75rem;
}

.sa-pie-legend-dot {
  width: 8px;
  height: 8px;
  border-radius: 50%;
  flex-shrink: 0;
}

.sa-pie-legend-label {
  color: var(--text-secondary);
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
  max-width: 100px;
}

/* Table */
.sa-table-container {
  overflow-x: auto;
}

.sa-table {
  width: 100%;
  border-collapse: collapse;
  font-size: 0.8125rem;
}

.sa-table th {
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

.sa-table td {
  padding: 0.75rem 1rem;
  border-bottom: 1px solid var(--border-primary);
  vertical-align: middle;
}

.sa-table tbody tr {
  transition: background var(--transition-fast);
}

.sa-table tbody tr:hover {
  background: var(--bg-secondary);
}

.sa-table tbody tr:last-child td {
  border-bottom: none;
}

.sa-table-route {
  font-family: var(--font-family-mono);
  font-size: 0.75rem;
  color: var(--color-primary);
}

.sa-table-value {
  font-weight: 600;
  color: var(--text-primary);
}

.sa-table-success {
  color: var(--color-success);
}

.sa-table-error {
  color: var(--color-error);
}

/* Empty State */
.sa-empty {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  padding: 3rem 2rem;
  text-align: center;
}

.sa-empty-icon {
  width: 56px;
  height: 56px;
  border-radius: 50%;
  background: var(--bg-tertiary);
  display: flex;
  align-items: center;
  justify-content: center;
  margin-bottom: 1rem;
}

.sa-empty-icon svg {
  width: 28px;
  height: 28px;
  color: var(--text-muted);
}

.sa-empty p {
  font-size: 0.875rem;
  color: var(--text-muted);
  margin: 0;
}

/* Error State */
.sa-error {
  background: var(--bg-elevated);
  border: 1px solid var(--color-error);
  border-radius: var(--radius-xl);
  padding: 2rem;
  text-align: center;
  margin: 2rem 0;
}

.sa-error-title {
  font-size: 1.125rem;
  font-weight: 600;
  color: var(--color-error);
  margin: 0 0 0.5rem 0;
}

.sa-error-message {
  font-size: 0.875rem;
  color: var(--text-secondary);
  margin: 0 0 1rem 0;
}

/* Responsive */
@media (max-width: 768px) {
  .sa-page-header {
    flex-direction: column;
    gap: 1rem;
  }

  .sa-page-actions {
    width: 100%;
    justify-content: space-between;
  }

  .sa-toolbar {
    flex-direction: column;
    align-items: stretch;
  }

  .sa-toolbar-section {
    width: 100%;
    justify-content: center;
  }
}

/* Spin animation */
@keyframes sa-spin {
  from { transform: rotate(0deg); }
  to { transform: rotate(360deg); }
}

.sa-icon-spin {
  animation: sa-spin 1s linear infinite;
}
`;

const COLORS = ['#3b82f6', '#10b981', '#f59e0b', '#ef4444', '#8b5cf6', '#ec4899', '#06b6d4', '#84cc16'];

const CustomTooltip = ({ active, payload, label }: any) => {
  if (!active || !payload?.length) return null;
  return (
    <div className="sa-tooltip">
      {label != null && label !== '' && (
        <div className="sa-tooltip-label">{label}</div>
      )}
      {payload.map((entry: any, i: number) => (
        <div key={i} className="sa-tooltip-row">
          <span className="sa-tooltip-dot" style={{ backgroundColor: entry.color || entry.fill }} />
          <span className="sa-tooltip-name">{entry.name}</span>
          <span className="sa-tooltip-value">
            {typeof entry.value === 'number' ? entry.value.toLocaleString() : entry.value}
          </span>
        </div>
      ))}
    </div>
  );
};

const PieLegend: React.FC<{ items: { name: string; color: string }[] }> = ({ items }) => (
  <div className="sa-pie-legend">
    {items.slice(0, 8).map((item, i) => (
      <div key={i} className="sa-pie-legend-item">
        <span className="sa-pie-legend-dot" style={{ backgroundColor: item.color }} />
        <span className="sa-pie-legend-label">{item.name}</span>
      </div>
    ))}
  </div>
);

const formatAxisNumber = (value: number): string => {
  if (value >= 1000000) return `${(value / 1000000).toFixed(1)}M`;
  if (value >= 1000) return `${(value / 1000).toFixed(0)}k`;
  return value.toString();
};

const StatsAnalytics: React.FC = () => {
  const [dailyStats, setDailyStats] = useState<DailyStatsDto[]>([]);
  const [geographicStats, setGeographicStats] = useState<GeographicStatsDto[]>([]);
  const [deviceStats, setDeviceStats] = useState<DeviceStatsDto[]>([]);
  const [browserStats, setBrowserStats] = useState<BrowserStatsDto[]>([]);
  const [routePerformance, setRoutePerformance] = useState<RoutePerformanceDto[]>([]);
  const [trafficTypeStats, setTrafficTypeStats] = useState<TrafficTypeStatsDto[]>([]);

  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const [dateRange, setDateRange] = useState('30d');

  const fetchStats = useCallback(async () => {
    try {
      setLoading(true);
      setError(null);

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
      }

      const fromDate = startDate.toISOString().split('T')[0];
      const toDate = endDate.toISOString().split('T')[0];

      const [daily, geographic, devices, browsers, performance, trafficTypes] = await Promise.all([
        apiService.clickstream.getDailyStats({ fromDate, toDate }),
        apiService.clickstream.getGeographicStats({ fromDate, toDate }),
        apiService.clickstream.getDeviceStats({ fromDate, toDate }),
        apiService.clickstream.getBrowserStats({ fromDate, toDate }),
        apiService.clickstream.getRoutePerformance({ fromDate, toDate, limit: 10 }),
        apiService.clickstream.getTrafficTypeStats({
          fromHour: startDate.toISOString().replace('T', ' ').substring(0, 13),
          toHour: endDate.toISOString().replace('T', ' ').substring(0, 13)
        }),
      ]);

      setDailyStats(daily);
      setGeographicStats(geographic.slice(0, 10));
      setDeviceStats(devices.slice(0, 8));
      setBrowserStats(browsers.slice(0, 8));
      setRoutePerformance(performance);
      setTrafficTypeStats(trafficTypes);
    } catch (err: any) {
      console.error('Failed to fetch stats:', err);
      const errorMessage = err.response?.data?.message || err.message || 'Failed to load statistics. Please try again.';
      setError(errorMessage);
    } finally {
      setLoading(false);
    }
  }, [dateRange]);

  useEffect(() => {
    fetchStats();
  }, [fetchStats]);

  if (loading) {
    return <LoadingSpinner message="Loading statistics..." />;
  }

  if (error) {
    return (
      <div className="container" style={{ paddingTop: '2rem' }}>
        <style>{statsStyles}</style>
        <div className="sa-error">
          <h3 className="sa-error-title">Error Loading Statistics</h3>
          <p className="sa-error-message">{error}</p>
          <button className="btn btn-primary" onClick={fetchStats}>
            <RefreshCw size={16} />
            Retry
          </button>
        </div>
      </div>
    );
  }

  // Calculate totals from daily stats
  const totals = dailyStats.reduce((acc, stat) => ({
    totalClicks: acc.totalClicks + stat.total_clicks,
    uniqueClicks: acc.uniqueClicks + stat.unique_clicks,
    botClicks: acc.botClicks + stat.bot_clicks,
    humanClicks: acc.humanClicks + stat.human_clicks,
  }), { totalClicks: 0, uniqueClicks: 0, botClicks: 0, humanClicks: 0 });

  // Prepare chart data
  const dailyChartData = dailyStats.map(stat => ({
    date: new Date(stat.date).toLocaleDateString('en-US', { month: 'short', day: 'numeric' }),
    clicks: stat.total_clicks,
    unique: stat.unique_clicks,
    human: stat.human_clicks,
    bot: stat.bot_clicks,
  }));

  const geographicChartData = geographicStats.map(stat => ({
    name: getCountryDisplayName(stat.country) || stat.country,
    clicks: stat.total_clicks,
    unique: stat.unique_clicks,
  }));

  const deviceChartData = deviceStats.map(stat => ({
    name: `${stat.device_family} (${stat.os_family})`,
    value: stat.total_clicks,
  }));

  const browserChartData = browserStats.map(stat => ({
    name: stat.user_agent_family,
    value: stat.total_clicks,
  }));

  const trafficChartData = trafficTypeStats.map(stat => ({
    name: stat.is_bot ? 'Bot Traffic' : 'Human Traffic',
    value: stat.total_clicks,
    ips: stat.unique_ips,
  }));

  const dateRanges = [
    { value: '7d', label: '7D' },
    { value: '30d', label: '30D' },
    { value: '90d', label: '90D' },
  ];

  return (
    <>
      <style>{statsStyles}</style>
      <div className="container" style={{ paddingTop: '1.5rem', paddingBottom: '2rem' }}>
        {/* Page Header */}
        <div className="sa-page-header">
          <div className="sa-page-title-section">
            <h1 className="sa-page-title">Analytics</h1>
            <p className="sa-page-subtitle">Detailed statistics and performance metrics</p>
          </div>
          <div className="sa-page-actions">
            <button
              className="btn btn-outline btn-sm"
              onClick={fetchStats}
              disabled={loading}
            >
              <RefreshCw size={14} className={loading ? 'sa-icon-spin' : ''} />
              Refresh
            </button>
          </div>
        </div>

        {/* Toolbar */}
        <div className="sa-toolbar">
          <div className="sa-toolbar-section">
            <div className="sa-range-group">
              {dateRanges.map((r) => (
                <button
                  key={r.value}
                  className={`sa-range-btn ${dateRange === r.value ? 'active' : ''}`}
                  onClick={() => setDateRange(r.value)}
                >
                  {r.label}
                </button>
              ))}
            </div>
          </div>
        </div>

        {/* Stats Grid */}
        <div className="sa-stats-grid">
          <div className="sa-stat-card">
            <div className="sa-stat-icon primary">
              <MousePointerClick />
            </div>
            <div className="sa-stat-content">
              <div className="sa-stat-value">{totals.totalClicks.toLocaleString()}</div>
              <div className="sa-stat-label">Total Clicks</div>
            </div>
          </div>

          <div className="sa-stat-card">
            <div className="sa-stat-icon success">
              <Users />
            </div>
            <div className="sa-stat-content">
              <div className="sa-stat-value">{totals.uniqueClicks.toLocaleString()}</div>
              <div className="sa-stat-label">Unique Clicks</div>
            </div>
          </div>

          <div className="sa-stat-card">
            <div className="sa-stat-icon info">
              <Activity />
            </div>
            <div className="sa-stat-content">
              <div className="sa-stat-value">{totals.humanClicks.toLocaleString()}</div>
              <div className="sa-stat-label">Human Clicks</div>
            </div>
          </div>

          <div className="sa-stat-card">
            <div className="sa-stat-icon warning">
              <Bot />
            </div>
            <div className="sa-stat-content">
              <div className="sa-stat-value">{totals.botClicks.toLocaleString()}</div>
              <div className="sa-stat-label">Bot Clicks</div>
            </div>
          </div>
        </div>

        {/* Daily Trend Chart */}
        <div className="sa-section">
          <div className="sa-chart-card">
            <div className="sa-chart-header">
              <h3 className="sa-chart-title">
                <TrendingUp size={16} />
                Daily Click Trends
              </h3>
              <p className="sa-chart-desc">Click volume breakdown over time</p>
            </div>
            <div className="sa-chart-body" style={{ height: '320px' }}>
              <ResponsiveContainer width="100%" height="100%">
                <AreaChart data={dailyChartData}>
                  <defs>
                    <linearGradient id="totalGradient" x1="0" y1="0" x2="0" y2="1">
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
                  <Area type="monotone" dataKey="clicks" stroke="#3b82f6" fill="url(#totalGradient)" name="Total" strokeWidth={2} />
                  <Line type="monotone" dataKey="unique" stroke="#10b981" name="Unique" strokeWidth={2} dot={false} />
                  <Line type="monotone" dataKey="human" stroke="#f59e0b" name="Human" strokeWidth={2} dot={false} />
                  <Line type="monotone" dataKey="bot" stroke="#ef4444" name="Bot" strokeWidth={2} dot={false} />
                </AreaChart>
              </ResponsiveContainer>
            </div>
          </div>
        </div>

        {/* Two Column Charts */}
        <div className="sa-section-row two-cols">
          {/* Geographic Distribution */}
          <div className="sa-chart-card">
            <div className="sa-chart-header">
              <h3 className="sa-chart-title">
                <Globe size={16} />
                Top Countries
              </h3>
              <p className="sa-chart-desc">Geographic distribution of clicks</p>
            </div>
            <div className="sa-chart-body" style={{ height: '320px' }}>
              {geographicChartData.length > 0 ? (
                <ResponsiveContainer width="100%" height="100%">
                  <BarChart data={geographicChartData} layout="vertical">
                    <CartesianGrid strokeDasharray="3 3" stroke="var(--border-secondary)" horizontal={false} />
                    <XAxis type="number" stroke="var(--text-muted)" fontSize={11} tickLine={false} axisLine={false} tickFormatter={formatAxisNumber} />
                    <YAxis dataKey="name" type="category" width={100} stroke="var(--text-muted)" fontSize={11} tickLine={false} axisLine={false} />
                    <Tooltip content={<CustomTooltip />} />
                    <Bar dataKey="clicks" fill="#3b82f6" name="Total Clicks" radius={[0, 4, 4, 0]} />
                    <Bar dataKey="unique" fill="#10b981" name="Unique Clicks" radius={[0, 4, 4, 0]} />
                  </BarChart>
                </ResponsiveContainer>
              ) : (
                <div className="sa-empty">
                  <div className="sa-empty-icon">
                    <Globe />
                  </div>
                  <p>No geographic data available</p>
                </div>
              )}
            </div>
          </div>

          {/* Traffic Type Distribution */}
          <div className="sa-chart-card">
            <div className="sa-chart-header">
              <h3 className="sa-chart-title">
                <Activity size={16} />
                Traffic Distribution
              </h3>
              <p className="sa-chart-desc">Bot vs Human traffic</p>
            </div>
            <div className="sa-chart-body" style={{ height: '260px' }}>
              {trafficChartData.length > 0 ? (
                <ResponsiveContainer width="100%" height="100%">
                  <PieChart>
                    <Pie
                      data={trafficChartData}
                      cx="50%"
                      cy="50%"
                      labelLine={false}
                      label={({ name, percent }) => `${name.replace(' Traffic', '')}: ${(percent * 100).toFixed(0)}%`}
                      outerRadius={90}
                      innerRadius={55}
                      dataKey="value"
                      paddingAngle={3}
                      stroke="var(--bg-primary)"
                      strokeWidth={2}
                    >
                      {trafficChartData.map((entry, index) => (
                        <Cell key={`cell-${index}`} fill={entry.name === 'Bot Traffic' ? '#ef4444' : '#10b981'} />
                      ))}
                    </Pie>
                    <Tooltip content={<CustomTooltip />} />
                  </PieChart>
                </ResponsiveContainer>
              ) : (
                <div className="sa-empty">
                  <div className="sa-empty-icon">
                    <Activity />
                  </div>
                  <p>No traffic data available</p>
                </div>
              )}
            </div>
            {trafficChartData.length > 0 && (
              <PieLegend
                items={trafficChartData.map(entry => ({
                  name: entry.name,
                  color: entry.name === 'Bot Traffic' ? '#ef4444' : '#10b981',
                }))}
              />
            )}
          </div>
        </div>

        {/* Device and Browser Stats */}
        <div className="sa-section-row two-cols">
          {/* Device Stats */}
          <div className="sa-chart-card">
            <div className="sa-chart-header">
              <h3 className="sa-chart-title">
                <Monitor size={16} />
                Device & OS Distribution
              </h3>
              <p className="sa-chart-desc">Device types and operating systems</p>
            </div>
            <div className="sa-chart-body" style={{ height: '240px' }}>
              {deviceChartData.length > 0 ? (
                <ResponsiveContainer width="100%" height="100%">
                  <PieChart>
                    <Pie
                      data={deviceChartData}
                      cx="50%"
                      cy="50%"
                      labelLine={false}
                      outerRadius={80}
                      innerRadius={50}
                      dataKey="value"
                      paddingAngle={2}
                      stroke="var(--bg-primary)"
                      strokeWidth={2}
                    >
                      {deviceChartData.map((_, index) => (
                        <Cell key={`cell-${index}`} fill={COLORS[index % COLORS.length]} />
                      ))}
                    </Pie>
                    <Tooltip content={<CustomTooltip />} />
                  </PieChart>
                </ResponsiveContainer>
              ) : (
                <div className="sa-empty">
                  <div className="sa-empty-icon">
                    <Monitor />
                  </div>
                  <p>No device data available</p>
                </div>
              )}
            </div>
            {deviceChartData.length > 0 && (
              <PieLegend
                items={deviceChartData.map((entry, i) => ({
                  name: entry.name,
                  color: COLORS[i % COLORS.length],
                }))}
              />
            )}
          </div>

          {/* Browser Stats */}
          <div className="sa-chart-card">
            <div className="sa-chart-header">
              <h3 className="sa-chart-title">
                <Globe size={16} />
                Browser Distribution
              </h3>
              <p className="sa-chart-desc">Browser usage statistics</p>
            </div>
            <div className="sa-chart-body" style={{ height: '320px' }}>
              {browserChartData.length > 0 ? (
                <ResponsiveContainer width="100%" height="100%">
                  <BarChart data={browserChartData}>
                    <CartesianGrid strokeDasharray="3 3" stroke="var(--border-secondary)" vertical={false} />
                    <XAxis dataKey="name" stroke="var(--text-muted)" fontSize={11} tickLine={false} axisLine={false} />
                    <YAxis stroke="var(--text-muted)" fontSize={11} tickLine={false} axisLine={false} tickFormatter={formatAxisNumber} />
                    <Tooltip content={<CustomTooltip />} />
                    <Bar dataKey="value" fill="#8b5cf6" name="Clicks" radius={[4, 4, 0, 0]} />
                  </BarChart>
                </ResponsiveContainer>
              ) : (
                <div className="sa-empty">
                  <div className="sa-empty-icon">
                    <Globe />
                  </div>
                  <p>No browser data available</p>
                </div>
              )}
            </div>
          </div>
        </div>

        {/* Route Performance Table */}
        {routePerformance.length > 0 && (
          <div className="sa-section">
            <div className="sa-chart-card">
              <div className="sa-chart-header">
                <h3 className="sa-chart-title">
                  <Target size={16} />
                  Top Performing Routes
                </h3>
                <p className="sa-chart-desc">Best performing routes by traffic</p>
              </div>
              <div className="sa-table-container">
                <table className="sa-table">
                  <thead>
                    <tr>
                      <th>Domain</th>
                      <th>Route</th>
                      <th>Total Clicks</th>
                      <th>Unique Visitors</th>
                      <th>Human Clicks</th>
                      <th>Bot Clicks</th>
                      <th>Countries</th>
                      <th>Device Types</th>
                    </tr>
                  </thead>
                  <tbody>
                    {routePerformance.map((route, index) => (
                      <tr key={index}>
                        <td className="sa-table-route">{route.route_domain_name ?? '—'}</td>
                        <td className="sa-table-route">{route.route_name ?? '—'}</td>
                        <td className="sa-table-value">{route.total_clicks.toLocaleString()}</td>
                        <td>{route.unique_visitors.toLocaleString()}</td>
                        <td className="sa-table-success">{route.human_clicks.toLocaleString()}</td>
                        <td className="sa-table-error">{route.bot_clicks.toLocaleString()}</td>
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

export default StatsAnalytics;
