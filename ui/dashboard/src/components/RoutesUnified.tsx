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
} from 'lucide-react';
// Removed Bootstrap Dropdown imports - using unified controls
import { apiService, RouteDto } from '../services/api';
import LoadingSpinner from './LoadingSpinner';
import RouteEditModal from './RouteEditModal';
import './DesignSystem.css';

const Routes: React.FC = () => {
  const [routes, setRoutes] = useState<RouteDto[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const [searchTerm, setSearchTerm] = useState('');
  const [statusFilter, setStatusFilter] = useState('all');
  const [showCreateModal, setShowCreateModal] = useState(false);
  const [editingRoute, setEditingRoute] = useState<RouteDto | null>(null);
  const [showEditModal, setShowEditModal] = useState(false);

  useEffect(() => {
    fetchRoutes();
  }, []);

  const fetchRoutes = async () => {
    try {
      setLoading(true);
      setError(null);
      const data = await apiService.routes.list({ limit: 100 });
      setRoutes(data);
    } catch (err) {
      console.error('Failed to fetch routes:', err);
      setError('Failed to load routes. Please try again.');
    } finally {
      setLoading(false);
    }
  };

  const handleDeleteRoute = async (route: RouteDto) => {
    if (!window.confirm(`Are you sure you want to delete the route "${route.link}"?`)) {
      return;
    }

    try {
      await apiService.routes.delete(route.switch, route.properties.domain_id, route.link);
      await fetchRoutes();
    } catch (err) {
      console.error('Failed to delete route:', err);
      alert('Failed to delete route. Please try again.');
    }
  };

  const handleEditRoute = (route: RouteDto) => {
    setEditingRoute(route);
    setShowEditModal(true);
  };

  const handleSaveRoute = async (routeData: any) => {
    try {
      if (editingRoute) {
        // Update existing route
        await apiService.routes.update(editingRoute.switch, editingRoute.properties.domain_id, editingRoute.link, routeData);
      } else {
        // Create new route
        await apiService.routes.create(routeData);
      }
      await fetchRoutes();
      setShowEditModal(false);
      setEditingRoute(null);
    } catch (err) {
      console.error('Failed to save route:', err);
      alert('Failed to save route. Please try again.');
    }
  };

  const handleCreateRoute = () => {
    setEditingRoute(null);
    setShowEditModal(true);
  };


  const copyToClipboard = (text: string) => {
    navigator.clipboard.writeText(text);
    // You could add a toast notification here
  };

  const filteredRoutes = routes.filter(
    (route) =>
      (searchTerm === '' || 
       route.link.toLowerCase().includes(searchTerm.toLowerCase()) ||
       route.dest.toLowerCase().includes(searchTerm.toLowerCase())) &&
      (statusFilter === 'all' || route.status.toLowerCase() === statusFilter.toLowerCase())
  );

  if (loading) {
    return <LoadingSpinner message="Loading routes..." />;
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
                  placeholder="Search routes..."
                  value={searchTerm}
                  onChange={(e) => setSearchTerm(e.target.value)}
                  style={{ minWidth: '200px' }}
                />
              </div>
              
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
          <h3 className="card-title">Routes ({filteredRoutes.length})</h3>
          <p className="card-subtitle">Manage your URL redirects and short links</p>
        </div>
        <div className="card-body p-0">
          <div className="table-container">
            <div className="table-wrapper">
              <table className="unified-table">
                <thead>
                  <tr>
                    <th>Short URL</th>
                    <th>Destination</th>
                    <th>Status</th>
                    <th>Code</th>
                    <th>TTL</th>
                    <th>Actions</th>
                  </tr>
                </thead>
                <tbody>
                  {filteredRoutes.map((route) => (
                    <tr key={`${route.switch}-${route.properties.domain_id}-${route.link}`}>
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
                        <div className="table-cell-content">
                          <span 
                            className="table-cell-primary" 
                            title={route.dest}
                          >
                            {route.dest.length > 50 ? `${route.dest.substring(0, 50)}...` : route.dest}
                          </span>
                          <button
                            className="table-action-btn table-action-btn-primary"
                            onClick={() => window.open(route.dest, '_blank')}
                            title="Open destination"
                          >
                            <ExternalLink size={14} />
                          </button>
                        </div>
                      </td>
                      <td>
                        <span className={`table-status-badge ${
                          route.status.toLowerCase() === 'active' ? 'table-status-success' :
                          route.status.toLowerCase() === 'inactive' ? 'table-status-error' :
                          'table-status-warning'
                        }`}>
                          {route.status}
                        </span>
                      </td>
                      <td>
                        <span className="table-status-badge table-status-secondary">
                          {route.code}
                        </span>
                      </td>
                      <td>
                        <span className="table-metric">{route.ttl}s</span>
                      </td>
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
              
              {filteredRoutes.length === 0 && (
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
                      className="table-action-btn"
                      onClick={() => setSearchTerm('')}
                    >
                      Clear search
                    </button>
                  )}
                </div>
              )}
            </div>
          </div>
        </div>
      </div>

      {/* Route Edit Modal */}
      <RouteEditModal
        show={showEditModal}
        onHide={() => {
          setShowEditModal(false);
          setEditingRoute(null);
        }}
        route={editingRoute}
        onSave={handleSaveRoute}
      />

    </div>
  );
};

export default Routes;
