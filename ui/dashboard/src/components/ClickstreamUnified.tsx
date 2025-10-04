import React, { useState, useEffect, useRef } from 'react';
import { 
  Activity, 
  Globe, 
  Clock, 
  Monitor, 
  Smartphone, 
  Tablet,
  Play,
  Pause,
  RotateCcw,
  Filter,
  Search,
  ChevronDown
} from 'lucide-react';
import { Dropdown } from 'react-bootstrap';
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
      <div className="loading-state">
        <div className="loading-spinner"></div>
        <p>Loading clickstream...</p>
      </div>
    );
  }

  if (error) {
    return (
      <div className="alert alert-error">
        <h3>Error Loading Clickstream</h3>
        <p>{error}</p>
        <button className="btn btn-primary" onClick={fetchInitialData}>
          Retry
        </button>
      </div>
    );
  }

  return (
    <div className="container">
      {/* Page Header */}
      <div className="page-header">
        <h1 className="page-title">Real-time Clickstream</h1>
        <p className="page-subtitle">Live monitoring of URL clicks and user interactions</p>
      </div>

      {/* Controls */}
      <div className="card mb-lg">
        <div className="card-body">
          <div className="flex items-center justify-between">
            <div className="flex gap-sm">
              <button
                className={`btn ${isLive ? 'btn-success' : 'btn-outline'}`}
                onClick={toggleLive}
              >
                {isLive ? <Pause size={16} /> : <Play size={16} />}
                {isLive ? 'Live' : 'Paused'}
              </button>
              <button
                className="btn btn-outline"
                onClick={clearEvents}
              >
                <RotateCcw size={16} />
                Clear
              </button>
            </div>
            
            <div className="flex gap-sm">
              <Dropdown drop="up">
                <Dropdown.Toggle 
                  variant="outline-secondary" 
                  className="btn btn-outline"
                >
                  {filters.device === 'all' ? 'All Devices' :
                   filters.device === 'desktop' ? 'Desktop' :
                   filters.device === 'mobile' ? 'Mobile' : 'Tablet'}
                  <ChevronDown size={14} className="ms-1" />
                </Dropdown.Toggle>
                <Dropdown.Menu 
                  className="dropdown-menu-end"
                  style={{  }}
                >
                  <Dropdown.Item 
                    active={filters.device === 'all'}
                    onClick={() => setFilters(prev => ({ ...prev, device: 'all' }))}
                  >
                    All Devices
                  </Dropdown.Item>
                  <Dropdown.Item 
                    active={filters.device === 'desktop'}
                    onClick={() => setFilters(prev => ({ ...prev, device: 'desktop' }))}
                  >
                    Desktop
                  </Dropdown.Item>
                  <Dropdown.Item 
                    active={filters.device === 'mobile'}
                    onClick={() => setFilters(prev => ({ ...prev, device: 'mobile' }))}
                  >
                    Mobile
                  </Dropdown.Item>
                  <Dropdown.Item 
                    active={filters.device === 'tablet'}
                    onClick={() => setFilters(prev => ({ ...prev, device: 'tablet' }))}
                  >
                    Tablet
                  </Dropdown.Item>
                </Dropdown.Menu>
              </Dropdown>
              
              <Dropdown drop="up">
                <Dropdown.Toggle 
                  variant="outline-secondary" 
                  className="btn btn-outline"
                >
                  {filters.status === 'all' ? 'All Status' :
                   filters.status === 'success' ? 'Success' :
                   filters.status === 'error' ? 'Error' : 'Redirect'}
                  <ChevronDown size={14} className="ms-1" />
                </Dropdown.Toggle>
                <Dropdown.Menu 
                  className="dropdown-menu-end"
                  style={{  }}
                >
                  <Dropdown.Item 
                    active={filters.status === 'all'}
                    onClick={() => setFilters(prev => ({ ...prev, status: 'all' }))}
                  >
                    All Status
                  </Dropdown.Item>
                  <Dropdown.Item 
                    active={filters.status === 'success'}
                    onClick={() => setFilters(prev => ({ ...prev, status: 'success' }))}
                  >
                    Success
                  </Dropdown.Item>
                  <Dropdown.Item 
                    active={filters.status === 'error'}
                    onClick={() => setFilters(prev => ({ ...prev, status: 'error' }))}
                  >
                    Error
                  </Dropdown.Item>
                  <Dropdown.Item 
                    active={filters.status === 'redirect'}
                    onClick={() => setFilters(prev => ({ ...prev, status: 'redirect' }))}
                  >
                    Redirect
                  </Dropdown.Item>
                </Dropdown.Menu>
              </Dropdown>

              <input
                type="text"
                className="form-control"
                placeholder="Search URLs, cities..."
                value={filters.search}
                onChange={(e) => setFilters(prev => ({ ...prev, search: e.target.value }))}
                style={{ minWidth: '200px' }}
              />
            </div>
          </div>
        </div>
      </div>

      {/* Stats Cards */}
      <div className="stats-grid">
        <div className="stats-card">
          <div className="stats-icon">
            <Activity size={24} />
          </div>
          <div className="stats-content">
            <div className="stats-value">{stats.totalClicks}</div>
            <div className="stats-label">Total Clicks</div>
          </div>
        </div>

        <div className="stats-card">
          <div className="stats-icon">
            <Globe size={24} />
          </div>
          <div className="stats-content">
            <div className="stats-value">{stats.uniqueUsers}</div>
            <div className="stats-label">Unique Users</div>
          </div>
        </div>

        <div className="stats-card">
          <div className="stats-icon">
            <Clock size={24} />
          </div>
          <div className="stats-content">
            <div className="stats-value">{stats.avgResponseTime}ms</div>
            <div className="stats-label">Avg Response</div>
          </div>
        </div>

        <div className="stats-card">
          <div className="stats-icon">
            <Activity size={24} />
          </div>
          <div className="stats-content">
            <div className="stats-value">{stats.errorRate}%</div>
            <div className="stats-label">Error Rate</div>
          </div>
        </div>
      </div>

      {/* Events Table */}
      <div className="card">
        <div className="card-header">
          <h3 className="card-title">
            Live Events ({filteredEvents.length})
            {isLive && <span className="badge badge-success ml-sm">LIVE</span>}
          </h3>
          <p className="card-subtitle">Auto-refreshing every 2-5 seconds</p>
        </div>
        <div className="card-body p-0">
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
        </div>
      </div>
    </div>
  );
};

export default Clickstream;
