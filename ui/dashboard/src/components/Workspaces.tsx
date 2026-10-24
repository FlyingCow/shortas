import React, { useState, useEffect } from 'react';
import { Plus, Edit, Trash2, Search, Briefcase, Calendar, Shield, X } from 'lucide-react';
import { apiService, WorkspaceDto, CreateWorkspaceDto } from '../services/api';
import { useAlert } from '../contexts/AlertContext';
import LoadingSpinner from './LoadingSpinner';
import './DesignSystem.css';

// Workspace-specific widgets with no shared DesignSystem.css equivalent —
// everything else (page header, toolbar, search box, card, badge, modal,
// form fields, empty state) uses the canonical classes directly.
const workspaceStyles = `
.ws-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(340px, 1fr));
  gap: 0.75rem;
}

.ws-card-top {
  display: flex;
  align-items: flex-start;
  gap: 0.875rem;
  margin-bottom: 0.75rem;
}

.ws-icon {
  width: 36px;
  height: 36px;
  display: flex;
  align-items: center;
  justify-content: center;
  border-radius: var(--radius-md);
  flex-shrink: 0;
}

.ws-icon-user {
  background: var(--primary-50, rgba(59, 130, 246, 0.08));
  color: var(--color-primary);
}

.ws-icon-system {
  background: rgba(139, 92, 246, 0.08);
  color: #8b5cf6;
}

.ws-info {
  flex: 1;
  min-width: 0;
}

.ws-name {
  font-size: 0.9375rem;
  font-weight: 600;
  color: var(--text-primary);
  margin: 0 0 0.25rem 0;
  display: flex;
  align-items: center;
  gap: 0.5rem;
}

.ws-desc {
  font-size: 0.8125rem;
  color: var(--text-secondary);
  line-height: 1.5;
  margin: 0;
  display: -webkit-box;
  -webkit-line-clamp: 2;
  -webkit-box-orient: vertical;
  overflow: hidden;
}

.ws-footer {
  display: flex;
  align-items: center;
  justify-content: space-between;
  margin-top: auto;
  padding-top: 0.75rem;
  border-top: 1px solid var(--border-primary);
}

.ws-meta {
  display: flex;
  align-items: center;
  gap: 0.375rem;
  font-size: 0.6875rem;
  color: var(--text-muted);
}

.ws-actions {
  display: flex;
  gap: 0.25rem;
}

.ws-action-btn {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  width: 30px;
  height: 30px;
  border: none;
  background: transparent;
  border-radius: var(--radius-sm);
  color: var(--text-muted);
  cursor: pointer;
  transition: all var(--transition-fast);
}

.ws-action-btn:hover {
  background: var(--bg-tertiary);
  color: var(--text-primary);
}

.ws-action-btn.danger:hover {
  background: rgba(239, 68, 68, 0.08);
  color: var(--color-error);
}
`;

/* ---------- Modal component ---------- */
interface WorkspaceModalProps {
  show: boolean;
  workspace?: WorkspaceDto;
  onClose: () => void;
  onSave: (data: CreateWorkspaceDto) => Promise<void>;
}

const WorkspaceModal: React.FC<WorkspaceModalProps> = ({ show, workspace, onClose, onSave }) => {
  const { showToast } = useAlert();
  const [name, setName] = useState('');
  const [description, setDescription] = useState('');
  const [saving, setSaving] = useState(false);

  useEffect(() => {
    if (show) {
      setName(workspace?.name || '');
      setDescription(workspace?.description || '');
      setSaving(false);
    }
  }, [workspace, show]);

  useEffect(() => {
    const onKey = (e: KeyboardEvent) => {
      if (e.key === 'Escape' && show && !saving) onClose();
    };
    document.addEventListener('keydown', onKey);
    return () => document.removeEventListener('keydown', onKey);
  }, [show, saving, onClose]);

  const handleSubmit = async () => {
    if (!name.trim()) return;
    setSaving(true);
    try {
      await onSave({ name: name.trim(), description: description.trim() });
      onClose();
    } catch {
      showToast('Failed to save workspace. Please try again.', 'error');
    } finally {
      setSaving(false);
    }
  };

  if (!show) return null;

  return (
    <div className="modal-overlay" onClick={() => !saving && onClose()}>
      <div className="modal" style={{ maxWidth: 460 }} onClick={(e) => e.stopPropagation()}>
        <div className="modal-header">
          <h3>{workspace ? 'Edit Workspace' : 'Create Workspace'}</h3>
          <button className="btn btn-outline btn-icon btn-sm" onClick={() => !saving && onClose()}>
            <X size={16} />
          </button>
        </div>
        <div className="modal-body">
          <div className="form-group">
            <label className="form-label">Name</label>
            <input
              className="form-control"
              type="text"
              value={name}
              onChange={(e) => setName(e.target.value)}
              onKeyDown={(e) => e.key === 'Enter' && !e.shiftKey && handleSubmit()}
              placeholder="Marketing, Engineering, ..."
              autoFocus
              disabled={saving}
            />
          </div>
          <div className="form-group">
            <label className="form-label">Description (optional)</label>
            <textarea
              className="form-control"
              value={description}
              onChange={(e) => setDescription(e.target.value)}
              placeholder="What is this workspace for?"
              rows={3}
              disabled={saving}
              style={{ resize: 'vertical' }}
            />
          </div>
        </div>
        <div className="modal-footer">
          <button className="btn btn-outline btn-sm" onClick={onClose} disabled={saving}>Cancel</button>
          <button className="btn btn-primary btn-sm" onClick={handleSubmit} disabled={saving || !name.trim()}>
            {saving ? 'Saving...' : workspace ? 'Save Changes' : 'Create Workspace'}
          </button>
        </div>
      </div>
    </div>
  );
};

/* ---------- Main component ---------- */
const Workspaces: React.FC = () => {
  const { showToast, showConfirm } = useAlert();
  const [workspaces, setWorkspaces] = useState<WorkspaceDto[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const [searchTerm, setSearchTerm] = useState('');
  const [showModal, setShowModal] = useState(false);
  const [editingWorkspace, setEditingWorkspace] = useState<WorkspaceDto | undefined>();

  useEffect(() => { loadWorkspaces(); }, []);

  const loadWorkspaces = async () => {
    try {
      setLoading(true);
      setError(null);
      const data = await apiService.workspaces.list();
      setWorkspaces(data);
    } catch (err: any) {
      console.error('Failed to load workspaces:', err);
      setError(err.response?.data?.message || 'Failed to load workspaces');
    } finally {
      setLoading(false);
    }
  };

  const handleCreate = () => { setEditingWorkspace(undefined); setShowModal(true); };
  const handleEdit = (ws: WorkspaceDto) => { setEditingWorkspace(ws); setShowModal(true); };

  const handleSave = async (data: CreateWorkspaceDto) => {
    if (editingWorkspace) {
      await apiService.workspaces.update(editingWorkspace.id, data);
    } else {
      await apiService.workspaces.create(data);
    }
    await loadWorkspaces();
  };

  const handleDelete = async (ws: WorkspaceDto) => {
    const confirmed = await showConfirm(
      `Delete "${ws.name}"? This cannot be undone.`,
      'Delete workspace',
      { confirmLabel: 'Delete', variant: 'danger' }
    );
    if (!confirmed) return;
    try {
      await apiService.workspaces.delete(ws.id);
      await loadWorkspaces();
    } catch (err: any) {
      showToast(err.response?.data?.message || 'Failed to delete workspace', 'error');
    }
  };

  const filtered = workspaces.filter((ws) =>
    ws.name.toLowerCase().includes(searchTerm.toLowerCase()) ||
    ws.description.toLowerCase().includes(searchTerm.toLowerCase())
  );

  const formatDate = (iso: string) =>
    new Date(iso).toLocaleDateString('en-US', { month: 'short', day: 'numeric', year: 'numeric' });

  if (loading) return <LoadingSpinner />;

  return (
    <>
      <style>{workspaceStyles}</style>
      <div className="container">
        <div className="page-header">
          <div className="page-title-section">
            <h1 className="page-title">Workspaces</h1>
            <p className="page-subtitle">Organize your routes and collaborate with your team</p>
          </div>
          <div className="page-actions">
            <button className="btn btn-primary btn-sm" onClick={handleCreate}>
              <Plus size={16} />
              Create Workspace
            </button>
          </div>
        </div>

        <div className="toolbar">
          <div className="toolbar-section">
            <div className="search-box" style={{ marginBottom: 0 }}>
              <Search size={15} />
              <input
                type="text"
                placeholder="Search workspaces..."
                value={searchTerm}
                onChange={(e) => setSearchTerm(e.target.value)}
              />
            </div>
          </div>
          <span className="text-sm text-muted">{filtered.length} workspace{filtered.length !== 1 ? 's' : ''}</span>
        </div>

        {error && (
          <div className="alert alert-error" style={{ marginBottom: '1rem' }}>{error}</div>
        )}

        {/* Cards */}
        <div className="ws-grid">
          {filtered.length === 0 ? (
            <div className="empty-state" style={{ gridColumn: '1 / -1' }}>
              <div className="empty-state-icon">
                <Briefcase size={24} />
              </div>
              <div className="empty-state-title">{searchTerm ? 'No matches' : 'No workspaces yet'}</div>
              <div className="empty-state-description" style={{ maxWidth: 320, margin: '0 auto 1.25rem' }}>
                {searchTerm
                  ? 'Try a different search term.'
                  : 'Create a workspace to organize your routes and collaborate with your team.'}
              </div>
              {!searchTerm && (
                <button className="btn btn-primary btn-sm" onClick={handleCreate}>
                  <Plus size={16} />
                  Create Workspace
                </button>
              )}
            </div>
          ) : (
            filtered.map((ws) => {
              const isSystem = ws.type === 'System';
              const canEdit = !isSystem && (ws.userRole === 'Owner' || ws.userRole === 'Admin');
              const canDelete = !isSystem && ws.userRole === 'Owner';

              return (
                <div key={ws.id} className="card" style={{ padding: '1.25rem', display: 'flex', flexDirection: 'column' }}>
                  <div className="ws-card-top">
                    <div className={`ws-icon ${isSystem ? 'ws-icon-system' : 'ws-icon-user'}`}>
                      {isSystem ? <Shield size={18} /> : <Briefcase size={18} />}
                    </div>
                    <div className="ws-info">
                      <h3 className="ws-name">
                        {ws.name}
                        {isSystem && <span className="badge badge-primary">System</span>}
                        {ws.userRole && <span className="badge badge-secondary">{ws.userRole}</span>}
                      </h3>
                      {ws.description && <p className="ws-desc">{ws.description}</p>}
                    </div>
                  </div>

                  <div className="ws-footer">
                    <div className="ws-meta">
                      <Calendar size={12} />
                      {formatDate(ws.createdAt)}
                    </div>
                    <div className="ws-actions">
                      {canEdit && (
                        <button className="ws-action-btn" onClick={() => handleEdit(ws)} title="Edit">
                          <Edit size={14} />
                        </button>
                      )}
                      {canDelete && (
                        <button className="ws-action-btn danger" onClick={() => handleDelete(ws)} title="Delete">
                          <Trash2 size={14} />
                        </button>
                      )}
                    </div>
                  </div>
                </div>
              );
            })
          )}
        </div>
      </div>

      <WorkspaceModal
        show={showModal}
        workspace={editingWorkspace}
        onClose={() => { setShowModal(false); setEditingWorkspace(undefined); }}
        onSave={handleSave}
      />
    </>
  );
};

export default Workspaces;
