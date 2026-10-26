import React, { useCallback, useEffect, useState } from 'react';
import { useNavigate, useParams } from 'react-router-dom';
import { Briefcase, CheckCircle, Mail } from 'lucide-react';
import { apiService, InvitationPreviewDto } from '../services/api';
import keycloak from '../config/keycloak';
import LoadingSpinner from './LoadingSpinner';
import Logo from './Logo';
import './DesignSystem.css';

interface InviteAcceptProps {
  authenticated: boolean;
}

const pageStyles = `
.invite-page {
  min-height: 100vh;
  display: flex;
  align-items: center;
  justify-content: center;
  padding: 1.5rem;
  background: var(--bg-primary);
}

.invite-card {
  max-width: 420px;
  width: 100%;
  background: var(--bg-secondary);
  border: 1px solid var(--border-primary);
  border-radius: var(--radius-lg);
  box-shadow: var(--shadow-md);
  padding: 2rem;
  text-align: center;
}

.invite-logo {
  display: flex;
  justify-content: center;
  margin-bottom: 1.5rem;
}

.invite-icon {
  width: 48px;
  height: 48px;
  margin: 0 auto 1rem;
  border-radius: 50%;
  background: var(--color-primary-light);
  color: var(--color-primary);
  display: flex;
  align-items: center;
  justify-content: center;
}

.invite-title {
  font-size: 1.125rem;
  font-weight: 600;
  margin-bottom: 0.5rem;
  color: var(--text-primary);
}

.invite-desc {
  font-size: 0.875rem;
  color: var(--text-secondary);
  line-height: 1.5;
  margin-bottom: 1.5rem;
}
`;

const InviteAccept: React.FC<InviteAcceptProps> = ({ authenticated }) => {
  const { token } = useParams<{ token: string }>();
  const navigate = useNavigate();
  const [preview, setPreview] = useState<InvitationPreviewDto | null>(null);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const [accepting, setAccepting] = useState(false);
  const [accepted, setAccepted] = useState(false);
  const [canRetry, setCanRetry] = useState(false);

  const loadPreview = useCallback(async () => {
    if (!token) return;
    setLoading(true);
    setError(null);
    setCanRetry(false);
    try {
      const data = await apiService.invitations.previewByToken(token);
      setPreview(data);
    } catch (err: any) {
      if (err.response?.status === 404) {
        setError('This invitation link is invalid.');
      } else if (err.response?.status === 409) {
        setError('This invitation is no longer valid — it may have already been used, declined, or expired.');
      } else {
        // No response at all means the request never completed (network/server
        // down), not that the invitation itself is bad — distinguish so a
        // transient blip doesn't read the same as an expired invite.
        console.error('Failed to load invitation preview:', err);
        setError(
          err.response
            ? `Unexpected error (HTTP ${err.response.status}) while loading this invitation.`
            : "Couldn't reach the server. Check your connection and try again."
        );
        setCanRetry(true);
      }
    } finally {
      setLoading(false);
    }
  }, [token]);

  useEffect(() => { loadPreview(); }, [loadPreview]);

  const handleAccept = async () => {
    if (!token) return;
    setAccepting(true);
    try {
      await apiService.invitations.acceptByToken(token);
      setAccepted(true);
      setTimeout(() => navigate('/workspaces'), 1500);
    } catch (err: any) {
      console.error('Failed to accept invitation:', err);
      setError(err.response?.data?.message || 'Failed to accept this invitation.');
    } finally {
      setAccepting(false);
    }
  };

  const handleLogin = () => {
    keycloak.login({ redirectUri: window.location.href });
  };

  const roleLabel = (role?: string) => (role === 'Member' ? 'User' : role || 'User');

  return (
    <div className="invite-page">
      <style>{pageStyles}</style>
      <div className="invite-card">
        <div className="invite-logo"><Logo size={28} /></div>

        {loading ? (
          <LoadingSpinner message="Loading invitation..." />
        ) : error ? (
          <>
            <div className="invite-icon"><Mail size={22} /></div>
            <div className="invite-title">{canRetry ? 'Couldn\'t load invitation' : 'Invitation unavailable'}</div>
            <div className="invite-desc">{error}</div>
            {canRetry && (
              <button className="btn btn-outline" onClick={loadPreview}>
                Try again
              </button>
            )}
          </>
        ) : accepted ? (
          <>
            <div className="invite-icon"><CheckCircle size={22} /></div>
            <div className="invite-title">You're in!</div>
            <div className="invite-desc">Taking you to your workspaces...</div>
          </>
        ) : preview ? (
          <>
            <div className="invite-icon"><Briefcase size={22} /></div>
            <div className="invite-title">Join "{preview.workspaceName}"</div>
            <div className="invite-desc">
              {preview.inviterName} invited <strong>{preview.invitedEmail}</strong> to join as{' '}
              <strong>{roleLabel(preview.role)}</strong>.
            </div>

            {authenticated ? (
              <button className="btn btn-primary" onClick={handleAccept} disabled={accepting}>
                {accepting ? 'Accepting...' : 'Accept invitation'}
              </button>
            ) : (
              <button className="btn btn-primary" onClick={handleLogin}>
                Log in or sign up to accept
              </button>
            )}
          </>
        ) : null}
      </div>
    </div>
  );
};

export default InviteAccept;
