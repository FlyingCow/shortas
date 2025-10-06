import React, { useState, useEffect } from 'react';
import { 
  Container, 
  Row, 
  Col, 
  Card, 
  Table, 
  Button, 
  Form, 
  InputGroup, 
  Badge, 
  Modal,
  Alert,
  ButtonGroup,
  Dropdown
} from 'react-bootstrap';
import { 
  Plus, 
  Edit, 
  Trash2, 
  ExternalLink, 
  Copy,
  Search,
  Filter,
  Link as LinkIcon
} from 'lucide-react';
import { apiService, RouteDto } from '../services/api';
import LoadingSpinner from './LoadingSpinner';
import './DesignSystem.css';

const Routes: React.FC = () => {
  const [routes, setRoutes] = useState<RouteDto[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const [searchTerm, setSearchTerm] = useState('');
  const [statusFilter, setStatusFilter] = useState('all');
  const [showCreateModal, setShowCreateModal] = useState(false);
  const [editingRoute, setEditingRoute] = useState<RouteDto | null>(null);

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

  const copyToClipboard = (text: string) => {
    navigator.clipboard.writeText(text);
    // You could add a toast notification here
  };

  const filteredRoutes = routes.filter(route => {
    const matchesSearch = route.link.toLowerCase().includes(searchTerm.toLowerCase()) ||
                         route.dest.toLowerCase().includes(searchTerm.toLowerCase());
    const matchesStatus = statusFilter === 'all' || route.status.toLowerCase() === statusFilter.toLowerCase();
    return matchesSearch && matchesStatus;
  });

  if (loading) {
    return <LoadingSpinner message="Loading routes..." />;
  }

  if (error) {
    return (
      <Alert variant="danger" className="text-center">
        <Alert.Heading>Error Loading Routes</Alert.Heading>
        <p>{error}</p>
        <Button variant="outline-danger" onClick={fetchRoutes}>
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
              <h2 className="mb-1">URL Routes</h2>
              <p className="text-muted mb-0">Manage your shortened URLs and redirects</p>
            </div>
            <Button 
              variant="primary"
              onClick={() => setShowCreateModal(true)}
            >
              <Plus size={16} className="me-2" />
              Create Route
            </Button>
          </div>
        </Col>
      </Row>

      {/* Filters */}
      <Row className="mb-4">
        <Col md={6}>
          <InputGroup>
            <InputGroup.Text>
              <Search size={16} />
            </InputGroup.Text>
            <Form.Control
              type="text"
              placeholder="Search routes..."
              value={searchTerm}
              onChange={(e) => setSearchTerm(e.target.value)}
            />
          </InputGroup>
        </Col>
        
        <Col md={3}>
          <InputGroup>
            <InputGroup.Text>
              <Filter size={16} />
            </InputGroup.Text>
            <Form.Select
              value={statusFilter}
              onChange={(e) => setStatusFilter(e.target.value)}
            >
              <option value="all">All Status</option>
              <option value="active">Active</option>
              <option value="inactive">Inactive</option>
              <option value="paused">Paused</option>
            </Form.Select>
          </InputGroup>
        </Col>
      </Row>

      {/* Routes Table */}
      <Row>
        <Col>
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
                            onClick={() => setEditingRoute(route)}
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
        </Col>
      </Row>

      {/* Modals */}
      <Modal show={showCreateModal} onHide={() => setShowCreateModal(false)} size="lg">
        <Modal.Header closeButton>
          <Modal.Title>Create New Route</Modal.Title>
        </Modal.Header>
        <Modal.Body>
          <p>Route creation form would go here...</p>
        </Modal.Body>
        <Modal.Footer>
          <Button variant="secondary" onClick={() => setShowCreateModal(false)}>
            Cancel
          </Button>
          <Button variant="primary">Create</Button>
        </Modal.Footer>
      </Modal>

      <Modal show={!!editingRoute} onHide={() => setEditingRoute(null)} size="lg">
        <Modal.Header closeButton>
          <Modal.Title>Edit Route</Modal.Title>
        </Modal.Header>
        <Modal.Body>
          <p>Route editing form would go here...</p>
        </Modal.Body>
        <Modal.Footer>
          <Button variant="secondary" onClick={() => setEditingRoute(null)}>
            Cancel
          </Button>
          <Button variant="primary">Save</Button>
        </Modal.Footer>
      </Modal>
    </>
  );
};

export default Routes;
