import React, { useState, useEffect } from 'react';
import { Plus, Edit, Trash2, Search, Briefcase, Users } from 'lucide-react';
import { apiService, WorkspaceDto, CreateWorkspaceDto } from '../services/api';
import LoadingSpinner from './LoadingSpinner';
import './DesignSystem.css';

const workspaceStyles = `
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

.form-group input:disabled,
.form-group textarea:disabled {
  opacity: 0.6;
  cursor: not-allowed;
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

.workspaces-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(320px, 1fr));
  gap: 1.5rem;
  margin-top: 1.5rem;
}

.workspace-card {
  padding: 1.5rem;
  transition: all 0.2s ease;
}

.workspace-card:hover {
  box-shadow: 0 2px 6px rgba(0, 0, 0, 0.06);
}

.workspace-card-header {
  display: flex;
  align-items: flex-start;
  gap: 1rem;
  margin-bottom: 1rem;
}

.workspace-icon {
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

.workspace-info {
  flex: 1;
  min-width: 0;
}

.workspace-name {
  font-size: 1.125rem;
  font-weight: 600;
  margin: 0 0 0.25rem 0;
  color: var(--text-primary);
}

.workspace-role {
  display: inline-block;
  font-size: 0.75rem;
  padding: 0.25rem 0.5rem;
  border-radius: 4px;
  background: var(--secondary-light);
  color: var(--secondary);
  font-weight: 500;
}

.workspace-type-badge {
  display: inline-block;
  font-size: 0.75rem;
  padding: 0.25rem 0.5rem;
  border-radius: 4px;
  font-weight: 600;
  text-transform: uppercase;
  letter-spacing: 0.5px;
}

.workspace-type-system {
  background: var(--color-primary);
  color: white;
}

.workspace-description {
  font-size: 0.875rem;
  color: var(--text-secondary);
  margin: 0.5rem 0 1rem 0;
  line-height: 1.5;
}

.workspace-meta {
  display: flex;
  gap: 1rem;
  font-size: 0.875rem;
  color: var(--text-tertiary);
  margin-bottom: 1rem;
}

.workspace-card-actions {
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

interface WorkspaceModalProps {
  show: boolean;
  workspace?: WorkspaceDto;
  onClose: () => void;
  onSave: (workspace: CreateWorkspaceDto) => Promise<void>;
}

const WorkspaceModal: React.FC<WorkspaceModalProps> = ({ show, workspace, onClose, onSave }) => {
  const [name, setName] = useState('');
  const [description, setDescription] = useState('');
  const [saving, setSaving] = useState(false);

  useEffect(() => {
    if (show) {
      if (workspace) {
        setName(workspace.name);
        setDescription(workspace.description);
      } else {
        setName('');
        setDescription('');
      }
      setSaving(false);
    }
  }, [workspace, show]);

  useEffect(() => {
    const handleEscape = (e: KeyboardEvent) => {
      if (e.key === 'Escape' && show && !saving) {
        onClose();
      }
    };

    document.addEventListener('keydown', handleEscape);
    return () => document.removeEventListener('keydown', handleEscape);
  }, [show, saving, onClose]);

  const handleSubmit = async () => {
    if (!name.trim()) {
      alert('Workspace name is required');
      return;
    }

    setSaving(true);
    try {
      await onSave({ name, description });
      onClose();
    } catch (error) {
      console.error('Failed to save workspace:', error);
      alert('Failed to save workspace. Please try again.');
    } finally {
      setSaving(false);
    }
  };

  if (!show) return null;

  return (
    <div className="modal-overlay" onClick={onClose}>
      <div className="modal-content" onClick={(e) => e.stopPropagation()}>
        <div className="modal-header">
          <h2>{workspace ? 'Edit Workspace' : 'Create New Workspace'}</h2>
          <button className="modal-close" onClick={onClose}>×</button>
        </div>

        <div className="modal-body">
          <div className="form-group">
            <label>Workspace Name *</label>
            <input
              type="text"
              value={name}
              onChange={(e) => setName(e.target.value)}
              onKeyDown={(e) => {
                if (e.key === 'Enter' && !e.shiftKey) {
                  e.preventDefault();
                  handleSubmit();
                }
              }}
              placeholder="My Workspace"
              autoFocus
              disabled={saving}
            />
          </div>
          <div className="form-group">
            <label>Description</label>
            <textarea
              value={description}
              onChange={(e) => setDescription(e.target.value)}
              placeholder="Describe your workspace..."
              rows={3}
              disabled={saving}
            />
          </div>
        </div>

        <div className="modal-footer">
          <button className="btn btn-outline" onClick={onClose} disabled={saving}>
            Cancel
          </button>
          <button className="btn btn-primary" onClick={handleSubmit} disabled={saving}>
            {saving ? 'Saving...' : (workspace ? 'Update Workspace' : 'Create Workspace')}
          </button>
        </div>
      </div>
    </div>
  );
};

const Workspaces: React.FC = () => {
  const [workspaces, setWorkspaces] = useState<WorkspaceDto[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const [searchTerm, setSearchTerm] = useState('');
  const [showModal, setShowModal] = useState(false);
  const [editingWorkspace, setEditingWorkspace] = useState<WorkspaceDto | undefined>();

  useEffect(() => {
    loadWorkspaces();
  }, []);

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

  const handleCreate = () => {
    setEditingWorkspace(undefined);
    setShowModal(true);
  };

  const handleEdit = (workspace: WorkspaceDto) => {
    setEditingWorkspace(workspace);
    setShowModal(true);
  };

  const handleSave = async (workspaceData: CreateWorkspaceDto) => {
    if (editingWorkspace) {
      await apiService.workspaces.update(editingWorkspace.id, workspaceData);
    } else {
      await apiService.workspaces.create(workspaceData);
    }
    await loadWorkspaces();
  };

  const handleDelete = async (id: string) => {
    if (!window.confirm('Are you sure you want to delete this workspace? This action cannot be undone.')) {
      return;
    }

    try {
      await apiService.workspaces.delete(id);
      await loadWorkspaces();
    } catch (err: any) {
      console.error('Failed to delete workspace:', err);
      alert(err.response?.data?.message || 'Failed to delete workspace');
    }
  };

  const filteredWorkspaces = workspaces.filter((workspace) =>
    workspace.name.toLowerCase().includes(searchTerm.toLowerCase()) ||
    workspace.description.toLowerCase().includes(searchTerm.toLowerCase())
  );

  if (loading) {
    return <LoadingSpinner />;
  }

  return (
    <>
      <style>{workspaceStyles}</style>
      <div className="container">
        <div className="page-header">
          <div className="page-title">
            <Briefcase size={32} />
            <div>
              <h1>Workspaces</h1>
              <p>Organize your routes into workspaces</p>
            </div>
          </div>
          <button className="btn btn-primary" onClick={handleCreate}>
            <Plus size={20} />
            Create Workspace
          </button>
        </div>

        {error && (
          <div className="alert alert-error">
            {error}
          </div>
        )}

        <div className="table-controls">
          <div className="search-box">
            <Search size={16} />
            <input
              type="text"
              placeholder="Search workspaces..."
              value={searchTerm}
              onChange={(e) => setSearchTerm(e.target.value)}
            />
          </div>
          <div className="control-group">
            <span className="control-label">Total: {workspaces.length}</span>
          </div>
        </div>

        <div className="workspaces-grid">
          {filteredWorkspaces.length === 0 ? (
            <div className="empty-state-full">
              <Briefcase size={64} />
              <h3>No workspaces found</h3>
              <p>
                {searchTerm
                  ? 'No workspaces match your search criteria'
                  : 'Create your first workspace to organize your routes'}
              </p>
              {!searchTerm && (
                <button className="btn btn-primary" onClick={handleCreate}>
                  <Plus size={20} />
                  Create Workspace
                </button>
              )}
            </div>
          ) : (
            filteredWorkspaces.map((workspace) => (
              <div key={workspace.id} className="workspace-card card">
                <div className="workspace-card-header">
                  <div className="workspace-icon">
                    <Briefcase size={24} />
                  </div>
                  <div className="workspace-info">
                    <h3 className="workspace-name">{workspace.name}</h3>
                    <div style={{ display: 'flex', gap: '0.5rem', alignItems: 'center', flexWrap: 'wrap' }}>
                      {workspace.type === 'System' && (
                        <span className="workspace-type-badge workspace-type-system">System</span>
                      )}
                      {workspace.userRole && (
                        <span className="workspace-role">{workspace.userRole}</span>
                      )}
                    </div>
                  </div>
                </div>

                {workspace.description && (
                  <p className="workspace-description">{workspace.description}</p>
                )}

                <div className="workspace-meta">
                  <span>Created {new Date(workspace.createdAt).toLocaleDateString()}</span>
                </div>

                <div className="workspace-card-actions">
                  {workspace.type !== 'System' && (workspace.userRole === 'Owner' || workspace.userRole === 'Admin') && (
                    <>
                      <button
                        className="btn btn-outline btn-sm"
                        onClick={() => handleEdit(workspace)}
                        title="Edit workspace"
                      >
                        <Edit size={16} />
                        Edit
                      </button>
                      {workspace.userRole === 'Owner' && (
                        <button
                          className="btn btn-ghost btn-sm"
                          onClick={() => handleDelete(workspace.id)}
                          title="Delete workspace"
                        >
                          <Trash2 size={16} />
                          Delete
                        </button>
                      )}
                    </>
                  )}
                </div>
              </div>
            ))
          )}
        </div>
      </div>

      <WorkspaceModal
        show={showModal}
        workspace={editingWorkspace}
        onClose={() => {
          setShowModal(false);
          setEditingWorkspace(undefined);
        }}
        onSave={handleSave}
      />
    </>
  );
};

export default Workspaces;
