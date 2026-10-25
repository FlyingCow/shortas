import React, { useState, useEffect, useRef, useCallback } from 'react';
import {
  Activity,
  Clock,
  Monitor,
  Smartphone,
  Tablet,
  Play,
  Pause,
  RotateCcw,
  Search,
  Bot,
  Users,
  MousePointerClick,
  X,
} from 'lucide-react';
import { apiService, ClickStreamEvent } from '../services/api';
import { useWorkspace } from '../contexts/WorkspaceContext';
import LoadingSpinner from './LoadingSpinner';
import './DesignSystem.css';

// Clickstream-specific widgets with no shared DesignSystem.css equivalent:
// the live/paused pulse indicator, the card-header split layout (title +
// right-aligned meta text), and the truncated destination-URL cell.
const clickstreamStyles = `
.cs-live-badge {
  display: inline-flex;
  align-items: center;
  gap: 0.5rem;
  padding: 0.5rem 1rem;
  border-radius: var(--radius-2xl);
  font-size: 0.8125rem;
  font-weight: 600;
  transition: all var(--transition-fast);
}

.cs-live-badge.live {
  background: linear-gradient(135deg, rgba(34, 197, 94, 0.15), rgba(34, 197, 94, 0.05));
  color: var(--color-success);
  border: 1px solid rgba(34, 197, 94, 0.3);
}

.cs-live-badge.paused {
  background: var(--bg-tertiary);
  color: var(--text-muted);
  border: 1px solid var(--border-primary);
}

.cs-live-dot {
  width: 8px;
  height: 8px;
  border-radius: 50%;
  background: currentColor;
}

.cs-live-badge.live .cs-live-dot {
  animation: cs-pulse 1.5s ease-in-out infinite;
  box-shadow: 0 0 8px currentColor;
}

@keyframes cs-pulse {
  0%, 100% { opacity: 1; transform: scale(1); }
  50% { opacity: 0.5; transform: scale(0.85); }
}

.cs-events-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 1rem;
  flex-wrap: wrap;
  margin-bottom: var(--space-lg);
}

.cs-events-meta {
  font-size: 0.75rem;
  color: var(--text-muted);
  display: flex;
  align-items: center;
  gap: 0.375rem;
  white-space: nowrap;
}

.cs-cell-url {
  max-width: 220px;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.cs-clear-btn {
  position: absolute;
  right: 0.5rem;
  top: 50%;
  transform: translateY(-50%);
  padding: 0.25rem;
  border: none;
  background: transparent;
  color: var(--text-muted);
  cursor: pointer;
  border-radius: var(--radius-sm);
  display: flex;
  align-items: center;
  justify-content: center;
}

.cs-clear-btn:hover {
  color: var(--text-primary);
  background: var(--bg-tertiary);
}
`;

interface ClickEvent {
  id: string;
  timestamp: string;
  url: string;
  routeName: string;
  routeDomainName: string;
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
  route: string;
  search: string;
}

const Clickstream: React.FC = () => {
  const { currentWorkspaceId } = useWorkspace();
  const [events, setEvents] = useState<ClickEvent[]>([]);
  const [filteredEvents, setFilteredEvents] = useState<ClickEvent[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const [isLive, setIsLive] = useState(true);
  const [filters, setFilters] = useState<ClickstreamFilters>({
    device: 'all',
    country: 'all',
    route: 'all',
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
      routeName: apiEvent.routeName || '',
      routeDomainName: apiEvent.routeDomainName || '',
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
        workspaceId: currentWorkspaceId ?? undefined,
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
      setError('Failed to load clickstream data');
      console.error('Error fetching clickstream data:', err);
    } finally {
      setLoading(false);
    }
  }, [getDateRange, currentWorkspaceId]);

  const refreshData = useCallback(async () => {
    if (!isLive) return;

    try {
      const { startDate, endDate } = getDateRange();
      const params = {
        startDate,
        endDate,
        workspaceId: currentWorkspaceId ?? undefined,
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
  }, [isLive, getDateRange, currentWorkspaceId]);

  const applyFilters = useCallback(() => {
    let filtered = events;

    if (filters.device !== 'all') {
      filtered = filtered.filter(e => e.device.toLowerCase() === filters.device.toLowerCase());
    }

    if (filters.country !== 'all') {
      filtered = filtered.filter(e => e.country.toLowerCase().includes(filters.country.toLowerCase()));
    }

    if (filters.route !== 'all') {
      const routeFilter = filters.route.toLowerCase();
      filtered = filtered.filter(e =>
        e.routeName.toLowerCase().includes(routeFilter) ||
        e.routeDomainName.toLowerCase().includes(routeFilter)
      );
    }

    if (filters.search) {
      filtered = filtered.filter(e =>
        e.url.toLowerCase().includes(filters.search.toLowerCase()) ||
        e.routeName.toLowerCase().includes(filters.search.toLowerCase()) ||
        e.routeDomainName.toLowerCase().includes(filters.search.toLowerCase()) ||
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

  const clearFilters = () => {
    setFilters({
      device: 'all',
      country: 'all',
      route: 'all',
      search: ''
    });
  };

  const hasActiveFilters = filters.device !== 'all' || filters.route !== 'all' || filters.search !== '';

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
      <div className="container">
        <style>{clickstreamStyles}</style>
        <div className="card" style={{ padding: '2rem', textAlign: 'center', borderColor: 'var(--color-error)' }}>
          <h3 className="status-error" style={{ margin: '0 0 0.5rem 0' }}>Error Loading Clickstream</h3>
          <p style={{ color: 'var(--text-secondary)', margin: '0 0 1rem 0' }}>{error}</p>
          <button className="btn btn-primary" onClick={fetchInitialData}>
            <RotateCcw size={16} />
            Retry
          </button>
        </div>
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
          <div className="page-title-section">
            <h1 className="page-title">Clickstream</h1>
            <p className="page-subtitle">Real-time monitoring of link clicks and visitor activity</p>
          </div>
          <div className="page-actions">
            <div className={`cs-live-badge ${isLive ? 'live' : 'paused'}`}>
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

        {/* Stats Grid */}
        <div className="stats-grid">
          <div className="stats-card">
            <div className="stats-icon primary">
              <MousePointerClick />
            </div>
            <div className="stats-content">
              <div className="stats-value">{stats.totalClicks.toLocaleString()}</div>
              <div className="stats-label">Total Clicks</div>
            </div>
          </div>

          <div className="stats-card">
            <div className="stats-icon success">
              <Users />
            </div>
            <div className="stats-content">
              <div className="stats-value">{stats.uniqueClicks.toLocaleString()}</div>
              <div className="stats-label">Unique Visitors</div>
            </div>
          </div>

          <div className="stats-card">
            <div className="stats-icon warning">
              <Bot />
            </div>
            <div className="stats-content">
              <div className="stats-value">{stats.botClicks.toLocaleString()}</div>
              <div className="stats-label">Bot Clicks</div>
            </div>
          </div>

          <div className="stats-card">
            <div className="stats-icon info">
              <Activity />
            </div>
            <div className="stats-content">
              <div className="stats-value">{filteredEvents.length.toLocaleString()}</div>
              <div className="stats-label">Showing</div>
            </div>
          </div>
        </div>

        {/* Toolbar */}
        <div className="toolbar">
          <div className="toolbar-section">
            <div className="range-group">
              {dateRanges.map((r) => (
                <button
                  key={r.value}
                  className={`range-btn ${dateRange === r.value ? 'active' : ''}`}
                  onClick={() => setDateRange(r.value)}
                >
                  {r.label}
                </button>
              ))}
            </div>

            <select
              className="form-select form-select-sm"
              style={{ minWidth: 130 }}
              value={filters.device}
              onChange={(e) => setFilters(prev => ({ ...prev, device: e.target.value }))}
            >
              <option value="all">All Devices</option>
              <option value="desktop">Desktop</option>
              <option value="mobile">Mobile</option>
              <option value="tablet">Tablet</option>
            </select>
          </div>

          <div className="toolbar-section">
            <div className="search-box" style={{ minWidth: 240, marginBottom: 0 }}>
              <Search size={16} />
              <input
                type="text"
                placeholder="Search routes, URLs, locations..."
                value={filters.search}
                onChange={(e) => setFilters(prev => ({ ...prev, search: e.target.value }))}
              />
              {filters.search && (
                <button
                  className="cs-clear-btn"
                  onClick={() => setFilters(prev => ({ ...prev, search: '' }))}
                >
                  <X size={14} />
                </button>
              )}
            </div>

            {hasActiveFilters && (
              <button className="btn btn-outline btn-sm" onClick={clearFilters}>
                Clear Filters
              </button>
            )}
          </div>
        </div>

        {/* Events Table */}
        <div className="chart-card">
          <div className="cs-events-header">
            <h3 className="chart-title">
              Click Events
              <span className="badge badge-secondary">{filteredEvents.length}</span>
            </h3>
            <div className="cs-events-meta">
              <Clock size={12} />
              {isLive ? 'Auto-refreshing every 5s' : 'Updates paused'}
            </div>
          </div>

          <div className="table-container">
            <div className="table-wrapper">
              {filteredEvents.length > 0 ? (
                <table className="unified-table">
                  <thead>
                    <tr>
                      <th>Time</th>
                      <th>Domain</th>
                      <th>Route</th>
                      <th>Destination</th>
                      <th>Location</th>
                      <th>Device</th>
                      <th>Browser / OS</th>
                      <th>IP Address</th>
                      <th>Type</th>
                    </tr>
                  </thead>
                  <tbody>
                    {filteredEvents.map((event) => (
                      <tr key={event.id} style={event.isBot ? { opacity: 0.6 } : undefined}>
                        <td>
                          <div className="table-cell-content">
                            <Clock size={14} className="table-cell-icon" />
                            <span className="table-cell-text">{formatTimestamp(event.timestamp)}</span>
                          </div>
                        </td>
                        <td className="table-cell-secondary">{event.routeDomainName || '—'}</td>
                        <td className="table-cell-primary">{event.routeName || '—'}</td>
                        <td>
                          <div className="table-cell-secondary cs-cell-url" title={event.url}>
                            {event.url || '—'}
                          </div>
                        </td>
                        <td>
                          <div className="table-location-city">{event.city}</div>
                          <div className="table-location-country">{event.country}</div>
                        </td>
                        <td>
                          <div className="table-cell-content">
                            <span className="table-cell-icon">{getDeviceIcon(event.device)}</span>
                            <span className="table-cell-text">{event.device}</span>
                          </div>
                        </td>
                        <td>
                          <div className="table-device-name">{event.browser}</div>
                          <div className="table-device">{event.os}</div>
                        </td>
                        <td className="table-cell-muted" style={{ fontFamily: 'var(--font-family-mono)', fontSize: '0.75rem' }}>
                          {event.ip}
                        </td>
                        <td>
                          <div className="table-action-buttons">
                            <span className={`badge ${event.userType === 'new' ? 'badge-success' : 'badge-secondary'}`}>
                              {event.userType}
                            </span>
                            {event.isBot && (
                              <span className="badge badge-warning">bot</span>
                            )}
                          </div>
                        </td>
                      </tr>
                    ))}
                  </tbody>
                </table>
              ) : (
                <div className="table-empty">
                  <div className="table-empty-icon">
                    <Activity />
                  </div>
                  <div className="table-empty-title">No events found</div>
                  <p className="table-empty-description">
                    {hasActiveFilters
                      ? 'No events match your current filters. Try adjusting your search criteria.'
                      : 'No click events have been recorded in this time period.'}
                  </p>
                  {hasActiveFilters && (
                    <button
                      className="btn btn-outline btn-sm"
                      onClick={clearFilters}
                      style={{ marginTop: '1rem' }}
                    >
                      Clear Filters
                    </button>
                  )}
                </div>
              )}
            </div>
          </div>
        </div>
      </div>
    </>
  );
};

export default Clickstream;
