import React, { useState, useEffect } from 'react';
import { Plus, Edit, Trash2, Search, Globe } from 'lucide-react';
import { apiService, DomainDto, CreateDomainDto } from '../services/api';
import LoadingSpinner from './LoadingSpinner';
import './DesignSystem.css';

const domainStyles = `
.modal-content {
  background: var(--bg-elevated);
  border-radius: var(--radius-xl);
  box-shadow: var(--shadow-xl);
  width: 100%;
  max-width: 500px;
  max-height: 90vh;
  overflow: hidden;
  display: flex;
  flex-direction: column;
}

.modal-header {
  padding: var(--space-lg);
  border-bottom: 1px solid var(--border-primary);
  display: flex;
  align-items: center;
  justify-content: space-between;
}

.modal-header h2 {
  margin: 0;
  font-size: var(--font-size-xl);
  font-weight: var(--font-weight-semibold);
  color: var(--text-primary);
}

.modal-close {
  background: none;
  border: none;
  font-size: 2rem;
  line-height: 1;
  color: var(--text-secondary);
  cursor: pointer;
  padding: 0;
  width: 32px;
  height: 32px;
  display: flex;
  align-items: center;
  justify-content: center;
  border-radius: var(--radius-md);
  transition: all var(--transition-fast);
}

.modal-close:hover {
  background: var(--bg-tertiary);
  color: var(--text-primary);
}

.modal-body {
  padding: var(--space-lg);
  overflow-y: auto;
  flex: 1;
}

.modal-footer {
  padding: var(--space-lg);
  border-top: 1px solid var(--border-primary);
  display: flex;
  gap: var(--space-md);
  justify-content: flex-end;
}

.form-group {
  display: flex;
  flex-direction: column;
  gap: 0.5rem;
  margin-bottom: 1.5rem;
}

.form-group:last-child {
  margin-bottom: 0;
}

.form-group label {
  font-weight: 500;
  color: var(--text-primary);
  font-size: 0.875rem;
}

.form-group input,
.form-group textarea {
  padding: 0.75rem;
  border: 1px solid var(--border-primary);
  border-radius: 0;
  background: var(--bg-secondary);
  color: var(--text-primary);
  font-size: 0.875rem;
  transition: all var(--transition-normal);
}

.form-group input:focus,
.form-group textarea:focus {
  outline: none;
  border-color: var(--color-primary);
  box-shadow: 0 0 0 3px var(--color-primary-alpha);
}

.form-group input::placeholder,
.form-group textarea::placeholder {
  color: var(--text-tertiary);
}

.form-group small {
  font-size: 0.75rem;
  color: var(--text-tertiary);
}

.table-controls {
  display: flex;
  gap: 1rem;
  align-items: center;
  justify-content: space-between;
  margin-bottom: 1.5rem;
}

.table-controls .search-box {
  margin-bottom: 0;
  flex: 1;
  max-width: 400px;
}

.control-group {
  display: flex;
  gap: 0.5rem;
  align-items: center;
}

.control-label {
  font-size: 0.875rem;
  color: var(--text-secondary);
  font-weight: 500;
}

.domains-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(300px, 1fr));
  gap: 1.5rem;
  margin-top: 1.5rem;
}

.domain-card {
  padding: 1.5rem;
  transition: all 0.2s ease;
}

.domain-card:hover {
  box-shadow: 0 2px 6px rgba(0, 0, 0, 0.06);
}

.domain-card-header {
  display: flex;
  align-items: flex-start;
  gap: 1rem;
  margin-bottom: 1.5rem;
}

.domain-icon {
  width: 48px;
  height: 48px;
  display: flex;
  align-items: center;
  justify-content: center;
  background: var(--color-primary);
  border-radius: 12px;
  color: white;
  flex-shrink: 0;
}

.domain-info {
  flex: 1;
  min-width: 0;
}

.domain-name {
  font-size: 1.125rem;
  font-weight: 600;
  margin: 0 0 0.25rem 0;
  color: var(--text-primary);
  word-break: break-all;
}

.domain-id {
  font-size: 0.75rem;
  color: var(--text-secondary);
  font-family: monospace;
}

.domain-card-actions {
  display: flex;
  gap: 0.5rem;
  justify-content: flex-end;
}

.empty-state-full {
  grid-column: 1 / -1;
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  padding: 4rem 2rem;
  text-align: center;
  color: var(--text-secondary);
}

.empty-state-full svg {
  color: var(--text-tertiary);
  margin-bottom: 1rem;
}

.empty-state-full h3 {
  margin: 0 0 0.5rem 0;
  color: var(--text-primary);
}

.empty-state-full p {
  margin: 0 0 1.5rem 0;
  max-width: 400px;
}
`;

const Domains: React.FC = () => {
  const [domains, setDomains] = useState<DomainDto[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const [searchTerm, setSearchTerm] = useState('');
  const [showModal, setShowModal] = useState(false);
  const [editingDomain, setEditingDomain] = useState<DomainDto | null>(null);
  const [formData, setFormData] = useState<CreateDomainDto>({ name: '' });

  useEffect(() => {
    fetchDomains();
  }, []);

  const fetchDomains = async () => {
    try {
      setLoading(true);
      setError(null);
      const response = await apiService.domains.list({ page: 1, pageSize: 100 });
      setDomains(response.data);
    } catch (err) {
      console.error('Failed to fetch domains:', err);
      setError('Failed to load domains. Please try again.');
    } finally {
      setLoading(false);
    }
  };

  const handleCreateDomain = () => {
    setEditingDomain(null);
    setFormData({ name: '' });
    setShowModal(true);
  };

  const handleEditDomain = (domain: DomainDto) => {
    setEditingDomain(domain);
    setFormData({ name: domain.name });
    setShowModal(true);
  };

  const handleSaveDomain = async () => {
    try {
      if (!formData.name.trim()) {
        alert('Domain name is required');
        return;
      }

      if (editingDomain) {
        await apiService.domains.update(editingDomain.id, formData);
      } else {
        await apiService.domains.create(formData);
      }

      await fetchDomains();
      setShowModal(false);
      setFormData({ name: '' });
      setEditingDomain(null);
    } catch (err: any) {
      console.error('Failed to save domain:', err);
      const errorMessage = err.response?.data?.message || 'Failed to save domain. Please try again.';
      alert(errorMessage);
    }
  };

  const handleDeleteDomain = async (domain: DomainDto) => {
    if (!window.confirm(`Are you sure you want to delete the domain "${domain.name}"?`)) {
      return;
    }

    try {
      await apiService.domains.delete(domain.id);
      await fetchDomains();
    } catch (err: any) {
      console.error('Failed to delete domain:', err);
      const errorMessage = err.response?.data?.message || 'Failed to delete domain. Please try again.';
      alert(errorMessage);
    }
  };

  const handleCloseModal = () => {
    setShowModal(false);
    setFormData({ name: '' });
    setEditingDomain(null);
  };

  const filteredDomains = domains.filter(domain =>
    domain.name.toLowerCase().includes(searchTerm.toLowerCase())
  );

  if (loading) {
    return <LoadingSpinner />;
  }

  if (error) {
    return (
      <div className="alert alert-error">
        <h3>Error Loading Domains</h3>
        <p>{error}</p>
        <button className="btn btn-primary" onClick={fetchDomains}>
          Retry
        </button>
      </div>
    );
  }

  return (
    <>
      <style>{domainStyles}</style>
      <div className="container">
        <div className="page-header" style={{ marginTop: '0.5rem' }}>
        <div />
        <button className="btn btn-primary" onClick={handleCreateDomain}>
          <Plus size={20} />
          Add Domain
        </button>
      </div>

      {/* Search and Filters */}
      <div className="table-controls">
        <div className="search-box">
          <Search size={16} />
          <input
            type="text"
            placeholder="Search domains..."
            value={searchTerm}
            onChange={(e) => setSearchTerm(e.target.value)}
          />
        </div>
        <div className="control-group">
          <span className="control-label">Total: {domains.length}</span>
        </div>
      </div>

      {/* Domains Grid */}
      <div className="domains-grid">
        {filteredDomains.length > 0 ? (
          filteredDomains.map((domain) => (
            <div key={domain.id} className="domain-card card">
              <div className="domain-card-header">
                <div className="domain-icon">
                  <Globe size={24} />
                </div>
                <div className="domain-info">
                  <h3 className="domain-name">{domain.name}</h3>
                  <span className="domain-id">{domain.id.substring(0, 8)}...</span>
                </div>
              </div>
              <div className="domain-card-actions">
                <button
                  className="btn btn-outline btn-sm"
                  onClick={() => handleEditDomain(domain)}
                  title="Edit domain"
                >
                  <Edit size={16} />
                  Edit
                </button>
                <button
                  className="btn btn-ghost btn-sm"
                  onClick={() => handleDeleteDomain(domain)}
                  title="Delete domain"
                >
                  <Trash2 size={16} />
                  Delete
                </button>
              </div>
            </div>
          ))
        ) : (
          <div className="empty-state-full">
            <Globe size={64} />
            <h3>No domains found</h3>
            <p>
              {searchTerm
                ? 'No domains match your search criteria.'
                : 'Create your first domain to get started.'}
            </p>
            {!searchTerm && (
              <button className="btn btn-primary" onClick={handleCreateDomain}>
                <Plus size={20} />
                Add Domain
              </button>
            )}
          </div>
        )}
      </div>

      {/* Domain Modal */}
      {showModal && (
        <div className="modal-overlay" onClick={handleCloseModal}>
          <div className="modal-content" onClick={(e) => e.stopPropagation()}>
            <div className="modal-header">
              <h2>{editingDomain ? 'Edit Domain' : 'Create New Domain'}</h2>
              <button className="modal-close" onClick={handleCloseModal}>
                ×
              </button>
            </div>

            <div className="modal-body">
              <div className="form-group">
                <label>Domain Name *</label>
                <input
                  type="text"
                  value={formData.name}
                  onChange={(e) =>
                    setFormData({ ...formData, name: e.target.value.toLowerCase() })
                  }
                  placeholder="example.com"
                  autoFocus
                />
                <small>Domain names are automatically converted to lowercase</small>
              </div>
            </div>

            <div className="modal-footer">
              <button className="btn btn-outline" onClick={handleCloseModal}>
                Cancel
              </button>
              <button className="btn btn-primary" onClick={handleSaveDomain}>
                {editingDomain ? 'Update Domain' : 'Create Domain'}
              </button>
            </div>
          </div>
        </div>
      )}
      </div>
    </>
  );
};

export default Domains;
