import React, { useState, useEffect } from 'react';
import { Plus, Trash2, Search, Globe, X, ExternalLink, Copy, Check } from 'lucide-react';
import { apiService, DomainDto, CreateDomainDto } from '../services/api';
import LoadingSpinner from './LoadingSpinner';
import './DesignSystem.css';

const domainStyles = `
.dom-toolbar {
  display: flex;
  align-items: center;
  gap: 0.75rem;
  margin-top: 1.5rem;
  margin-bottom: 1.25rem;
}

.dom-search {
  position: relative;
  flex: 1;
  max-width: 360px;
}

.dom-search svg {
  position: absolute;
  left: 0.75rem;
  top: 50%;
  transform: translateY(-50%);
  color: var(--text-muted);
  pointer-events: none;
}

.dom-search input {
  width: 100%;
  padding: 0.5rem 0.75rem 0.5rem 2.25rem;
  border: 1px solid var(--border-secondary);
  border-radius: var(--radius-md);
  background: var(--bg-primary);
  color: var(--text-primary);
  font-size: 0.8125rem;
  transition: border-color var(--transition-fast), box-shadow var(--transition-fast);
}

.dom-search input:focus {
  outline: none;
  border-color: var(--color-primary);
  box-shadow: 0 0 0 3px var(--color-primary-light);
}

.dom-search input::placeholder {
  color: var(--text-muted);
}

.dom-count {
  font-size: 0.8125rem;
  color: var(--text-muted);
  white-space: nowrap;
  margin-left: auto;
}

.dom-list {
  display: flex;
  flex-direction: column;
  gap: 0.5rem;
}

.dom-item {
  display: flex;
  align-items: center;
  gap: 1rem;
  padding: 1rem 1.25rem;
  background: var(--bg-primary);
  border: 1px solid var(--border-primary);
  border-radius: var(--radius-lg);
  transition: border-color var(--transition-fast), box-shadow var(--transition-fast);
}

.dom-item:hover {
  border-color: var(--border-secondary);
  box-shadow: 0 1px 4px rgba(0, 0, 0, 0.04);
}

.dom-icon {
  width: 36px;
  height: 36px;
  display: flex;
  align-items: center;
  justify-content: center;
  background: var(--primary-50, rgba(59, 130, 246, 0.08));
  border-radius: var(--radius-md);
  color: var(--color-primary);
  flex-shrink: 0;
}

.dom-details {
  flex: 1;
  min-width: 0;
}

.dom-name {
  font-size: 0.9375rem;
  font-weight: 600;
  color: var(--text-primary);
  word-break: break-all;
}

.dom-id {
  font-size: 0.6875rem;
  color: var(--text-muted);
  font-family: var(--font-mono, monospace);
  margin-top: 2px;
}

.dom-actions {
  display: flex;
  align-items: center;
  gap: 0.25rem;
  flex-shrink: 0;
}

.dom-action-btn {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  width: 32px;
  height: 32px;
  border: none;
  background: transparent;
  border-radius: var(--radius-sm);
  color: var(--text-muted);
  cursor: pointer;
  transition: all var(--transition-fast);
}

.dom-action-btn:hover {
  background: var(--bg-tertiary);
  color: var(--text-primary);
}

.dom-action-btn.danger:hover {
  background: rgba(239, 68, 68, 0.08);
  color: var(--color-error);
}

.dom-action-btn.copied {
  color: var(--color-success);
}

/* Empty state */
.dom-empty {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  padding: 4rem 2rem;
  text-align: center;
}

.dom-empty-icon {
  width: 56px;
  height: 56px;
  display: flex;
  align-items: center;
  justify-content: center;
  background: var(--bg-tertiary);
  border-radius: 50%;
  color: var(--text-muted);
  margin-bottom: 1rem;
}

.dom-empty h3 {
  margin: 0 0 0.375rem 0;
  font-size: 1rem;
  font-weight: 600;
  color: var(--text-primary);
}

.dom-empty p {
  margin: 0 0 1.25rem 0;
  font-size: 0.8125rem;
  color: var(--text-secondary);
  max-width: 320px;
}

/* Modal */
.dom-modal-overlay {
  position: fixed;
  inset: 0;
  background: rgba(0, 0, 0, 0.4);
  backdrop-filter: blur(4px);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 1000;
  padding: 1rem;
}

.dom-modal {
  background: var(--bg-elevated);
  border-radius: var(--radius-xl);
  box-shadow: var(--shadow-xl);
  width: 100%;
  max-width: 440px;
  overflow: hidden;
}

.dom-modal-header {
  padding: 1.25rem 1.5rem;
  border-bottom: 1px solid var(--border-primary);
  display: flex;
  align-items: center;
  justify-content: space-between;
}

.dom-modal-header h2 {
  margin: 0;
  font-size: 1rem;
  font-weight: 600;
  color: var(--text-primary);
}

.dom-modal-close {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 28px;
  height: 28px;
  border: none;
  background: transparent;
  border-radius: var(--radius-sm);
  color: var(--text-muted);
  cursor: pointer;
  transition: all var(--transition-fast);
}

.dom-modal-close:hover {
  background: var(--bg-tertiary);
  color: var(--text-primary);
}

.dom-modal-body {
  padding: 1.5rem;
}

.dom-field {
  display: flex;
  flex-direction: column;
  gap: 0.375rem;
}

.dom-field label {
  font-size: 0.8125rem;
  font-weight: 500;
  color: var(--text-secondary);
}

.dom-field input {
  padding: 0.625rem 0.75rem;
  border: 1px solid var(--border-secondary);
  border-radius: var(--radius-md);
  background: var(--bg-primary);
  color: var(--text-primary);
  font-size: 0.875rem;
  transition: border-color var(--transition-fast), box-shadow var(--transition-fast);
}

.dom-field input:focus {
  outline: none;
  border-color: var(--color-primary);
  box-shadow: 0 0 0 3px var(--color-primary-light);
}

.dom-field input::placeholder {
  color: var(--text-muted);
}

.dom-field .dom-hint {
  font-size: 0.6875rem;
  color: var(--text-muted);
}

.dom-modal-footer {
  padding: 1rem 1.5rem;
  border-top: 1px solid var(--border-primary);
  display: flex;
  gap: 0.5rem;
  justify-content: flex-end;
}
`;

const Domains: React.FC = () => {
  const [domains, setDomains] = useState<DomainDto[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const [searchTerm, setSearchTerm] = useState('');
  const [showModal, setShowModal] = useState(false);
  const [formName, setFormName] = useState('');
  const [saving, setSaving] = useState(false);
  const [copiedId, setCopiedId] = useState<string | null>(null);

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

  const handleCreate = () => {
    setFormName('');
    setShowModal(true);
  };

  const handleSave = async () => {
    const name = formName.trim().toLowerCase();
    if (!name) return;

    setSaving(true);
    try {
      await apiService.domains.create({ name });
      await fetchDomains();
      setShowModal(false);
      setFormName('');
    } catch (err: any) {
      console.error('Failed to save domain:', err);
      alert(err.response?.data?.message || 'Failed to create domain.');
    } finally {
      setSaving(false);
    }
  };

  const handleDelete = async (domain: DomainDto) => {
    if (!window.confirm(`Delete "${domain.name}"? Routes using this domain will be affected.`)) return;
    try {
      await apiService.domains.delete(domain.id);
      await fetchDomains();
    } catch (err: any) {
      console.error('Failed to delete domain:', err);
      alert(err.response?.data?.message || 'Failed to delete domain.');
    }
  };

  const handleCopy = (domain: DomainDto) => {
    navigator.clipboard.writeText(domain.name);
    setCopiedId(domain.id);
    setTimeout(() => setCopiedId(null), 1500);
  };

  const filteredDomains = domains.filter(d =>
    d.name.toLowerCase().includes(searchTerm.toLowerCase())
  );

  if (loading) return <LoadingSpinner />;

  if (error) {
    return (
      <div className="container" style={{ paddingTop: '1.5rem' }}>
        <div className="alert alert-error">
          <h3>Error Loading Domains</h3>
          <p>{error}</p>
          <button className="btn btn-primary" onClick={fetchDomains}>Retry</button>
        </div>
      </div>
    );
  }

  return (
    <>
      <style>{domainStyles}</style>
      <div className="container">
        {/* Toolbar */}
        <div className="dom-toolbar">
          <div className="dom-search">
            <Search size={15} />
            <input
              type="text"
              placeholder="Search domains..."
              value={searchTerm}
              onChange={(e) => setSearchTerm(e.target.value)}
            />
          </div>
          <span className="dom-count">{filteredDomains.length} domain{filteredDomains.length !== 1 ? 's' : ''}</span>
          <button className="btn btn-primary btn-sm" onClick={handleCreate}>
            <Plus size={16} />
            Add Domain
          </button>
        </div>

        {/* Domain list */}
        {filteredDomains.length > 0 ? (
          <div className="dom-list">
            {filteredDomains.map((domain) => (
              <div key={domain.id} className="dom-item">
                <div className="dom-icon">
                  <Globe size={18} />
                </div>
                <div className="dom-details">
                  <div className="dom-name">{domain.name}</div>
                  <div className="dom-id">{domain.id}</div>
                </div>
                <div className="dom-actions">
                  <button
                    className={`dom-action-btn ${copiedId === domain.id ? 'copied' : ''}`}
                    onClick={() => handleCopy(domain)}
                    title="Copy domain name"
                  >
                    {copiedId === domain.id ? <Check size={15} /> : <Copy size={15} />}
                  </button>
                  <button
                    className="dom-action-btn"
                    onClick={() => window.open(`https://${domain.name}`, '_blank')}
                    title="Open in browser"
                  >
                    <ExternalLink size={15} />
                  </button>
                  <button
                    className="dom-action-btn danger"
                    onClick={() => handleDelete(domain)}
                    title="Delete domain"
                  >
                    <Trash2 size={15} />
                  </button>
                </div>
              </div>
            ))}
          </div>
        ) : (
          <div className="dom-empty">
            <div className="dom-empty-icon">
              <Globe size={24} />
            </div>
            <h3>{searchTerm ? 'No matches' : 'No domains yet'}</h3>
            <p>
              {searchTerm
                ? 'Try a different search term.'
                : 'Add your first custom domain to start creating short links.'}
            </p>
            {!searchTerm && (
              <button className="btn btn-primary btn-sm" onClick={handleCreate}>
                <Plus size={16} />
                Add Domain
              </button>
            )}
          </div>
        )}

        {/* Create modal */}
        {showModal && (
          <div className="dom-modal-overlay" onClick={() => !saving && setShowModal(false)}>
            <div className="dom-modal" onClick={(e) => e.stopPropagation()}>
              <div className="dom-modal-header">
                <h2>Add Domain</h2>
                <button className="dom-modal-close" onClick={() => !saving && setShowModal(false)}>
                  <X size={16} />
                </button>
              </div>
              <div className="dom-modal-body">
                <div className="dom-field">
                  <label>Domain name</label>
                  <input
                    type="text"
                    value={formName}
                    onChange={(e) => setFormName(e.target.value.toLowerCase())}
                    onKeyDown={(e) => e.key === 'Enter' && handleSave()}
                    placeholder="links.example.com"
                    autoFocus
                    disabled={saving}
                  />
                  <span className="dom-hint">Enter the fully qualified domain name. It will be lowercased automatically.</span>
                </div>
              </div>
              <div className="dom-modal-footer">
                <button className="btn btn-outline btn-sm" onClick={() => setShowModal(false)} disabled={saving}>
                  Cancel
                </button>
                <button className="btn btn-primary btn-sm" onClick={handleSave} disabled={saving || !formName.trim()}>
                  {saving ? 'Adding...' : 'Add Domain'}
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
