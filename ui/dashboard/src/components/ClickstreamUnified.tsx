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
  Users,
  MousePointerClick,
  X,
  ChevronDown,
} from 'lucide-react';
import { apiService, ClickStreamEvent } from '../services/api';
import LoadingSpinner from './LoadingSpinner';
import './DesignSystem.css';

const clickstreamStyles = `
/* ===== CLICKSTREAM PAGE STYLES ===== */

/* Page Header */
.cs-page-header {
  display: flex;
  align-items: flex-start;
  justify-content: space-between;
  gap: 1.5rem;
  margin-bottom: 2rem;
  flex-wrap: wrap;
}

.cs-page-title-section {
  flex: 1;
  min-width: 200px;
}

.cs-page-title {
  font-size: 1.5rem;
  font-weight: 700;
  color: var(--text-primary);
  margin: 0 0 0.25rem 0;
  letter-spacing: -0.025em;
}

.cs-page-subtitle {
  font-size: 0.875rem;
  color: var(--text-muted);
  margin: 0;
}

.cs-page-actions {
  display: flex;
  align-items: center;
  gap: 0.75rem;
  flex-wrap: wrap;
}

/* Live Indicator */
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

/* Stats Grid - Enhanced */
.cs-stats-grid {
  display: grid;
  grid-template-columns: repeat(4, 1fr);
  gap: 1rem;
  margin-bottom: 1.5rem;
}

@media (max-width: 1200px) {
  .cs-stats-grid {
    grid-template-columns: repeat(2, 1fr);
  }
}

@media (max-width: 600px) {
  .cs-stats-grid {
    grid-template-columns: 1fr;
  }
}

.cs-stat-card {
  background: var(--bg-elevated);
  border: 1px solid var(--border-primary);
  border-radius: var(--radius-xl);
  padding: 1.25rem;
  display: flex;
  align-items: center;
  gap: 1rem;
  transition: all var(--transition-normal);
}

.cs-stat-card:hover {
  box-shadow: var(--shadow-md);
  border-color: var(--border-secondary);
}

.cs-stat-icon {
  width: 48px;
  height: 48px;
  border-radius: var(--radius-lg);
  display: flex;
  align-items: center;
  justify-content: center;
  flex-shrink: 0;
}

.cs-stat-icon svg {
  width: 24px;
  height: 24px;
  color: #fff;
}

.cs-stat-icon.primary { background: var(--color-primary); }
.cs-stat-icon.success { background: var(--color-success); }
.cs-stat-icon.warning { background: var(--color-warning); }
.cs-stat-icon.info { background: #8b5cf6; }

.cs-stat-content {
  flex: 1;
  min-width: 0;
}

.cs-stat-value {
  font-size: 1.75rem;
  font-weight: 700;
  color: var(--text-primary);
  line-height: 1.2;
  margin-bottom: 0.125rem;
}

.cs-stat-label {
  font-size: 0.8125rem;
  color: var(--text-secondary);
  font-weight: 500;
  text-transform: uppercase;
  letter-spacing: 0.03em;
}

/* Toolbar */
.cs-toolbar {
  background: var(--bg-elevated);
  border: 1px solid var(--border-primary);
  border-radius: var(--radius-xl);
  padding: 1rem 1.25rem;
  margin-bottom: 1.5rem;
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 1rem;
  flex-wrap: wrap;
}

.cs-toolbar-section {
  display: flex;
  align-items: center;
  gap: 0.75rem;
  flex-wrap: wrap;
}

/* Date Range Buttons */
.cs-range-group {
  display: inline-flex;
  border: 1px solid var(--border-secondary);
  border-radius: var(--radius-lg);
  overflow: hidden;
  background: var(--bg-primary);
}

.cs-range-btn {
  padding: 0.5rem 0.875rem;
  font-size: 0.8125rem;
  font-weight: 600;
  border: none;
  background: transparent;
  color: var(--text-secondary);
  cursor: pointer;
  transition: all var(--transition-fast);
  font-family: inherit;
}

.cs-range-btn:not(:last-child) {
  border-right: 1px solid var(--border-secondary);
}

.cs-range-btn:hover:not(.active) {
  background: var(--bg-tertiary);
  color: var(--text-primary);
}

.cs-range-btn.active {
  background: var(--color-primary);
  color: #ffffff;
}

/* Filter Controls */
.cs-filter-select {
  padding: 0.5rem 2.25rem 0.5rem 0.75rem;
  border: 1px solid var(--border-secondary);
  border-radius: var(--radius-md);
  font-size: 0.8125rem;
  font-weight: 500;
  color: var(--text-primary);
  background-color: var(--bg-primary);
  background-image: url("data:image/svg+xml,%3csvg xmlns='http://www.w3.org/2000/svg' fill='none' viewBox='0 0 20 20'%3e%3cpath stroke='%236b7280' stroke-linecap='round' stroke-linejoin='round' stroke-width='1.5' d='m6 8 4 4 4-4'/%3e%3c/svg%3e");
  background-position: right 0.5rem center;
  background-repeat: no-repeat;
  background-size: 1rem;
  cursor: pointer;
  transition: all var(--transition-fast);
  appearance: none;
  -webkit-appearance: none;
  min-width: 130px;
}

.cs-filter-select:hover {
  border-color: var(--color-primary);
  background-color: var(--bg-elevated);
}

.cs-filter-select:focus {
  outline: none;
  border-color: var(--color-primary);
  box-shadow: 0 0 0 3px var(--color-primary-light);
}

.cs-search-box {
  position: relative;
  min-width: 240px;
}

.cs-search-box input {
  width: 100%;
  padding: 0.5rem 0.75rem 0.5rem 2.25rem;
  border: 1px solid var(--border-secondary);
  border-radius: var(--radius-md);
  font-size: 0.8125rem;
  color: var(--text-primary);
  background: var(--bg-primary);
  transition: all var(--transition-fast);
}

.cs-search-box input::placeholder {
  color: var(--text-muted);
}

.cs-search-box input:hover {
  border-color: var(--color-primary);
  background-color: var(--bg-elevated);
}

.cs-search-box input:focus {
  outline: none;
  border-color: var(--color-primary);
  box-shadow: 0 0 0 3px var(--color-primary-light);
}

.cs-search-icon {
  position: absolute;
  left: 0.75rem;
  top: 50%;
  transform: translateY(-50%);
  color: var(--text-muted);
  pointer-events: none;
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

/* Events Table Card */
.cs-events-card {
  background: var(--bg-elevated);
  border: 1px solid var(--border-primary);
  border-radius: var(--radius-xl);
  overflow: hidden;
}

.cs-events-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 1rem 1.25rem;
  border-bottom: 1px solid var(--border-primary);
  background: var(--bg-secondary);
}

.cs-events-title {
  font-size: 0.9375rem;
  font-weight: 600;
  color: var(--text-primary);
  margin: 0;
  display: flex;
  align-items: center;
  gap: 0.5rem;
}

.cs-events-count {
  font-size: 0.75rem;
  font-weight: 600;
  padding: 0.125rem 0.5rem;
  background: var(--bg-tertiary);
  color: var(--text-secondary);
  border-radius: var(--radius-2xl);
}

.cs-events-meta {
  font-size: 0.75rem;
  color: var(--text-muted);
  display: flex;
  align-items: center;
  gap: 0.375rem;
}

/* Table Styles */
.cs-table-container {
  overflow-x: auto;
}

.cs-table {
  width: 100%;
  border-collapse: collapse;
  font-size: 0.8125rem;
}

.cs-table th {
  text-align: left;
  padding: 0.75rem 1rem;
  font-size: 0.6875rem;
  font-weight: 600;
  text-transform: uppercase;
  letter-spacing: 0.05em;
  color: var(--text-muted);
  background: var(--bg-secondary);
  border-bottom: 1px solid var(--border-primary);
  white-space: nowrap;
}

.cs-table td {
  padding: 0.75rem 1rem;
  border-bottom: 1px solid var(--border-primary);
  vertical-align: middle;
}

.cs-table tbody tr {
  transition: background var(--transition-fast);
}

.cs-table tbody tr:hover {
  background: var(--bg-secondary);
}

.cs-table tbody tr:last-child td {
  border-bottom: none;
}

.cs-table tbody tr.is-bot {
  opacity: 0.6;
}

/* Cell Styles */
.cs-cell-time {
  display: flex;
  align-items: center;
  gap: 0.375rem;
  color: var(--text-secondary);
  white-space: nowrap;
}

.cs-cell-time svg {
  color: var(--text-muted);
  flex-shrink: 0;
}

.cs-cell-route {
  font-weight: 500;
  color: var(--text-primary);
}

.cs-cell-domain {
  color: var(--text-secondary);
}

.cs-cell-url {
  max-width: 220px;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
  color: var(--text-secondary);
}

.cs-cell-location {
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

.cs-cell-device {
  display: flex;
  align-items: center;
  gap: 0.375rem;
}

.cs-cell-device svg {
  color: var(--text-muted);
  flex-shrink: 0;
}

.cs-cell-browser {
  line-height: 1.4;
}

.cs-browser-name {
  color: var(--text-primary);
}

.cs-browser-os {
  color: var(--text-muted);
  font-size: 0.75rem;
}

.cs-cell-ip {
  font-family: var(--font-family-mono);
  font-size: 0.75rem;
  color: var(--text-secondary);
}

/* Badges */
.cs-badges {
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
  letter-spacing: 0.02em;
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

/* Empty State */
.cs-empty {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  padding: 4rem 2rem;
  text-align: center;
}

.cs-empty-icon {
  width: 64px;
  height: 64px;
  border-radius: 50%;
  background: var(--bg-tertiary);
  display: flex;
  align-items: center;
  justify-content: center;
  margin-bottom: 1.25rem;
}

.cs-empty-icon svg {
  width: 32px;
  height: 32px;
  color: var(--text-muted);
}

.cs-empty-title {
  font-size: 1rem;
  font-weight: 600;
  color: var(--text-primary);
  margin: 0 0 0.5rem 0;
}

.cs-empty-description {
  font-size: 0.875rem;
  color: var(--text-muted);
  max-width: 320px;
  margin: 0;
}

/* Error State */
.cs-error {
  background: var(--bg-elevated);
  border: 1px solid var(--color-error);
  border-radius: var(--radius-xl);
  padding: 2rem;
  text-align: center;
  margin: 2rem 0;
}

.cs-error-title {
  font-size: 1.125rem;
  font-weight: 600;
  color: var(--color-error);
  margin: 0 0 0.5rem 0;
}

.cs-error-message {
  font-size: 0.875rem;
  color: var(--text-secondary);
  margin: 0 0 1rem 0;
}

/* Responsive */
@media (max-width: 768px) {
  .cs-page-header {
    flex-direction: column;
    gap: 1rem;
  }

  .cs-page-actions {
    width: 100%;
    justify-content: space-between;
  }

  .cs-toolbar {
    flex-direction: column;
    align-items: stretch;
  }

  .cs-toolbar-section {
    width: 100%;
    justify-content: space-between;
  }

  .cs-search-box {
    min-width: 100%;
  }

  .cs-filter-select {
    flex: 1;
  }
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
  }, [getDateRange]);

  const refreshData = useCallback(async () => {
    if (!isLive) return;

    try {
      const { startDate, endDate } = getDateRange();
      const params = {
        startDate,
        endDate,
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
  }, [isLive, getDateRange]);

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
      <div className="container" style={{ paddingTop: '2rem' }}>
        <div className="cs-error">
          <h3 className="cs-error-title">Error Loading Clickstream</h3>
          <p className="cs-error-message">{error}</p>
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
      <div className="container" style={{ paddingTop: '1.5rem', paddingBottom: '2rem' }}>
        {/* Page Header */}
        <div className="cs-page-header">
          <div className="cs-page-title-section">
            <h1 className="cs-page-title">Clickstream</h1>
            <p className="cs-page-subtitle">Real-time monitoring of link clicks and visitor activity</p>
          </div>
          <div className="cs-page-actions">
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
        <div className="cs-stats-grid">
          <div className="cs-stat-card">
            <div className="cs-stat-icon primary">
              <MousePointerClick />
            </div>
            <div className="cs-stat-content">
              <div className="cs-stat-value">{stats.totalClicks.toLocaleString()}</div>
              <div className="cs-stat-label">Total Clicks</div>
            </div>
          </div>

          <div className="cs-stat-card">
            <div className="cs-stat-icon success">
              <Users />
            </div>
            <div className="cs-stat-content">
              <div className="cs-stat-value">{stats.uniqueClicks.toLocaleString()}</div>
              <div className="cs-stat-label">Unique Visitors</div>
            </div>
          </div>

          <div className="cs-stat-card">
            <div className="cs-stat-icon warning">
              <Bot />
            </div>
            <div className="cs-stat-content">
              <div className="cs-stat-value">{stats.botClicks.toLocaleString()}</div>
              <div className="cs-stat-label">Bot Clicks</div>
            </div>
          </div>

          <div className="cs-stat-card">
            <div className="cs-stat-icon info">
              <Activity />
            </div>
            <div className="cs-stat-content">
              <div className="cs-stat-value">{filteredEvents.length.toLocaleString()}</div>
              <div className="cs-stat-label">Showing</div>
            </div>
          </div>
        </div>

        {/* Toolbar */}
        <div className="cs-toolbar">
          <div className="cs-toolbar-section">
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
          </div>

          <div className="cs-toolbar-section">
            <div className="cs-search-box">
              <Search size={16} className="cs-search-icon" />
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
        <div className="cs-events-card">
          <div className="cs-events-header">
            <h3 className="cs-events-title">
              Click Events
              <span className="cs-events-count">{filteredEvents.length}</span>
            </h3>
            <div className="cs-events-meta">
              <Clock size={12} />
              {isLive ? 'Auto-refreshing every 5s' : 'Updates paused'}
            </div>
          </div>

          <div className="cs-table-container">
            {filteredEvents.length > 0 ? (
              <table className="cs-table">
                <thead>
                  <tr>
                    <th>Time</th>
                    <th>Route</th>
                    <th>Domain</th>
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
                    <tr key={event.id} className={event.isBot ? 'is-bot' : ''}>
                      <td>
                        <div className="cs-cell-time">
                          <Clock size={14} />
                          {formatTimestamp(event.timestamp)}
                        </div>
                      </td>
                      <td>
                        <span className="cs-cell-route">{event.routeName || '—'}</span>
                      </td>
                      <td>
                        <span className="cs-cell-domain">{event.routeDomainName || '—'}</span>
                      </td>
                      <td>
                        <div className="cs-cell-url" title={event.url}>
                          {event.url || '—'}
                        </div>
                      </td>
                      <td>
                        <div className="cs-cell-location">
                          <div className="cs-location-city">{event.city}</div>
                          <div className="cs-location-country">{event.country}</div>
                        </div>
                      </td>
                      <td>
                        <div className="cs-cell-device">
                          {getDeviceIcon(event.device)}
                          <span>{event.device}</span>
                        </div>
                      </td>
                      <td>
                        <div className="cs-cell-browser">
                          <div className="cs-browser-name">{event.browser}</div>
                          <div className="cs-browser-os">{event.os}</div>
                        </div>
                      </td>
                      <td>
                        <span className="cs-cell-ip">{event.ip}</span>
                      </td>
                      <td>
                        <div className="cs-badges">
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
            ) : (
              <div className="cs-empty">
                <div className="cs-empty-icon">
                  <Activity />
                </div>
                <h3 className="cs-empty-title">No events found</h3>
                <p className="cs-empty-description">
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
    </>
  );
};

export default Clickstream;
