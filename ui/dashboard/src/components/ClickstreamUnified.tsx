import React, { useState, useEffect, useRef, useCallback } from 'react';
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
  Bot,
  Filter,
} from 'lucide-react';
import { apiService, ClickStreamEvent } from '../services/api';
import LoadingSpinner from './LoadingSpinner';
import './DesignSystem.css';

const clickstreamStyles = `
.cs-toolbar {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 1rem;
  margin-bottom: 1.5rem;
  flex-wrap: wrap;
}

.cs-toolbar-left {
  display: flex;
  align-items: center;
  gap: 0.5rem;
}

.cs-toolbar-right {
  display: flex;
  align-items: center;
  gap: 0.5rem;
}

.cs-range-group {
  display: flex;
  border: 1px solid var(--border-primary);
  border-radius: var(--radius-md);
  overflow: hidden;
}

.cs-range-btn {
  padding: 0.375rem 0.75rem;
  font-size: 0.8125rem;
  font-weight: 500;
  border: none;
  background: var(--bg-primary);
  color: var(--text-secondary);
  cursor: pointer;
  transition: all var(--transition-fast);
  font-family: inherit;
}

.cs-range-btn:not(:last-child) {
  border-right: 1px solid var(--border-primary);
}

.cs-range-btn:hover:not(.active) {
  background: var(--bg-tertiary);
  color: var(--text-primary);
}

.cs-range-btn.active {
  background: var(--color-primary);
  color: #ffffff;
}

.cs-live-indicator {
  display: inline-flex;
  align-items: center;
  gap: 0.375rem;
  padding: 0.25rem 0.625rem;
  border-radius: var(--radius-2xl);
  font-size: 0.75rem;
  font-weight: 600;
  letter-spacing: 0.025em;
  text-transform: uppercase;
}

.cs-live-indicator.live {
  background: rgba(34, 197, 94, 0.1);
  color: var(--color-success);
}

.cs-live-indicator.paused {
  background: var(--bg-tertiary);
  color: var(--text-muted);
}

.cs-live-dot {
  width: 6px;
  height: 6px;
  border-radius: 50%;
  background: currentColor;
}

.cs-live-indicator.live .cs-live-dot {
  animation: cs-pulse 1.5s ease-in-out infinite;
}

@keyframes cs-pulse {
  0%, 100% { opacity: 1; }
  50% { opacity: 0.3; }
}

.cs-filter-row {
  display: flex;
  align-items: center;
  gap: 0.5rem;
  margin-bottom: 1.5rem;
  flex-wrap: wrap;
}

.cs-filter-row .search-box {
  margin-bottom: 0;
  flex: 1;
  max-width: 320px;
}

.cs-filter-select {
  padding: 0.5rem 2rem 0.5rem 0.75rem;
  border: 1px solid var(--border-secondary);
  border-radius: var(--radius-md);
  font-size: 0.8125rem;
  font-weight: 500;
  color: var(--text-primary);
  background: var(--bg-primary);
  background-image: url("data:image/svg+xml,%3csvg xmlns='http://www.w3.org/2000/svg' fill='none' viewBox='0 0 20 20'%3e%3cpath stroke='%236b7280' stroke-linecap='round' stroke-linejoin='round' stroke-width='1.5' d='m6 8 4 4 4-4'/%3e%3c/svg%3e");
  background-position: right 0.375rem center;
  background-repeat: no-repeat;
  background-size: 1.25em 1.25em;
  cursor: pointer;
  transition: all var(--transition-fast);
  appearance: none;
  -webkit-appearance: none;
  font-family: inherit;
}

.cs-filter-select:hover {
  border-color: var(--border-primary);
}

.cs-filter-select:focus {
  outline: none;
  border-color: var(--color-primary);
  box-shadow: 0 0 0 3px var(--color-primary-light);
}

.cs-filter-input {
  padding: 0.5rem 0.75rem;
  border: 1px solid var(--border-secondary);
  border-radius: var(--radius-md);
  font-size: 0.8125rem;
  color: var(--text-primary);
  background: var(--bg-primary);
  transition: all var(--transition-fast);
  font-family: inherit;
  width: 160px;
}

.cs-filter-input::placeholder {
  color: var(--text-muted);
}

.cs-filter-input:focus {
  outline: none;
  border-color: var(--color-primary);
  box-shadow: 0 0 0 3px var(--color-primary-light);
}

.cs-event-row.is-bot {
  opacity: 0.6;
}

.cs-type-badges {
  display: flex;
  gap: 0.25rem;
  flex-wrap: wrap;
}

.cs-badge {
  display: inline-flex;
  align-items: center;
  padding: 0.125rem 0.5rem;
  border-radius: var(--radius-2xl);
  font-size: 0.6875rem;
  font-weight: 600;
  letter-spacing: 0.025em;
  text-transform: uppercase;
}

.cs-badge-new {
  background: rgba(34, 197, 94, 0.1);
  color: var(--color-success);
}

.cs-badge-returning {
  background: var(--bg-tertiary);
  color: var(--text-muted);
}

.cs-badge-bot {
  background: rgba(245, 158, 11, 0.1);
  color: var(--color-warning);
}

.cs-device {
  display: flex;
  align-items: center;
  gap: 0.375rem;
  font-size: 0.8125rem;
}

.cs-device svg {
  color: var(--text-muted);
  flex-shrink: 0;
}

.cs-location {
  font-size: 0.8125rem;
  line-height: 1.4;
}

.cs-location-city {
  color: var(--text-primary);
  font-weight: 500;
}

.cs-location-country {
  color: var(--text-muted);
  font-size: 0.75rem;
}

.cs-timestamp {
  display: flex;
  align-items: center;
  gap: 0.375rem;
  font-size: 0.8125rem;
  color: var(--text-secondary);
  white-space: nowrap;
}

.cs-timestamp svg {
  color: var(--text-muted);
  flex-shrink: 0;
}

.cs-url {
  font-size: 0.8125rem;
  color: var(--text-secondary);
  max-width: 280px;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.cs-route-id {
  font-size: 0.8125rem;
  font-family: monospace;
  color: var(--text-primary);
  font-weight: 500;
}

.cs-ip {
  font-size: 0.8125rem;
  font-family: monospace;
  color: var(--text-secondary);
}

.cs-browser {
  font-size: 0.8125rem;
  line-height: 1.4;
}

.cs-browser-name {
  color: var(--text-primary);
}

.cs-browser-os {
  color: var(--text-muted);
  font-size: 0.75rem;
}

.cs-empty {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  padding: 4rem 2rem;
  text-align: center;
  color: var(--text-secondary);
}

.cs-empty svg {
  color: var(--text-tertiary);
  margin-bottom: 1rem;
}

.cs-empty h3 {
  margin: 0 0 0.5rem 0;
  color: var(--text-primary);
  font-size: 1rem;
  font-weight: 600;
}

.cs-empty p {
  margin: 0;
  font-size: 0.875rem;
  max-width: 360px;
}

@media (max-width: 768px) {
  .cs-toolbar {
    flex-direction: column;
    align-items: stretch;
  }
  .cs-toolbar-left,
  .cs-toolbar-right {
    justify-content: space-between;
  }
  .cs-filter-row {
    flex-direction: column;
    align-items: stretch;
  }
  .cs-filter-row .search-box {
    max-width: none;
  }
}
`;

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

  const mapToClickEvent = (apiEvent: ClickStreamEvent): ClickEvent => {
    const city = apiEvent.location
      ? (apiEvent.location.split(',')[0] || apiEvent.location)
      : 'Unknown';

    return {
      id: apiEvent.id,
      timestamp: apiEvent.created,
      url: apiEvent.dest || '',
      routeId: apiEvent.routeId || '',
      country: apiEvent.country || 'Unknown',
      city: city,
      device: apiEvent.deviceFamily || 'Unknown',
      browser: apiEvent.userAgentFamily || 'Unknown',
      os: `${apiEvent.osFamily || 'Unknown'} ${apiEvent.osVersion || ''}`.trim(),
      ip: apiEvent.ip || 'Unknown',
      userType: apiEvent.isUnique ? 'new' : 'returning',
      isBot: apiEvent.isBot
    };
  };

  const updateStats = (raw: any) => {
    if (!raw) return;
    // Log the raw response so we can see the actual shape
    console.debug('[clickstream] raw stats response:', JSON.stringify(raw));
    // Handle both camelCase and snake_case field names
    setStats({
      totalClicks: raw.totalClicks ?? raw.total_clicks ?? raw.TotalClicks ?? 0,
      uniqueClicks: raw.uniqueClicks ?? raw.unique_clicks ?? raw.UniqueClicks ?? 0,
      botClicks: raw.botClicks ?? raw.bot_clicks ?? raw.BotClicks ?? 0,
    });
  };

  const getDateRange = useCallback(() => {
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
  }, [dateRange]);

  const fetchInitialData = useCallback(async () => {
    try {
      setLoading(true);
      setError(null);

      const { startDate, endDate } = getDateRange();
      const params = {
        startDate,
        endDate,
        ...(filters.routeId !== 'all' && { routeId: filters.routeId })
      };

      const [rawEvents, rawStats] = await Promise.all([
        apiService.clickstream.getAll(params).catch(() => null),
        apiService.clickstream.getStats(params).catch(() => null)
      ]);

      // Handle both array and paginated { data: [...] } response shapes
      const eventList = Array.isArray(rawEvents)
        ? rawEvents
        : Array.isArray((rawEvents as any)?.data)
          ? (rawEvents as any).data
          : [];

      setEvents(eventList.map(mapToClickEvent));
      updateStats(rawStats);
    } catch (err) {
      setError('Failed to load clickstream data');
      console.error('Error fetching clickstream data:', err);
    } finally {
      setLoading(false);
    }
  }, [filters.routeId, getDateRange]);

  const refreshData = useCallback(async () => {
    if (!isLive) return;

    try {
      const { startDate, endDate } = getDateRange();
      const params = {
        startDate,
        endDate,
        ...(filters.routeId !== 'all' && { routeId: filters.routeId })
      };

      const [rawEvents, rawStats] = await Promise.all([
        apiService.clickstream.getAll(params).catch(() => null),
        apiService.clickstream.getStats(params).catch(() => null)
      ]);

      const eventList = Array.isArray(rawEvents)
        ? rawEvents
        : Array.isArray((rawEvents as any)?.data)
          ? (rawEvents as any).data
          : [];

      setEvents(eventList.map(mapToClickEvent));
      updateStats(rawStats);
    } catch (err) {
      console.error('Error refreshing clickstream data:', err);
    }
  }, [isLive, filters.routeId, getDateRange]);

  const applyFilters = useCallback(() => {
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
  }, [events, filters]);

  const startLiveUpdates = useCallback(() => {
    if (intervalRef.current) {
      clearInterval(intervalRef.current);
    }

    intervalRef.current = setInterval(() => {
      if (isLive) {
        refreshData();
      }
    }, 5000);
  }, [isLive, refreshData]);

  const stopLiveUpdates = () => {
    if (intervalRef.current) {
      clearInterval(intervalRef.current);
      intervalRef.current = null;
    }
  };

  const getDeviceIcon = (device: string) => {
    switch (device.toLowerCase()) {
      case 'mobile':
        return <Smartphone size={14} />;
      case 'tablet':
        return <Tablet size={14} />;
      default:
        return <Monitor size={14} />;
    }
  };

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

  useEffect(() => {
    fetchInitialData();
    return () => { stopLiveUpdates(); };
  }, [fetchInitialData]);

  useEffect(() => {
    applyFilters();
  }, [applyFilters]);

  useEffect(() => {
    if (isLive) {
      startLiveUpdates();
    } else {
      stopLiveUpdates();
    }
    return () => stopLiveUpdates();
  }, [isLive, startLiveUpdates]);

  if (loading) {
    return <LoadingSpinner />;
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

  const dateRanges = [
    { value: '1h', label: '1H' },
    { value: '24h', label: '24H' },
    { value: '7d', label: '7D' },
    { value: '30d', label: '30D' },
  ];

  return (
    <>
      <style>{clickstreamStyles}</style>
      <div className="container">
        {/* Page Header */}
        <div className="page-header">
          <div className="page-title">
            <Activity size={32} />
            <div>
              <h1>Clickstream</h1>
              <p>Real-time monitoring of URL clicks and user interactions</p>
            </div>
          </div>
          <div style={{ display: 'flex', alignItems: 'center', gap: '0.75rem' }}>
            <div className={`cs-live-indicator ${isLive ? 'live' : 'paused'}`}>
              <span className="cs-live-dot" />
              {isLive ? 'Live' : 'Paused'}
            </div>
            <button
              className={`btn btn-sm ${isLive ? 'btn-outline' : 'btn-primary'}`}
              onClick={() => setIsLive(!isLive)}
            >
              {isLive ? <Pause size={14} /> : <Play size={14} />}
              {isLive ? 'Pause' : 'Resume'}
            </button>
            <button className="btn btn-outline btn-sm" onClick={fetchInitialData}>
              <RotateCcw size={14} />
              Refresh
            </button>
          </div>
        </div>

        {/* Stats Cards */}
        <div className="stats-grid">
          <div className="stats-card">
            <div className="stats-icon">
              <Activity size={24} />
            </div>
            <div className="stats-content">
              <div className="stats-value">{stats.totalClicks.toLocaleString()}</div>
              <div className="stats-label">Total Clicks</div>
            </div>
          </div>

          <div className="stats-card">
            <div className="stats-icon">
              <Globe size={24} />
            </div>
            <div className="stats-content">
              <div className="stats-value">{stats.uniqueClicks.toLocaleString()}</div>
              <div className="stats-label">Unique Clicks</div>
            </div>
          </div>

          <div className="stats-card">
            <div className="stats-icon">
              <Bot size={24} />
            </div>
            <div className="stats-content">
              <div className="stats-value">{stats.botClicks.toLocaleString()}</div>
              <div className="stats-label">Bot Clicks</div>
            </div>
          </div>

          <div className="stats-card">
            <div className="stats-icon">
              <Filter size={24} />
            </div>
            <div className="stats-content">
              <div className="stats-value">{filteredEvents.length.toLocaleString()}</div>
              <div className="stats-label">Filtered Events</div>
            </div>
          </div>
        </div>

        {/* Toolbar: date range + filters */}
        <div className="cs-toolbar">
          <div className="cs-toolbar-left">
            <div className="cs-range-group">
              {dateRanges.map((r) => (
                <button
                  key={r.value}
                  className={`cs-range-btn ${dateRange === r.value ? 'active' : ''}`}
                  onClick={() => setDateRange(r.value)}
                >
                  {r.label}
                </button>
              ))}
            </div>
          </div>

          <div className="cs-toolbar-right">
            <select
              className="cs-filter-select"
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
              className="cs-filter-input"
              placeholder="Route ID..."
              value={filters.routeId === 'all' ? '' : filters.routeId}
              onChange={(e) => setFilters(prev => ({ ...prev, routeId: e.target.value || 'all' }))}
            />

            <div className="search-box" style={{ marginBottom: 0 }}>
              <Search size={16} />
              <input
                type="text"
                placeholder="Search URLs, cities..."
                value={filters.search}
                onChange={(e) => setFilters(prev => ({ ...prev, search: e.target.value }))}
              />
            </div>
          </div>
        </div>

        {/* Events Table */}
        <div className="card">
          <div className="card-header">
            <h3 className="card-title">
              Events ({filteredEvents.length})
            </h3>
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
                      <tr key={event.id} className={`cs-event-row ${event.isBot ? 'is-bot' : ''}`}>
                        <td>
                          <div className="cs-timestamp">
                            <Clock size={14} />
                            {formatTimestamp(event.timestamp)}
                          </div>
                        </td>
                        <td>
                          <span className="cs-route-id">{event.routeId}</span>
                        </td>
                        <td>
                          <div className="cs-url" title={event.url}>
                            {event.url}
                          </div>
                        </td>
                        <td>
                          <div className="cs-location">
                            <div className="cs-location-city">{event.city}</div>
                            <div className="cs-location-country">{event.country}</div>
                          </div>
                        </td>
                        <td>
                          <div className="cs-device">
                            {getDeviceIcon(event.device)}
                            <span>{event.device}</span>
                          </div>
                        </td>
                        <td>
                          <div className="cs-browser">
                            <div className="cs-browser-name">{event.browser}</div>
                            <div className="cs-browser-os">{event.os}</div>
                          </div>
                        </td>
                        <td>
                          <span className="cs-ip">{event.ip}</span>
                        </td>
                        <td>
                          <div className="cs-type-badges">
                            <span className={`cs-badge ${event.userType === 'new' ? 'cs-badge-new' : 'cs-badge-returning'}`}>
                              {event.userType}
                            </span>
                            {event.isBot && (
                              <span className="cs-badge cs-badge-bot">bot</span>
                            )}
                          </div>
                        </td>
                      </tr>
                    ))}
                  </tbody>
                </table>

                {filteredEvents.length === 0 && (
                  <div className="cs-empty">
                    <Activity size={48} />
                    <h3>No events found</h3>
                    <p>
                      {filters.search || filters.device !== 'all' || filters.routeId !== 'all' || filters.country !== 'all'
                        ? 'No events match your current filters.'
                        : 'No click events have been recorded yet.'}
                    </p>
                  </div>
                )}
              </div>
            </div>
          </div>
        </div>
      </div>
    </>
  );
};

export default Clickstream;
