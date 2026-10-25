import React, { useState, useEffect, useCallback } from 'react';
import { Plus, Edit, Trash2, Search, Briefcase, Calendar, Shield, X, Users, UserPlus, UserMinus } from 'lucide-react';
import { apiService, WorkspaceDto, CreateWorkspaceDto, UserWorkspaceDto } from '../services/api';
import { useAlert } from '../contexts/AlertContext';
import { getUserInfo } from '../config/keycloak';
import Dropdown from './Dropdown';
import LoadingSpinner from './LoadingSpinner';
import './DesignSystem.css';

// Workspace-specific widgets with no shared DesignSystem.css equivalent —
// everything else (page header, toolbar, search box, card, badge, modal,
// form fields, empty state) uses the canonical classes directly.
const workspaceStyles = `
.ws-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(340px, 1fr));
  gap: 1rem;
  align-items: start;
}

.ws-card-top {
  display: flex;
  align-items: flex-start;
  gap: 0.875rem;
  margin-bottom: 0.75rem;
}

.ws-icon {
  width: 38px;
  height: 38px;
  display: flex;
  align-items: center;
  justify-content: center;
  border-radius: var(--radius-md);
  flex-shrink: 0;
}

.ws-icon-user {
  background: var(--color-primary-light);
  color: var(--color-primary);
}

.ws-icon-system {
  background: rgba(139, 92, 246, 0.12);
  color: #8b5cf6;
}

.ws-info {
  flex: 1;
  min-width: 0;
}

.ws-name-row {
  display: flex;
  align-items: center;
  gap: 0.4rem;
  flex-wrap: wrap;
  margin-bottom: 0.3rem;
}

.ws-name {
  font-size: 0.9375rem;
  font-weight: 600;
  color: var(--text-primary);
  margin: 0;
  line-height: 1.3;
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
  min-height: 2.25em;
}

.ws-footer {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 0.75rem;
  margin-top: 0.875rem;
  padding-top: 0.75rem;
  border-top: 1px solid var(--border-primary);
}

.ws-meta-group {
  display: flex;
  align-items: center;
  gap: 0.875rem;
  flex-wrap: wrap;
  min-width: 0;
}

.ws-meta {
  display: flex;
  align-items: center;
  gap: 0.3rem;
  font-size: 0.75rem;
  color: var(--text-muted);
  white-space: nowrap;
}

.ws-meta-btn {
  display: flex;
  align-items: center;
  gap: 0.3rem;
  font-size: 0.75rem;
  color: var(--text-secondary);
  background: none;
  border: none;
  padding: 0;
  cursor: pointer;
  white-space: nowrap;
  font-weight: 500;
}

.ws-meta-btn:hover {
  color: var(--color-primary);
}

.ws-actions {
  display: flex;
  gap: 0.25rem;
  flex-shrink: 0;
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
  flex-shrink: 0;
}

.ws-action-btn:hover {
  background: var(--bg-tertiary);
  color: var(--text-primary);
}

.ws-action-btn.danger:hover {
  background: rgba(239, 68, 68, 0.08);
  color: var(--color-error);
}

/* Members modal */
.wm-list {
  display: flex;
  flex-direction: column;
  gap: 0.5rem;
  max-height: 340px;
  overflow-y: auto;
}

.wm-row {
  display: flex;
  align-items: center;
  gap: 0.75rem;
  padding: 0.5rem;
  border-radius: var(--radius-md);
  border: 1px solid var(--border-primary);
}

.wm-avatar {
  width: 32px;
  height: 32px;
  border-radius: 50%;
  background: var(--bg-tertiary);
  color: var(--text-secondary);
  display: flex;
  align-items: center;
  justify-content: center;
  font-size: 0.75rem;
  font-weight: 600;
  flex-shrink: 0;
  text-transform: uppercase;
}

.wm-info {
  flex: 1;
  min-width: 0;
}

.wm-id {
  font-family: var(--font-family-mono);
  font-size: 0.75rem;
  color: var(--text-primary);
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
  display: block;
}

.wm-joined {
  font-size: 0.6875rem;
  color: var(--text-muted);
}

.wm-role-select {
  font-size: 0.75rem;
}

.wm-add-row {
  display: flex;
  gap: 0.5rem;
  align-items: flex-end;
  margin-top: 1rem;
  padding-top: 1rem;
  border-top: 1px solid var(--border-primary);
}
`;

/* ---------- Workspace create/edit modal ---------- */
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

/* ---------- Members modal ---------- */
interface MembersModalProps {
  workspace: WorkspaceDto;
  canManage: boolean;
  currentUserId?: string;
  onClose: () => void;
  onMemberCountChange: (workspaceId: string, count: number) => void;
}

const ROLE_OPTIONS = ['Owner', 'Admin', 'Member'];

const MembersModal: React.FC<MembersModalProps> = ({ workspace, canManage, currentUserId, onClose, onMemberCountChange }) => {
  const { showToast, showConfirm } = useAlert();
  const [members, setMembers] = useState<UserWorkspaceDto[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const [busyUserId, setBusyUserId] = useState<string | null>(null);
  const [newUserId, setNewUserId] = useState('');
  const [newRole, setNewRole] = useState('Member');
  const [adding, setAdding] = useState(false);

  const loadMembers = useCallback(async () => {
    try {
      setLoading(true);
      setError(null);
      const data = await apiService.workspaces.getMembers(workspace.id);
      setMembers(data);
      onMemberCountChange(workspace.id, data.length);
    } catch (err: any) {
      setError(err.response?.data?.message || 'Failed to load members');
    } finally {
      setLoading(false);
    }
  }, [workspace.id, onMemberCountChange]);

  useEffect(() => { loadMembers(); }, [loadMembers]);

  useEffect(() => {
    const onKey = (e: KeyboardEvent) => { if (e.key === 'Escape') onClose(); };
    document.addEventListener('keydown', onKey);
    return () => document.removeEventListener('keydown', onKey);
  }, [onClose]);

  const handleRoleChange = async (member: UserWorkspaceDto, role: string) => {
    setBusyUserId(member.userId);
    try {
      await apiService.workspaces.updateMemberRole(workspace.id, member.userId, role);
      await loadMembers();
    } catch (err: any) {
      showToast(err.response?.data?.message || 'Failed to update role', 'error');
    } finally {
      setBusyUserId(null);
    }
  };

  const handleRemove = async (member: UserWorkspaceDto) => {
    const confirmed = await showConfirm(
      `Remove this member from "${workspace.name}"?`,
      'Remove member',
      { confirmLabel: 'Remove', variant: 'danger' }
    );
    if (!confirmed) return;
    setBusyUserId(member.userId);
    try {
      await apiService.workspaces.removeMember(workspace.id, member.userId);
      await loadMembers();
    } catch (err: any) {
      showToast(err.response?.data?.message || 'Failed to remove member', 'error');
    } finally {
      setBusyUserId(null);
    }
  };

  const handleAdd = async () => {
    if (!newUserId.trim()) return;
    setAdding(true);
    try {
      await apiService.workspaces.addMember(workspace.id, newUserId.trim(), newRole);
      setNewUserId('');
      setNewRole('Member');
      await loadMembers();
      showToast('Member added', 'success');
    } catch (err: any) {
      showToast(err.response?.data?.message || 'Failed to add member', 'error');
    } finally {
      setAdding(false);
    }
  };

  const formatDate = (iso?: string) =>
    iso ? new Date(iso).toLocaleDateString('en-US', { month: 'short', day: 'numeric', year: 'numeric' }) : 'Unknown';

  return (
    <div className="modal-overlay" onClick={onClose}>
      <div className="modal" style={{ maxWidth: 480 }} onClick={(e) => e.stopPropagation()}>
        <div className="modal-header">
          <h3>Members — {workspace.name}</h3>
          <button className="btn btn-outline btn-icon btn-sm" onClick={onClose}>
            <X size={16} />
          </button>
        </div>
        <div className="modal-body">
          {loading ? (
            <LoadingSpinner message="Loading members..." />
          ) : error ? (
            <div className="alert alert-error">{error}</div>
          ) : members.length === 0 ? (
            <div className="empty-state">
              <div className="empty-state-icon"><Users size={24} /></div>
              <div className="empty-state-title">No members</div>
              <div className="empty-state-description">This workspace has no members yet.</div>
            </div>
          ) : (
            <div className="wm-list">
              {members.map((member) => {
                const isSelf = currentUserId && member.userId === currentUserId;
                const isBusy = busyUserId === member.userId;
                return (
                  <div key={member.userId} className="wm-row">
                    <div className="wm-avatar">{(member.userId || '?').slice(0, 2)}</div>
                    <div className="wm-info">
                      <span className="wm-id" title={member.userId}>
                        {member.userId}{isSelf ? ' (You)' : ''}
                      </span>
                      <span className="wm-joined">Joined {formatDate(member.joinedAt)}</span>
                    </div>
                    {canManage ? (
                      <>
                        <Dropdown
                          compact
                          align="right"
                          value={member.role}
                          disabled={isBusy}
                          onChange={(v) => handleRoleChange(member, v)}
                          options={ROLE_OPTIONS.map((r) => ({ value: r, label: r }))}
                        />
                        <button
                          className="ws-action-btn danger"
                          title="Remove member"
                          disabled={isBusy}
                          onClick={() => handleRemove(member)}
                        >
                          <UserMinus size={14} />
                        </button>
                      </>
                    ) : (
                      <span className="badge badge-secondary">{member.role}</span>
                    )}
                  </div>
                );
              })}
            </div>
          )}

          {canManage && (
            <div className="wm-add-row">
              <div className="form-group" style={{ flex: 1, marginBottom: 0 }}>
                <label className="form-label">Add member by user ID</label>
                <input
                  className="form-control"
                  value={newUserId}
                  onChange={(e) => setNewUserId(e.target.value)}
                  onKeyDown={(e) => e.key === 'Enter' && handleAdd()}
                  placeholder="User ID"
                  disabled={adding}
                />
              </div>
              <Dropdown
                compact
                value={newRole}
                onChange={setNewRole}
                disabled={adding}
                options={ROLE_OPTIONS.map((r) => ({ value: r, label: r }))}
              />
              <button className="btn btn-primary btn-sm" onClick={handleAdd} disabled={adding || !newUserId.trim()}>
                <UserPlus size={14} />
                Add
              </button>
            </div>
          )}
        </div>
        <div className="modal-footer">
          <button className="btn btn-outline btn-sm" onClick={onClose}>Close</button>
        </div>
      </div>
    </div>
  );
};

/* ---------- Main component ---------- */
const Workspaces: React.FC = () => {
  const { showToast, showConfirm } = useAlert();
  const currentUserId = getUserInfo()?.sub;
  const [workspaces, setWorkspaces] = useState<WorkspaceDto[]>([]);
  const [memberCounts, setMemberCounts] = useState<Record<string, number>>({});
  const [myMemberships, setMyMemberships] = useState<Record<string, UserWorkspaceDto | undefined>>({});
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const [searchTerm, setSearchTerm] = useState('');
  const [showModal, setShowModal] = useState(false);
  const [editingWorkspace, setEditingWorkspace] = useState<WorkspaceDto | undefined>();
  const [membersWorkspace, setMembersWorkspace] = useState<WorkspaceDto | undefined>();

  const loadWorkspaces = useCallback(async () => {
    try {
      setLoading(true);
      setError(null);
      const data = await apiService.workspaces.list();
      setWorkspaces(data);

      // The list endpoint doesn't say what role the current user holds in
      // each workspace, so derive it from the member list we fetch anyway
      // (also used for the member-count badge).
      const results = await Promise.all(
        data.map((ws) =>
          apiService.workspaces.getMembers(ws.id)
            .then((members) => ({
              id: ws.id,
              count: members.length,
              mine: members.find((m) => m.userId === currentUserId),
            }))
            .catch(() => ({ id: ws.id, count: 0, mine: undefined }))
        )
      );
      setMemberCounts(Object.fromEntries(results.map((r) => [r.id, r.count])));
      setMyMemberships(Object.fromEntries(results.map((r) => [r.id, r.mine])));
    } catch (err: any) {
      console.error('Failed to load workspaces:', err);
      setError(err.response?.data?.message || 'Failed to load workspaces');
    } finally {
      setLoading(false);
    }
  }, [currentUserId]);

  useEffect(() => { loadWorkspaces(); }, [loadWorkspaces]);

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

  const handleMemberCountChange = useCallback((workspaceId: string, count: number) => {
    setMemberCounts((prev) => ({ ...prev, [workspaceId]: count }));
  }, []);

  const filtered = workspaces.filter((ws) =>
    ws.name.toLowerCase().includes(searchTerm.toLowerCase()) ||
    ws.description.toLowerCase().includes(searchTerm.toLowerCase())
  );

  const formatDate = (iso?: string) =>
    iso ? new Date(iso).toLocaleDateString('en-US', { month: 'short', day: 'numeric', year: 'numeric' }) : 'Unknown';

  if (loading) return <LoadingSpinner message="Loading workspaces..." />;

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
              const isSystem = ws.isSystem;
              const myMembership = myMemberships[ws.id];
              const canEdit = !isSystem && (myMembership?.isOwner || myMembership?.isAdmin);
              const canDelete = !isSystem && myMembership?.isOwner;
              const memberCount = memberCounts[ws.id];

              return (
                <div key={ws.id} className="card" style={{ padding: '1.25rem', display: 'flex', flexDirection: 'column' }}>
                  <div className="ws-card-top">
                    <div className={`ws-icon ${isSystem ? 'ws-icon-system' : 'ws-icon-user'}`}>
                      {isSystem ? <Shield size={18} /> : <Briefcase size={18} />}
                    </div>
                    <div className="ws-info">
                      <div className="ws-name-row">
                        <h3 className="ws-name">{ws.name}</h3>
                        {isSystem && <span className="badge badge-primary">System</span>}
                        {myMembership?.role && <span className="badge badge-secondary">{myMembership.role}</span>}
                      </div>
                      <p className="ws-desc">{ws.description || 'No description.'}</p>
                    </div>
                  </div>

                  <div className="ws-footer">
                    <div className="ws-meta-group">
                      <span className="ws-meta">
                        <Calendar size={12} />
                        {formatDate(ws.createdAt)}
                      </span>
                      <button className="ws-meta-btn" onClick={() => setMembersWorkspace(ws)}>
                        <Users size={12} />
                        {memberCount ?? '—'} member{memberCount === 1 ? '' : 's'}
                      </button>
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

      {membersWorkspace && (
        <MembersModal
          workspace={membersWorkspace}
          canManage={!!(myMemberships[membersWorkspace.id]?.isOwner || myMemberships[membersWorkspace.id]?.isAdmin)}
          currentUserId={currentUserId}
          onClose={() => setMembersWorkspace(undefined)}
          onMemberCountChange={handleMemberCountChange}
        />
      )}
    </>
  );
};

export default Workspaces;
