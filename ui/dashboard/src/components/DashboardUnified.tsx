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
  MousePointer,
  Globe,
  Activity,
  Bot,
  Target,
  RefreshCw
} from 'lucide-react';
import { apiService, ClickAnalytics, DailyStatsDto, GeographicStatsDto, DeviceStatsDto, BrowserStatsDto, TrafficTypeStatsDto, RoutePerformanceDto } from '../services/api';
import LoadingSpinner from './LoadingSpinner';
import WorldMap from './WorldMap';
import './DesignSystem.css';

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
          country: stat.country,
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

  // Only show full-page spinner on initial load
  if (loading && !analytics) {
    return <LoadingSpinner message="Loading dashboard..." />;
  }

  if (error && !analytics) {
    return (
      <div className="alert alert-error">
        <h3>Error Loading Dashboard</h3>
        <p>{error}</p>
        <button className="btn btn-primary" onClick={fetchDashboardData}>
          Retry
        </button>
      </div>
    );
  }

  const humanRate = stats && stats.totalClicks > 0
    ? `${((stats.humanClicks / stats.totalClicks) * 100).toFixed(0)}%`
    : null;
  const botRate = stats && stats.totalClicks > 0
    ? `${((stats.botClicks / stats.totalClicks) * 100).toFixed(0)}%`
    : null;
  const uniqueRate = stats && stats.totalClicks > 0
    ? `${((stats.uniqueClicks / stats.totalClicks) * 100).toFixed(1)}%`
    : '0%';

  const trafficData = (() => {
    const humanStat = trafficTypeStats.find(stat => !stat.is_bot);
    const botStat = trafficTypeStats.find(stat => stat.is_bot);
    return [
      { name: 'Human', value: humanStat?.total_clicks || 0 },
      { name: 'Bot', value: botStat?.total_clicks || 0 },
    ];
  })();

  return (
    <div className="container">
      {/* Refreshing indicator */}
      {loading && analytics && (
        <div className="dashboard-refreshing">Updating...</div>
      )}

      {/* Header */}
      <div className="dashboard-header" style={{ marginTop: '0.5rem' }}>
        <div />
        <div className="dashboard-controls">
          <div className="dashboard-date-range">
            {[
              { value: '24h', label: '24h' },
              { value: '7d', label: '7d' },
              { value: '30d', label: '30d' },
              { value: '90d', label: '90d' },
            ].map((option) => (
              <button
                key={option.value}
                className={`dashboard-date-btn ${dateRange === option.value ? 'active' : ''}`}
                onClick={() => setDateRange(option.value)}
              >
                {option.label}
              </button>
            ))}
          </div>
          <button
            className="btn btn-outline btn-sm"
            onClick={fetchDashboardData}
            disabled={loading}
            title="Refresh data"
          >
            <RefreshCw size={14} className={loading ? 'icon-spin' : ''} />
          </button>
        </div>
      </div>

      {/* Stats Cards */}
      <div className="dashboard-stats">
        {[
          { icon: MousePointer, value: stats?.totalClicks || 0, label: 'Total Clicks', color: 'var(--primary-500)' },
          { icon: Users, value: stats?.uniqueClicks || 0, label: 'Unique Clicks', color: 'var(--success-500)' },
          { icon: Activity, value: stats?.humanClicks || 0, label: humanRate ? `Human (${humanRate})` : 'Human', color: 'var(--success-600)' },
          { icon: Bot, value: stats?.botClicks || 0, label: botRate ? `Bot (${botRate})` : 'Bot', color: 'var(--error-500)' },
          { icon: TrendingUp, value: uniqueRate, label: 'Unique Rate', color: 'var(--warning-500)' },
          { icon: Globe, value: stats?.activeRoutes || 0, label: 'Active Routes', color: 'var(--primary-600)' },
        ].map((stat, idx) => (
          <div key={idx} className="card dashboard-stat-card">
            <div className="dashboard-stat-icon" style={{ color: stat.color }}>
              <stat.icon size={20} />
            </div>
            <div>
              <div className="dashboard-stat-value">
                {typeof stat.value === 'number' ? stat.value.toLocaleString() : stat.value}
              </div>
              <div className="dashboard-stat-label">{stat.label}</div>
            </div>
          </div>
        ))}
      </div>

      {/* Clicks Over Time - Full Width */}
      <div className="dashboard-section">
        <div className="card dashboard-chart-card">
          <h3 className="dashboard-chart-title">Clicks Over Time</h3>
          <p className="dashboard-chart-desc">Daily click trends</p>
          <div style={{ height: '280px' }}>
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
      <div className="dashboard-section dashboard-row-geo">
        <div className="card dashboard-chart-card">
          <h3 className="dashboard-chart-title">
            <Globe size={16} style={{ color: 'var(--primary-500)' }} />
            Geographic Distribution
          </h3>
          <p className="dashboard-chart-desc">Hover to see details</p>
          <div style={{ height: '300px' }}>
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
        <div className="card dashboard-chart-card">
          <h3 className="dashboard-chart-title">Top Countries</h3>
          <p className="dashboard-chart-desc">By click volume</p>
          <div className="route-analytics-countries">
            {(analytics?.clicks_by_country || []).length > 0 ? (
              (analytics?.clicks_by_country || []).map((country: any, index: number) => {
                const totalClicks = stats?.totalClicks || 1;
                const percentage = Math.round((country.clicks / totalClicks) * 100);
                return (
                  <div key={country.country} className="ra-country-row">
                    <span className="ra-country-rank">{index + 1}</span>
                    <span className="ra-country-name">{country.country}</span>
                    <div className="ra-country-bar-wrap">
                      <div className="ra-country-bar">
                        <div
                          className="ra-country-bar-fill"
                          style={{ width: `${percentage}%` }}
                        />
                      </div>
                    </div>
                    <span className="ra-country-value">{country.clicks.toLocaleString()}</span>
                    <span className="ra-country-pct">{percentage}%</span>
                  </div>
                );
              })
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
      <div className="dashboard-section dashboard-row-distribution">
        {/* Device Distribution */}
        <div className="card dashboard-chart-card">
          <h3 className="dashboard-chart-title">Devices</h3>
          <p className="dashboard-chart-desc">By device type</p>
          <div style={{ height: '200px' }}>
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
          </div>
          <PieLegend
            items={(analytics?.clicks_by_device || []).map((entry, i) => ({
              name: entry.device,
              color: CHART_COLORS[i % CHART_COLORS.length],
            }))}
          />
        </div>

        {/* Browser Distribution */}
        <div className="card dashboard-chart-card">
          <h3 className="dashboard-chart-title">Browsers</h3>
          <p className="dashboard-chart-desc">By browser</p>
          <div style={{ height: '200px' }}>
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
          </div>
          <PieLegend
            items={(analytics?.clicks_by_browser || []).map((entry, i) => ({
              name: entry.browser,
              color: CHART_COLORS[i % CHART_COLORS.length],
            }))}
          />
        </div>

        {/* Traffic Type */}
        <div className="card dashboard-chart-card">
          <h3 className="dashboard-chart-title">Traffic Type</h3>
          <p className="dashboard-chart-desc">Bot vs Human</p>
          <div style={{ height: '200px' }}>
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
              <div className="dashboard-empty-state">
                <Activity size={32} style={{ opacity: 0.4 }} />
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

      {/* Route Performance Table */}
      {routePerformance.length > 0 && (
        <div className="dashboard-section">
          <div className="card dashboard-chart-card">
            <h3 className="dashboard-chart-title">
              <Target size={16} style={{ color: 'var(--primary-500)' }} />
              Top Performing Routes
            </h3>
            <p className="dashboard-chart-desc">Routes ranked by total clicks</p>
            <div style={{ overflowX: 'auto' }}>
              <table className="dashboard-table">
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
                  {routePerformance.slice(0, 5).map((route, index) => (
                    <tr key={index}>
                      <td style={{ fontFamily: 'var(--font-mono)', color: 'var(--primary-500)' }}>
                        {route.route_id.substring(0, 8)}...
                      </td>
                      <td style={{ fontWeight: 600 }}>{route.total_clicks.toLocaleString()}</td>
                      <td>{route.unique_visitors.toLocaleString()}</td>
                      <td style={{ color: 'var(--success-500)' }}>{route.human_clicks.toLocaleString()}</td>
                      <td style={{ color: 'var(--error-500)' }}>{route.bot_clicks.toLocaleString()}</td>
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
  );
};

export default Dashboard;
