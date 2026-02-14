import React, { useState, useEffect } from 'react';
import { Plus, Edit, Trash2, Search, Briefcase, Calendar, Shield, X } from 'lucide-react';
import { apiService, WorkspaceDto, CreateWorkspaceDto } from '../services/api';
import { useAlert } from '../contexts/AlertContext';
import LoadingSpinner from './LoadingSpinner';
import './DesignSystem.css';

const workspaceStyles = `
.ws-toolbar {
  display: flex;
  align-items: center;
  gap: 0.75rem;
  margin-top: 1.5rem;
  margin-bottom: 1.25rem;
}

.ws-search {
  position: relative;
  flex: 1;
  max-width: 360px;
}

.ws-search svg {
  position: absolute;
  left: 0.75rem;
  top: 50%;
  transform: translateY(-50%);
  color: var(--text-muted);
  pointer-events: none;
}

.ws-search input {
  width: 100%;
  padding: 0.5rem 0.75rem 0.5rem 2.25rem;
  border: 1px solid var(--border-secondary);
  border-radius: var(--radius-md);
  background: var(--bg-primary);
  color: var(--text-primary);
  font-size: 0.8125rem;
  transition: border-color var(--transition-fast), box-shadow var(--transition-fast);
}

.ws-search input:focus {
  outline: none;
  border-color: var(--color-primary);
  box-shadow: 0 0 0 3px var(--color-primary-light);
}

.ws-search input::placeholder {
  color: var(--text-muted);
}

.ws-count {
  font-size: 0.8125rem;
  color: var(--text-muted);
  white-space: nowrap;
  margin-left: auto;
}

/* Cards grid */
.ws-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(340px, 1fr));
  gap: 0.75rem;
}

.ws-card {
  display: flex;
  flex-direction: column;
  padding: 1.25rem;
  background: var(--bg-primary);
  border: 1px solid var(--border-primary);
  border-radius: var(--radius-lg);
  transition: border-color var(--transition-fast), box-shadow var(--transition-fast);
}

.ws-card:hover {
  border-color: var(--border-secondary);
  box-shadow: 0 1px 4px rgba(0, 0, 0, 0.04);
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

.ws-badge {
  display: inline-flex;
  align-items: center;
  font-size: 0.625rem;
  font-weight: 600;
  padding: 0.125rem 0.375rem;
  border-radius: 3px;
  text-transform: uppercase;
  letter-spacing: 0.5px;
  flex-shrink: 0;
}

.ws-badge-system {
  background: rgba(139, 92, 246, 0.1);
  color: #8b5cf6;
}

.ws-badge-role {
  background: var(--bg-tertiary);
  color: var(--text-secondary);
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

/* Empty state */
.ws-empty {
  grid-column: 1 / -1;
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  padding: 4rem 2rem;
  text-align: center;
}

.ws-empty-icon {
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

.ws-empty h3 {
  margin: 0 0 0.375rem 0;
  font-size: 1rem;
  font-weight: 600;
  color: var(--text-primary);
}

.ws-empty p {
  margin: 0 0 1.25rem 0;
  font-size: 0.8125rem;
  color: var(--text-secondary);
  max-width: 320px;
}

/* Modal */
.ws-modal-overlay {
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

.ws-modal {
  background: var(--bg-elevated);
  border-radius: var(--radius-xl);
  box-shadow: var(--shadow-xl);
  width: 100%;
  max-width: 460px;
  overflow: hidden;
}

.ws-modal-header {
  padding: 1.25rem 1.5rem;
  border-bottom: 1px solid var(--border-primary);
  display: flex;
  align-items: center;
  justify-content: space-between;
}

.ws-modal-header h2 {
  margin: 0;
  font-size: 1rem;
  font-weight: 600;
  color: var(--text-primary);
}

.ws-modal-close {
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

.ws-modal-close:hover {
  background: var(--bg-tertiary);
  color: var(--text-primary);
}

.ws-modal-body {
  padding: 1.5rem;
  display: flex;
  flex-direction: column;
  gap: 1rem;
}

.ws-field {
  display: flex;
  flex-direction: column;
  gap: 0.375rem;
}

.ws-field label {
  font-size: 0.8125rem;
  font-weight: 500;
  color: var(--text-secondary);
}

.ws-field input,
.ws-field textarea {
  padding: 0.625rem 0.75rem;
  border: 1px solid var(--border-secondary);
  border-radius: var(--radius-md);
  background: var(--bg-primary);
  color: var(--text-primary);
  font-size: 0.875rem;
  font-family: inherit;
  transition: border-color var(--transition-fast), box-shadow var(--transition-fast);
  resize: vertical;
}

.ws-field input:focus,
.ws-field textarea:focus {
  outline: none;
  border-color: var(--color-primary);
  box-shadow: 0 0 0 3px var(--color-primary-light);
}

.ws-field input::placeholder,
.ws-field textarea::placeholder {
  color: var(--text-muted);
}

.ws-field input:disabled,
.ws-field textarea:disabled {
  opacity: 0.6;
  cursor: not-allowed;
}

.ws-modal-footer {
  padding: 1rem 1.5rem;
  border-top: 1px solid var(--border-primary);
  display: flex;
  gap: 0.5rem;
  justify-content: flex-end;
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
  const { showAlert } = useAlert();
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
      showAlert('Failed to save workspace. Please try again.', 'Error');
    } finally {
      setSaving(false);
    }
  };

  if (!show) return null;

  return (
    <div className="ws-modal-overlay" onClick={() => !saving && onClose()}>
      <div className="ws-modal" onClick={(e) => e.stopPropagation()}>
        <div className="ws-modal-header">
          <h2>{workspace ? 'Edit Workspace' : 'Create Workspace'}</h2>
          <button className="ws-modal-close" onClick={() => !saving && onClose()}>
            <X size={16} />
          </button>
        </div>
        <div className="ws-modal-body">
          <div className="ws-field">
            <label>Name</label>
            <input
              type="text"
              value={name}
              onChange={(e) => setName(e.target.value)}
              onKeyDown={(e) => e.key === 'Enter' && !e.shiftKey && handleSubmit()}
              placeholder="Marketing, Engineering, ..."
              autoFocus
              disabled={saving}
            />
          </div>
          <div className="ws-field">
            <label>Description (optional)</label>
            <textarea
              value={description}
              onChange={(e) => setDescription(e.target.value)}
              placeholder="What is this workspace for?"
              rows={3}
              disabled={saving}
            />
          </div>
        </div>
        <div className="ws-modal-footer">
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
  const { showAlert, showConfirm } = useAlert();
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
      showAlert(err.response?.data?.message || 'Failed to delete workspace', 'Error');
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
        {/* Toolbar */}
        <div className="ws-toolbar">
          <div className="ws-search">
            <Search size={15} />
            <input
              type="text"
              placeholder="Search workspaces..."
              value={searchTerm}
              onChange={(e) => setSearchTerm(e.target.value)}
            />
          </div>
          <span className="ws-count">{filtered.length} workspace{filtered.length !== 1 ? 's' : ''}</span>
          <button className="btn btn-primary btn-sm" onClick={handleCreate}>
            <Plus size={16} />
            Create Workspace
          </button>
        </div>

        {error && (
          <div className="alert alert-error" style={{ marginBottom: '1rem' }}>{error}</div>
        )}

        {/* Cards */}
        <div className="ws-grid">
          {filtered.length === 0 ? (
            <div className="ws-empty">
              <div className="ws-empty-icon">
                <Briefcase size={24} />
              </div>
              <h3>{searchTerm ? 'No matches' : 'No workspaces yet'}</h3>
              <p>
                {searchTerm
                  ? 'Try a different search term.'
                  : 'Create a workspace to organize your routes and collaborate with your team.'}
              </p>
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
                <div key={ws.id} className="ws-card">
                  <div className="ws-card-top">
                    <div className={`ws-icon ${isSystem ? 'ws-icon-system' : 'ws-icon-user'}`}>
                      {isSystem ? <Shield size={18} /> : <Briefcase size={18} />}
                    </div>
                    <div className="ws-info">
                      <h3 className="ws-name">
                        {ws.name}
                        {isSystem && <span className="ws-badge ws-badge-system">System</span>}
                        {ws.userRole && <span className="ws-badge ws-badge-role">{ws.userRole}</span>}
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
