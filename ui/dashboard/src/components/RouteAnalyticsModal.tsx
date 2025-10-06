import React, { useState, useEffect } from 'react';
import { 
  X, 
  BarChart3, 
  Globe, 
  Smartphone, 
  Clock, 
  TrendingUp,
  Users,
  MousePointer,
  MapPin,
  Monitor,
  Tablet,
  Smartphone as PhoneIcon,
  ExternalLink,
  Calendar,
  Activity
} from 'lucide-react';
import { Modal, Row, Col, Card, Badge, Button, Alert } from 'react-bootstrap';
import { 
  BarChart, 
  Bar, 
  XAxis, 
  YAxis, 
  CartesianGrid, 
  Tooltip, 
  ResponsiveContainer,
  PieChart,
  Pie,
  Cell,
  LineChart,
  Line,
  Area,
  AreaChart
} from 'recharts';
import { RouteDto } from '../services/api';
import './DesignSystem.css';

interface RouteAnalyticsModalProps {
  show: boolean;
  onHide: () => void;
  route: RouteDto | null;
}

interface RouteAnalytics {
  totalClicks: number;
  uniqueVisitors: number;
  avgResponseTime: number;
  errorRate: number;
  topCountries: Array<{ country: string; clicks: number; percentage: number }>;
  deviceBreakdown: Array<{ device: string; clicks: number; percentage: number }>;
  browserBreakdown: Array<{ browser: string; clicks: number; percentage: number }>;
  hourlyClicks: Array<{ hour: string; clicks: number }>;
  dailyClicks: Array<{ date: string; clicks: number }>;
  referrers: Array<{ referrer: string; clicks: number; percentage: number }>;
  responseTimeDistribution: Array<{ range: string; count: number }>;
}

const RouteAnalyticsModal: React.FC<RouteAnalyticsModalProps> = ({ show, onHide, route }) => {
  const [analytics, setAnalytics] = useState<RouteAnalytics | null>(null);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [timeRange, setTimeRange] = useState<'24h' | '7d' | '30d'>('7d');

  // Mock data generator
  const generateMockAnalytics = (route: RouteDto): RouteAnalytics => {
    const baseClicks = Math.floor(Math.random() * 1000) + 100;
    const uniqueVisitors = Math.floor(baseClicks * 0.7);
    
    return {
      totalClicks: baseClicks,
      uniqueVisitors,
      avgResponseTime: Math.floor(Math.random() * 200) + 50,
      errorRate: Math.random() * 5,
      topCountries: [
        { country: 'United States', clicks: Math.floor(baseClicks * 0.35), percentage: 35 },
        { country: 'United Kingdom', clicks: Math.floor(baseClicks * 0.20), percentage: 20 },
        { country: 'Germany', clicks: Math.floor(baseClicks * 0.15), percentage: 15 },
        { country: 'France', clicks: Math.floor(baseClicks * 0.12), percentage: 12 },
        { country: 'Canada', clicks: Math.floor(baseClicks * 0.10), percentage: 10 },
        { country: 'Other', clicks: Math.floor(baseClicks * 0.08), percentage: 8 }
      ],
      deviceBreakdown: [
        { device: 'Mobile', clicks: Math.floor(baseClicks * 0.55), percentage: 55 },
        { device: 'Desktop', clicks: Math.floor(baseClicks * 0.35), percentage: 35 },
        { device: 'Tablet', clicks: Math.floor(baseClicks * 0.10), percentage: 10 }
      ],
      browserBreakdown: [
        { browser: 'Chrome', clicks: Math.floor(baseClicks * 0.45), percentage: 45 },
        { browser: 'Safari', clicks: Math.floor(baseClicks * 0.25), percentage: 25 },
        { browser: 'Firefox', clicks: Math.floor(baseClicks * 0.15), percentage: 15 },
        { browser: 'Edge', clicks: Math.floor(baseClicks * 0.10), percentage: 10 },
        { browser: 'Other', clicks: Math.floor(baseClicks * 0.05), percentage: 5 }
      ],
      hourlyClicks: Array.from({ length: 24 }, (_, i) => ({
        hour: `${i.toString().padStart(2, '0')}:00`,
        clicks: Math.floor(Math.random() * 20) + 5
      })),
      dailyClicks: Array.from({ length: 7 }, (_, i) => {
        const date = new Date();
        date.setDate(date.getDate() - (6 - i));
        return {
          date: date.toLocaleDateString('en-US', { month: 'short', day: 'numeric' }),
          clicks: Math.floor(Math.random() * 100) + 20
        };
      }),
      referrers: [
        { referrer: 'google.com', clicks: Math.floor(baseClicks * 0.40), percentage: 40 },
        { referrer: 'facebook.com', clicks: Math.floor(baseClicks * 0.25), percentage: 25 },
        { referrer: 'twitter.com', clicks: Math.floor(baseClicks * 0.15), percentage: 15 },
        { referrer: 'linkedin.com', clicks: Math.floor(baseClicks * 0.10), percentage: 10 },
        { referrer: 'Direct', clicks: Math.floor(baseClicks * 0.10), percentage: 10 }
      ],
      responseTimeDistribution: [
        { range: '0-100ms', count: Math.floor(baseClicks * 0.60) },
        { range: '100-200ms', count: Math.floor(baseClicks * 0.25) },
        { range: '200-500ms', count: Math.floor(baseClicks * 0.10) },
        { range: '500ms+', count: Math.floor(baseClicks * 0.05) }
      ]
    };
  };

  useEffect(() => {
    if (show && route) {
      fetchAnalytics();
    }
  }, [show, route, timeRange]);

  const fetchAnalytics = async () => {
    if (!route) return;
    
    setLoading(true);
    setError(null);
    
    try {
      // Simulate API call
      await new Promise(resolve => setTimeout(resolve, 1000));
      const mockData = generateMockAnalytics(route);
      setAnalytics(mockData);
    } catch (err) {
      setError('Failed to load analytics data');
    } finally {
      setLoading(false);
    }
  };

  const getDeviceIcon = (device: string) => {
    switch (device.toLowerCase()) {
      case 'mobile':
        return <PhoneIcon size={16} />;
      case 'desktop':
        return <Monitor size={16} />;
      case 'tablet':
        return <Tablet size={16} />;
      default:
        return <Monitor size={16} />;
    }
  };

  const getBrowserColor = (browser: string) => {
    switch (browser.toLowerCase()) {
      case 'chrome':
        return '#4285f4';
      case 'safari':
        return '#007aff';
      case 'firefox':
        return '#ff9500';
      case 'edge':
        return '#0078d4';
      default:
        return '#6b7280';
    }
  };

  const getCountryColor = (index: number) => {
    const colors = ['#3b82f6', '#10b981', '#f59e0b', '#ef4444', '#8b5cf6', '#06b6d4'];
    return colors[index % colors.length];
  };

  if (!route) return null;

  return (
    <Modal show={show} onHide={onHide} size="xl" centered className="route-analytics-modal">
      <Modal.Header closeButton>
        <Modal.Title>
          <div className="d-flex align-items-center">
            <BarChart3 size={24} className="me-2" />
            Route Analytics: {route.link}
          </div>
        </Modal.Title>
      </Modal.Header>
      
      <Modal.Body style={{ maxHeight: '80vh', overflowY: 'auto' }}>
        {loading ? (
          <div className="text-center py-5">
            <div className="spinner-border text-primary" role="status">
              <span className="visually-hidden">Loading...</span>
            </div>
            <p className="mt-3">Loading analytics data...</p>
          </div>
        ) : error ? (
          <Alert variant="danger">
            <h4>Error Loading Analytics</h4>
            <p>{error}</p>
            <Button variant="outline-primary" onClick={fetchAnalytics}>
              Retry
            </Button>
          </Alert>
        ) : analytics ? (
          <>
            {/* Time Range Selector */}
            <div className="d-flex justify-content-end mb-4">
              <div className="btn-group" role="group">
                {(['24h', '7d', '30d'] as const).map((range) => (
                  <Button
                    key={range}
                    variant={timeRange === range ? 'primary' : 'outline-primary'}
                    size="sm"
                    onClick={() => setTimeRange(range)}
                  >
                    {range}
                  </Button>
                ))}
              </div>
            </div>

            {/* Key Metrics */}
            <Row className="mb-4">
              <Col md={3}>
                <Card className="metric-card">
                  <Card.Body>
                    <div className="d-flex align-items-center">
                      <div className="metric-icon">
                        <MousePointer size={24} />
                      </div>
                      <div className="metric-content">
                        <div className="metric-value">{analytics.totalClicks.toLocaleString()}</div>
                        <div className="metric-label">Total Clicks</div>
                      </div>
                    </div>
                  </Card.Body>
                </Card>
              </Col>
              <Col md={3}>
                <Card className="metric-card">
                  <Card.Body>
                    <div className="d-flex align-items-center">
                      <div className="metric-icon">
                        <Users size={24} />
                      </div>
                      <div className="metric-content">
                        <div className="metric-value">{analytics.uniqueVisitors.toLocaleString()}</div>
                        <div className="metric-label">Unique Visitors</div>
                      </div>
                    </div>
                  </Card.Body>
                </Card>
              </Col>
              <Col md={3}>
                <Card className="metric-card">
                  <Card.Body>
                    <div className="d-flex align-items-center">
                      <div className="metric-icon">
                        <Clock size={24} />
                      </div>
                      <div className="metric-content">
                        <div className="metric-value">{analytics.avgResponseTime}ms</div>
                        <div className="metric-label">Avg Response Time</div>
                      </div>
                    </div>
                  </Card.Body>
                </Card>
              </Col>
              <Col md={3}>
                <Card className="metric-card">
                  <Card.Body>
                    <div className="d-flex align-items-center">
                      <div className="metric-icon">
                        <Activity size={24} />
                      </div>
                      <div className="metric-content">
                        <div className="metric-value">{analytics.errorRate.toFixed(1)}%</div>
                        <div className="metric-label">Error Rate</div>
                      </div>
                    </div>
                  </Card.Body>
                </Card>
              </Col>
            </Row>

            {/* Charts Row 1 */}
            <Row className="mb-4">
              <Col lg={6}>
                <Card>
                  <Card.Header>
                    <h5 className="mb-0">Clicks Over Time</h5>
                  </Card.Header>
                  <Card.Body>
                    <ResponsiveContainer width="100%" height={300}>
                      <AreaChart data={analytics.dailyClicks}>
                        <CartesianGrid strokeDasharray="3 3" />
                        <XAxis dataKey="date" />
                        <YAxis />
                        <Tooltip />
                        <Area 
                          type="monotone" 
                          dataKey="clicks" 
                          stroke="#3b82f6" 
                          fill="#3b82f6" 
                          fillOpacity={0.3}
                        />
                      </AreaChart>
                    </ResponsiveContainer>
                  </Card.Body>
                </Card>
              </Col>
              <Col lg={6}>
                <Card>
                  <Card.Header>
                    <h5 className="mb-0">Top Countries</h5>
                  </Card.Header>
                  <Card.Body>
                    <ResponsiveContainer width="100%" height={300}>
                      <BarChart data={analytics.topCountries} layout="horizontal">
                        <CartesianGrid strokeDasharray="3 3" />
                        <XAxis type="number" />
                        <YAxis dataKey="country" type="category" width={100} />
                        <Tooltip />
                        <Bar dataKey="clicks" fill="#3b82f6" />
                      </BarChart>
                    </ResponsiveContainer>
                  </Card.Body>
                </Card>
              </Col>
            </Row>

            {/* Charts Row 2 */}
            <Row className="mb-4">
              <Col lg={6}>
                <Card>
                  <Card.Header>
                    <h5 className="mb-0">Device Breakdown</h5>
                  </Card.Header>
                  <Card.Body>
                    <ResponsiveContainer width="100%" height={300}>
                      <PieChart>
                        <Pie
                          data={analytics.deviceBreakdown}
                          cx="50%"
                          cy="50%"
                          labelLine={false}
                          label={({ device, percentage }) => `${device} (${percentage}%)`}
                          outerRadius={80}
                          fill="#8884d8"
                          dataKey="clicks"
                        >
                          {analytics.deviceBreakdown.map((entry, index) => (
                            <Cell key={`cell-${index}`} fill={getCountryColor(index)} />
                          ))}
                        </Pie>
                        <Tooltip />
                      </PieChart>
                    </ResponsiveContainer>
                  </Card.Body>
                </Card>
              </Col>
              <Col lg={6}>
                <Card>
                  <Card.Header>
                    <h5 className="mb-0">Browser Breakdown</h5>
                  </Card.Header>
                  <Card.Body>
                    <ResponsiveContainer width="100%" height={300}>
                      <PieChart>
                        <Pie
                          data={analytics.browserBreakdown}
                          cx="50%"
                          cy="50%"
                          labelLine={false}
                          label={({ browser, percentage }) => `${browser} (${percentage}%)`}
                          outerRadius={80}
                          fill="#8884d8"
                          dataKey="clicks"
                        >
                          {analytics.browserBreakdown.map((entry, index) => (
                            <Cell key={`cell-${index}`} fill={getBrowserColor(entry.browser)} />
                          ))}
                        </Pie>
                        <Tooltip />
                      </PieChart>
                    </ResponsiveContainer>
                  </Card.Body>
                </Card>
              </Col>
            </Row>

            {/* Charts Row 3 */}
            <Row className="mb-4">
              <Col lg={6}>
                <Card>
                  <Card.Header>
                    <h5 className="mb-0">Hourly Distribution</h5>
                  </Card.Header>
                  <Card.Body>
                    <ResponsiveContainer width="100%" height={300}>
                      <BarChart data={analytics.hourlyClicks}>
                        <CartesianGrid strokeDasharray="3 3" />
                        <XAxis dataKey="hour" />
                        <YAxis />
                        <Tooltip />
                        <Bar dataKey="clicks" fill="#10b981" />
                      </BarChart>
                    </ResponsiveContainer>
                  </Card.Body>
                </Card>
              </Col>
              <Col lg={6}>
                <Card>
                  <Card.Header>
                    <h5 className="mb-0">Response Time Distribution</h5>
                  </Card.Header>
                  <Card.Body>
                    <ResponsiveContainer width="100%" height={300}>
                      <BarChart data={analytics.responseTimeDistribution}>
                        <CartesianGrid strokeDasharray="3 3" />
                        <XAxis dataKey="range" />
                        <YAxis />
                        <Tooltip />
                        <Bar dataKey="count" fill="#f59e0b" />
                      </BarChart>
                    </ResponsiveContainer>
                  </Card.Body>
                </Card>
              </Col>
            </Row>

            {/* Top Referrers Table */}
            <Row>
              <Col>
                <Card>
                  <Card.Header>
                    <h5 className="mb-0">Top Referrers</h5>
                  </Card.Header>
                  <Card.Body>
                    <div className="table-responsive">
                      <table className="table table-sm">
                        <thead>
                          <tr>
                            <th>Referrer</th>
                            <th>Clicks</th>
                            <th>Percentage</th>
                          </tr>
                        </thead>
                        <tbody>
                          {analytics.referrers.map((referrer, index) => (
                            <tr key={index}>
                              <td>
                                <div className="d-flex align-items-center">
                                  <Globe size={16} className="me-2" />
                                  {referrer.referrer}
                                </div>
                              </td>
                              <td>
                                <Badge bg="primary">{referrer.clicks.toLocaleString()}</Badge>
                              </td>
                              <td>
                                <div className="progress" style={{ height: '8px' }}>
                                  <div 
                                    className="progress-bar" 
                                    style={{ width: `${referrer.percentage}%` }}
                                  ></div>
                                </div>
                                <small className="text-muted">{referrer.percentage}%</small>
                              </td>
                            </tr>
                          ))}
                        </tbody>
                      </table>
                    </div>
                  </Card.Body>
                </Card>
              </Col>
            </Row>
          </>
        ) : null}
      </Modal.Body>
      
      <Modal.Footer>
        <Button variant="outline-secondary" onClick={onHide}>
          Close
        </Button>
        <Button variant="primary" onClick={() => window.open(route.dest, '_blank')}>
          <ExternalLink size={16} className="me-1" />
          Visit Destination
        </Button>
      </Modal.Footer>
    </Modal>
  );
};

export default RouteAnalyticsModal;
