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
      {/* Compact Page Header with Inline Controls */}
      <div style={{
        display: 'flex',
        justifyContent: 'space-between',
        alignItems: 'center',
        marginBottom: '1rem',
        padding: '0.5rem 0'
      }}>
        <div>
          <h1 style={{
            fontSize: '1.5rem',
            fontWeight: '600',
            margin: '0 0 0.25rem 0',
            color: 'var(--text-primary)'
          }}>Dashboard</h1>
          <p style={{
            fontSize: '0.875rem',
            margin: 0,
            color: 'var(--text-muted)'
          }}>Performance overview</p>
        </div>

        {/* Inline Date Range Selector */}
        <div style={{ display: 'flex', gap: '0.5rem', alignItems: 'center' }}>
          {[
            { value: '24h', label: '24h' },
            { value: '7d', label: '7d' },
            { value: '30d', label: '30d' },
            { value: '90d', label: '90d' },
          ].map((option) => (
            <button
              key={option.value}
              className={`btn ${dateRange === option.value ? 'btn-primary' : 'btn-outline'}`}
              onClick={() => setDateRange(option.value)}
              style={{
                padding: '0.375rem 0.75rem',
                fontSize: '0.875rem',
                minWidth: '50px'
              }}
            >
              {option.label}
            </button>
          ))}
          <button
            className="btn btn-secondary"
            onClick={fetchDashboardData}
            disabled={loading}
            style={{
              padding: '0.375rem 0.75rem',
              fontSize: '0.875rem',
              display: 'flex',
              alignItems: 'center',
              gap: '0.25rem'
            }}
          >
            <Activity size={14} />
            Refresh
          </button>
        </div>
      </div>

      {/* Compact Stats Cards */}
      <div style={{
        display: 'grid',
        gridTemplateColumns: 'repeat(auto-fit, minmax(150px, 1fr))',
        gap: '0.75rem',
        marginBottom: '1rem'
      }}>
        {[
          { icon: MousePointer, value: stats?.totalClicks, label: 'Total Clicks', change: '+12.5%', color: 'var(--primary-500)' },
          { icon: Users, value: stats?.uniqueClicks, label: 'Unique', change: '+8.3%', color: 'var(--success-500)' },
          { icon: Activity, value: stats?.humanClicks, label: 'Human', change: `${stats && stats.totalClicks > 0 ? ((stats.humanClicks / stats.totalClicks) * 100).toFixed(0) : '0'}%`, color: 'var(--success-600)' },
          { icon: Bot, value: stats?.botClicks, label: 'Bot', change: `${stats && stats.totalClicks > 0 ? ((stats.botClicks / stats.totalClicks) * 100).toFixed(0) : '0'}%`, color: 'var(--error-500)' },
          { icon: TrendingUp, value: `${stats ? ((stats.uniqueClicks / stats.totalClicks) * 100).toFixed(1) : '0'}%`, label: 'Unique Rate', change: '-2.1%', color: 'var(--warning-500)' },
          { icon: Globe, value: stats?.activeRoutes, label: 'Active Routes', change: '+3', color: 'var(--primary-600)' },
        ].map((stat, idx) => (
          <div key={idx} className="card" style={{
            padding: '0.75rem',
            display: 'flex',
            alignItems: 'center',
            gap: '0.75rem'
          }}>
            <div style={{
              padding: '0.5rem',
              borderRadius: '8px',
              backgroundColor: 'var(--bg-secondary)',
              color: stat.color,
              display: 'flex',
              alignItems: 'center',
              justifyContent: 'center'
            }}>
              <stat.icon size={20} />
            </div>
            <div style={{ flex: 1, minWidth: 0 }}>
              <div style={{
                fontSize: '1.25rem',
                fontWeight: '600',
                lineHeight: 1.2,
                marginBottom: '0.125rem',
                color: 'var(--text-primary)'
              }}>{typeof stat.value === 'number' ? stat.value.toLocaleString() : stat.value || '0'}</div>
              <div style={{
                fontSize: '0.75rem',
                color: 'var(--text-muted)',
                marginBottom: '0.125rem'
              }}>{stat.label}</div>
              <div style={{
                fontSize: '0.7rem',
                color: stat.change.startsWith('+') ? 'var(--success-500)' : 'var(--text-muted)'
              }}>{stat.change}</div>
            </div>
          </div>
        ))}
      </div>

      {/* Charts Grid - 3 columns on large screens, 2 on medium, 1 on small */}
      <div style={{
        display: 'grid',
        gridTemplateColumns: 'repeat(auto-fit, minmax(350px, 1fr))',
        gap: '0.75rem',
        marginBottom: '1rem'
      }}>
        {/* Clicks Over Time - Takes 2 columns on large screens */}
        <div className="card" style={{
          gridColumn: window.innerWidth > 1400 ? 'span 2' : 'span 1',
          padding: '0.75rem'
        }}>
          <div style={{ marginBottom: '0.5rem' }}>
            <h3 style={{
              fontSize: '0.95rem',
              fontWeight: '600',
              margin: '0 0 0.125rem 0',
              color: 'var(--text-primary)'
            }}>Clicks Over Time</h3>
            <p style={{
              fontSize: '0.75rem',
              margin: 0,
              color: 'var(--text-muted)'
            }}>Daily trends</p>
          </div>
          <div style={{ height: '200px' }}>
            <ResponsiveContainer width="100%" height="100%">
              <LineChart data={analytics?.clicks_by_date || []}>
                <XAxis
                  dataKey="date"
                  stroke="var(--text-muted)"
                  fontSize={10}
                  tick={{ fontSize: 10 }}
                />
                <YAxis
                  stroke="var(--text-muted)"
                  fontSize={10}
                  tick={{ fontSize: 10 }}
                  width={40}
                />
                <Tooltip
                  contentStyle={{
                    backgroundColor: 'var(--bg-primary)',
                    border: '1px solid var(--border-primary)',
                    borderRadius: '4px',
                    color: 'var(--text-primary)',
                    fontSize: '0.75rem',
                    padding: '0.5rem'
                  }}
                />
                <Line
                  type="monotone"
                  dataKey="clicks"
                  stroke="var(--primary-500)"
                  strokeWidth={2}
                  dot={{ fill: 'var(--primary-500)', strokeWidth: 1, r: 3 }}
                />
              </LineChart>
            </ResponsiveContainer>
          </div>
        </div>

        {/* Geographic Distribution Map - Takes more space on large screens */}
        <div className="card" style={{
          gridColumn: window.innerWidth > 1400 ? 'span 2' : 'span 1',
          padding: '0.75rem'
        }}>
          <div style={{ marginBottom: '0.5rem' }}>
            <h3 style={{
              fontSize: '0.95rem',
              fontWeight: '600',
              margin: '0 0 0.125rem 0',
              color: 'var(--text-primary)',
              display: 'flex',
              alignItems: 'center',
              gap: '0.5rem'
            }}>
              <Globe size={16} />
              Geographic Distribution
            </h3>
            <p style={{
              fontSize: '0.75rem',
              margin: 0,
              color: 'var(--text-muted)'
            }}>Clicks by country - hover to see details</p>
          </div>
          <div style={{ height: '250px' }}>
            <WorldMap
              data={(analytics?.clicks_by_country || []).map(country => {
                const totalClicks = stats?.totalClicks || 1;
                return {
                  name: country.country,
                  clicks: country.clicks,
                  percentage: parseFloat(((country.clicks / totalClicks) * 100).toFixed(2))
                };
              })}
              height={250}
            />
          </div>
        </div>

        {/* Top Countries - Next to map on same line */}
        <div className="card" style={{ padding: '0.75rem' }}>
          <div style={{ marginBottom: '0.5rem' }}>
            <h3 style={{
              fontSize: '0.95rem',
              fontWeight: '600',
              margin: '0 0 0.125rem 0',
              color: 'var(--text-primary)'
            }}>Top Countries</h3>
            <p style={{
              fontSize: '0.75rem',
              margin: 0,
              color: 'var(--text-muted)'
            }}>Geographic reach</p>
          </div>
          <div style={{ height: '250px' }}>
            <ResponsiveContainer width="100%" height="100%">
              <BarChart data={analytics?.clicks_by_country?.slice(0, 8) || []}>
                <XAxis
                  dataKey="country"
                  stroke="var(--text-muted)"
                  fontSize={10}
                  tick={{ fontSize: 10 }}
                  angle={-45}
                  textAnchor="end"
                  height={70}
                />
                <YAxis
                  stroke="var(--text-muted)"
                  fontSize={10}
                  tick={{ fontSize: 10 }}
                  width={40}
                />
                <Tooltip
                  contentStyle={{
                    backgroundColor: 'var(--bg-primary)',
                    border: '1px solid var(--border-primary)',
                    borderRadius: '4px',
                    color: 'var(--text-primary)',
                    fontSize: '0.75rem',
                    padding: '0.5rem'
                  }}
                />
                <Bar dataKey="clicks" fill="var(--primary-500)" radius={[4, 4, 0, 0]} />
              </BarChart>
            </ResponsiveContainer>
          </div>
        </div>

        {/* Device Distribution */}
        <div className="card" style={{ padding: '0.75rem' }}>
          <div style={{ marginBottom: '0.5rem' }}>
            <h3 style={{
              fontSize: '0.95rem',
              fontWeight: '600',
              margin: '0 0 0.125rem 0',
              color: 'var(--text-primary)'
            }}>Devices</h3>
            <p style={{
              fontSize: '0.75rem',
              margin: 0,
              color: 'var(--text-muted)'
            }}>By device type</p>
          </div>
          <div style={{ height: '200px' }}>
            <ResponsiveContainer width="100%" height="100%">
              <PieChart>
                <Pie
                  data={analytics?.clicks_by_device || []}
                  cx="50%"
                  cy="50%"
                  labelLine={false}
                  label={({ percent }) => percent > 0.08 ? `${(percent * 100).toFixed(0)}%` : ''}
                  outerRadius={70}
                  innerRadius={45}
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
                    color: 'var(--text-primary)',
                    fontSize: '0.75rem',
                    padding: '0.5rem'
                  }}
                  formatter={(value: any, name: string, props: any) => [
                    `${value} (${((props.payload.percent || 0) * 100).toFixed(1)}%)`,
                    props.payload.device
                  ]}
                />
              </PieChart>
            </ResponsiveContainer>
          </div>
        </div>

        {/* Browser Distribution */}
        <div className="card" style={{ padding: '0.75rem' }}>
          <div style={{ marginBottom: '0.5rem' }}>
            <h3 style={{
              fontSize: '0.95rem',
              fontWeight: '600',
              margin: '0 0 0.125rem 0',
              color: 'var(--text-primary)'
            }}>Browsers</h3>
            <p style={{
              fontSize: '0.75rem',
              margin: 0,
              color: 'var(--text-muted)'
            }}>By browser</p>
          </div>
          <div style={{ height: '200px' }}>
            <ResponsiveContainer width="100%" height="100%">
              <PieChart>
                <Pie
                  data={analytics?.clicks_by_browser || []}
                  cx="50%"
                  cy="50%"
                  labelLine={false}
                  label={({ percent }) => percent > 0.08 ? `${(percent * 100).toFixed(0)}%` : ''}
                  outerRadius={70}
                  innerRadius={45}
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
                    color: 'var(--text-primary)',
                    fontSize: '0.75rem',
                    padding: '0.5rem'
                  }}
                  formatter={(value: any, name: string, props: any) => [
                    `${value} (${((props.payload.percent || 0) * 100).toFixed(1)}%)`,
                    props.payload.browser
                  ]}
                />
              </PieChart>
            </ResponsiveContainer>
          </div>
        </div>

        {/* Traffic Type Distribution */}
        <div className="card" style={{ padding: '0.75rem' }}>
          <div style={{ marginBottom: '0.5rem' }}>
            <h3 style={{
              fontSize: '0.95rem',
              fontWeight: '600',
              margin: '0 0 0.125rem 0',
              color: 'var(--text-primary)'
            }}>Traffic Type</h3>
            <p style={{
              fontSize: '0.75rem',
              margin: 0,
              color: 'var(--text-muted)'
            }}>Bot vs Human</p>
          </div>
          <div style={{ height: '200px' }}>
            {trafficTypeStats.length > 0 ? (
              <ResponsiveContainer width="100%" height="100%">
                <PieChart>
                  <Pie
                    data={(() => {
                      const humanStat = trafficTypeStats.find(stat => !stat.is_bot);
                      const botStat = trafficTypeStats.find(stat => stat.is_bot);
                      return [
                        { name: 'Human', value: humanStat?.total_clicks || 0 },
                        { name: 'Bot', value: botStat?.total_clicks || 0 }
                      ];
                    })()}
                    cx="50%"
                    cy="50%"
                    labelLine={false}
                    label={({ name, percent }) => percent > 0.05 ? `${name} ${(percent * 100).toFixed(0)}%` : ''}
                    outerRadius={70}
                    innerRadius={45}
                    fill="var(--primary-400)"
                    dataKey="value"
                    paddingAngle={3}
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
                      borderRadius: '4px',
                      color: 'var(--text-primary)',
                      fontSize: '0.75rem',
                      padding: '0.5rem'
                    }}
                  />
                </PieChart>
              </ResponsiveContainer>
            ) : (
              <div style={{ height: '200px', display: 'flex', alignItems: 'center', justifyContent: 'center', color: 'var(--text-muted)' }}>
                <div style={{ textAlign: 'center', fontSize: '0.75rem' }}>
                  <Activity size={32} style={{ marginBottom: '0.5rem', opacity: 0.5 }} />
                  <p>No data available</p>
                </div>
              </div>
            )}
          </div>
        </div>
      </div>

      {/* Compact Route Performance Table */}
      {routePerformance.length > 0 && (
        <div className="card" style={{ padding: '0.75rem', marginBottom: '1rem' }}>
          <div style={{ marginBottom: '0.5rem', display: 'flex', alignItems: 'center', gap: '0.5rem' }}>
            <Target size={16} />
            <h3 style={{
              fontSize: '0.95rem',
              fontWeight: '600',
              margin: 0,
              color: 'var(--text-primary)'
            }}>Top Performing Routes</h3>
          </div>
          <div style={{ overflowX: 'auto' }}>
            <table style={{
              width: '100%',
              fontSize: '0.8rem',
              borderCollapse: 'collapse'
            }}>
              <thead>
                <tr style={{
                  borderBottom: '1px solid var(--border-primary)',
                  textAlign: 'left'
                }}>
                  <th style={{ padding: '0.5rem', fontWeight: '600', color: 'var(--text-muted)' }}>Route ID</th>
                  <th style={{ padding: '0.5rem', fontWeight: '600', color: 'var(--text-muted)' }}>Clicks</th>
                  <th style={{ padding: '0.5rem', fontWeight: '600', color: 'var(--text-muted)' }}>Unique</th>
                  <th style={{ padding: '0.5rem', fontWeight: '600', color: 'var(--text-muted)' }}>Human</th>
                  <th style={{ padding: '0.5rem', fontWeight: '600', color: 'var(--text-muted)' }}>Bot</th>
                  <th style={{ padding: '0.5rem', fontWeight: '600', color: 'var(--text-muted)' }}>Countries</th>
                  <th style={{ padding: '0.5rem', fontWeight: '600', color: 'var(--text-muted)' }}>Devices</th>
                </tr>
              </thead>
              <tbody>
                {routePerformance.slice(0, 5).map((route, index) => (
                  <tr key={index} style={{
                    borderBottom: '1px solid var(--border-secondary)',
                    transition: 'background-color 0.2s'
                  }}>
                    <td style={{
                      padding: '0.5rem',
                      fontFamily: 'monospace',
                      color: 'var(--primary-500)'
                    }}>{route.route_id.substring(0, 8)}...</td>
                    <td style={{ padding: '0.5rem', fontWeight: '500' }}>{route.total_clicks.toLocaleString()}</td>
                    <td style={{ padding: '0.5rem' }}>{route.unique_visitors.toLocaleString()}</td>
                    <td style={{ padding: '0.5rem', color: 'var(--success-500)' }}>{route.human_clicks.toLocaleString()}</td>
                    <td style={{ padding: '0.5rem', color: 'var(--error-500)' }}>{route.bot_clicks.toLocaleString()}</td>
                    <td style={{ padding: '0.5rem' }}>{route.countries_reached}</td>
                    <td style={{ padding: '0.5rem' }}>{route.device_types}</td>
                  </tr>
                ))}
              </tbody>
            </table>
          </div>
        </div>
      )}
    </div>
  );
};

export default Dashboard;
