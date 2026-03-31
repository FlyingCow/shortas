import React, { useState, useEffect, useCallback, useRef } from 'react';
import {
  Plus,
  Edit,
  Trash2,
  Search,
  BarChart3,
  MousePointer,
  Users,
  Activity,
  Bot,
  Globe,
  RefreshCw,
  Copy,
  QrCode,
  ShieldOff
} from 'lucide-react';
import {
  BarChart,
  Bar,
  XAxis,
  YAxis,
  Tooltip,
  ResponsiveContainer,
  AreaChart,
  Area,
  PieChart,
  Pie,
  Cell,
  CartesianGrid
} from 'recharts';
import { apiService, RouteDto, RoutingPolicy, DomainDto, RouteSearchResult } from '../services/api';
import { useAlert } from '../contexts/AlertContext';
import { getRouteImagesBaseUrl } from '../config/runtimeEnv';
import { getCountryDisplayName } from '../utils/countries';
import LoadingSpinner from './LoadingSpinner';
import WorldMap from './WorldMap';
import RouteForm from './RouteForm';
import QRCodeDesigner from './QRCodeDesigner';
import './DesignSystem.css';

// Route Stats Styles - matching DashboardUnified
const routeStatsStyles = `
/* ===== ROUTE STATS STYLES (matching Dashboard) ===== */

/* Stats Grid - 4 columns for route stats */
.rs-stats-grid {
  display: grid;
  grid-template-columns: repeat(4, 1fr);
  gap: 1rem;
  margin-bottom: 1.5rem;
}

@media (max-width: 1200px) {
  .rs-stats-grid {
    grid-template-columns: repeat(2, 1fr);
  }
}

@media (max-width: 600px) {
  .rs-stats-grid {
    grid-template-columns: 1fr;
  }
}

.rs-stat-card {
  background: var(--bg-elevated);
  border: 1px solid var(--border-primary);
  border-radius: var(--radius-xl);
  padding: 1.25rem;
  display: flex;
  align-items: center;
  gap: 1rem;
  transition: all var(--transition-normal);
}

.rs-stat-card:hover {
  box-shadow: var(--shadow-md);
  border-color: var(--border-secondary);
}

.rs-stat-icon {
  width: 44px;
  height: 44px;
  border-radius: var(--radius-lg);
  display: flex;
  align-items: center;
  justify-content: center;
  flex-shrink: 0;
}

.rs-stat-icon svg {
  width: 22px;
  height: 22px;
  color: #fff;
}

.rs-stat-icon.primary { background: var(--color-primary); }
.rs-stat-icon.success { background: var(--color-success); }
.rs-stat-icon.warning { background: var(--color-warning); }
.rs-stat-icon.error { background: var(--color-error); }
.rs-stat-icon.info { background: #8b5cf6; }

.rs-stat-content {
  flex: 1;
  min-width: 0;
}

.rs-stat-value {
  font-size: 1.5rem;
  font-weight: 700;
  color: var(--text-primary);
  line-height: 1.2;
  margin-bottom: 0.125rem;
}

.rs-stat-label {
  font-size: 0.75rem;
  color: var(--text-secondary);
  font-weight: 500;
  text-transform: uppercase;
  letter-spacing: 0.03em;
}

/* Date Range Buttons */
.rs-range-group {
  display: inline-flex;
  border: 1px solid var(--border-secondary);
  border-radius: var(--radius-lg);
  overflow: hidden;
  background: var(--bg-primary);
}

.rs-range-btn {
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

.rs-range-btn:not(:last-child) {
  border-right: 1px solid var(--border-secondary);
}

.rs-range-btn:hover:not(.active) {
  background: var(--bg-tertiary);
  color: var(--text-primary);
}

.rs-range-btn.active {
  background: var(--color-primary);
  color: #ffffff;
}

/* Chart Card */
.rs-chart-card {
  background: var(--bg-elevated);
  border: 1px solid var(--border-primary);
  border-radius: var(--radius-xl);
  overflow: hidden;
}

.rs-chart-header {
  padding: 1rem 1.25rem;
  border-bottom: 1px solid var(--border-primary);
  background: var(--bg-secondary);
}

.rs-chart-title {
  font-size: 0.9375rem;
  font-weight: 600;
  color: var(--text-primary);
  margin: 0 0 0.25rem 0;
  display: flex;
  align-items: center;
  gap: 0.5rem;
}

.rs-chart-title svg {
  color: var(--color-primary);
}

.rs-chart-desc {
  font-size: 0.75rem;
  color: var(--text-muted);
  margin: 0;
}

.rs-chart-body {
  padding: 1.25rem;
}

/* Tooltip */
.rs-tooltip {
  background: var(--bg-elevated);
  border: 1px solid var(--border-primary);
  border-radius: var(--radius-md);
  padding: 0.75rem;
  box-shadow: var(--shadow-lg);
}

.rs-tooltip-label {
  font-size: 0.75rem;
  font-weight: 600;
  color: var(--text-primary);
  margin-bottom: 0.5rem;
  padding-bottom: 0.5rem;
  border-bottom: 1px solid var(--border-primary);
}

.rs-tooltip-row {
  display: flex;
  align-items: center;
  gap: 0.5rem;
  padding: 0.25rem 0;
  font-size: 0.8125rem;
}

.rs-tooltip-dot {
  width: 8px;
  height: 8px;
  border-radius: 50%;
  flex-shrink: 0;
}

.rs-tooltip-name {
  color: var(--text-secondary);
  flex: 1;
}

.rs-tooltip-value {
  font-weight: 600;
  color: var(--text-primary);
}

/* Pie Legend */
.rs-pie-legend {
  display: flex;
  flex-wrap: wrap;
  gap: 0.75rem;
  justify-content: center;
  padding: 0.75rem 1rem;
  border-top: 1px solid var(--border-primary);
  background: var(--bg-secondary);
}

.rs-pie-legend-item {
  display: flex;
  align-items: center;
  gap: 0.375rem;
  font-size: 0.75rem;
}

.rs-pie-legend-dot {
  width: 8px;
  height: 8px;
  border-radius: 50%;
  flex-shrink: 0;
}

.rs-pie-legend-label {
  color: var(--text-secondary);
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
  max-width: 100px;
}

/* Country List */
.rs-country-list {
  display: flex;
  flex-direction: column;
  gap: 0.5rem;
}

.rs-country-row {
  display: flex;
  align-items: center;
  gap: 0.75rem;
  padding: 0.5rem 0;
}

.rs-country-rank {
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

.rs-country-name {
  flex: 1;
  font-size: 0.8125rem;
  font-weight: 500;
  color: var(--text-primary);
  min-width: 0;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.rs-country-bar-wrap {
  flex: 1;
  max-width: 120px;
}

.rs-country-bar {
  height: 6px;
  background: var(--bg-tertiary);
  border-radius: 3px;
  overflow: hidden;
}

.rs-country-bar-fill {
  height: 100%;
  background: var(--color-primary);
  border-radius: 3px;
  transition: width var(--transition-normal);
}

.rs-country-value {
  font-size: 0.8125rem;
  font-weight: 600;
  color: var(--text-primary);
  min-width: 50px;
  text-align: right;
}

.rs-country-pct {
  font-size: 0.75rem;
  color: var(--text-muted);
  min-width: 40px;
  text-align: right;
}

/* Section Grid */
.rs-section {
  margin-bottom: 1.5rem;
}

.rs-section-row {
  display: grid;
  gap: 1.5rem;
  margin-bottom: 1.5rem;
}

.rs-section-row.two-cols {
  grid-template-columns: repeat(2, 1fr);
}

.rs-section-row.three-cols {
  grid-template-columns: repeat(3, 1fr);
}

@media (max-width: 1024px) {
  .rs-section-row.two-cols,
  .rs-section-row.three-cols {
    grid-template-columns: 1fr;
  }
}

/* Refreshing indicator */
.rs-refreshing {
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

/* Empty State */
.rs-empty {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  padding: 3rem 2rem;
  text-align: center;
}

.rs-empty-icon {
  width: 56px;
  height: 56px;
  border-radius: 50%;
  background: var(--bg-tertiary);
  display: flex;
  align-items: center;
  justify-content: center;
  margin-bottom: 1rem;
}

.rs-empty-icon svg {
  width: 28px;
  height: 28px;
  color: var(--text-muted);
}

.rs-empty p {
  font-size: 0.875rem;
  color: var(--text-muted);
  margin: 0;
}

/* Spin animation */
@keyframes rs-spin {
  from { transform: rotate(0deg); }
  to { transform: rotate(360deg); }
}

.rs-icon-spin {
  animation: rs-spin 1s linear infinite;
}

/* ===== REDESIGNED SIDEBAR STYLES ===== */

/* Sidebar Container */
.rs-sidebar {
  width: 380px;
  background: var(--bg-elevated);
  border-right: 1px solid var(--border-primary);
  display: flex;
  flex-direction: column;
  height: 100%;
}

/* Sidebar Header */
.rs-sidebar-header {
  padding: 1.25rem 1.25rem 1rem;
  border-bottom: 1px solid var(--border-primary);
  background: linear-gradient(135deg, var(--bg-elevated) 0%, var(--bg-secondary) 100%);
}

.rs-sidebar-title {
  font-size: 1.125rem;
  font-weight: 700;
  color: var(--text-primary);
  margin: 0 0 0.25rem 0;
  letter-spacing: -0.01em;
}

.rs-sidebar-subtitle {
  font-size: 0.75rem;
  color: var(--text-muted);
  margin: 0;
}

/* Search Section */
.rs-search-section {
  padding: 1rem 1.25rem;
  background: var(--bg-primary);
  border-bottom: 1px solid var(--border-primary);
}

.rs-search-box {
  position: relative;
  margin-bottom: 0.875rem;
}

.rs-search-box svg {
  position: absolute;
  left: 0.875rem;
  top: 50%;
  transform: translateY(-50%);
  color: var(--text-muted);
  width: 16px;
  height: 16px;
  transition: color var(--transition-fast);
}

.rs-search-box input {
  width: 100%;
  padding: 0.625rem 0.875rem 0.625rem 2.5rem;
  border: 1px solid var(--border-secondary);
  border-radius: var(--radius-lg);
  background: var(--bg-elevated);
  color: var(--text-primary);
  font-size: 0.8125rem;
  transition: all var(--transition-fast);
}

.rs-search-box input::placeholder {
  color: var(--text-muted);
}

.rs-search-box input:hover {
  border-color: var(--border-primary);
}

.rs-search-box input:focus {
  outline: none;
  border-color: var(--color-primary);
  box-shadow: 0 0 0 3px var(--color-primary-light);
  background: var(--bg-primary);
}

.rs-search-box input:focus + svg,
.rs-search-box:focus-within svg {
  color: var(--color-primary);
}

/* Filter Row */
.rs-filter-row {
  display: flex;
  gap: 0.5rem;
}

.rs-filter-select {
  flex: 1;
  padding: 0.5rem 2rem 0.5rem 0.75rem;
  border: 1px solid var(--border-secondary);
  border-radius: var(--radius-md);
  background: var(--bg-elevated);
  color: var(--text-primary);
  font-size: 0.75rem;
  font-weight: 500;
  cursor: pointer;
  transition: all var(--transition-fast);
  appearance: none;
  background-image: url("data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' width='12' height='12' viewBox='0 0 24 24' fill='none' stroke='%236b7280' stroke-width='2' stroke-linecap='round' stroke-linejoin='round'%3E%3Cpolyline points='6 9 12 15 18 9'%3E%3C/polyline%3E%3C/svg%3E");
  background-repeat: no-repeat;
  background-position: right 0.5rem center;
}

.rs-filter-select:hover {
  border-color: var(--border-primary);
  background-color: var(--bg-primary);
}

.rs-filter-select:focus {
  outline: none;
  border-color: var(--color-primary);
  box-shadow: 0 0 0 2px var(--color-primary-light);
}

/* Routes List */
.rs-routes-list {
  flex: 1;
  overflow-y: auto;
  padding: 0.75rem;
}

.rs-routes-list::-webkit-scrollbar {
  width: 6px;
}

.rs-routes-list::-webkit-scrollbar-track {
  background: transparent;
}

.rs-routes-list::-webkit-scrollbar-thumb {
  background: var(--border-secondary);
  border-radius: 3px;
}

.rs-routes-list::-webkit-scrollbar-thumb:hover {
  background: var(--text-muted);
}

/* Route Card */
.rs-route-card {
  background: var(--bg-primary);
  border: 1px solid var(--border-primary);
  border-radius: var(--radius-lg);
  padding: 0.875rem;
  margin-bottom: 0.5rem;
  cursor: pointer;
  transition: all var(--transition-fast);
  position: relative;
}

.rs-route-card:hover {
  border-color: var(--color-primary);
  background: var(--bg-elevated);
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.06);
  transform: translateY(-1px);
}

.rs-route-card.selected {
  border-color: var(--color-primary);
  background: var(--color-primary-light);
  box-shadow: 0 2px 12px rgba(59, 130, 246, 0.15);
}

.rs-route-card.selected::before {
  content: '';
  position: absolute;
  left: 0;
  top: 0;
  bottom: 0;
  width: 3px;
  background: var(--color-primary);
  border-radius: var(--radius-lg) 0 0 var(--radius-lg);
}

/* Route Card Header */
.rs-route-header {
  display: flex;
  align-items: flex-start;
  justify-content: space-between;
  gap: 0.5rem;
  margin-bottom: 0.5rem;
}

.rs-route-header-content {
  display: flex;
  align-items: center;
  gap: 0.5rem;
  min-width: 0;
}

.rs-route-favicon {
  width: 20px;
  height: 20px;
  border-radius: var(--radius-sm);
  flex-shrink: 0;
  object-fit: contain;
  background: var(--bg-tertiary);
}

.rs-route-link {
  font-size: 0.875rem;
  font-weight: 600;
  color: var(--text-primary);
  word-break: break-all;
  line-height: 1.3;
}

.rs-route-card:hover .rs-route-link {
  color: var(--color-primary);
}

.rs-route-domain {
  font-size: 0.6875rem;
  color: var(--text-muted);
  margin-top: 0.125rem;
  display: flex;
  align-items: center;
  gap: 0.25rem;
}

.rs-route-domain::before {
  content: '';
  width: 4px;
  height: 4px;
  background: var(--text-muted);
  border-radius: 50%;
  opacity: 0.5;
}

/* Route Destination */
.rs-route-dest {
  font-size: 0.75rem;
  color: var(--text-tertiary);
  margin-bottom: 0.625rem;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
  padding: 0.375rem 0.5rem;
  background: var(--bg-secondary);
  border-radius: var(--radius-sm);
  font-family: var(--font-family-mono);
}

/* Route Footer */
.rs-route-footer {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 0.5rem;
}

/* Route Tags */
.rs-route-tags {
  display: flex;
  align-items: center;
  gap: 0.375rem;
  flex-wrap: wrap;
}

.rs-route-tag {
  padding: 0.1875rem 0.5rem;
  border-radius: var(--radius-full);
  font-size: 0.6875rem;
  font-weight: 600;
  text-transform: uppercase;
  letter-spacing: 0.02em;
}

.rs-route-tag.active {
  background: rgba(34, 197, 94, 0.1);
  color: var(--color-success);
}

.rs-route-tag.inactive {
  background: rgba(239, 68, 68, 0.1);
  color: var(--color-error);
}

.rs-route-tag.blocked {
  background: rgba(239, 68, 68, 0.15);
  color: #dc2626;
  border: 1px solid rgba(239, 68, 68, 0.3);
}

.rs-route-tag.policy {
  background: rgba(59, 130, 246, 0.1);
  color: var(--color-primary);
}

.rs-route-tag.code {
  background: var(--bg-tertiary);
  color: var(--text-secondary);
  font-family: var(--font-family-mono);
}

/* Route Actions */
.rs-route-actions {
  display: flex;
  gap: 0.25rem;
  opacity: 0;
  transition: opacity var(--transition-fast);
}

.rs-route-card:hover .rs-route-actions {
  opacity: 1;
}

.rs-action-btn {
  width: 28px;
  height: 28px;
  border: none;
  background: var(--bg-secondary);
  color: var(--text-muted);
  border-radius: var(--radius-md);
  cursor: pointer;
  transition: all var(--transition-fast);
  display: flex;
  align-items: center;
  justify-content: center;
}

.rs-action-btn:hover {
  background: var(--bg-tertiary);
  color: var(--text-primary);
}

.rs-action-btn.edit:hover {
  background: var(--color-primary-light);
  color: var(--color-primary);
}

.rs-action-btn.delete:hover {
  background: rgba(239, 68, 68, 0.1);
  color: var(--color-error);
}

.rs-action-btn.unblock:hover {
  background: rgba(34, 197, 94, 0.1);
  color: var(--color-success);
}

.rs-action-btn.copy:hover {
  background: var(--color-primary-light);
  color: var(--color-primary);
}

.rs-action-btn.copy.copied {
  background: rgba(34, 197, 94, 0.1);
  color: var(--color-success);
}

.rs-action-btn.qr:hover {
  background: var(--color-primary-light);
  color: var(--color-primary);
}

/* Empty State */
.rs-empty-list {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  padding: 3rem 1.5rem;
  text-align: center;
}

.rs-empty-list-icon {
  width: 64px;
  height: 64px;
  border-radius: 50%;
  background: var(--bg-secondary);
  display: flex;
  align-items: center;
  justify-content: center;
  margin-bottom: 1rem;
}

.rs-empty-list-icon svg {
  width: 28px;
  height: 28px;
  color: var(--text-muted);
}

.rs-empty-list-title {
  font-size: 0.9375rem;
  font-weight: 600;
  color: var(--text-primary);
  margin: 0 0 0.375rem 0;
}

.rs-empty-list-desc {
  font-size: 0.8125rem;
  color: var(--text-muted);
  margin: 0;
}

/* Sidebar Footer */
.rs-sidebar-footer {
  padding: 1rem 1.25rem;
  border-top: 1px solid var(--border-primary);
  background: var(--bg-secondary);
}

.rs-footer-stats {
  display: grid;
  grid-template-columns: repeat(2, 1fr);
  gap: 0.75rem;
  margin-bottom: 1rem;
}

.rs-footer-stat {
  background: var(--bg-primary);
  border: 1px solid var(--border-primary);
  border-radius: var(--radius-lg);
  padding: 0.75rem;
  text-align: center;
  transition: all var(--transition-fast);
}

.rs-footer-stat:hover {
  border-color: var(--border-secondary);
  box-shadow: var(--shadow-sm);
}

.rs-footer-stat-value {
  font-size: 1.25rem;
  font-weight: 700;
  color: var(--text-primary);
  line-height: 1;
  margin-bottom: 0.25rem;
}

.rs-footer-stat-label {
  font-size: 0.6875rem;
  color: var(--text-muted);
  text-transform: uppercase;
  letter-spacing: 0.03em;
  font-weight: 500;
}

.rs-create-btn {
  width: 100%;
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 0.5rem;
  padding: 0.75rem 1rem;
  background: var(--color-primary);
  color: #ffffff;
  border: none;
  border-radius: var(--radius-lg);
  font-size: 0.875rem;
  font-weight: 600;
  cursor: pointer;
  transition: all var(--transition-fast);
}

.rs-create-btn:hover {
  background: var(--color-primary-dark);
  box-shadow: 0 4px 12px rgba(59, 130, 246, 0.3);
  transform: translateY(-1px);
}

.rs-create-btn:active {
  transform: translateY(0);
}

.rs-create-btn svg {
  width: 18px;
  height: 18px;
}

/* Override parent styles */
.routes-with-sidebar .rs-sidebar {
  width: 380px;
}

.routes-with-sidebar.editing .rs-sidebar {
  opacity: 0.3;
  pointer-events: none;
}
`;

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

const CustomTooltip = ({ active, payload, label }: any) => {
  if (!active || !payload?.length) return null;
  return (
    <div className="rs-tooltip">
      {label != null && label !== '' && (
        <div className="rs-tooltip-label">{label}</div>
      )}
      {payload.map((entry: any, i: number) => {
        const percent = entry.payload?.percent;
        return (
          <div key={i} className="rs-tooltip-row">
            <span className="rs-tooltip-dot" style={{ backgroundColor: entry.color || entry.fill }} />
            <span className="rs-tooltip-name">{entry.name}</span>
            <span className="rs-tooltip-value">
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
  <div className="rs-pie-legend">
    {items.slice(0, 6).map((item, i) => (
      <div key={i} className="rs-pie-legend-item">
        <span className="rs-pie-legend-dot" style={{ backgroundColor: item.color }} />
        <span className="rs-pie-legend-label">{item.name}</span>
      </div>
    ))}
  </div>
);

const RoutesWithSidebar: React.FC = () => {
  const [routes, setRoutes] = useState<RouteDto[]>([]);
  const [domains, setDomains] = useState<DomainDto[]>([]);
  const [workspaces, setWorkspaces] = useState<any[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const [searchTerm, setSearchTerm] = useState('');
  const [statusFilter, setStatusFilter] = useState('all');
  const [workspaceFilter, setWorkspaceFilter] = useState('all');
  const [selectedRoute, setSelectedRoute] = useState<RouteDto | null>(null);
  const [isEditing, setIsEditing] = useState(false);
  const [editingRoute, setEditingRoute] = useState<RouteDto | null>(null);
  const [analytics, setAnalytics] = useState<any>(null);
  const [analyticsLoading, setAnalyticsLoading] = useState(false);
  const [timeRange, setTimeRange] = useState('7d');
  const [searchResults, setSearchResults] = useState<RouteDto[] | null>(null);
  const [searchLoading, setSearchLoading] = useState(false);
  const searchDebounceRef = React.useRef<ReturnType<typeof setTimeout> | null>(null);
  const [copiedRouteId, setCopiedRouteId] = useState<string | null>(null);
  const [iconRefreshKey, setIconRefreshKey] = useState(0);
  const iconRefreshTimeoutRef = useRef<ReturnType<typeof setTimeout> | null>(null);
  const [qrDesignerRoute, setQrDesignerRoute] = useState<RouteDto | null>(null);
  const { showToast, showConfirm } = useAlert();

  useEffect(() => {
    fetchRoutes();
    fetchDomains();
    fetchWorkspaces();
  }, []);

  const scheduleIconRefresh = useCallback(() => {
    if (iconRefreshTimeoutRef.current) clearTimeout(iconRefreshTimeoutRef.current);
    iconRefreshTimeoutRef.current = setTimeout(() => {
      fetchRoutes().then(() => {
        setIconRefreshKey((k) => k + 1);
      });
      iconRefreshTimeoutRef.current = null;
    }, 2000);
  }, []);

  useEffect(() => {
    return () => {
      if (iconRefreshTimeoutRef.current) clearTimeout(iconRefreshTimeoutRef.current);
    };
  }, []);

  useEffect(() => {
    fetchRoutes();
  }, [workspaceFilter]);

  // Debounced Elasticsearch search
  useEffect(() => {
    if (searchDebounceRef.current) {
      clearTimeout(searchDebounceRef.current);
    }

    if (!searchTerm.trim()) {
      setSearchResults(null);
      setSearchLoading(false);
      return;
    }

    setSearchLoading(true);
    searchDebounceRef.current = setTimeout(async () => {
      try {
        const wsId = workspaceFilter !== 'all' ? workspaceFilter : undefined;
        const response = await apiService.routes.search({
          q: searchTerm.trim(),
          page: 1,
          pageSize: 100,
          workspaceId: wsId,
        });
        // Map search results to RouteDto-compatible objects
        const mapped: RouteDto[] = response.data.map((r: RouteSearchResult) => ({
          id: r.id,
          switch: r.switch,
          link: r.link,
          dest: r.dest || '',
          destFormat: 'Http',
          code: 0,
          ttl: 0,
          status: r.status,
          terminal: 'External',
          domain: r.domainName ? { id: '', name: r.domainName, ownerId: r.ownerId ?? '', verificationStatus: 'Verified' as const, verificationReason: '' } : undefined,
          properties: r.ownerId ? { routeId: r.id, ownerId: r.ownerId, domainId: '', scripts: [], tags: [], custom: {}, opengraph: false, allowDebug: false } : undefined,
        }));
        setSearchResults(mapped);
      } catch (err: any) {
        console.error('Search failed:', err);
        // Fall back to showing no results rather than error
        setSearchResults([]);
      } finally {
        setSearchLoading(false);
      }
    }, 300);

    return () => {
      if (searchDebounceRef.current) {
        clearTimeout(searchDebounceRef.current);
      }
    };
  }, [searchTerm, workspaceFilter]);

  const fetchRoutes = async () => {
    try {
      setLoading(true);
      setError(null);
      const params: any = { page: 1, pageSize: 100 };
      if (workspaceFilter && workspaceFilter !== 'all') {
        params.workspaceId = workspaceFilter;
      }
      const response = await apiService.routes.list(params);
      setRoutes(response.data);
    } catch (err: any) {
      console.error('Failed to fetch routes:', err);
      setError('Failed to load routes. Please try again.');
    } finally {
      setLoading(false);
    }
  };

  const fetchDomains = async () => {
    try {
      const response = await apiService.domains.list({ page: 1, pageSize: 100 });
      setDomains(response.data);
    } catch (err: any) {
      console.error('Failed to fetch domains:', err);
    }
  };

  const fetchWorkspaces = async () => {
    try {
      const data = await apiService.workspaces.list();
      setWorkspaces(data);
    } catch (err: any) {
      console.error('Failed to fetch workspaces:', err);
    }
  };

  const handleDeleteRoute = async (route: RouteDto) => {
    const confirmed = await showConfirm(
      `Are you sure you want to delete the route "${route.link}"?`,
      'Delete route',
      { confirmLabel: 'Delete', variant: 'danger' }
    );
    if (!confirmed) return;

    if (!route.id) {
      showToast('Cannot delete route: missing ID', 'error');
      return;
    }

    try {
      await apiService.routes.delete(route.id);
      await fetchRoutes();
      if (selectedRoute?.id === route.id) {
        setSelectedRoute(null);
      }
    } catch (err: any) {
      console.error('Failed to delete route:', err);
      showToast('Failed to delete route. Please try again.', 'error');
    }
  };

  const handleEditRoute = (route: RouteDto) => {
    setEditingRoute(route);
    setIsEditing(true);
  };

  const handleCopyRouteUrl = async (route: RouteDto, e: React.MouseEvent) => {
    e.stopPropagation();
    const domain = route.domain?.name || 'example.com';
    const url = `https://${domain}/${route.link}`;
    try {
      await navigator.clipboard.writeText(url);
      setCopiedRouteId(route.id || null);
      setTimeout(() => setCopiedRouteId(null), 2000);
    } catch (err) {
      console.error('Failed to copy URL:', err);
    }
  };

  const handleCancelEdit = () => {
    setEditingRoute(null);
    setIsEditing(false);
  };

  useEffect(() => {
    if (!qrDesignerRoute && !isEditing) return;
    const onKey = (e: KeyboardEvent) => {
      if (e.key === 'Escape') {
        if (qrDesignerRoute) setQrDesignerRoute(null);
        else handleCancelEdit();
        e.preventDefault();
        e.stopImmediatePropagation();
      }
    };
    document.addEventListener('keydown', onKey);
    return () => document.removeEventListener('keydown', onKey);
  }, [qrDesignerRoute, isEditing]);

  useEffect(() => {
    if (isEditing || qrDesignerRoute) return;
    const isInputFocused = () => {
      const el = document.activeElement as HTMLElement | null;
      if (!el || el === document.body) return false;
      const tag = el.tagName;
      return tag === 'INPUT' || tag === 'TEXTAREA' || tag === 'SELECT' || el.isContentEditable;
    };
    const onKey = (e: KeyboardEvent) => {
      if (isInputFocused()) return;
      if ((e.ctrlKey || e.metaKey) && e.key.toLowerCase() === 'n') {
        setEditingRoute(null);
        setIsEditing(true);
        e.preventDefault();
      } else if (e.altKey && e.key.toLowerCase() === 'n') {
        setEditingRoute(null);
        setIsEditing(true);
        e.preventDefault();
      }
    };
    document.addEventListener('keydown', onKey);
    return () => document.removeEventListener('keydown', onKey);
  }, [isEditing, qrDesignerRoute]);

  const handleCreateRoute = () => {
    setEditingRoute(null);
    setIsEditing(true);
  };

  const fetchAnalytics = useCallback(async (route: RouteDto) => {
    try {
      setAnalyticsLoading(true);

      const endDate = new Date();
      const startDate = new Date();

      switch (timeRange) {
        case '24h': startDate.setHours(startDate.getHours() - 24); break;
        case '7d': startDate.setDate(startDate.getDate() - 7); break;
        case '30d': startDate.setDate(startDate.getDate() - 30); break;
        case '90d': startDate.setDate(startDate.getDate() - 90); break;
      }

      const fromDate = startDate.toISOString().split('T')[0];
      const toDate = endDate.toISOString().split('T')[0];
      const fromHour = startDate.toISOString().replace('T', ' ').substring(0, 13);
      const toHour = endDate.toISOString().replace('T', ' ').substring(0, 13);

      const [dailyStats, geographicStats, deviceStats, browserStats, trafficStats] = await Promise.all([
        apiService.clickstream.getDailyStats({ routeId: route.id, fromDate, toDate }),
        apiService.clickstream.getGeographicStats({ routeId: route.id, fromDate, toDate }),
        apiService.clickstream.getDeviceStats({ routeId: route.id, fromDate, toDate }),
        apiService.clickstream.getBrowserStats({ routeId: route.id, fromDate, toDate }),
        apiService.clickstream.getTrafficTypeStats({ routeId: route.id, fromHour, toHour }),
      ]);

      const totals = dailyStats.reduce((acc, stat) => ({
        totalClicks: acc.totalClicks + stat.total_clicks,
        uniqueClicks: acc.uniqueClicks + stat.unique_clicks,
        botClicks: acc.botClicks + stat.bot_clicks,
        humanClicks: acc.humanClicks + stat.human_clicks,
      }), { totalClicks: 0, uniqueClicks: 0, botClicks: 0, humanClicks: 0 });

      const humanStat = trafficStats.find(stat => !stat.is_bot);
      const botStat = trafficStats.find(stat => stat.is_bot);

      const analyticsData = {
        totalClicks: totals.totalClicks,
        uniqueVisitors: totals.uniqueClicks,
        humanClicks: totals.humanClicks,
        botClicks: totals.botClicks,
        topCountries: geographicStats.slice(0, 10).map(stat => {
          const percentage = totals.totalClicks > 0
            ? (stat.total_clicks / totals.totalClicks) * 100
            : 0;
          return {
            name: getCountryDisplayName(stat.country) || stat.country,
            clicks: stat.total_clicks,
            percentage: Math.round(percentage)
          };
        }),
        topBrowsers: browserStats.slice(0, 8).map(stat => ({
          name: stat.user_agent_family,
          clicks: stat.total_clicks,
        })),
        topDevices: deviceStats.slice(0, 8).map(stat => ({
          device: `${stat.device_family} (${stat.os_family})`,
          clicks: stat.total_clicks,
        })),
        trafficType: [
          { name: 'Human', value: humanStat?.total_clicks || 0 },
          { name: 'Bot', value: botStat?.total_clicks || 0 },
        ],
        dailyClicks: dailyStats.map(stat => ({
          date: new Date(stat.date).toLocaleDateString('en-US', { month: 'short', day: 'numeric' }),
          clicks: stat.total_clicks,
        })),
      };

      setAnalytics(analyticsData);
    } catch (err: any) {
      console.error('Failed to fetch analytics:', err);
    } finally {
      setAnalyticsLoading(false);
    }
  }, [timeRange]);

  // Refetch analytics when route or time range changes
  useEffect(() => {
    if (selectedRoute) {
      fetchAnalytics(selectedRoute);
    }
  }, [selectedRoute, fetchAnalytics]);

  const handleSelectRoute = (route: RouteDto) => {
    setSelectedRoute(route);
  };

  const getPolicyType = (policy?: RoutingPolicy): string => {
    if (!policy || policy === 'Basic') return 'Basic';
    if (policy === 'Mirroring') return 'Mirroring';
    if (typeof policy === 'object') {
      if ('Conditional' in policy) return 'Conditional';
      if ('Challenge' in policy) return 'Challenge';
      if ('File' in policy) return 'File';
    }
    return 'Basic';
  };

  // Helper functions for blocked status handling
  const getStatusClass = (status: string): string => {
    const lower = status.toLowerCase();
    if (lower.startsWith('blocked')) return 'blocked';
    if (lower === 'active') return 'active';
    return 'inactive';
  };

  const getStatusDisplay = (status: string): string => {
    if (status.toLowerCase().startsWith('blocked')) return 'Blocked';
    return status;
  };

  const isRouteBlocked = (status: string): boolean => {
    return status.toLowerCase().startsWith('blocked');
  };

  const handleUnblockRoute = async (route: RouteDto) => {
    if (!route.id) {
      showToast('Cannot unblock route: missing ID', 'error');
      return;
    }

    const confirmed = await showConfirm(
      `Are you sure you want to unblock the route "${route.link}"? This will make it active again.`,
      'Unblock route',
      { confirmLabel: 'Unblock', variant: 'primary' }
    );
    if (!confirmed) return;

    try {
      await apiService.routes.unblock(route.id);
      await fetchRoutes();
      showToast('Route unblocked successfully', 'success');
    } catch (err: any) {
      console.error('Failed to unblock route:', err);
      showToast('Failed to unblock route. Please try again.', 'error');
    }
  };

  // Use ES search results when search is active, otherwise use local routes with status filter
  const baseRoutes = searchResults !== null ? searchResults : routes;
  const filteredRoutes = baseRoutes.filter(route => {
    if (statusFilter === 'all') return true;
    if (statusFilter === 'blocked') return isRouteBlocked(route.status);
    return route.status.toLowerCase() === statusFilter.toLowerCase();
  });

  if (loading) {
    return <LoadingSpinner />;
  }

  if (error) {
    return (
      <div className="alert alert-error">
        <h3>Error Loading Routes</h3>
        <p>{error}</p>
        <button className="btn btn-primary" onClick={fetchRoutes}>
          Retry
        </button>
      </div>
    );
  }

  const humanRate = analytics && analytics.totalClicks > 0
    ? `${((analytics.humanClicks / analytics.totalClicks) * 100).toFixed(0)}%`
    : null;
  const botRate = analytics && analytics.totalClicks > 0
    ? `${((analytics.botClicks / analytics.totalClicks) * 100).toFixed(0)}%`
    : null;

  return (
    <>
      <style>{routeStatsStyles}</style>
      <div className={`routes-with-sidebar ${isEditing || qrDesignerRoute ? 'editing' : ''}`}>
        {/* Redesigned Sidebar */}
        <div className="rs-sidebar">
          {/* Header */}
          <div className="rs-sidebar-header">
            <h2 className="rs-sidebar-title">Routes</h2>
            <p className="rs-sidebar-subtitle">Manage your shortened URLs</p>
          </div>

          {/* Search Section */}
          <div className="rs-search-section">
            <div className="rs-search-box">
              <input
                type="text"
                placeholder="Search routes..."
                value={searchTerm}
                onChange={(e) => setSearchTerm(e.target.value)}
              />
              {searchLoading ? <RefreshCw size={16} className="rs-icon-spin" /> : <Search size={16} />}
            </div>

            <div className="rs-filter-row">
              <select
                className="rs-filter-select"
                value={statusFilter}
                onChange={(e) => setStatusFilter(e.target.value)}
              >
                <option value="all">All Status</option>
                <option value="active">Active</option>
                <option value="inactive">Inactive</option>
                <option value="blocked">Blocked</option>
              </select>

              <select
                className="rs-filter-select"
                value={workspaceFilter}
                onChange={(e) => setWorkspaceFilter(e.target.value)}
              >
                <option value="all">All Workspaces</option>
                {workspaces.map((workspace) => (
                  <option key={workspace.id} value={workspace.id}>
                    {workspace.name}
                  </option>
                ))}
              </select>
            </div>
          </div>

          {/* Routes List */}
          <div className="rs-routes-list">
            {filteredRoutes.map((route) => (
              <div
                key={route.id || route.link}
                className={`rs-route-card ${selectedRoute?.id === route.id ? 'selected' : ''}`}
                onClick={() => handleSelectRoute(route)}
              >
                <div className="rs-route-header">
                  <div className="rs-route-header-content">
                    {route.properties?.ownerId && route.id && (
                      <img
                        key={`favicon-${route.id}-${iconRefreshKey}`}
                        src={`${getRouteImagesBaseUrl()}/route-images/${route.properties.ownerId}/${route.id}/fav.ico${iconRefreshKey ? `?t=${iconRefreshKey}` : ''}`}
                        alt=""
                        className="rs-route-favicon"
                        onError={(e) => { e.currentTarget.style.display = 'none'; }}
                      />
                    )}
                    <div>
                      <div className="rs-route-link">{route.link}</div>
                      {route.domain?.name && (
                        <div className="rs-route-domain">{route.domain.name}/{route.link}</div>
                      )}
                    </div>
                  </div>
                </div>

                <div className="rs-route-dest">
                  {route.dest.length > 40 ? `${route.dest.substring(0, 40)}...` : route.dest}
                </div>

                <div className="rs-route-footer">
                  <div className="rs-route-tags">
                    <span className={`rs-route-tag ${getStatusClass(route.status)}`}>
                      {getStatusDisplay(route.status)}
                    </span>
                    {route.policy && getPolicyType(route.policy) !== 'Basic' && (
                      <span className="rs-route-tag policy">
                        {getPolicyType(route.policy)}
                      </span>
                    )}
                    {route.code > 0 && (
                      <span className="rs-route-tag code">{route.code}</span>
                    )}
                  </div>

                  <div className="rs-route-actions">
                    <button
                      className="rs-action-btn qr"
                      onClick={(e) => {
                        e.stopPropagation();
                        setQrDesignerRoute(route);
                      }}
                      title="QR Code"
                    >
                      <QrCode size={14} />
                    </button>
                    <button
                      className={`rs-action-btn copy ${copiedRouteId === route.id ? 'copied' : ''}`}
                      onClick={(e) => handleCopyRouteUrl(route, e)}
                      title={copiedRouteId === route.id ? 'Copied!' : 'Copy URL'}
                    >
                      <Copy size={14} />
                    </button>
                    {isRouteBlocked(route.status) && (
                      <button
                        className="rs-action-btn unblock"
                        onClick={(e) => {
                          e.stopPropagation();
                          handleUnblockRoute(route);
                        }}
                        title="Unblock route"
                      >
                        <ShieldOff size={14} />
                      </button>
                    )}
                    <button
                      className="rs-action-btn edit"
                      onClick={(e) => {
                        e.stopPropagation();
                        handleEditRoute(route);
                      }}
                      title="Edit route"
                    >
                      <Edit size={14} />
                    </button>
                    <button
                      className="rs-action-btn delete"
                      onClick={(e) => {
                        e.stopPropagation();
                        handleDeleteRoute(route);
                      }}
                      title="Delete route"
                    >
                      <Trash2 size={14} />
                    </button>
                  </div>
                </div>
              </div>
            ))}

            {filteredRoutes.length === 0 && (
              <div className="rs-empty-list">
                <div className="rs-empty-list-icon">
                  <BarChart3 />
                </div>
                <h3 className="rs-empty-list-title">No routes found</h3>
                <p className="rs-empty-list-desc">
                  {searchTerm ? 'Try adjusting your search or filters.' : 'Create your first route to get started.'}
                </p>
              </div>
            )}
          </div>

          {/* Footer */}
          <div className="rs-sidebar-footer">
            <div className="rs-footer-stats">
              <div className="rs-footer-stat">
                <div className="rs-footer-stat-value">{filteredRoutes.length}</div>
                <div className="rs-footer-stat-label">{searchResults !== null ? 'Results' : 'Total'}</div>
              </div>
              <div className="rs-footer-stat">
                <div className="rs-footer-stat-value">{filteredRoutes.filter(r => r.status.toLowerCase() === 'active').length}</div>
                <div className="rs-footer-stat-label">Active</div>
              </div>
            </div>
            <button className="rs-create-btn" onClick={handleCreateRoute}>
              <Plus />
              Create Route
            </button>
          </div>
        </div>

      {/* Main Content */}
      <div className="main-content">
        {qrDesignerRoute ? (() => {
          const domain = qrDesignerRoute.domain?.name ?? '';
          const link = (qrDesignerRoute.link ?? '').replace(/^\//, '');
          const qrUrl = domain ? `http://${domain}/${link}` : '';
          return (
            <div className="route-qr-content" style={{ padding: '1.5rem' }}>
              <div className="route-qr-header" style={{ marginBottom: '1rem' }}>
                <h2 style={{ margin: '0 0 0.25rem 0', fontSize: '1.25rem' }}>QR Code</h2>
                {qrUrl && <p style={{ margin: 0, fontSize: '0.875rem', color: 'var(--text-muted)' }}>{qrUrl}</p>}
              </div>
              {qrUrl ? (
                <QRCodeDesigner
                  url={qrUrl}
                  routeId={qrDesignerRoute.id}
                  ownerId={qrDesignerRoute.properties?.ownerId}
                />
              ) : (
                <div className="db-empty" style={{ padding: '3rem 2rem' }}>
                  <p>This route has no domain. Assign a domain to generate a QR code URL.</p>
                </div>
              )}
              <div style={{ marginTop: '1.5rem', display: 'flex', justifyContent: 'flex-end' }}>
                <button
                  type="button"
                  className="btn btn-outline"
                  onClick={() => setQrDesignerRoute(null)}
                >
                  Cancel
                </button>
              </div>
            </div>
          );
        })() : isEditing ? (
          <div className="route-qr-content" style={{ padding: '1.5rem' }}>
            <div className="route-qr-header" style={{ marginBottom: '1rem' }}>
              <h2 style={{ margin: '0 0 0.25rem 0', fontSize: '1.25rem' }}>
                {editingRoute ? 'Edit Route' : 'Create New Route'}
              </h2>
              <p style={{ margin: 0, fontSize: '0.875rem', color: 'var(--text-muted)' }}>
                {editingRoute ? 'Update destination and options' : 'Set up a new shortened link'}
              </p>
            </div>
            <RouteForm
              route={editingRoute}
              domains={domains}
              workspaces={workspaces}
              showWorkspace
              onSave={async (data) => {
                if (editingRoute?.id) {
                  await apiService.routes.update(editingRoute.id, data);
                } else {
                  await apiService.routes.create(data);
                }
                await fetchRoutes();
                setEditingRoute(null);
                setIsEditing(false);
                scheduleIconRefresh();
              }}
              onCancel={handleCancelEdit}
            />
          </div>
        ) : selectedRoute ? (
          <div className="analytics-content">
            {/* Analytics Header */}
            <div className="analytics-header">
              <div className="route-info">
                <h2>{selectedRoute.link}</h2>
                <p className="route-destination">{selectedRoute.dest}</p>
                <div className="route-actions-header">
                  <button
                    className="btn btn-outline btn-sm"
                    onClick={() => handleEditRoute(selectedRoute)}
                  >
                    <Edit size={16} />
                    Edit Route
                  </button>

                  <div className="rs-range-group">
                    {[
                      { value: '24h', label: '24H' },
                      { value: '7d', label: '7D' },
                      { value: '30d', label: '30D' },
                      { value: '90d', label: '90D' },
                    ].map((option) => (
                      <button
                        key={option.value}
                        className={`rs-range-btn ${timeRange === option.value ? 'active' : ''}`}
                        onClick={() => setTimeRange(option.value)}
                      >
                        {option.label}
                      </button>
                    ))}
                  </div>

                  <button
                    className="btn btn-outline btn-sm"
                    onClick={() => selectedRoute && fetchAnalytics(selectedRoute)}
                    disabled={analyticsLoading}
                    title="Refresh analytics"
                  >
                    <RefreshCw size={14} className={analyticsLoading ? 'rs-icon-spin' : ''} />
                  </button>
                </div>
              </div>
            </div>

            {/* Analytics Body */}
            {analyticsLoading && !analytics ? (
              <div className="welcome-content">
                <LoadingSpinner />
                <p>Loading analytics...</p>
              </div>
            ) : analytics ? (
              <div className="route-analytics-body">
                {/* Refreshing indicator */}
                {analyticsLoading && (
                  <div className="rs-refreshing">
                    <RefreshCw size={14} className="rs-icon-spin" />
                    Updating...
                  </div>
                )}

                {/* Stats Cards - same style as dashboard */}
                <div className="rs-stats-grid">
                  <div className="rs-stat-card">
                    <div className="rs-stat-icon primary">
                      <MousePointer size={22} />
                    </div>
                    <div className="rs-stat-content">
                      <div className="rs-stat-value">
                        {typeof analytics.totalClicks === 'number' ? analytics.totalClicks.toLocaleString() : analytics.totalClicks || '0'}
                      </div>
                      <div className="rs-stat-label">Total Clicks</div>
                    </div>
                  </div>
                  <div className="rs-stat-card">
                    <div className="rs-stat-icon success">
                      <Users size={22} />
                    </div>
                    <div className="rs-stat-content">
                      <div className="rs-stat-value">
                        {typeof analytics.uniqueVisitors === 'number' ? analytics.uniqueVisitors.toLocaleString() : analytics.uniqueVisitors || '0'}
                      </div>
                      <div className="rs-stat-label">Unique Visitors</div>
                    </div>
                  </div>
                  <div className="rs-stat-card">
                    <div className="rs-stat-icon info">
                      <Activity size={22} />
                    </div>
                    <div className="rs-stat-content">
                      <div className="rs-stat-value">
                        {typeof analytics.humanClicks === 'number' ? analytics.humanClicks.toLocaleString() : analytics.humanClicks || '0'}
                      </div>
                      <div className="rs-stat-label">{humanRate ? `Human (${humanRate})` : 'Human'}</div>
                    </div>
                  </div>
                  <div className="rs-stat-card">
                    <div className="rs-stat-icon error">
                      <Bot size={22} />
                    </div>
                    <div className="rs-stat-content">
                      <div className="rs-stat-value">
                        {typeof analytics.botClicks === 'number' ? analytics.botClicks.toLocaleString() : analytics.botClicks || '0'}
                      </div>
                      <div className="rs-stat-label">{botRate ? `Bot (${botRate})` : 'Bot'}</div>
                    </div>
                  </div>
                </div>

                {/* Daily Clicks - Full Width Area Chart */}
                <div className="rs-section">
                  <div className="rs-chart-card">
                    <div className="rs-chart-header">
                      <h3 className="rs-chart-title">
                        <BarChart3 size={16} />
                        Clicks Over Time
                      </h3>
                      <p className="rs-chart-desc">Daily click trends for this route</p>
                    </div>
                    <div className="rs-chart-body" style={{ height: '280px' }}>
                      <ResponsiveContainer width="100%" height="100%">
                        <AreaChart data={analytics.dailyClicks}>
                          <defs>
                            <linearGradient id="routeClicksGradient" x1="0" y1="0" x2="0" y2="1">
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
                            fill="url(#routeClicksGradient)"
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
                <div className="rs-section-row two-cols">
                  <div className="rs-chart-card">
                    <div className="rs-chart-header">
                      <h3 className="rs-chart-title">
                        <Globe size={16} />
                        Geographic Distribution
                      </h3>
                      <p className="rs-chart-desc">Hover to see details</p>
                    </div>
                    <div className="rs-chart-body" style={{ height: '300px' }}>
                      <WorldMap data={analytics.topCountries} height={300} />
                    </div>
                  </div>
                  <div className="rs-chart-card">
                    <div className="rs-chart-header">
                      <h3 className="rs-chart-title">
                        <Globe size={16} />
                        Top Countries
                      </h3>
                      <p className="rs-chart-desc">By click volume</p>
                    </div>
                    <div className="rs-chart-body">
                      {analytics.topCountries.length > 0 ? (
                        <div className="rs-country-list">
                          {analytics.topCountries.map((country: any, index: number) => (
                            <div key={country.name} className="rs-country-row">
                              <span className="rs-country-rank">{index + 1}</span>
                              <span className="rs-country-name">{country.name}</span>
                              <div className="rs-country-bar-wrap">
                                <div className="rs-country-bar">
                                  <div
                                    className="rs-country-bar-fill"
                                    style={{ width: `${country.percentage || 0}%` }}
                                  />
                                </div>
                              </div>
                              <span className="rs-country-value">{(country.clicks || 0).toLocaleString()}</span>
                              <span className="rs-country-pct">{country.percentage || 0}%</span>
                            </div>
                          ))}
                        </div>
                      ) : (
                        <div className="rs-empty">
                          <div className="rs-empty-icon">
                            <Globe />
                          </div>
                          <p>No geographic data</p>
                        </div>
                      )}
                    </div>
                  </div>
                </div>

                {/* Distribution Section */}
                <div className="rs-section-row three-cols">
                  {/* Browser Distribution - Pie Chart */}
                  <div className="rs-chart-card">
                    <div className="rs-chart-header">
                      <h3 className="rs-chart-title">
                        <Globe size={16} />
                        Browsers
                      </h3>
                      <p className="rs-chart-desc">By browser</p>
                    </div>
                    <div className="rs-chart-body" style={{ height: '200px' }}>
                      {analytics.topBrowsers.length > 0 ? (
                        <ResponsiveContainer width="100%" height="100%">
                          <PieChart>
                            <Pie
                              data={analytics.topBrowsers}
                              cx="50%"
                              cy="50%"
                              outerRadius={70}
                              innerRadius={45}
                              dataKey="clicks"
                              nameKey="name"
                              paddingAngle={2}
                              stroke="var(--bg-primary)"
                              strokeWidth={2}
                            >
                              {analytics.topBrowsers.map((_: any, index: number) => (
                                <Cell key={`cell-${index}`} fill={CHART_COLORS[index % CHART_COLORS.length]} />
                              ))}
                            </Pie>
                            <Tooltip content={<CustomTooltip />} />
                          </PieChart>
                        </ResponsiveContainer>
                      ) : (
                        <div className="rs-empty">
                          <div className="rs-empty-icon">
                            <Activity />
                          </div>
                          <p>No browser data</p>
                        </div>
                      )}
                    </div>
                    {analytics.topBrowsers.length > 0 && (
                      <PieLegend
                        items={analytics.topBrowsers.map((entry: any, i: number) => ({
                          name: entry.name,
                          color: CHART_COLORS[i % CHART_COLORS.length],
                        }))}
                      />
                    )}
                  </div>

                  {/* Device Distribution - Pie Chart */}
                  <div className="rs-chart-card">
                    <div className="rs-chart-header">
                      <h3 className="rs-chart-title">
                        <Activity size={16} />
                        Devices
                      </h3>
                      <p className="rs-chart-desc">By device type</p>
                    </div>
                    <div className="rs-chart-body" style={{ height: '200px' }}>
                      {analytics.topDevices.length > 0 ? (
                        <ResponsiveContainer width="100%" height="100%">
                          <PieChart>
                            <Pie
                              data={analytics.topDevices}
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
                              {analytics.topDevices.map((_: any, index: number) => (
                                <Cell key={`cell-${index}`} fill={CHART_COLORS[index % CHART_COLORS.length]} />
                              ))}
                            </Pie>
                            <Tooltip content={<CustomTooltip />} />
                          </PieChart>
                        </ResponsiveContainer>
                      ) : (
                        <div className="rs-empty">
                          <div className="rs-empty-icon">
                            <Activity />
                          </div>
                          <p>No device data</p>
                        </div>
                      )}
                    </div>
                    {analytics.topDevices.length > 0 && (
                      <PieLegend
                        items={analytics.topDevices.map((entry: any, i: number) => ({
                          name: entry.device,
                          color: CHART_COLORS[i % CHART_COLORS.length],
                        }))}
                      />
                    )}
                  </div>

                  {/* Traffic Type - Pie Chart */}
                  <div className="rs-chart-card">
                    <div className="rs-chart-header">
                      <h3 className="rs-chart-title">
                        <Activity size={16} />
                        Traffic Type
                      </h3>
                      <p className="rs-chart-desc">Bot vs Human</p>
                    </div>
                    <div className="rs-chart-body" style={{ height: '200px' }}>
                      {analytics.trafficType.some((t: any) => t.value > 0) ? (
                        <ResponsiveContainer width="100%" height="100%">
                          <PieChart>
                            <Pie
                              data={analytics.trafficType}
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
                        <div className="rs-empty">
                          <div className="rs-empty-icon">
                            <Activity />
                          </div>
                          <p>No traffic data</p>
                        </div>
                      )}
                    </div>
                    {analytics.trafficType.some((t: any) => t.value > 0) && (
                      <PieLegend
                        items={analytics.trafficType.map((entry: any, i: number) => ({
                          name: entry.name,
                          color: TRAFFIC_COLORS[i],
                        }))}
                      />
                    )}
                  </div>
                </div>
              </div>
            ) : (
              <div className="welcome-content">
                <div className="welcome-icon">
                  <BarChart3 size={64} />
                </div>
                <h2>Route Analytics</h2>
                <p>Analytics for {selectedRoute.link} will be displayed here.</p>
                <div className="welcome-features">
                  <div className="feature-item">
                    <BarChart3 size={20} />
                    <span>Detailed Analytics</span>
                  </div>
                  <div className="feature-item">
                    <Edit size={20} />
                    <span>Performance Metrics</span>
                  </div>
                  <div className="feature-item">
                    <Search size={20} />
                    <span>Traffic Analysis</span>
                  </div>
                </div>
              </div>
            )}
          </div>
        ) : (
          <div className="welcome-content">
            <div className="welcome-icon">
              <BarChart3 size={64} />
            </div>
            <h2>Select a Route</h2>
            <p>Choose a route from the sidebar to view its analytics and performance metrics.</p>
            <div className="welcome-features">
              <div className="feature-item">
                <BarChart3 size={20} />
                <span>Detailed Analytics</span>
              </div>
              <div className="feature-item">
                <Edit size={20} />
                <span>Performance Metrics</span>
              </div>
              <div className="feature-item">
                <Search size={20} />
                <span>Traffic Analysis</span>
              </div>
            </div>
          </div>
        )}
      </div>

      </div>
    </>
  );
};

export default RoutesWithSidebar;
