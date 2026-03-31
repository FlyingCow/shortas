import React, { useState, useEffect } from 'react';
import {
  Plus,
  Edit,
  Trash2,
  ExternalLink,
  Copy,
  Search,
  Filter,
  Link as LinkIcon,
  Settings,
  ChevronLeft,
  ChevronRight,
} from 'lucide-react';
// Removed Bootstrap Dropdown imports - using unified controls
import { apiService, RouteDto, PaginatedResponse, RoutingPolicy, RouteSearchResult } from '../services/api';
import { useAlert } from '../contexts/AlertContext';
import LoadingSpinner from './LoadingSpinner';
import RouteFormModal from './RouteFormModal';
import './DesignSystem.css';

const Routes: React.FC = () => {
  const { showToast, showConfirm } = useAlert();
  const [routes, setRoutes] = useState<RouteDto[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const [searchTerm, setSearchTerm] = useState('');
  const [statusFilter, setStatusFilter] = useState('all');
  const [currentPage, setCurrentPage] = useState(1);
  const [pageSize] = useState(20);
  const [totalPages, setTotalPages] = useState(1);
  const [totalCount, setTotalCount] = useState(0);
  const [showCreateModal, setShowCreateModal] = useState(false);
  const [editingRoute, setEditingRoute] = useState<RouteDto | null>(null);
  const [showEditModal, setShowEditModal] = useState(false);
  const [isSearchMode, setIsSearchMode] = useState(false);

  useEffect(() => {
    fetchRoutes();
  }, [currentPage, statusFilter]);

  const fetchRoutes = async (overrideSearch?: string) => {
    try {
      setLoading(true);
      setError(null);
      const effectiveSearch = overrideSearch !== undefined ? overrideSearch : searchTerm;

      if (effectiveSearch.trim()) {
        // Use Elasticsearch full-text search
        const response = await apiService.routes.search({
          q: effectiveSearch.trim(),
          page: currentPage,
          pageSize,
        });
        // Map search results to RouteDto-compatible objects for display
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
          domain: r.domainName ? { id: '', name: r.domainName, ownerId: '', verificationStatus: 'Verified' as const, verificationReason: '' } : undefined,
        }));
        setRoutes(mapped);
        setTotalPages(response.pagination.totalPages);
        setTotalCount(response.pagination.totalCount);
        setIsSearchMode(true);
      } else {
        // Use standard list endpoint
        const params: any = {
          page: currentPage,
          pageSize,
        };

        if (statusFilter !== 'all') {
          params.status = statusFilter;
        }

        const response: PaginatedResponse<RouteDto> = await apiService.routes.list(params);
        setRoutes(response.data);
        setTotalPages(response.pagination.totalPages);
        setTotalCount(response.pagination.totalCount);
        setIsSearchMode(false);
      }
    } catch (err) {
      console.error('Failed to fetch routes:', err);
      setError('Failed to load routes. Please try again.');
    } finally {
      setLoading(false);
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
    } catch (err) {
      console.error('Failed to delete route:', err);
      showToast('Failed to delete route. Please try again.', 'error');
    }
  };

  const handleEditRoute = (route: RouteDto) => {
    setEditingRoute(route);
    setShowEditModal(true);
  };

  const handleSaveRoute = async (routeData: Partial<RouteDto>) => {
    try {
      // RouteForm already strips API-managed fields and validates
      if (editingRoute?.id) {
        await apiService.routes.update(editingRoute.id, routeData);
      } else {
        await apiService.routes.create(routeData);
      }
      await fetchRoutes();
      setShowEditModal(false);
      setEditingRoute(null);
    } catch (err) {
      console.error('Failed to save route:', err);
      throw err; // Re-throw so RouteForm can show inline error
    }
  };

  const handleCreateRoute = () => {
    setEditingRoute(null);
    setShowEditModal(true);
  };

  const handleSearch = () => {
    setCurrentPage(1);
    fetchRoutes();
  };


  const copyToClipboard = (text: string) => {
    navigator.clipboard.writeText(text);
    // You could add a toast notification here
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

  const getPolicyBadgeClass = (policyType: string): string => {
    switch (policyType) {
      case 'Conditional':
        return 'table-status-info';
      case 'Challenge':
        return 'table-status-warning';
      case 'File':
        return 'table-status-secondary';
      case 'Mirroring':
        return 'table-status-info';
      default:
        return 'table-status-secondary';
    }
  };

  if (loading) {
    return <LoadingSpinner message="Loading routes..." />;
  }

  if (error) {
    return (
      <div className="alert alert-error">
        <h3>Error Loading Routes</h3>
        <p>{error}</p>
        <button className="btn btn-primary" onClick={() => fetchRoutes()}>
          Retry
        </button>
      </div>
    );
  }

  return (
    <div className="container">
      {/* Page Header */}
      <div className="page-header">
        <h1 className="page-title">URL Routes</h1>
        <p className="page-subtitle">Manage your shortened URLs and redirects</p>
      </div>

      {/* Actions Bar */}
      <div className="card mb-lg">
        <div className="card-body">
          <div className="flex items-center justify-between">
            <div className="flex gap-sm">
              <button 
                className="btn btn-primary"
                onClick={handleCreateRoute}
              >
                <Plus size={16} />
                Create Route
              </button>
            </div>
            
            <div className="control-group">
              <div className="control-input">
                <Search size={16} className="input-icon" />
                <input
                  type="text"
                  placeholder="Search by link, domain, or destination..."
                  value={searchTerm}
                  onChange={(e) => setSearchTerm(e.target.value)}
                  onKeyDown={(e) => e.key === 'Enter' && handleSearch()}
                  style={{ minWidth: '280px' }}
                />
              </div>

              <button
                className="btn btn-secondary"
                onClick={handleSearch}
              >
                Search
              </button>

              <div className="control-select">
                <select
                  value={statusFilter}
                  onChange={(e) => setStatusFilter(e.target.value)}
                >
                  <option value="all">All Status</option>
                  <option value="active">Active</option>
                  <option value="inactive">Inactive</option>
                </select>
              </div>
            </div>
          </div>
        </div>
      </div>

      {/* Routes Table */}
      <div className="card">
        <div className="card-header">
          <h3 className="card-title">Routes ({totalCount})</h3>
          <p className="card-subtitle">Manage your URL redirects and short links</p>
        </div>
        <div className="card-body p-0">
          <div className="table-container">
            <div className="table-wrapper">
              <table className="unified-table">
                <thead>
                  <tr>
                    <th>Short URL</th>
                    <th>Domain</th>
                    <th>Destination</th>
                    {!isSearchMode && <th>Policy</th>}
                    <th>Status</th>
                    {!isSearchMode && <th>Code</th>}
                    {!isSearchMode && <th>TTL</th>}
                    <th>Actions</th>
                  </tr>
                </thead>
                <tbody>
                  {routes.map((route) => (
                    <tr key={route.id || route.link}>
                      <td>
                        <div className="table-cell-content">
                          <span className="table-url">{route.link}</span>
                          <button
                            className="table-action-btn"
                            onClick={() => copyToClipboard(route.link)}
                            title="Copy to clipboard"
                          >
                            <Copy size={14} />
                          </button>
                        </div>
                      </td>
                      <td>
                        <span className="table-cell-secondary">
                          {route.domain?.name || '—'}
                        </span>
                      </td>
                      <td>
                        <div className="table-cell-content">
                          <span 
                            className="table-cell-primary" 
                            title={route.dest}
                          >
                            {route.dest.length > 50 ? `${route.dest.substring(0, 50)}...` : route.dest}
                          </span>
                          {route.dest && (
                            <button
                              className="table-action-btn table-action-btn-primary"
                              onClick={() => window.open(route.dest, '_blank')}
                              title="Open destination"
                            >
                              <ExternalLink size={14} />
                            </button>
                          )}
                        </div>
                      </td>
                      {!isSearchMode && (
                        <td>
                          <span className={`table-status-badge ${getPolicyBadgeClass(getPolicyType(route.policy))}`}>
                            {getPolicyType(route.policy)}
                          </span>
                        </td>
                      )}
                      <td>
                        <span className={`table-status-badge ${
                          route.status.toLowerCase() === 'active' ? 'table-status-success' :
                          route.status.toLowerCase() === 'inactive' ? 'table-status-error' :
                          'table-status-warning'
                        }`}>
                          {route.status}
                        </span>
                      </td>
                      {!isSearchMode && (
                        <td>
                          <span className="table-status-badge table-status-secondary">
                            {route.code}
                          </span>
                        </td>
                      )}
                      {!isSearchMode && (
                        <td>
                          <span className="table-metric">{route.ttl}s</span>
                        </td>
                      )}
                      <td>
                        <div className="table-action-buttons">
                          <button
                            className="table-action-btn table-action-btn-primary"
                            onClick={() => handleEditRoute(route)}
                            title="Edit route"
                          >
                            <Edit size={14} />
                          </button>
                          <button
                            className="table-action-btn table-action-btn-danger"
                            onClick={() => handleDeleteRoute(route)}
                            title="Delete route"
                          >
                            <Trash2 size={14} />
                          </button>
                        </div>
                      </td>
                    </tr>
                  ))}
                </tbody>
              </table>
              
              {routes.length === 0 && (
                <div className="table-empty">
                  <div className="table-empty-icon">
                    <LinkIcon size={48} />
                  </div>
                  <div className="table-empty-title">No routes found</div>
                  <div className="table-empty-description">
                    {searchTerm ? 'No routes match your search criteria.' : 'Create your first URL route to get started.'}
                  </div>
                  {searchTerm && (
                    <button
                      className="btn btn-secondary"
                      onClick={() => {
                        setSearchTerm('');
                        setIsSearchMode(false);
                        setCurrentPage(1);
                        fetchRoutes('');
                      }}
                    >
                      Clear search
                    </button>
                  )}
                </div>
              )}
            </div>
          </div>
        </div>

        {/* Pagination */}
        {totalPages > 1 && (
          <div className="card-footer">
            <div className="flex items-center justify-between">
              <div className="text-sm">
                Showing {(currentPage - 1) * pageSize + 1} to {Math.min(currentPage * pageSize, totalCount)} of {totalCount} routes
              </div>
              <div className="flex gap-sm">
                <button
                  className="btn btn-secondary"
                  onClick={() => setCurrentPage(Math.max(1, currentPage - 1))}
                  disabled={currentPage === 1}
                >
                  <ChevronLeft size={16} />
                  Previous
                </button>
                <div className="flex items-center gap-xs px-md">
                  Page {currentPage} of {totalPages}
                </div>
                <button
                  className="btn btn-secondary"
                  onClick={() => setCurrentPage(Math.min(totalPages, currentPage + 1))}
                  disabled={currentPage === totalPages}
                >
                  Next
                  <ChevronRight size={16} />
                </button>
              </div>
            </div>
          </div>
        )}
      </div>

      {/* Route Form Modal */}
      <RouteFormModal
        show={showEditModal}
        onClose={() => {
          setShowEditModal(false);
          setEditingRoute(null);
        }}
        onSave={handleSaveRoute}
        route={editingRoute}
      />
    </div>
  );
};

export default Routes;
