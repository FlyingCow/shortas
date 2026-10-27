import React, { useState, useEffect, useCallback } from 'react';
import { Link } from 'react-router-dom';
import {
  AreaChart,
  Area,
  XAxis,
  YAxis,
  Tooltip,
  ResponsiveContainer,
  PieChart,
  Pie,
  Cell,
  CartesianGrid
} from 'recharts';
import {
  Users,
  MousePointerClick,
  Globe,
  Activity,
  Bot,
  Target,
  RefreshCw,
  BarChart3,
  Monitor,
  AlertTriangle,
  CheckCircle2,
  Ban,
  ShieldAlert,
  ArrowUp,
  ArrowDown,
} from 'lucide-react';
import {
  apiService,
  ClickAnalytics,
  TrafficTypeStatsDto,
  RoutePerformanceDto,
  BlockedRouteSummaryDto,
  DomainSummaryDto,
} from '../services/api';
import { getCountryDisplayName } from '../utils/countries';
import { useWorkspace } from '../contexts/WorkspaceContext';
import LoadingSpinner from './LoadingSpinner';
import WorldMap from './WorldMap';
import './DesignSystem.css';

// Dashboard-specific widgets not covered by the shared DesignSystem.css
// primitives (tooltip, pie legend, bar list, attention list, refresh badge).
const dashboardStyles = `
.db-refreshing {
  position: fixed;
  top: 70px;
  right: 20px;
  background: var(--bg-elevated);
  border: 1px solid var(--border-primary);
  padding: 0.5rem 1rem;
  border-radius: var(--radius-lg);
  font-size: 0.8125rem;
  color: var(--text-secondary);
  box-shadow: var(--shadow-lg);
  z-index: 100;
  display: flex;
  align-items: center;
  gap: 0.5rem;
}

.db-tooltip {
  background: var(--bg-elevated);
  border: 1px solid var(--border-primary);
  border-radius: var(--radius-md);
  padding: 0.75rem;
  box-shadow: var(--shadow-lg);
}

.db-tooltip-label {
  font-size: 0.75rem;
  font-weight: 600;
  color: var(--text-primary);
  margin-bottom: 0.5rem;
  padding-bottom: 0.5rem;
  border-bottom: 1px solid var(--border-primary);
}

.db-tooltip-row {
  display: flex;
  align-items: center;
  gap: 0.5rem;
  padding: 0.25rem 0;
  font-size: 0.8125rem;
}

.db-tooltip-dot {
  width: 8px;
  height: 8px;
  border-radius: 50%;
  flex-shrink: 0;
}

.db-tooltip-name {
  color: var(--text-secondary);
  flex: 1;
}

.db-tooltip-value {
  font-weight: 600;
  color: var(--text-primary);
}

.db-pie-legend {
  display: flex;
  flex-wrap: wrap;
  gap: 0.75rem;
  justify-content: center;
  padding: 0.75rem 1rem;
  border-top: 1px solid var(--border-primary);
  background: var(--bg-secondary);
  margin: var(--space-lg) calc(-1 * var(--space-lg)) calc(-1 * var(--space-lg));
}

.db-pie-legend-item {
  display: flex;
  align-items: center;
  gap: 0.375rem;
  font-size: 0.75rem;
}

.db-pie-legend-dot {
  width: 8px;
  height: 8px;
  border-radius: 50%;
  flex-shrink: 0;
}

.db-pie-legend-label {
  color: var(--text-secondary);
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
  max-width: 100px;
}

.db-country-list {
  display: flex;
  flex-direction: column;
  gap: 0.5rem;
}

.db-country-row {
  display: flex;
  align-items: center;
  gap: 0.75rem;
  padding: 0.5rem 0;
}

.db-country-rank {
  width: 20px;
  height: 20px;
  border-radius: 50%;
  background: var(--bg-tertiary);
  display: flex;
  align-items: center;
  justify-content: center;
  font-size: 0.6875rem;
  font-weight: 600;
  color: var(--text-muted);
  flex-shrink: 0;
}

.db-country-name {
  flex: 1;
  font-size: 0.8125rem;
  font-weight: 500;
  color: var(--text-primary);
  min-width: 0;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.db-country-bar-wrap {
  flex: 1;
  max-width: 120px;
}

.db-country-bar {
  height: 6px;
  background: var(--bg-tertiary);
  border-radius: 3px;
  overflow: hidden;
}

.db-country-bar-fill {
  height: 100%;
  background: var(--color-primary);
  border-radius: 3px;
  transition: width var(--transition-normal);
}

.db-country-value {
  font-size: 0.8125rem;
  font-weight: 600;
  color: var(--text-primary);
  min-width: 50px;
  text-align: right;
}

.db-country-pct {
  font-size: 0.75rem;
  color: var(--text-muted);
  min-width: 40px;
  text-align: right;
}

.db-attention-list {
  display: flex;
  flex-direction: column;
  gap: 0.25rem;
}

.db-attention-row {
  display: flex;
  align-items: center;
  gap: 0.75rem;
  padding: 0.625rem 0.5rem;
  border-radius: var(--radius-md);
  text-decoration: none;
  color: inherit;
  transition: background var(--transition-fast);
}

.db-attention-row:hover {
  background: var(--bg-tertiary);
}

.db-attention-label {
  flex: 1;
  min-width: 0;
  font-size: 0.8125rem;
  font-weight: 500;
  color: var(--text-primary);
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.db-attention-arrow {
  color: var(--text-muted);
  flex-shrink: 0;
}

.db-attention-more {
  padding: 0.5rem 0.5rem 0;
  font-size: 0.75rem;
  color: var(--text-muted);
  display: flex;
  gap: 1rem;
}

@keyframes db-spin {
  from { transform: rotate(0deg); }
  to { transform: rotate(360deg); }
}

.db-icon-spin {
  animation: db-spin 1s linear infinite;
}
`;

interface PeriodTotals {
  totalClicks: number;
  uniqueClicks: number;
  botClicks: number;
  humanClicks: number;
}

interface DashboardStats extends PeriodTotals {
  totalRoutes: number;
  activeRoutes: number;
}

const CHART_COLORS = [
  'var(--primary-500)',
  'var(--success-500)',
  'var(--warning-500)',
  'var(--error-500)',
  'var(--primary-400)',
  'var(--success-400)',
  'var(--warning-400)',
  'var(--primary-600)',
];

const TRAFFIC_COLORS = ['var(--success-500)', 'var(--error-500)'];

const formatAxisNumber = (value: number): string => {
  if (value >= 1000000) return `${(value / 1000000).toFixed(1)}M`;
  if (value >= 1000) return `${(value / 1000).toFixed(0)}k`;
  return value.toString();
};

type ChangeModifier = 'positive' | 'negative' | 'neutral';

const formatChange = (
  current: number,
  previous: number,
  periodLabel: string,
  invert = false
): { text: string; mod: ChangeModifier } => {
  if (previous === 0) {
    if (current === 0) return { text: 'No activity yet', mod: 'neutral' };
    return { text: `New vs prior ${periodLabel}`, mod: invert ? 'negative' : 'positive' };
  }
  const pct = ((current - previous) / previous) * 100;
  if (Math.abs(pct) < 0.5) return { text: `Flat vs prior ${periodLabel}`, mod: 'neutral' };
  const up = pct > 0;
  const mod: ChangeModifier = invert ? (up ? 'negative' : 'positive') : (up ? 'positive' : 'negative');
  return { text: `${up ? '+' : ''}${pct.toFixed(0)}% vs prior ${periodLabel}`, mod };
};

const ChangeIndicator: React.FC<{ current: number; previous: number; periodLabel: string; invert?: boolean }> = ({
  current,
  previous,
  periodLabel,
  invert,
}) => {
  const { text, mod } = formatChange(current, previous, periodLabel, invert);
  return (
    <div className={`stats-change ${mod}`}>
      {mod === 'positive' && <ArrowUp size={12} />}
      {mod === 'negative' && <ArrowDown size={12} />}
      {text}
    </div>
  );
};

const CustomTooltip = ({ active, payload, label }: any) => {
  if (!active || !payload?.length) return null;
  return (
    <div className="db-tooltip">
      {label != null && label !== '' && (
        <div className="db-tooltip-label">{label}</div>
      )}
      {payload.map((entry: any, i: number) => {
        const percent = entry.payload?.percent;
        return (
          <div key={i} className="db-tooltip-row">
            <span className="db-tooltip-dot" style={{ backgroundColor: entry.color || entry.fill }} />
            <span className="db-tooltip-name">{entry.name}</span>
            <span className="db-tooltip-value">
              {typeof entry.value === 'number' ? entry.value.toLocaleString() : entry.value}
              {percent != null && ` (${(percent * 100).toFixed(1)}%)`}
            </span>
          </div>
        );
      })}
    </div>
  );
};

const PieLegend: React.FC<{ items: { name: string; color: string }[] }> = ({ items }) => (
  <div className="db-pie-legend">
    {items.slice(0, 6).map((item, i) => (
      <div key={i} className="db-pie-legend-item">
        <span className="db-pie-legend-dot" style={{ backgroundColor: item.color }} />
        <span className="db-pie-legend-label">{item.name}</span>
      </div>
    ))}
  </div>
);

const dateRanges = [
  { value: '24h', label: '24H', periodLabel: '24h' },
  { value: '7d', label: '7D', periodLabel: '7d' },
  { value: '30d', label: '30D', periodLabel: '30d' },
  { value: '90d', label: '90D', periodLabel: '90d' },
];

const Dashboard: React.FC = () => {
  const { currentWorkspaceId } = useWorkspace();
  const [analytics, setAnalytics] = useState<ClickAnalytics | null>(null);
  const [stats, setStats] = useState<DashboardStats | null>(null);
  const [prevTotals, setPrevTotals] = useState<PeriodTotals | null>(null);
  const [trafficTypeStats, setTrafficTypeStats] = useState<TrafficTypeStatsDto[]>([]);
  const [routePerformance, setRoutePerformance] = useState<RoutePerformanceDto[]>([]);
  const [blockedRoutes, setBlockedRoutes] = useState<BlockedRouteSummaryDto[]>([]);
  const [blockedRouteCount, setBlockedRouteCount] = useState(0);
  const [attentionDomains, setAttentionDomains] = useState<DomainSummaryDto[]>([]);
  const [unverifiedDomainCount, setUnverifiedDomainCount] = useState(0);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const [dateRange, setDateRange] = useState('7d');

  const periodLabel = dateRanges.find((r) => r.value === dateRange)?.periodLabel || dateRange;

  const fetchDashboardData = useCallback(async () => {
    try {
      setLoading(true);
      setError(null);

      const endDate = new Date();
      const startDate = new Date();

      switch (dateRange) {
        case '24h': startDate.setHours(startDate.getHours() - 24); break;
        case '7d': startDate.setDate(startDate.getDate() - 7); break;
        case '30d': startDate.setDate(startDate.getDate() - 30); break;
        case '90d': startDate.setDate(startDate.getDate() - 90); break;
      }

      const periodMs = endDate.getTime() - startDate.getTime();
      const prevEndDate = new Date(startDate.getTime());
      const prevStartDate = new Date(startDate.getTime() - periodMs);

      const fromDate = startDate.toISOString().split('T')[0];
      const toDate = endDate.toISOString().split('T')[0];
      const prevFromDate = prevStartDate.toISOString().split('T')[0];
      const prevToDate = prevEndDate.toISOString().split('T')[0];
      const fromHour = startDate.toISOString().replace('T', ' ').substring(0, 13);
      const toHour = endDate.toISOString().replace('T', ' ').substring(0, 13);

      const workspaceStatsPromise = currentWorkspaceId
        ? apiService.workspaces.getStats(currentWorkspaceId)
        : Promise.resolve({
            workspaceId: '',
            routeCount: 0,
            activeRouteCount: 0,
            blockedRouteCount: 0,
            memberCount: 0,
            blockedRoutes: [],
          });

      const [
        dailyStats,
        geographicStats,
        deviceStats,
        browserStats,
        trafficStats,
        routePerf,
        workspaceStats,
        domainsAttention,
        prevDailyStats,
      ] = await Promise.all([
        apiService.clickstream.getDailyStats({ fromDate, toDate, workspaceId: currentWorkspaceId ?? undefined }),
        apiService.clickstream.getGeographicStats({ fromDate, toDate, workspaceId: currentWorkspaceId ?? undefined }),
        apiService.clickstream.getDeviceStats({ fromDate, toDate, workspaceId: currentWorkspaceId ?? undefined }),
        apiService.clickstream.getBrowserStats({ fromDate, toDate, workspaceId: currentWorkspaceId ?? undefined }),
        apiService.clickstream.getTrafficTypeStats({ fromHour, toHour, workspaceId: currentWorkspaceId ?? undefined }),
        apiService.clickstream.getRoutePerformance({ fromDate, toDate, limit: 10, workspaceId: currentWorkspaceId ?? undefined }),
        workspaceStatsPromise,
        apiService.domains.getAttention(),
        apiService.clickstream.getDailyStats({ fromDate: prevFromDate, toDate: prevToDate, workspaceId: currentWorkspaceId ?? undefined }),
      ]);

      const sumTotals = (rows: { total_clicks: number; unique_clicks: number; bot_clicks: number; human_clicks: number }[]): PeriodTotals =>
        rows.reduce((acc, stat) => ({
          totalClicks: acc.totalClicks + stat.total_clicks,
          uniqueClicks: acc.uniqueClicks + stat.unique_clicks,
          botClicks: acc.botClicks + stat.bot_clicks,
          humanClicks: acc.humanClicks + stat.human_clicks,
        }), { totalClicks: 0, uniqueClicks: 0, botClicks: 0, humanClicks: 0 });

      const totals = sumTotals(dailyStats);

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
          country: getCountryDisplayName(stat.country) || stat.country,
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
      setPrevTotals(sumTotals(prevDailyStats));
      setBlockedRoutes(workspaceStats.blockedRoutes);
      setBlockedRouteCount(workspaceStats.blockedRouteCount);
      setAttentionDomains(domainsAttention.domains);
      setUnverifiedDomainCount(domainsAttention.unverifiedCount);

      setStats({
        totalClicks: totals.totalClicks,
        uniqueClicks: totals.uniqueClicks,
        botClicks: totals.botClicks,
        humanClicks: totals.humanClicks,
        totalRoutes: workspaceStats.routeCount,
        activeRoutes: workspaceStats.activeRouteCount,
      });
    } catch (err: any) {
      console.error('Failed to fetch dashboard data:', err);
      const errorMessage = err.response?.data?.message || err.message || 'Failed to load dashboard data. Please try again.';
      setError(errorMessage);
    } finally {
      setLoading(false);
    }
  }, [dateRange, currentWorkspaceId]);

  useEffect(() => {
    fetchDashboardData();
  }, [fetchDashboardData]);

  useEffect(() => {
    const isInputFocused = () => {
      const el = document.activeElement as HTMLElement | null;
      if (!el || el === document.body) return false;
      const tag = el.tagName;
      return tag === 'INPUT' || tag === 'TEXTAREA' || tag === 'SELECT' || el.isContentEditable;
    };
    const onKey = (e: KeyboardEvent) => {
      if (e.altKey && e.key.toLowerCase() === 'r' && !isInputFocused()) {
        fetchDashboardData();
        e.preventDefault();
      }
    };
    document.addEventListener('keydown', onKey);
    return () => document.removeEventListener('keydown', onKey);
  }, [fetchDashboardData]);

  if (loading && !analytics) {
    return <LoadingSpinner message="Loading dashboard..." />;
  }

  if (error && !analytics) {
    return (
      <div className="container">
        <style>{dashboardStyles}</style>
        <div className="card" style={{ padding: '2rem', textAlign: 'center', borderColor: 'var(--color-error)' }}>
          <h3 className="status-error" style={{ margin: '0 0 0.5rem 0' }}>Error Loading Dashboard</h3>
          <p style={{ color: 'var(--text-secondary)', margin: '0 0 1rem 0' }}>{error}</p>
          <button className="btn btn-primary" onClick={fetchDashboardData}>
            <RefreshCw size={16} />
            Retry
          </button>
        </div>
      </div>
    );
  }

  const humanRate = stats && stats.totalClicks > 0
    ? ((stats.humanClicks / stats.totalClicks) * 100).toFixed(0)
    : '0';
  const botRate = stats && stats.totalClicks > 0
    ? ((stats.botClicks / stats.totalClicks) * 100).toFixed(0)
    : '0';
  const uniqueRate = stats && stats.totalClicks > 0
    ? ((stats.uniqueClicks / stats.totalClicks) * 100).toFixed(1)
    : '0';

  const trafficData = (() => {
    const humanStat = trafficTypeStats.find(stat => !stat.is_bot);
    const botStat = trafficTypeStats.find(stat => stat.is_bot);
    return [
      { name: 'Human', value: humanStat?.total_clicks || 0 },
      { name: 'Bot', value: botStat?.total_clicks || 0 },
    ];
  })();

  const needsAttentionCount = blockedRouteCount + unverifiedDomainCount;

  return (
    <>
      <style>{dashboardStyles}</style>
      <div className="container">
        {/* Refreshing indicator */}
        {loading && analytics && (
          <div className="db-refreshing">
            <RefreshCw size={14} className="db-icon-spin" />
            Updating...
          </div>
        )}

        {/* Page Header */}
        <div className="page-header">
          <div className="page-title-section">
            <h1 className="page-title">Dashboard</h1>
            <p className="page-subtitle">Overview of your link performance and analytics</p>
          </div>
          <div className="page-actions">
            <button
              className="btn btn-outline btn-sm"
              onClick={fetchDashboardData}
              disabled={loading}
            >
              <RefreshCw size={14} className={loading ? 'db-icon-spin' : ''} />
              Refresh
            </button>
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
          </div>
        </div>

        {/* Stats Grid */}
        <div className="stats-grid">
          <div className="stats-card">
            <div className="stats-icon primary">
              <MousePointerClick />
            </div>
            <div className="stats-content">
              <div className="stats-value">{stats?.totalClicks.toLocaleString() || 0}</div>
              <div className="stats-label">Total Clicks</div>
              {stats && prevTotals && (
                <ChangeIndicator current={stats.totalClicks} previous={prevTotals.totalClicks} periodLabel={periodLabel} />
              )}
            </div>
          </div>

          <div className="stats-card">
            <div className="stats-icon success">
              <Users />
            </div>
            <div className="stats-content">
              <div className="stats-value">{stats?.uniqueClicks.toLocaleString() || 0}</div>
              <div className="stats-label">Unique Clicks</div>
              {stats && prevTotals && (
                <ChangeIndicator current={stats.uniqueClicks} previous={prevTotals.uniqueClicks} periodLabel={periodLabel} />
              )}
            </div>
          </div>

          <div className="stats-card">
            <div className="stats-icon info">
              <Activity />
            </div>
            <div className="stats-content">
              <div className="stats-value">{stats?.humanClicks.toLocaleString() || 0}</div>
              <div className="stats-label">Human ({humanRate}%)</div>
              {stats && prevTotals && (
                <ChangeIndicator current={stats.humanClicks} previous={prevTotals.humanClicks} periodLabel={periodLabel} />
              )}
            </div>
          </div>

          <div className="stats-card">
            <div className="stats-icon error">
              <Bot />
            </div>
            <div className="stats-content">
              <div className="stats-value">{stats?.botClicks.toLocaleString() || 0}</div>
              <div className="stats-label">Bot ({botRate}%)</div>
              {stats && prevTotals && (
                <ChangeIndicator current={stats.botClicks} previous={prevTotals.botClicks} periodLabel={periodLabel} invert />
              )}
            </div>
          </div>

          <div className="stats-card">
            <div className="stats-icon warning">
              <Target />
            </div>
            <div className="stats-content">
              <div className="stats-value">{uniqueRate}%</div>
              <div className="stats-label">Unique Rate</div>
              <div className="stats-change neutral">Share of clicks that are unique</div>
            </div>
          </div>

          <div className="stats-card">
            <div className="stats-icon primary">
              <Globe />
            </div>
            <div className="stats-content">
              <div className="stats-value">{stats?.activeRoutes || 0}</div>
              <div className="stats-label">Active Routes</div>
              <div className="stats-change neutral">{stats?.totalRoutes || 0} total routes</div>
            </div>
          </div>
        </div>

        {/* Needs Attention */}
        <div className="section">
          <div className="chart-card">
            <div className="chart-header">
              <h3 className="chart-title">
                <AlertTriangle size={16} />
                Needs Attention
              </h3>
              <p className="chart-desc">Blocked routes and domains that aren't fully verified</p>
            </div>
            {needsAttentionCount === 0 ? (
              <div className="empty-state">
                <div className="empty-state-icon">
                  <CheckCircle2 />
                </div>
                <div className="empty-state-title">All clear</div>
                <div className="empty-state-description">No blocked routes or unverified domains right now.</div>
              </div>
            ) : (
              <div className="db-attention-list">
                {blockedRoutes.map((route) => (
                  <Link key={route.id} to="/routes" className="db-attention-row">
                    <span className="badge badge-error">
                      <Ban size={12} style={{ marginRight: 4 }} />
                      Blocked
                    </span>
                    <span className="db-attention-label">{route.link}</span>
                    <ArrowUp size={14} className="db-attention-arrow" style={{ transform: 'rotate(45deg)' }} />
                  </Link>
                ))}
                {attentionDomains.map((domain) => (
                  <Link key={domain.id} to="/domains" className="db-attention-row">
                    <span className={`badge ${domain.verificationStatus === 'Failed' ? 'badge-error' : 'badge-warning'}`}>
                      <ShieldAlert size={12} style={{ marginRight: 4 }} />
                      {domain.verificationStatus}
                    </span>
                    <span className="db-attention-label">{domain.name}</span>
                    <ArrowUp size={14} className="db-attention-arrow" style={{ transform: 'rotate(45deg)' }} />
                  </Link>
                ))}
                {(blockedRouteCount > blockedRoutes.length || unverifiedDomainCount > attentionDomains.length) && (
                  <div className="db-attention-more">
                    {blockedRouteCount > blockedRoutes.length && (
                      <span>+{blockedRouteCount - blockedRoutes.length} more blocked route(s)</span>
                    )}
                    {unverifiedDomainCount > attentionDomains.length && (
                      <span>+{unverifiedDomainCount - attentionDomains.length} more domain(s)</span>
                    )}
                  </div>
                )}
              </div>
            )}
          </div>
        </div>

        {/* Clicks Over Time - Full Width */}
        <div className="section">
          <div className="chart-card">
            <div className="chart-header">
              <h3 className="chart-title">
                <BarChart3 size={16} />
                Clicks Over Time
              </h3>
              <p className="chart-desc">Daily click trends for the selected period</p>
            </div>
            <div className="chart-body" style={{ height: '300px' }}>
              <ResponsiveContainer width="100%" height="100%">
                <AreaChart data={analytics?.clicks_by_date || []}>
                  <defs>
                    <linearGradient id="clicksGradient" x1="0" y1="0" x2="0" y2="1">
                      <stop offset="0%" stopColor="#3b82f6" stopOpacity={0.15} />
                      <stop offset="100%" stopColor="#3b82f6" stopOpacity={0} />
                    </linearGradient>
                  </defs>
                  <CartesianGrid strokeDasharray="3 3" stroke="var(--border-secondary)" vertical={false} />
                  <XAxis
                    dataKey="date"
                    stroke="var(--text-muted)"
                    fontSize={11}
                    tickLine={false}
                    axisLine={false}
                  />
                  <YAxis
                    stroke="var(--text-muted)"
                    fontSize={11}
                    tickLine={false}
                    axisLine={false}
                    width={45}
                    tickFormatter={formatAxisNumber}
                  />
                  <Tooltip content={<CustomTooltip />} />
                  <Area
                    type="monotone"
                    dataKey="clicks"
                    name="Clicks"
                    stroke="var(--primary-500)"
                    fill="url(#clicksGradient)"
                    strokeWidth={2}
                    dot={false}
                    activeDot={{ r: 4, fill: 'var(--primary-500)', stroke: 'var(--bg-primary)', strokeWidth: 2 }}
                  />
                </AreaChart>
              </ResponsiveContainer>
            </div>
          </div>
        </div>

        {/* Geographic Section */}
        <div className="section-row two-cols">
          <div className="chart-card">
            <div className="chart-header">
              <h3 className="chart-title">
                <Globe size={16} />
                Geographic Distribution
              </h3>
              <p className="chart-desc">Hover to see details</p>
            </div>
            <div className="chart-body" style={{ height: '300px' }}>
              <WorldMap
                data={(analytics?.clicks_by_country || []).map(country => {
                  const totalClicks = stats?.totalClicks || 1;
                  return {
                    name: country.country,
                    clicks: country.clicks,
                    percentage: parseFloat(((country.clicks / totalClicks) * 100).toFixed(2))
                  };
                })}
                height={300}
              />
            </div>
          </div>

          <div className="chart-card">
            <div className="chart-header">
              <h3 className="chart-title">
                <Globe size={16} />
                Top Countries
              </h3>
              <p className="chart-desc">By click volume</p>
            </div>
            <div className="chart-body">
              {(analytics?.clicks_by_country || []).length > 0 ? (
                <div className="db-country-list">
                  {(analytics?.clicks_by_country || []).map((country: any, index: number) => {
                    const totalClicks = stats?.totalClicks || 1;
                    const percentage = Math.round((country.clicks / totalClicks) * 100);
                    return (
                      <div key={country.country} className="db-country-row">
                        <span className="db-country-rank">{index + 1}</span>
                        <span className="db-country-name">{country.country}</span>
                        <div className="db-country-bar-wrap">
                          <div className="db-country-bar">
                            <div
                              className="db-country-bar-fill"
                              style={{ width: `${percentage}%` }}
                            />
                          </div>
                        </div>
                        <span className="db-country-value">{country.clicks.toLocaleString()}</span>
                        <span className="db-country-pct">{percentage}%</span>
                      </div>
                    );
                  })}
                </div>
              ) : (
                <div className="empty-state">
                  <div className="empty-state-icon">
                    <Globe />
                  </div>
                  <p>No geographic data available</p>
                </div>
              )}
            </div>
          </div>
        </div>

        {/* Distribution Section */}
        <div className="section-row three-cols">
          {/* Device Distribution */}
          <div className="chart-card">
            <div className="chart-header">
              <h3 className="chart-title">
                <Monitor size={16} />
                Devices
              </h3>
              <p className="chart-desc">By device type</p>
            </div>
            <div className="chart-body" style={{ height: '200px' }}>
              {(analytics?.clicks_by_device || []).length > 0 ? (
                <>
                  <ResponsiveContainer width="100%" height="100%">
                    <PieChart>
                      <Pie
                        data={analytics?.clicks_by_device || []}
                        cx="50%"
                        cy="50%"
                        outerRadius={70}
                        innerRadius={45}
                        dataKey="clicks"
                        nameKey="device"
                        paddingAngle={2}
                        stroke="var(--bg-primary)"
                        strokeWidth={2}
                      >
                        {(analytics?.clicks_by_device || []).map((_, index) => (
                          <Cell key={`cell-${index}`} fill={CHART_COLORS[index % CHART_COLORS.length]} />
                        ))}
                      </Pie>
                      <Tooltip content={<CustomTooltip />} />
                    </PieChart>
                  </ResponsiveContainer>
                  <PieLegend
                    items={(analytics?.clicks_by_device || []).map((entry, i) => ({
                      name: entry.device,
                      color: CHART_COLORS[i % CHART_COLORS.length],
                    }))}
                  />
                </>
              ) : (
                <div className="empty-state">
                  <div className="empty-state-icon">
                    <Monitor />
                  </div>
                  <p>No device data</p>
                </div>
              )}
            </div>
          </div>

          {/* Browser Distribution */}
          <div className="chart-card">
            <div className="chart-header">
              <h3 className="chart-title">
                <Globe size={16} />
                Browsers
              </h3>
              <p className="chart-desc">By browser</p>
            </div>
            <div className="chart-body" style={{ height: '200px' }}>
              {(analytics?.clicks_by_browser || []).length > 0 ? (
                <>
                  <ResponsiveContainer width="100%" height="100%">
                    <PieChart>
                      <Pie
                        data={analytics?.clicks_by_browser || []}
                        cx="50%"
                        cy="50%"
                        outerRadius={70}
                        innerRadius={45}
                        dataKey="clicks"
                        nameKey="browser"
                        paddingAngle={2}
                        stroke="var(--bg-primary)"
                        strokeWidth={2}
                      >
                        {(analytics?.clicks_by_browser || []).map((_, index) => (
                          <Cell key={`cell-${index}`} fill={CHART_COLORS[index % CHART_COLORS.length]} />
                        ))}
                      </Pie>
                      <Tooltip content={<CustomTooltip />} />
                    </PieChart>
                  </ResponsiveContainer>
                  <PieLegend
                    items={(analytics?.clicks_by_browser || []).map((entry, i) => ({
                      name: entry.browser,
                      color: CHART_COLORS[i % CHART_COLORS.length],
                    }))}
                  />
                </>
              ) : (
                <div className="empty-state">
                  <div className="empty-state-icon">
                    <Globe />
                  </div>
                  <p>No browser data</p>
                </div>
              )}
            </div>
          </div>

          {/* Traffic Type */}
          <div className="chart-card">
            <div className="chart-header">
              <h3 className="chart-title">
                <Activity size={16} />
                Traffic Type
              </h3>
              <p className="chart-desc">Bot vs Human</p>
            </div>
            <div className="chart-body" style={{ height: '200px' }}>
              {trafficTypeStats.length > 0 ? (
                <ResponsiveContainer width="100%" height="100%">
                  <PieChart>
                    <Pie
                      data={trafficData}
                      cx="50%"
                      cy="50%"
                      labelLine={false}
                      label={({ name, percent }) => percent > 0.05 ? `${name} ${(percent * 100).toFixed(0)}%` : ''}
                      outerRadius={70}
                      innerRadius={45}
                      dataKey="value"
                      paddingAngle={3}
                      stroke="var(--bg-primary)"
                      strokeWidth={2}
                    >
                      <Cell key="cell-0" fill={TRAFFIC_COLORS[0]} />
                      <Cell key="cell-1" fill={TRAFFIC_COLORS[1]} />
                    </Pie>
                    <Tooltip content={<CustomTooltip />} />
                  </PieChart>
                </ResponsiveContainer>
              ) : (
                <div className="empty-state">
                  <div className="empty-state-icon">
                    <Activity />
                  </div>
                  <p>No data available</p>
                </div>
              )}
            </div>
            {trafficTypeStats.length > 0 && (
              <PieLegend
                items={trafficData.map((entry, i) => ({
                  name: entry.name,
                  color: TRAFFIC_COLORS[i],
                }))}
              />
            )}
          </div>
        </div>

        {/* Top Performing Routes */}
        <div className="section">
          <div className="chart-card">
            <div className="chart-header">
              <h3 className="chart-title">
                <Target size={16} />
                Top Performing Routes
              </h3>
              <p className="chart-desc">Routes ranked by total clicks</p>
            </div>
            <div className="table-container">
              <div className="table-wrapper">
                {routePerformance.length > 0 ? (
                  <table className="unified-table">
                    <thead>
                      <tr>
                        <th>Domain</th>
                        <th>Route</th>
                        <th>Clicks</th>
                        <th>Unique</th>
                        <th>Human</th>
                        <th>Bot</th>
                        <th>Countries</th>
                        <th>Devices</th>
                      </tr>
                    </thead>
                    <tbody>
                      {routePerformance.slice(0, 5).map((route) => (
                        <tr key={route.route_id}>
                          <td className="table-cell-secondary">{route.route_domain_name ?? '—'}</td>
                          <td className="table-cell-primary">{route.route_name ?? '—'}</td>
                          <td className="table-metric-large">{route.total_clicks.toLocaleString()}</td>
                          <td className="table-cell-text">{route.unique_visitors.toLocaleString()}</td>
                          <td className="status-success">{route.human_clicks.toLocaleString()}</td>
                          <td className="status-error">{route.bot_clicks.toLocaleString()}</td>
                          <td className="table-cell-muted">{route.countries_reached}</td>
                          <td className="table-cell-muted">{route.device_types}</td>
                        </tr>
                      ))}
                    </tbody>
                  </table>
                ) : (
                  <div className="table-empty">
                    <div className="table-empty-icon">
                      <Target />
                    </div>
                    <div className="table-empty-title">No route performance data</div>
                    <div className="table-empty-description">Nothing for the selected period yet.</div>
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

export default Dashboard;
