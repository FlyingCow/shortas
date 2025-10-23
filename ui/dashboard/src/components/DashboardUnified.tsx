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
  Legend
} from 'recharts';
import {
  TrendingUp,
  Users,
  MousePointer,
  Globe,
  Activity,
  Bot,
  Monitor,
  Chrome,
  Target,
  LinkIcon
} from 'lucide-react';
import { apiService, ClickAnalytics, DailyStatsDto, GeographicStatsDto, DeviceStatsDto, BrowserStatsDto, TrafficTypeStatsDto, RoutePerformanceDto } from '../services/api';
import LoadingSpinner from './LoadingSpinner';
import './DesignSystem.css';

interface DashboardStats {
  totalClicks: number;
  uniqueClicks: number;
  botClicks: number;
  humanClicks: number;
  totalRoutes: number;
  activeRoutes: number;
}

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

      // Calculate date range
      const endDate = new Date();
      const startDate = new Date();
      
      switch (dateRange) {
        case '24h':
          startDate.setHours(startDate.getHours() - 24);
          break;
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
      const fromHour = startDate.toISOString().replace('T', ' ').substring(0, 13);
      const toHour = endDate.toISOString().replace('T', ' ').substring(0, 13);

      // Fetch stats using new materialized view endpoints
      const [dailyStats, geographicStats, deviceStats, browserStats, trafficStats, routePerf, routesResponse] = await Promise.all([
        apiService.clickstream.getDailyStats({ fromDate, toDate }),
        apiService.clickstream.getGeographicStats({ fromDate, toDate }),
        apiService.clickstream.getDeviceStats({ fromDate, toDate }),
        apiService.clickstream.getBrowserStats({ fromDate, toDate }),
        apiService.clickstream.getTrafficTypeStats({ fromHour, toHour }),
        apiService.clickstream.getRoutePerformance({ fromDate, toDate, limit: 10 }),
        apiService.routes.list({ page: 1, pageSize: 1000 }),
      ]);

      // Calculate totals from daily stats
      const totals = dailyStats.reduce((acc, stat) => ({
        totalClicks: acc.totalClicks + stat.total_clicks,
        uniqueClicks: acc.uniqueClicks + stat.unique_clicks,
        botClicks: acc.botClicks + stat.bot_clicks,
        humanClicks: acc.humanClicks + stat.human_clicks,
      }), { totalClicks: 0, uniqueClicks: 0, botClicks: 0, humanClicks: 0 });

      // Transform data to match chart format
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

      console.log('Traffic Type Stats:', trafficStats);
      console.log('Route Performance:', routePerf);

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
      console.error('Error response:', err.response?.data);
      console.error('Error status:', err.response?.status);
      const errorMessage = err.response?.data?.message || err.message || 'Failed to load dashboard data. Please try again.';
      setError(errorMessage);
    } finally {
      setLoading(false);
    }
  }, [dateRange]);

  useEffect(() => {
    fetchDashboardData();
  }, [fetchDashboardData]);

  if (loading) {
    return <LoadingSpinner message="Loading dashboard..." />;
  }

  if (error) {
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

  // Enhanced color palettes for different chart types
  const DEVICE_COLORS = ['var(--primary-500)', 'var(--success-500)', 'var(--warning-500)', 'var(--error-500)', 'var(--primary-600)', 'var(--primary-400)', 'var(--success-400)', 'var(--warning-400)'];
  const BROWSER_COLORS = ['var(--primary-700)', 'var(--success-600)', 'var(--warning-600)', 'var(--error-600)', 'var(--primary-800)', 'var(--primary-500)', 'var(--success-500)', 'var(--warning-500)'];
  const GENERAL_COLORS = ['var(--primary-500)', 'var(--success-500)', 'var(--warning-500)', 'var(--error-500)', 'var(--primary-600)'];

  return (
    <div className="container">
      {/* Page Header */}
      <div className="page-header">
        <h1 className="page-title">Dashboard</h1>
        <p className="page-subtitle">Overview of your URL shortener performance</p>
      </div>

      {/* Date Range Selector */}
      <div className="card mb-lg">
        <div className="card-body">
          <div className="flex items-center justify-between">
            <div className="flex gap-sm">
              {[
                { value: '24h', label: 'Last 24 Hours' },
                { value: '7d', label: 'Last 7 Days' },
                { value: '30d', label: 'Last 30 Days' },
                { value: '90d', label: 'Last 90 Days' },
              ].map((option) => (
                <button
                  key={option.value}
                  className={`btn ${dateRange === option.value ? 'btn-primary' : 'btn-outline'}`}
                  onClick={() => setDateRange(option.value)}
                >
                  {option.label}
                </button>
              ))}
            </div>
            <button 
              className="btn btn-secondary" 
              onClick={fetchDashboardData} 
              disabled={loading}
            >
              <Activity size={16} />
              Refresh
            </button>
          </div>
        </div>
      </div>

      {/* Stats Cards */}
      <div className="stats-grid">
        <div className="stats-card">
          <div className="stats-icon">
            <MousePointer size={24} />
          </div>
          <div className="stats-content">
            <div className="stats-value">{stats?.totalClicks.toLocaleString() || '0'}</div>
            <div className="stats-label">Total Clicks</div>
            <div className="stats-change">+12.5% from last period</div>
          </div>
        </div>

        <div className="stats-card">
          <div className="stats-icon">
            <Users size={24} />
          </div>
          <div className="stats-content">
            <div className="stats-value">{stats?.uniqueClicks.toLocaleString() || '0'}</div>
            <div className="stats-label">Unique Visitors</div>
            <div className="stats-change">+8.3% from last period</div>
          </div>
        </div>

        <div className="stats-card">
          <div className="stats-icon">
            <Activity size={24} />
          </div>
          <div className="stats-content">
            <div className="stats-value">{stats?.humanClicks.toLocaleString() || '0'}</div>
            <div className="stats-label">Human Clicks</div>
            <div className="stats-change">
              {stats && stats.totalClicks > 0
                ? `${((stats.humanClicks / stats.totalClicks) * 100).toFixed(1)}% of total`
                : 'N/A'}
            </div>
          </div>
        </div>

        <div className="stats-card">
          <div className="stats-icon">
            <Bot size={24} />
          </div>
          <div className="stats-content">
            <div className="stats-value">{stats?.botClicks.toLocaleString() || '0'}</div>
            <div className="stats-label">Bot Clicks</div>
            <div className="stats-change">
              {stats && stats.totalClicks > 0
                ? `${((stats.botClicks / stats.totalClicks) * 100).toFixed(1)}% of total`
                : 'N/A'}
            </div>
          </div>
        </div>

        <div className="stats-card">
          <div className="stats-icon">
            <TrendingUp size={24} />
          </div>
          <div className="stats-content">
            <div className="stats-value">
              {stats ? ((stats.uniqueClicks / stats.totalClicks) * 100).toFixed(1) : '0'}%
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
            <div className="stats-value">{stats?.activeRoutes || '0'}</div>
            <div className="stats-label">Active Routes</div>
            <div className="stats-change">+3 new routes</div>
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

        {/* Device Distribution */}
        <div className="card">
          <div className="card-header">
            <h3 className="card-title">Device Distribution</h3>
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
                      <Cell key={`cell-${index}`} fill={DEVICE_COLORS[index % DEVICE_COLORS.length]} />
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
                  <Legend 
                    iconType="circle"
                    formatter={(value, entry) => (entry.payload as any)?.device || value}
                  />
                </PieChart>
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
                      <Cell key={`cell-${index}`} fill={BROWSER_COLORS[index % BROWSER_COLORS.length]} />
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
                  <Legend
                    iconType="circle"
                    formatter={(value, entry) => (entry.payload as any)?.browser || value}
                  />
                </PieChart>
              </ResponsiveContainer>
            </div>
          </div>
        </div>

        {/* Traffic Type Distribution */}
        <div className="card">
          <div className="card-header">
            <h3 className="card-title">Traffic Distribution</h3>
            <p className="card-subtitle">Bot vs Human traffic</p>
          </div>
          <div className="card-body">
            {trafficTypeStats.length > 0 ? (
              <div style={{ height: '300px' }}>
                <ResponsiveContainer width="100%" height="100%">
                  <PieChart>
                    <Pie
                      data={(() => {
                        // Ensure we always have both bot and human traffic in the data
                        const humanStat = trafficTypeStats.find(stat => !stat.is_bot);
                        const botStat = trafficTypeStats.find(stat => stat.is_bot);

                        return [
                          {
                            name: 'Human Traffic',
                            value: humanStat?.total_clicks || 0,
                            ips: humanStat?.unique_ips || 0,
                          },
                          {
                            name: 'Bot Traffic',
                            value: botStat?.total_clicks || 0,
                            ips: botStat?.unique_ips || 0,
                          }
                        ];
                      })()}
                      cx="50%"
                      cy="50%"
                      labelLine={false}
                      label={({ name, percent }) => percent > 0.01 ? `${name}: ${(percent * 100).toFixed(0)}%` : ''}
                      outerRadius={120}
                      innerRadius={60}
                      fill="var(--primary-400)"
                      dataKey="value"
                      paddingAngle={2}
                      stroke="var(--bg-primary)"
                      strokeWidth={2}
                    >
                      <Cell key="cell-0" fill="var(--success-500)" />
                      <Cell key="cell-1" fill="var(--error-500)" />
                    </Pie>
                    <Tooltip
                      contentStyle={{
                        backgroundColor: 'var(--bg-primary)',
                        border: '1px solid var(--border-primary)',
                        borderRadius: '0px',
                        color: 'var(--text-primary)'
                      }}
                      formatter={(value: any, name: string, props: any) => [
                        `${value} clicks`,
                        props.payload.name
                      ]}
                    />
                    <Legend
                      iconType="circle"
                      formatter={(value, entry) => (entry.payload as any)?.name || value}
                    />
                  </PieChart>
                </ResponsiveContainer>
              </div>
            ) : (
              <div style={{ height: '300px', display: 'flex', alignItems: 'center', justifyContent: 'center', color: 'var(--text-muted)' }}>
                <div style={{ textAlign: 'center' }}>
                  <Activity size={48} style={{ marginBottom: '1rem', opacity: 0.5 }} />
                  <p>No traffic data available for the selected period</p>
                </div>
              </div>
            )}
          </div>
        </div>
      </div>

      {/* Route Performance Table */}
      {routePerformance.length > 0 && (
        <div className="card mb-2xl">
          <div className="card-header">
            <h3 className="card-title">
              <Target size={20} style={{ marginRight: '8px', verticalAlign: 'middle' }} />
              Top Performing Routes
            </h3>
            <p className="card-subtitle">Best performing routes by traffic</p>
          </div>
          <div className="card-body">
            <div className="table-responsive">
              <table className="data-table">
                <thead>
                  <tr>
                    <th>Route ID</th>
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
                      <td className="font-mono">{route.route_id.substring(0, 8)}...</td>
                      <td>{route.total_clicks.toLocaleString()}</td>
                      <td>{route.unique_visitors.toLocaleString()}</td>
                      <td>{route.human_clicks.toLocaleString()}</td>
                      <td>{route.bot_clicks.toLocaleString()}</td>
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
