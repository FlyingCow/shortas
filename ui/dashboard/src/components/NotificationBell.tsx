import React, { useCallback, useEffect, useRef, useState } from 'react';
import { Bell, Check, X } from 'lucide-react';
import { apiService, NotificationDto } from '../services/api';
import { useAlert } from '../contexts/AlertContext';
import { useWorkspace } from '../contexts/WorkspaceContext';

const POLL_INTERVAL_MS = 30000;

const bellStyles = `
.nb-wrapper {
  position: relative;
}

.nb-button {
  position: relative;
  display: inline-flex;
  align-items: center;
  justify-content: center;
  width: 36px;
  height: 36px;
  border: 1px solid var(--border-primary);
  background: transparent;
  border-radius: var(--radius-md);
  color: var(--text-secondary);
  cursor: pointer;
}

.nb-button:hover {
  background: var(--bg-tertiary);
  color: var(--text-primary);
}

.nb-badge {
  position: absolute;
  top: -4px;
  right: -4px;
  min-width: 16px;
  height: 16px;
  padding: 0 4px;
  border-radius: 999px;
  background: var(--color-error);
  color: #fff;
  font-size: 0.625rem;
  font-weight: 700;
  display: flex;
  align-items: center;
  justify-content: center;
  line-height: 1;
}

.nb-panel {
  position: absolute;
  top: calc(100% + 0.5rem);
  right: 0;
  width: 340px;
  max-height: 420px;
  overflow-y: auto;
  background: var(--bg-secondary);
  border: 1px solid var(--border-primary);
  border-radius: var(--radius-md);
  box-shadow: var(--shadow-md);
  z-index: 1000;
}

.nb-panel-header {
  padding: 0.75rem 1rem;
  font-size: 0.8125rem;
  font-weight: 600;
  border-bottom: 1px solid var(--border-primary);
}

.nb-item {
  padding: 0.75rem 1rem;
  border-bottom: 1px solid var(--border-primary);
  font-size: 0.8125rem;
}

.nb-item:last-child {
  border-bottom: none;
}

.nb-item-text {
  color: var(--text-primary);
  margin-bottom: 0.5rem;
  line-height: 1.4;
}

.nb-item-actions {
  display: flex;
  gap: 0.5rem;
}

.nb-empty {
  padding: 2rem 1rem;
  text-align: center;
  font-size: 0.8125rem;
  color: var(--text-muted);
}
`;

const describeNotification = (n: NotificationDto): string => {
  if (n.renderedText) {
    return n.renderedText;
  }
  // Fallback for notifications created before backend template rendering
  // existed (or any sent outside the templating pipeline).
  if (n.type === 'WorkspaceInvite') {
    const workspaceName = n.payload?.workspace_name || n.payload?.workspaceName || 'a workspace';
    const inviterName = n.payload?.inviter_name || n.payload?.inviterName || 'Someone';
    const role = n.payload?.role === 'Member' ? 'User' : n.payload?.role || 'Member';
    return `${inviterName} invited you to join "${workspaceName}" as ${role}`;
  }
  return n.type;
};

const NotificationBell: React.FC = () => {
  const { showToast } = useAlert();
  const { refreshWorkspaces } = useWorkspace();
  const [notifications, setNotifications] = useState<NotificationDto[]>([]);
  const [open, setOpen] = useState(false);
  const [busyId, setBusyId] = useState<string | null>(null);
  const wrapperRef = useRef<HTMLDivElement>(null);

  const load = useCallback(async () => {
    try {
      const data = await apiService.notifications.list();
      setNotifications(data);
    } catch {
      // Silent - don't interrupt the dashboard over a notification fetch failure.
    }
  }, []);

  useEffect(() => {
    load();
    const interval = setInterval(load, POLL_INTERVAL_MS);
    return () => clearInterval(interval);
  }, [load]);

  useEffect(() => {
    const onClickOutside = (e: MouseEvent) => {
      if (wrapperRef.current && !wrapperRef.current.contains(e.target as Node)) {
        setOpen(false);
      }
    };
    document.addEventListener('mousedown', onClickOutside);
    return () => document.removeEventListener('mousedown', onClickOutside);
  }, []);

  const unreadCount = notifications.filter((n) => n.status === 'Unread').length;

  const dismiss = (id: string) => setNotifications((prev) => prev.filter((n) => n.id !== id));

  const handleAccept = async (n: NotificationDto) => {
    const invitationId = n.payload?.invitation_id || n.payload?.invitationId;
    if (!invitationId) return;
    setBusyId(n.id);
    try {
      await apiService.invitations.accept(invitationId);
      dismiss(n.id);
      await refreshWorkspaces();
      showToast('Invitation accepted', 'success');
    } catch (err: any) {
      showToast(err.response?.data?.message || 'Failed to accept invitation', 'error');
    } finally {
      setBusyId(null);
    }
  };

  const handleDecline = async (n: NotificationDto) => {
    const invitationId = n.payload?.invitation_id || n.payload?.invitationId;
    if (!invitationId) return;
    setBusyId(n.id);
    try {
      await apiService.invitations.decline(invitationId);
      dismiss(n.id);
    } catch (err: any) {
      showToast(err.response?.data?.message || 'Failed to decline invitation', 'error');
    } finally {
      setBusyId(null);
    }
  };

  const handleOpen = () => {
    setOpen((v) => !v);
  };

  return (
    <div className="nb-wrapper" ref={wrapperRef}>
      <style>{bellStyles}</style>
      <button className="nb-button" onClick={handleOpen} title="Notifications">
        <Bell size={16} />
        {unreadCount > 0 && <span className="nb-badge">{unreadCount > 9 ? '9+' : unreadCount}</span>}
      </button>

      {open && (
        <div className="nb-panel">
          <div className="nb-panel-header">Notifications</div>
          {notifications.length === 0 ? (
            <div className="nb-empty">You're all caught up.</div>
          ) : (
            notifications.map((n) => {
              // 'WorkspaceInvite' is the legacy type for notifications created before
              // template-based rendering; 'workspace.invite_created' is the current one.
              const isInvite =
                (n.type === 'WorkspaceInvite' || n.type === 'workspace.invite_created') &&
                n.status !== 'Actioned';
              const isBusy = busyId === n.id;
              return (
                <div key={n.id} className="nb-item">
                  <div className="nb-item-text">{describeNotification(n)}</div>
                  {isInvite && (
                    <div className="nb-item-actions">
                      <button
                        className="btn btn-primary btn-sm"
                        disabled={isBusy}
                        onClick={() => handleAccept(n)}
                      >
                        <Check size={12} />
                        Accept
                      </button>
                      <button
                        className="btn btn-outline btn-sm"
                        disabled={isBusy}
                        onClick={() => handleDecline(n)}
                      >
                        <X size={12} />
                        Decline
                      </button>
                    </div>
                  )}
                </div>
              );
            })
          )}
        </div>
      )}
    </div>
  );
};

export default NotificationBell;
