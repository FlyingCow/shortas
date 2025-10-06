import React, { useState, useEffect } from 'react';
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
  Cell,
  Legend
} from 'recharts';
import { 
  TrendingUp, 
  Users, 
  MousePointer, 
  Globe,
  Activity,
  Calendar
} from 'lucide-react';
import { apiService, ClickAnalytics } from '../services/api';
import LoadingSpinner from './LoadingSpinner';
import './DesignSystem.css';

interface DashboardStats {
  totalClicks: number;
  uniqueClicks: number;
  totalRoutes: number;
  activeRoutes: number;
}

const Dashboard: React.FC = () => {
  const [analytics, setAnalytics] = useState<ClickAnalytics | null>(null);
  const [stats, setStats] = useState<DashboardStats | null>(null);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const [dateRange, setDateRange] = useState('7d');

  useEffect(() => {
    fetchDashboardData();
  }, [dateRange]);

  const fetchDashboardData = async () => {
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

      const dateRangeParams = {
        start: startDate.toISOString(),
        end: endDate.toISOString(),
      };

      // Fetch analytics data
      const analyticsData = await apiService.analytics.getOverview(dateRangeParams);
      setAnalytics(analyticsData);

      // Fetch routes for stats
      const routesData = await apiService.routes.list({ limit: 1000 });
      const activeRoutes = routesData.filter((route: any) => route.status === 'Active').length;

      setStats({
        totalClicks: analyticsData.total_clicks,
        uniqueClicks: analyticsData.unique_clicks,
        totalRoutes: routesData.length,
        activeRoutes,
      });
    } catch (err) {
      console.error('Failed to fetch dashboard data:', err);
      setError('Failed to load dashboard data. Please try again.');
    } finally {
      setLoading(false);
    }
  };

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
                  <CartesianGrid strokeDasharray="3 3" stroke="var(--border-primary)" />
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
                      borderRadius: '4px',
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
                    outerRadius={80}
                    innerRadius={40}
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
                      borderRadius: '4px',
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
                  <CartesianGrid strokeDasharray="3 3" stroke="var(--border-primary)" />
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
                      borderRadius: '4px',
                      color: 'var(--text-primary)'
                    }}
                  />
                  <Bar dataKey="clicks" fill="var(--primary-500)" radius={[2, 2, 0, 0]} />
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
                    outerRadius={80}
                    innerRadius={40}
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
                      borderRadius: '4px',
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
      </div>
    </div>
  );
};

export default Dashboard;
