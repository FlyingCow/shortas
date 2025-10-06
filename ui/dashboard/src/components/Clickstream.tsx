import React, { useState, useEffect, useRef } from 'react';
import { 
  Row, 
  Col, 
  Card, 
  Table, 
  Badge, 
  Button, 
  Alert,
  Form,
  InputGroup,
  Spinner
} from 'react-bootstrap';
import { 
  Activity, 
  Globe, 
  Clock, 
  MapPin, 
  Monitor, 
  Smartphone, 
  Tablet,
  Play,
  Pause,
  RotateCcw,
  Filter,
  Search
} from 'lucide-react';
import './DesignSystem.css';

interface ClickEvent {
  id: string;
  timestamp: string;
  url: string;
  shortUrl: string;
  country: string;
  city: string;
  device: string;
  browser: string;
  os: string;
  referrer: string;
  userAgent: string;
  ip: string;
  status: 'success' | 'error' | 'redirect';
  responseTime: number;
  userType: 'new' | 'returning';
}

interface ClickstreamFilters {
  device: string;
  country: string;
  status: string;
  search: string;
}

const Clickstream: React.FC = () => {
  const [events, setEvents] = useState<ClickEvent[]>([]);
  const [filteredEvents, setFilteredEvents] = useState<ClickEvent[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const [isLive, setIsLive] = useState(true);
  const [filters, setFilters] = useState<ClickstreamFilters>({
    device: 'all',
    country: 'all',
    status: 'all',
    search: ''
  });
  const [stats, setStats] = useState({
    totalClicks: 0,
    uniqueUsers: 0,
    avgResponseTime: 0,
    errorRate: 0
  });
  
  const intervalRef = useRef<NodeJS.Timeout | null>(null);
  const wsRef = useRef<WebSocket | null>(null);

  // Mock data generator for development
  const generateMockEvent = (): ClickEvent => {
    const countries = ['United States', 'Germany', 'France', 'United Kingdom', 'Canada', 'Australia', 'Japan', 'Brazil'];
    const cities = ['New York', 'London', 'Berlin', 'Paris', 'Toronto', 'Sydney', 'Tokyo', 'São Paulo'];
    const devices = ['Desktop', 'Mobile', 'Tablet'];
    const browsers = ['Chrome', 'Firefox', 'Safari', 'Edge'];
    const os = ['Windows', 'macOS', 'Linux', 'iOS', 'Android'];
    const urls = [
      'https://example.com/product/123',
      'https://example.com/blog/article',
      'https://example.com/landing-page',
      'https://example.com/pricing',
      'https://example.com/contact'
    ];
    const shortUrls = ['short.ly/abc123', 'bit.ly/xyz789', 'tinyurl.com/def456'];
    const statuses: ('success' | 'error' | 'redirect')[] = ['success', 'success', 'success', 'success', 'error'];
    const userTypes: ('new' | 'returning')[] = ['new', 'returning'];

    const country = countries[Math.floor(Math.random() * countries.length)];
    const city = cities[Math.floor(Math.random() * cities.length)];
    const device = devices[Math.floor(Math.random() * devices.length)];
    const browser = browsers[Math.floor(Math.random() * browsers.length)];
    const operatingSystem = os[Math.floor(Math.random() * os.length)];
    const url = urls[Math.floor(Math.random() * urls.length)];
    const shortUrl = shortUrls[Math.floor(Math.random() * shortUrls.length)];
    const status = statuses[Math.floor(Math.random() * statuses.length)];
    const userType = userTypes[Math.floor(Math.random() * userTypes.length)];

    return {
      id: `click_${Date.now()}_${Math.random().toString(36).substr(2, 9)}`,
      timestamp: new Date().toISOString(),
      url,
      shortUrl,
      country,
      city,
      device,
      browser,
      os: operatingSystem,
      referrer: Math.random() > 0.5 ? 'https://google.com' : 'https://facebook.com',
      userAgent: `${browser} on ${operatingSystem}`,
      ip: `${Math.floor(Math.random() * 255)}.${Math.floor(Math.random() * 255)}.${Math.floor(Math.random() * 255)}.${Math.floor(Math.random() * 255)}`,
      status,
      responseTime: Math.floor(Math.random() * 500) + 50,
      userType
    };
  };

  // Fetch initial data
  const fetchInitialData = async () => {
    try {
      setLoading(true);
      setError(null);
      
      // Generate initial mock events
      const initialEvents: ClickEvent[] = [];
      for (let i = 0; i < 20; i++) {
        initialEvents.push(generateMockEvent());
      }
      
      setEvents(initialEvents);
      updateStats(initialEvents);
    } catch (err) {
      setError('Failed to load clickstream data');
      console.error('Error fetching clickstream data:', err);
    } finally {
      setLoading(false);
    }
  };

  // Update statistics
  const updateStats = (eventList: ClickEvent[]) => {
    const totalClicks = eventList.length;
    const uniqueUsers = new Set(eventList.map(e => e.ip)).size;
    const avgResponseTime = eventList.reduce((sum, e) => sum + e.responseTime, 0) / totalClicks;
    const errorRate = (eventList.filter(e => e.status === 'error').length / totalClicks) * 100;

    setStats({
      totalClicks,
      uniqueUsers,
      avgResponseTime: Math.round(avgResponseTime),
      errorRate: Math.round(errorRate * 100) / 100
    });
  };

  // Apply filters
  const applyFilters = () => {
    let filtered = events;

    if (filters.device !== 'all') {
      filtered = filtered.filter(e => e.device.toLowerCase() === filters.device.toLowerCase());
    }

    if (filters.country !== 'all') {
      filtered = filtered.filter(e => e.country.toLowerCase().includes(filters.country.toLowerCase()));
    }

    if (filters.status !== 'all') {
      filtered = filtered.filter(e => e.status === filters.status);
    }

    if (filters.search) {
      filtered = filtered.filter(e => 
        e.url.toLowerCase().includes(filters.search.toLowerCase()) ||
        e.shortUrl.toLowerCase().includes(filters.search.toLowerCase()) ||
        e.city.toLowerCase().includes(filters.search.toLowerCase())
      );
    }

    setFilteredEvents(filtered);
  };

  // Start live updates
  const startLiveUpdates = () => {
    if (intervalRef.current) {
      clearInterval(intervalRef.current);
    }

    intervalRef.current = setInterval(() => {
      if (isLive) {
        const newEvent = generateMockEvent();
        setEvents(prev => {
          const updated = [newEvent, ...prev].slice(0, 100); // Keep last 100 events
          updateStats(updated);
          return updated;
        });
      }
    }, 2000 + Math.random() * 3000); // Random interval between 2-5 seconds
  };

  // Stop live updates
  const stopLiveUpdates = () => {
    if (intervalRef.current) {
      clearInterval(intervalRef.current);
      intervalRef.current = null;
    }
  };

  // Toggle live mode
  const toggleLive = () => {
    setIsLive(!isLive);
  };

  // Clear all events
  const clearEvents = () => {
    setEvents([]);
    setFilteredEvents([]);
    setStats({
      totalClicks: 0,
      uniqueUsers: 0,
      avgResponseTime: 0,
      errorRate: 0
    });
  };

  // Get device icon
  const getDeviceIcon = (device: string) => {
    switch (device.toLowerCase()) {
      case 'mobile':
        return <Smartphone size={16} className="text-primary" />;
      case 'tablet':
        return <Tablet size={16} className="text-info" />;
      case 'desktop':
        return <Monitor size={16} className="text-success" />;
      default:
        return <Monitor size={16} className="text-secondary" />;
    }
  };

  // Get status badge
  const getStatusBadge = (status: string) => {
    switch (status) {
      case 'success':
        return <Badge bg="success">Success</Badge>;
      case 'error':
        return <Badge bg="danger">Error</Badge>;
      case 'redirect':
        return <Badge bg="info">Redirect</Badge>;
      default:
        return <Badge bg="secondary">{status}</Badge>;
    }
  };

  // Format timestamp
  const formatTimestamp = (timestamp: string) => {
    const date = new Date(timestamp);
    const now = new Date();
    const diff = now.getTime() - date.getTime();
    const seconds = Math.floor(diff / 1000);
    const minutes = Math.floor(seconds / 60);
    const hours = Math.floor(minutes / 60);

    if (seconds < 60) return `${seconds}s ago`;
    if (minutes < 60) return `${minutes}m ago`;
    if (hours < 24) return `${hours}h ago`;
    return date.toLocaleString();
  };

  // Initialize
  useEffect(() => {
    fetchInitialData();
    return () => {
      stopLiveUpdates();
      if (wsRef.current) {
        wsRef.current.close();
      }
    };
  }, []);

  // Apply filters when filters change
  useEffect(() => {
    applyFilters();
  }, [filters, events]);

  // Start/stop live updates
  useEffect(() => {
    if (isLive) {
      startLiveUpdates();
    } else {
      stopLiveUpdates();
    }

    return () => stopLiveUpdates();
  }, [isLive]);

  if (loading) {
    return (
      <div className="d-flex justify-content-center align-items-center" style={{ height: '400px' }}>
        <Spinner animation="border" role="status">
          <span className="visually-hidden">Loading clickstream...</span>
        </Spinner>
      </div>
    );
  }

  if (error) {
    return (
      <Alert variant="danger" className="text-center">
        <Alert.Heading>Error Loading Clickstream</Alert.Heading>
        <p>{error}</p>
        <Button variant="outline-danger" onClick={fetchInitialData}>
          Retry
        </Button>
      </Alert>
    );
  }

  return (
    <>
      {/* Header */}
      <Row className="mb-4">
        <Col>
          <div className="d-flex justify-content-between align-items-center">
            <div>
              <h2 className="mb-1">Real-time Clickstream</h2>
              <p className="text-muted mb-0">Live monitoring of URL clicks and user interactions</p>
            </div>
            <div className="d-flex gap-2">
              <Button
                variant={isLive ? "success" : "outline-secondary"}
                onClick={toggleLive}
                className="d-flex align-items-center"
              >
                {isLive ? <Pause size={16} className="me-2" /> : <Play size={16} className="me-2" />}
                {isLive ? 'Live' : 'Paused'}
              </Button>
              <Button
                variant="outline-danger"
                onClick={clearEvents}
                className="d-flex align-items-center"
              >
                <RotateCcw size={16} className="me-2" />
                Clear
              </Button>
            </div>
          </div>
        </Col>
      </Row>

      {/* Stats Cards */}
      <Row className="mb-4">
        <Col md={3} className="mb-3">
          <Card className="h-100 stats-card">
            <Card.Body className="d-flex align-items-center">
              <div className="me-3 text-primary">
                <Activity size={32} />
              </div>
              <div>
                <div className="h4 mb-0 text-dark">{stats.totalClicks}</div>
                <div className="text-dark small fw-medium">Total Clicks</div>
              </div>
            </Card.Body>
          </Card>
        </Col>

        <Col md={3} className="mb-3">
          <Card className="h-100 stats-card">
            <Card.Body className="d-flex align-items-center">
              <div className="me-3 text-success">
                <Globe size={32} />
              </div>
              <div>
                <div className="h4 mb-0 text-dark">{stats.uniqueUsers}</div>
                <div className="text-dark small fw-medium">Unique Users</div>
              </div>
            </Card.Body>
          </Card>
        </Col>

        <Col md={3} className="mb-3">
          <Card className="h-100 stats-card">
            <Card.Body className="d-flex align-items-center">
              <div className="me-3 text-info">
                <Clock size={32} />
              </div>
              <div>
                <div className="h4 mb-0 text-dark">{stats.avgResponseTime}ms</div>
                <div className="text-dark small fw-medium">Avg Response</div>
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
                <div className="h4 mb-0 text-dark">{stats.errorRate}%</div>
                <div className="text-dark small fw-medium">Error Rate</div>
              </div>
            </Card.Body>
          </Card>
        </Col>
      </Row>

      {/* Filters */}
      <Row className="mb-4">
        <Col md={3}>
          <Form.Select
            value={filters.device}
            onChange={(e) => setFilters(prev => ({ ...prev, device: e.target.value }))}
          >
            <option value="all">All Devices</option>
            <option value="desktop">Desktop</option>
            <option value="mobile">Mobile</option>
            <option value="tablet">Tablet</option>
          </Form.Select>
        </Col>
        
        <Col md={3}>
          <Form.Select
            value={filters.status}
            onChange={(e) => setFilters(prev => ({ ...prev, status: e.target.value }))}
          >
            <option value="all">All Status</option>
            <option value="success">Success</option>
            <option value="error">Error</option>
            <option value="redirect">Redirect</option>
          </Form.Select>
        </Col>

        <Col md={4}>
          <InputGroup>
            <InputGroup.Text>
              <Search size={16} />
            </InputGroup.Text>
            <Form.Control
              type="text"
              placeholder="Search URLs, cities..."
              value={filters.search}
              onChange={(e) => setFilters(prev => ({ ...prev, search: e.target.value }))}
            />
          </InputGroup>
        </Col>

        <Col md={2}>
          <Button
            variant="outline-secondary"
            className="w-100"
            onClick={() => setFilters({ device: 'all', country: 'all', status: 'all', search: '' })}
          >
            <Filter size={16} className="me-2" />
            Clear
          </Button>
        </Col>
      </Row>

      {/* Events Table */}
      <Row>
        <Col>
          <Card>
            <Card.Header className="d-flex justify-content-between align-items-center">
              <Card.Title className="mb-0">
                Live Events ({filteredEvents.length})
                {isLive && <Badge bg="success" className="ms-2">LIVE</Badge>}
              </Card.Title>
              <div className="d-flex align-items-center text-muted">
                <Activity size={16} className="me-2" />
                <small>Auto-refreshing every 2-5 seconds</small>
              </div>
            </Card.Header>
            <Card.Body className="p-0">
              <div className="table-container">
                <div className="table-wrapper">
                  <table className="unified-table">
                    <thead>
                      <tr>
                        <th>Time</th>
                        <th>URL</th>
                        <th>Location</th>
                        <th>Device</th>
                        <th>Browser</th>
                        <th>Status</th>
                        <th>Response</th>
                        <th>User</th>
                      </tr>
                    </thead>
                    <tbody>
                      {filteredEvents.map((event) => (
                        <tr key={event.id}>
                          <td>
                            <div className="table-timestamp">
                              <Clock size={14} />
                              {formatTimestamp(event.timestamp)}
                            </div>
                          </td>
                          <td>
                            <div className="table-cell-text">
                              <div className="table-url" title={event.url}>
                                {event.url}
                              </div>
                              <div className="table-url-short">{event.shortUrl}</div>
                            </div>
                          </td>
                          <td>
                            <div className="table-location">
                              <div className="table-location-city">{event.city}</div>
                              <div className="table-location-country">{event.country}</div>
                            </div>
                          </td>
                          <td>
                            <div className="table-device">
                              {getDeviceIcon(event.device)}
                              <span className="table-device-name">{event.device}</span>
                            </div>
                          </td>
                          <td>
                            <div className="table-cell-content">
                              <Globe size={14} />
                              <span>{event.browser}</span>
                            </div>
                          </td>
                          <td>
                            <span className={`table-status-badge ${
                              event.status === 'success' ? 'table-status-success' :
                              event.status === 'error' ? 'table-status-error' :
                              'table-status-info'
                            }`}>
                              {event.status}
                            </span>
                          </td>
                          <td>
                            <span className={`table-metric ${
                              event.responseTime < 200 ? 'table-response-fast' :
                              event.responseTime < 500 ? 'table-response-medium' : 'table-response-slow'
                            }`}>
                              {event.responseTime}ms
                            </span>
                          </td>
                          <td>
                            <span className={`table-status-badge ${
                              event.userType === 'new' ? 'table-status-info' : 'table-status-secondary'
                            }`}>
                              {event.userType}
                            </span>
                          </td>
                        </tr>
                      ))}
                    </tbody>
                  </table>
                
                {filteredEvents.length === 0 && (
                  <div className="table-empty">
                    <div className="table-empty-icon">
                      <Activity size={48} />
                    </div>
                    <div className="table-empty-title">No events found</div>
                    <div className="table-empty-description">
                      {filters.search || filters.device !== 'all' || filters.status !== 'all' 
                        ? 'No events match your current filters.' 
                        : 'No click events have been recorded yet.'}
                    </div>
                  </div>
                )}
                </div>
              </div>
            </Card.Body>
          </Card>
        </Col>
      </Row>
    </>
  );
};

export default Clickstream;
