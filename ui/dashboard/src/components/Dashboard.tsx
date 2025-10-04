import React, { useState, useEffect } from 'react';
import { Container, Row, Col, Card, Button, ButtonGroup, Spinner, Alert } from 'react-bootstrap';
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
import './Dashboard.css';
import './ChartEnhancements.css';
import './DashboardImprovements.css';
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
      <Alert variant="danger" className="text-center">
        <Alert.Heading>Error Loading Dashboard</Alert.Heading>
        <p>{error}</p>
        <Button variant="outline-danger" onClick={fetchDashboardData}>
          Retry
        </Button>
      </Alert>
    );
  }

  // Enhanced color palettes for different chart types
  const DEVICE_COLORS = ['#3b82f6', '#10b981', '#f59e0b', '#ef4444', '#8b5cf6', '#06b6d4', '#84cc16', '#f97316'];
  const BROWSER_COLORS = ['#1e40af', '#059669', '#d97706', '#dc2626', '#7c3aed', '#0891b2', '#65a30d', '#ea580c'];
  const GENERAL_COLORS = ['#3b82f6', '#10b981', '#f59e0b', '#ef4444', '#8b5cf6'];

  return (
    <>
      {/* Date Range Selector */}
      <Row className="mb-4">
        <Col md={8}>
          <ButtonGroup>
            {[
              { value: '24h', label: 'Last 24 Hours' },
              { value: '7d', label: 'Last 7 Days' },
              { value: '30d', label: 'Last 30 Days' },
              { value: '90d', label: 'Last 90 Days' },
            ].map((option) => (
              <Button
                key={option.value}
                variant={dateRange === option.value ? 'primary' : 'outline-primary'}
                onClick={() => setDateRange(option.value)}
              >
                {option.label}
              </Button>
            ))}
          </ButtonGroup>
        </Col>
        <Col md={4} className="text-end">
          <Button 
            variant="outline-secondary" 
            onClick={fetchDashboardData} 
            disabled={loading}
            className="d-flex align-items-center"
          >
            {loading ? (
              <Spinner animation="border" size="sm" className="me-2" />
            ) : (
              <Activity className="me-2" size={16} />
            )}
            Refresh
          </Button>
        </Col>
      </Row>

      {/* Stats Cards */}
      <Row className="mb-4">
        <Col md={3} className="mb-3">
          <Card className="h-100 stats-card">
            <Card.Body className="d-flex align-items-center">
              <div className="me-3 text-primary">
                <MousePointer size={32} />
              </div>
              <div>
                <div className="h4 mb-0 text-dark">{stats?.totalClicks.toLocaleString() || '0'}</div>
                <div className="text-dark small fw-medium">Total Clicks</div>
              </div>
            </Card.Body>
          </Card>
        </Col>

        <Col md={3} className="mb-3">
          <Card className="h-100 stats-card">
            <Card.Body className="d-flex align-items-center">
              <div className="me-3 text-success">
                <Users size={32} />
              </div>
              <div>
                <div className="h4 mb-0 text-dark">{stats?.uniqueClicks.toLocaleString() || '0'}</div>
                <div className="text-dark small fw-medium">Unique Clicks</div>
              </div>
            </Card.Body>
          </Card>
        </Col>

        <Col md={3} className="mb-3">
          <Card className="h-100 stats-card">
            <Card.Body className="d-flex align-items-center">
              <div className="me-3 text-info">
                <Globe size={32} />
              </div>
              <div>
                <div className="h4 mb-0 text-dark">{stats?.totalRoutes || '0'}</div>
                <div className="text-dark small fw-medium">Total Routes</div>
              </div>
            </Card.Body>
          </Card>
        </Col>

        <Col md={3} className="mb-3">
          <Card className="h-100 stats-card">
            <Card.Body className="d-flex align-items-center">
              <div className="me-3 text-warning">
                <Activity size={32} />
              </div>
              <div>
                <div className="h4 mb-0 text-dark">{stats?.activeRoutes || '0'}</div>
                <div className="text-dark small fw-medium">Active Routes</div>
              </div>
            </Card.Body>
          </Card>
        </Col>
      </Row>

      {/* Charts Grid */}
      <Row className="mb-4">
        {/* Clicks Over Time */}
        <Col lg={8} className="mb-4">
          <Card className="h-100 chart-card">
            <Card.Header className="d-flex justify-content-between align-items-center">
              <Card.Title className="mb-0">Clicks Over Time</Card.Title>
              <Calendar size={20} className="text-muted" />
            </Card.Header>
            <Card.Body>
              <div style={{ width: '100%', height: '300px' }}>
                <ResponsiveContainer width="100%" height="100%">
                  <LineChart data={analytics?.clicks_by_date || []}>
                    <CartesianGrid strokeDasharray="3 3" />
                    <XAxis 
                      dataKey="date" 
                      fontSize={12}
                    />
                    <YAxis 
                      fontSize={12}
                    />
                    <Tooltip />
                    <Line 
                      type="monotone" 
                      dataKey="clicks" 
                      stroke="#0d6efd" 
                      strokeWidth={2}
                      dot={{ fill: '#0d6efd', strokeWidth: 2, r: 4 }}
                    />
                  </LineChart>
                </ResponsiveContainer>
              </div>
            </Card.Body>
          </Card>
        </Col>

        {/* Top Countries */}
        <Col lg={4} className="mb-4">
          <Card className="h-100 chart-card">
            <Card.Header className="d-flex justify-content-between align-items-center">
              <Card.Title className="mb-0">Top Countries</Card.Title>
              <Globe size={20} className="text-muted" />
            </Card.Header>
            <Card.Body>
              <div style={{ width: '100%', height: '300px' }}>
                <ResponsiveContainer width="100%" height="100%">
                  <BarChart data={analytics?.clicks_by_country?.slice(0, 5) || []}>
                    <CartesianGrid strokeDasharray="3 3" />
                    <XAxis 
                      dataKey="country" 
                      fontSize={12}
                    />
                    <YAxis 
                      fontSize={12}
                    />
                    <Tooltip />
                    <Bar dataKey="clicks" fill="#198754" />
                  </BarChart>
                </ResponsiveContainer>
              </div>
            </Card.Body>
          </Card>
        </Col>
      </Row>

      <Row className="mb-4">
        {/* Device Distribution */}
        <Col lg={6} className="mb-4">
          <Card className="h-100">
            <Card.Header className="d-flex justify-content-between align-items-center">
              <Card.Title className="mb-0">Device Distribution</Card.Title>
              <TrendingUp size={20} className="text-muted" />
            </Card.Header>
            <Card.Body>
              <div className="chart-container device-chart" style={{ width: '100%', height: '350px' }}>
                {analytics?.clicks_by_device && analytics.clicks_by_device.length > 0 ? (
                  <ResponsiveContainer width="100%" height="100%">
                    <PieChart>
                      <Pie
                        data={analytics.clicks_by_device}
                        cx="50%"
                        cy="50%"
                        labelLine={false}
                        label={({ device, percent, clicks }) => 
                          percent > 0.05 ? `${device}: ${(percent * 100).toFixed(1)}%` : ''
                        }
                        outerRadius={100}
                        innerRadius={30}
                        paddingAngle={2}
                        dataKey="clicks"
                        stroke="#fff"
                        strokeWidth={2}
                      >
                        {analytics.clicks_by_device.map((entry, index) => (
                          <Cell 
                            key={`cell-${index}`} 
                            fill={DEVICE_COLORS[index % DEVICE_COLORS.length]}
                            stroke={DEVICE_COLORS[index % DEVICE_COLORS.length]}
                            strokeWidth={2}
                          />
                        ))}
                      </Pie>
                      <Tooltip 
                        formatter={(value, name, props) => [
                          `${value.toLocaleString()} clicks (${(props.payload.percent * 100).toFixed(1)}%)`,
                          props.payload.device
                        ]}
                        contentStyle={{
                          backgroundColor: '#fff',
                          border: '1px solid #e5e7eb',
                          borderRadius: '8px',
                          boxShadow: '0 4px 6px -1px rgba(0, 0, 0, 0.1)',
                          fontSize: '14px'
                        }}
                      />
                      <Legend 
                        verticalAlign="bottom" 
                        height={36}
                        iconType="circle"
                        formatter={(value, entry) => (
                          <span style={{ color: entry.color, fontSize: '12px', fontWeight: '500' }}>
                            {(entry.payload as any)?.device || (entry.payload as any)?.browser || (entry.payload as any)?.country || (entry.payload as any)?.source || value}
                          </span>
                        )}
                      />
                    </PieChart>
                  </ResponsiveContainer>
                ) : (
                  <div className="chart-empty">
                    <div className="chart-empty-icon">📱</div>
                    <div>No device data available</div>
                  </div>
                )}
              </div>
            </Card.Body>
          </Card>
        </Col>

        {/* Geographic Distribution */}
        <Col lg={6} className="mb-4">
          <Card className="h-100">
            <Card.Header className="d-flex justify-content-between align-items-center">
              <Card.Title className="mb-0">Geographic Distribution</Card.Title>
              <Globe size={20} className="text-muted" />
            </Card.Header>
            <Card.Body>
              <div style={{ width: '100%', height: '350px' }}>
                <ResponsiveContainer width="100%" height="100%">
                  <PieChart>
                    <Pie
                      data={analytics?.clicks_by_country?.slice(0, 6) || []}
                      cx="50%"
                      cy="50%"
                      labelLine={false}
                      label={({ country, percent }) => 
                        percent > 0.05 ? `${country}: ${(percent * 100).toFixed(1)}%` : ''
                      }
                      outerRadius={100}
                      innerRadius={30}
                      paddingAngle={2}
                      dataKey="clicks"
                      stroke="#fff"
                      strokeWidth={2}
                    >
                      {(analytics?.clicks_by_country?.slice(0, 6) || []).map((entry, index) => (
                        <Cell 
                          key={`cell-${index}`} 
                          fill={GENERAL_COLORS[index % GENERAL_COLORS.length]}
                          stroke={GENERAL_COLORS[index % GENERAL_COLORS.length]}
                          strokeWidth={2}
                        />
                      ))}
                    </Pie>
                    <Tooltip 
                      formatter={(value, name, props) => [
                        `${value.toLocaleString()} clicks (${(props.payload.percent * 100).toFixed(1)}%)`,
                        props.payload.country
                      ]}
                      contentStyle={{
                        backgroundColor: '#fff',
                        border: '1px solid #e5e7eb',
                        borderRadius: '8px',
                        boxShadow: '0 4px 6px -1px rgba(0, 0, 0, 0.1)',
                        fontSize: '14px'
                      }}
                    />
                    <Legend 
                      verticalAlign="bottom" 
                      height={36}
                      iconType="circle"
                        formatter={(value, entry) => (
                          <span style={{ color: entry.color, fontSize: '12px', fontWeight: '500' }}>
                            {(entry.payload as any)?.country || value}
                          </span>
                        )}
                    />
                  </PieChart>
                </ResponsiveContainer>
              </div>
            </Card.Body>
          </Card>
        </Col>
      </Row>

      <Row className="mb-4">
        {/* Browser Distribution */}
        <Col lg={6} className="mb-4">
          <Card className="h-100">
            <Card.Header className="d-flex justify-content-between align-items-center">
              <Card.Title className="mb-0">Browser Distribution</Card.Title>
              <Globe size={20} className="text-muted" />
            </Card.Header>
            <Card.Body>
              <div style={{ width: '100%', height: '350px' }}>
                <ResponsiveContainer width="100%" height="100%">
                  <PieChart>
                    <Pie
                      data={analytics?.clicks_by_browser?.slice(0, 6) || []}
                      cx="50%"
                      cy="50%"
                      labelLine={false}
                      label={({ browser, percent }) => 
                        percent > 0.05 ? `${browser}: ${(percent * 100).toFixed(1)}%` : ''
                      }
                      outerRadius={100}
                      innerRadius={30}
                      paddingAngle={2}
                      dataKey="clicks"
                      stroke="#fff"
                      strokeWidth={2}
                    >
                      {(analytics?.clicks_by_browser?.slice(0, 6) || []).map((entry, index) => (
                        <Cell 
                          key={`cell-${index}`} 
                          fill={BROWSER_COLORS[index % BROWSER_COLORS.length]}
                          stroke={BROWSER_COLORS[index % BROWSER_COLORS.length]}
                          strokeWidth={2}
                        />
                      ))}
                    </Pie>
                    <Tooltip 
                      formatter={(value, name, props) => [
                        `${value.toLocaleString()} clicks (${(props.payload.percent * 100).toFixed(1)}%)`,
                        props.payload.browser
                      ]}
                      contentStyle={{
                        backgroundColor: '#fff',
                        border: '1px solid #e5e7eb',
                        borderRadius: '8px',
                        boxShadow: '0 4px 6px -1px rgba(0, 0, 0, 0.1)',
                        fontSize: '14px'
                      }}
                    />
                    <Legend 
                      verticalAlign="bottom" 
                      height={36}
                      iconType="circle"
                        formatter={(value, entry) => (
                          <span style={{ color: entry.color, fontSize: '12px', fontWeight: '500' }}>
                            {(entry.payload as any)?.browser || value}
                          </span>
                        )}
                    />
                  </PieChart>
                </ResponsiveContainer>
              </div>
            </Card.Body>
          </Card>
        </Col>

        {/* Traffic Sources */}
        <Col lg={6} className="mb-4">
          <Card className="h-100">
            <Card.Header className="d-flex justify-content-between align-items-center">
              <Card.Title className="mb-0">Traffic Sources</Card.Title>
              <Users size={20} className="text-muted" />
            </Card.Header>
            <Card.Body>
              <div style={{ width: '100%', height: '350px' }}>
                <ResponsiveContainer width="100%" height="100%">
                  <PieChart>
                    <Pie
                      data={[
                        { source: 'Direct', clicks: 45, percent: 0.45 },
                        { source: 'Social Media', clicks: 25, percent: 0.25 },
                        { source: 'Search Engines', clicks: 20, percent: 0.20 },
                        { source: 'Referrals', clicks: 10, percent: 0.10 }
                      ]}
                      cx="50%"
                      cy="50%"
                      labelLine={false}
                      label={({ source, percent }) => 
                        percent > 0.05 ? `${source}: ${(percent * 100).toFixed(1)}%` : ''
                      }
                      outerRadius={100}
                      innerRadius={30}
                      paddingAngle={2}
                      dataKey="clicks"
                      stroke="#fff"
                      strokeWidth={2}
                    >
                      {[
                        { source: 'Direct', clicks: 45 },
                        { source: 'Social Media', clicks: 25 },
                        { source: 'Search Engines', clicks: 20 },
                        { source: 'Referrals', clicks: 10 }
                      ].map((entry, index) => (
                        <Cell 
                          key={`cell-${index}`} 
                          fill={GENERAL_COLORS[index % GENERAL_COLORS.length]}
                          stroke={GENERAL_COLORS[index % GENERAL_COLORS.length]}
                          strokeWidth={2}
                        />
                      ))}
                    </Pie>
                    <Tooltip 
                      formatter={(value, name, props) => [
                        `${value.toLocaleString()} clicks (${(props.payload.percent * 100).toFixed(1)}%)`,
                        props.payload.source
                      ]}
                      contentStyle={{
                        backgroundColor: '#fff',
                        border: '1px solid #e5e7eb',
                        borderRadius: '8px',
                        boxShadow: '0 4px 6px -1px rgba(0, 0, 0, 0.1)',
                        fontSize: '14px'
                      }}
                    />
                    <Legend 
                      verticalAlign="bottom" 
                      height={36}
                      iconType="circle"
                        formatter={(value, entry) => (
                          <span style={{ color: entry.color, fontSize: '12px', fontWeight: '500' }}>
                            {(entry.payload as any)?.source || value}
                          </span>
                        )}
                    />
                  </PieChart>
                </ResponsiveContainer>
              </div>
            </Card.Body>
          </Card>
        </Col>
      </Row>
    </>
  );
};

export default Dashboard;
