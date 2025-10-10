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
  Search,
} from 'lucide-react';
import { apiService, ClickStreamEvent } from '../services/api';
import './DesignSystem.css';

interface ClickEvent {
  id: string;
  timestamp: string;
  url: string;
  routeId: string;
  country: string;
  city: string;
  device: string;
  browser: string;
  os: string;
  ip: string;
  userType: 'new' | 'returning';
  isBot: boolean;
}

interface ClickstreamFilters {
  device: string;
  country: string;
  routeId: string;
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
    routeId: 'all',
    search: ''
  });
  const [stats, setStats] = useState({
    totalClicks: 0,
    uniqueClicks: 0,
    botClicks: 0
  });
  const [dateRange, setDateRange] = useState('24h');
  
  const intervalRef = useRef<NodeJS.Timeout | null>(null);
  const wsRef = useRef<WebSocket | null>(null);

  // Map API response to component interface
  const mapToClickEvent = (apiEvent: ClickStreamEvent): ClickEvent => {
    // Parse city from location (e.g., "New York, NY" -> "New York")
    const city = apiEvent.location.split(',')[0] || apiEvent.location;

    return {
      id: apiEvent.id,
      timestamp: apiEvent.created,
      url: apiEvent.dest,
      routeId: apiEvent.routeId,
      country: apiEvent.country,
      city: city,
      device: apiEvent.deviceFamily,
      browser: apiEvent.userAgentFamily,
      os: `${apiEvent.osFamily} ${apiEvent.osVersion}`,
      ip: apiEvent.ip,
      userType: apiEvent.isUnique ? 'new' : 'returning',
      isBot: apiEvent.isBot
    };
  };

  // Calculate date range
  const getDateRange = () => {
    const endDate = new Date();
    const startDate = new Date();

    switch (dateRange) {
      case '1h':
        startDate.setHours(startDate.getHours() - 1);
        break;
      case '24h':
        startDate.setHours(startDate.getHours() - 24);
        break;
      case '7d':
        startDate.setDate(startDate.getDate() - 7);
        break;
      case '30d':
        startDate.setDate(startDate.getDate() - 30);
        break;
    }

    return {
      startDate: startDate.toISOString(),
      endDate: endDate.toISOString()
    };
  };

  // Fetch initial data
  const fetchInitialData = async () => {
    try {
      setLoading(true);
      setError(null);

      const { startDate, endDate } = getDateRange();
      const params = {
        startDate,
        endDate,
        ...(filters.routeId !== 'all' && { routeId: filters.routeId })
      };

      // Fetch clickstream events and stats in parallel
      const [apiEvents, apiStats] = await Promise.all([
        apiService.clickstream.getAll(params),
        apiService.clickstream.getStats(params)
      ]);

      // Map API events to component format
      const mappedEvents = apiEvents.map(mapToClickEvent);

      setEvents(mappedEvents);
      setStats({
        totalClicks: apiStats.totalClicks,
        uniqueClicks: apiStats.uniqueClicks,
        botClicks: apiStats.botClicks
      });
    } catch (err) {
      setError('Failed to load clickstream data');
      console.error('Error fetching clickstream data:', err);
    } finally {
      setLoading(false);
    }
  };

  // Refresh data periodically when live
  const refreshData = async () => {
    if (!isLive) return;

    try {
      const { startDate, endDate } = getDateRange();
      const params = {
        startDate,
        endDate,
        ...(filters.routeId !== 'all' && { routeId: filters.routeId })
      };

      const [apiEvents, apiStats] = await Promise.all([
        apiService.clickstream.getAll(params),
        apiService.clickstream.getStats(params)
      ]);

      const mappedEvents = apiEvents.map(mapToClickEvent);
      setEvents(mappedEvents);
      setStats({
        totalClicks: apiStats.totalClicks,
        uniqueClicks: apiStats.uniqueClicks,
        botClicks: apiStats.botClicks
      });
    } catch (err) {
      console.error('Error refreshing clickstream data:', err);
    }
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

    if (filters.routeId !== 'all') {
      filtered = filtered.filter(e => e.routeId === filters.routeId);
    }

    if (filters.search) {
      filtered = filtered.filter(e =>
        e.url.toLowerCase().includes(filters.search.toLowerCase()) ||
        e.routeId.toLowerCase().includes(filters.search.toLowerCase()) ||
        e.city.toLowerCase().includes(filters.search.toLowerCase()) ||
        e.country.toLowerCase().includes(filters.search.toLowerCase())
      );
    }

    setFilteredEvents(filtered);
  };

  // Start live updates
  const startLiveUpdates = () => {
    if (intervalRef.current) {
      clearInterval(intervalRef.current);
    }

    // Refresh data every 5 seconds when live
    intervalRef.current = setInterval(() => {
      if (isLive) {
        refreshData();
      }
    }, 5000);
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

  // Refresh events
  const handleRefresh = () => {
    fetchInitialData();
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
    };
  }, [dateRange, filters.routeId]);

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
                onClick={handleRefresh}
              >
                <RotateCcw size={16} />
                Refresh
              </button>

              {/* Date Range Selector */}
              <select
                className="control-dropdown"
                value={dateRange}
                onChange={(e) => setDateRange(e.target.value)}
              >
                <option value="1h">Last Hour</option>
                <option value="24h">Last 24 Hours</option>
                <option value="7d">Last 7 Days</option>
                <option value="30d">Last 30 Days</option>
              </select>
            </div>
            
            <div className="control-group">
              <select 
                className="control-dropdown"
                value={filters.device}
                onChange={(e) => setFilters(prev => ({ ...prev, device: e.target.value }))}
              >
                <option value="all">All Devices</option>
                <option value="desktop">Desktop</option>
                <option value="mobile">Mobile</option>
                <option value="tablet">Tablet</option>
              </select>
              
              <input
                type="text"
                className="control-dropdown"
                placeholder="Filter by Route ID"
                value={filters.routeId === 'all' ? '' : filters.routeId}
                onChange={(e) => setFilters(prev => ({ ...prev, routeId: e.target.value || 'all' }))}
              />

              <div className="control-input">
                <Search size={16} className="input-icon" />
                <input
                  type="text"
                  placeholder="Search URLs, cities..."
                  value={filters.search}
                  onChange={(e) => setFilters(prev => ({ ...prev, search: e.target.value }))}
                  style={{ minWidth: '200px' }}
                />
              </div>
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
            <div className="stats-value">{stats.uniqueClicks}</div>
            <div className="stats-label">Unique Clicks</div>
          </div>
        </div>

        <div className="stats-card">
          <div className="stats-icon">
            <Activity size={24} />
          </div>
          <div className="stats-content">
            <div className="stats-value">{stats.botClicks}</div>
            <div className="stats-label">Bot Clicks</div>
          </div>
        </div>

        <div className="stats-card">
          <div className="stats-icon">
            <Clock size={24} />
          </div>
          <div className="stats-content">
            <div className="stats-value">{filteredEvents.length}</div>
            <div className="stats-label">Filtered Events</div>
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
                    <th>Route ID</th>
                    <th>Destination</th>
                    <th>Location</th>
                    <th>Device</th>
                    <th>Browser / OS</th>
                    <th>IP</th>
                    <th>Type</th>
                  </tr>
                </thead>
                <tbody>
                  {filteredEvents.map((event) => (
                    <tr key={event.id} className={event.isBot ? 'bot-event' : ''}>
                      <td>
                        <div className="table-timestamp">
                          <Clock size={14} />
                          {formatTimestamp(event.timestamp)}
                        </div>
                      </td>
                      <td>
                        <div className="table-cell-content">
                          <span className="table-metric">{event.routeId}</span>
                        </div>
                      </td>
                      <td>
                        <div className="table-cell-text">
                          <div className="table-url" title={event.url}>
                            {event.url.length > 50 ? `${event.url.substring(0, 50)}...` : event.url}
                          </div>
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
                        <div className="table-cell-text">
                          <div>{event.browser}</div>
                          <div className="table-url-short">{event.os}</div>
                        </div>
                      </td>
                      <td>
                        <span className="table-metric">{event.ip}</span>
                      </td>
                      <td>
                        <div className="flex gap-xs">
                          <span className={`table-status-badge ${
                            event.userType === 'new' ? 'table-status-success' : 'table-status-secondary'
                          }`}>
                            {event.userType}
                          </span>
                          {event.isBot && (
                            <span className="table-status-badge table-status-warning">
                              bot
                            </span>
                          )}
                        </div>
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
                    {filters.search || filters.device !== 'all' || filters.routeId !== 'all' || filters.country !== 'all'
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
