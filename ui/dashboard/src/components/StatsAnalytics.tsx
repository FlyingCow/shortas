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
  Legend,
  CartesianGrid
} from 'recharts';
import {
  TrendingUp,
  Users,
  MousePointer,
  Globe,
  Monitor,
  Chrome,
  Activity,
  Target,
  Bot
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
import LoadingSpinner from './LoadingSpinner';
import './DesignSystem.css';

const COLORS = ['#3b82f6', '#10b981', '#f59e0b', '#ef4444', '#8b5cf6', '#ec4899', '#06b6d4', '#84cc16'];

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
      }

      const fromDate = startDate.toISOString().split('T')[0];
      const toDate = endDate.toISOString().split('T')[0];

      // Fetch all stats in parallel
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
      setGeographicStats(geographic.slice(0, 10)); // Top 10 countries
      setDeviceStats(devices.slice(0, 8));
      setBrowserStats(browsers.slice(0, 8));
      setRoutePerformance(performance);
      setTrafficTypeStats(trafficTypes);
    } catch (err: any) {
      console.error('Failed to fetch stats:', err);
      console.error('Error response:', err.response?.data);
      console.error('Error status:', err.response?.status);
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
      <div className="container">
        <div className="alert alert-error" style={{ marginTop: '2rem' }}>
          <h3>Error Loading Statistics</h3>
          <p>{error}</p>
          <button className="btn btn-primary" onClick={fetchStats}>
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
    name: stat.country,
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

  return (
    <div className="container" style={{ paddingTop: '1.5rem' }}>
      {/* Date Range Selector */}
      <div className="card mb-lg">
        <div className="card-body">
          <div className="flex items-center justify-between">
            <div className="flex gap-sm">
              {[
                { value: '7d', label: 'Last 7 Days' },
                { value: '30d', label: 'Last 30 Days' },
                { value: '90d', label: 'Last 90 Days' },
              ].map((option) => (
                <button
                  key={option.value}
                  className={`btn ${dateRange === option.value ? 'btn-primary' : 'btn-secondary'}`}
                  onClick={() => setDateRange(option.value)}
                >
                  {option.label}
                </button>
              ))}
            </div>
          </div>
        </div>
      </div>

      {/* Summary Stats Cards */}
      <div className="stats-grid">
        <div className="stats-card">
          <div className="stats-icon">
            <MousePointer size={24} />
          </div>
          <div className="stats-content">
            <div className="stats-value">{totals.totalClicks.toLocaleString()}</div>
            <div className="stats-label">Total Clicks</div>
          </div>
        </div>

        <div className="stats-card">
          <div className="stats-icon">
            <Users size={24} />
          </div>
          <div className="stats-content">
            <div className="stats-value">{totals.uniqueClicks.toLocaleString()}</div>
            <div className="stats-label">Unique Clicks</div>
          </div>
        </div>

        <div className="stats-card">
          <div className="stats-icon">
            <Activity size={24} />
          </div>
          <div className="stats-content">
            <div className="stats-value">{totals.humanClicks.toLocaleString()}</div>
            <div className="stats-label">Human Clicks</div>
          </div>
        </div>

        <div className="stats-card">
          <div className="stats-icon">
            <Bot size={24} />
          </div>
          <div className="stats-content">
            <div className="stats-value">{totals.botClicks.toLocaleString()}</div>
            <div className="stats-label">Bot Clicks</div>
          </div>
        </div>
      </div>

      {/* Daily Trend Chart */}
      <div className="card mb-lg">
        <div className="card-header">
          <h3 className="card-title">
            <TrendingUp size={20} style={{ marginRight: '8px' }} />
            Daily Click Trends
          </h3>
          <p className="card-subtitle">Click volume over time</p>
        </div>
        <div className="card-body">
          <div style={{ height: '300px' }}>
            <ResponsiveContainer width="100%" height="100%">
          <LineChart data={dailyChartData}>
            <CartesianGrid strokeDasharray="3 3" />
            <XAxis dataKey="date" />
            <YAxis />
            <Tooltip />
            <Legend />
            <Line type="monotone" dataKey="clicks" stroke="#3b82f6" name="Total Clicks" strokeWidth={2} />
            <Line type="monotone" dataKey="unique" stroke="#10b981" name="Unique Clicks" strokeWidth={2} />
            <Line type="monotone" dataKey="human" stroke="#f59e0b" name="Human" strokeWidth={2} />
            <Line type="monotone" dataKey="bot" stroke="#ef4444" name="Bot" strokeWidth={2} />
          </LineChart>
        </ResponsiveContainer>
          </div>
        </div>
      </div>

      {/* Two Column Charts */}
      <div className="grid grid-cols-1 lg:grid-cols-2 gap-lg mb-lg">
        {/* Geographic Distribution */}
        <div className="card">
          <div className="card-header">
            <h3 className="card-title">
              <Globe size={20} style={{ marginRight: '8px' }} />
              Top Countries
            </h3>
            <p className="card-subtitle">Geographic distribution of clicks</p>
          </div>
          <div className="card-body">
            <div style={{ height: '300px' }}>
              <ResponsiveContainer width="100%" height="100%">
            <BarChart data={geographicChartData} layout="horizontal">
              <CartesianGrid strokeDasharray="3 3" />
              <XAxis type="number" />
              <YAxis dataKey="name" type="category" width={100} />
              <Tooltip />
              <Bar dataKey="clicks" fill="#3b82f6" name="Total Clicks" />
              <Bar dataKey="unique" fill="#10b981" name="Unique Clicks" />
            </BarChart>
          </ResponsiveContainer>
            </div>
          </div>
        </div>

        {/* Traffic Type Distribution */}
        <div className="card">
          <div className="card-header">
            <h3 className="card-title">
              <Activity size={20} style={{ marginRight: '8px' }} />
              Traffic Distribution
            </h3>
            <p className="card-subtitle">Bot vs Human traffic</p>
          </div>
          <div className="card-body">
            <div style={{ height: '300px' }}>
              <ResponsiveContainer width="100%" height="100%">
            <PieChart>
              <Pie
                data={trafficChartData}
                cx="50%"
                cy="50%"
                labelLine={false}
                label={({ name, percent }) => `${name}: ${(percent * 100).toFixed(0)}%`}
                outerRadius={80}
                fill="#8884d8"
                dataKey="value"
              >
                {trafficChartData.map((entry, index) => (
                  <Cell key={`cell-${index}`} fill={entry.name === 'Bot Traffic' ? '#ef4444' : '#10b981'} />
                ))}
              </Pie>
              <Tooltip />
            </PieChart>
          </ResponsiveContainer>
            </div>
          </div>
        </div>
      </div>

      {/* Device and Browser Stats */}
      <div className="grid grid-cols-1 lg:grid-cols-2 gap-lg mb-lg">
        {/* Device Stats */}
        <div className="card">
          <div className="card-header">
            <h3 className="card-title">
              <Monitor size={20} style={{ marginRight: '8px' }} />
              Device & OS Distribution
            </h3>
            <p className="card-subtitle">Device types and operating systems</p>
          </div>
          <div className="card-body">
            <div style={{ height: '300px' }}>
              <ResponsiveContainer width="100%" height="100%">
            <PieChart>
              <Pie
                data={deviceChartData}
                cx="50%"
                cy="50%"
                labelLine={false}
                label={({ name, percent }) => `${(percent * 100).toFixed(0)}%`}
                outerRadius={80}
                fill="#8884d8"
                dataKey="value"
              >
                {deviceChartData.map((entry, index) => (
                  <Cell key={`cell-${index}`} fill={COLORS[index % COLORS.length]} />
                ))}
              </Pie>
              <Tooltip />
              <Legend />
            </PieChart>
          </ResponsiveContainer>
            </div>
          </div>
        </div>

        {/* Browser Stats */}
        <div className="card">
          <div className="card-header">
            <h3 className="card-title">
              <Chrome size={20} style={{ marginRight: '8px' }} />
              Browser Distribution
            </h3>
            <p className="card-subtitle">Browser usage statistics</p>
          </div>
          <div className="card-body">
            <div style={{ height: '300px' }}>
              <ResponsiveContainer width="100%" height="100%">
            <BarChart data={browserChartData}>
              <CartesianGrid strokeDasharray="3 3" />
              <XAxis dataKey="name" />
              <YAxis />
              <Tooltip />
              <Bar dataKey="value" fill="#8b5cf6" name="Clicks" />
            </BarChart>
          </ResponsiveContainer>
            </div>
          </div>
        </div>
      </div>

      {/* Route Performance Table */}
      {routePerformance.length > 0 && (
        <div className="card mb-lg">
          <div className="card-header">
            <h3 className="card-title">
              <Target size={20} style={{ marginRight: '8px' }} />
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

export default StatsAnalytics;
