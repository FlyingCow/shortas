import React from 'react';
import {
  Sun,
  Moon,
  Monitor,
  Shield,
  KeyRound,
  Users,
  ExternalLink,
} from 'lucide-react';
import { useTheme } from '../contexts/ThemeContext';
import './DesignSystem.css';

const KEYCLOAK_URL = process.env.REACT_APP_KEYCLOAK_URL || 'http://localhost:8080';
const KEYCLOAK_REALM = 'shortas-dev';
const ACCOUNT_URL = `${KEYCLOAK_URL}/realms/${KEYCLOAK_REALM}/account`;

const securityStyles = `
.sec-links {
  display: flex;
  flex-direction: column;
  gap: 0.5rem;
}
.sec-link {
  display: flex;
  align-items: center;
  gap: 0.75rem;
  padding: 0.75rem 1rem;
  border: 1px solid var(--border-primary);
  border-radius: var(--radius-md);
  background: var(--bg-primary);
  color: var(--text-primary);
  text-decoration: none;
  transition: border-color var(--transition-fast), box-shadow var(--transition-fast);
}
.sec-link:hover {
  border-color: var(--color-primary);
  box-shadow: 0 0 0 3px var(--color-primary-light);
}
.sec-link-icon {
  width: 36px;
  height: 36px;
  display: flex;
  align-items: center;
  justify-content: center;
  background: var(--bg-tertiary);
  border-radius: var(--radius-sm);
  color: var(--text-secondary);
  flex-shrink: 0;
}
.sec-link-text {
  flex: 1;
  min-width: 0;
}
.sec-link-title {
  font-size: 0.875rem;
  font-weight: 600;
}
.sec-link-desc {
  font-size: 0.75rem;
  color: var(--text-muted);
  margin-top: 0.125rem;
}
.sec-link-arrow {
  color: var(--text-muted);
  flex-shrink: 0;
}
`;

const Settings: React.FC = () => {
  const { theme, setTheme } = useTheme();

  return (
    <>
      <style>{securityStyles}</style>
      <div className="container" style={{ paddingTop: '1.5rem', maxWidth: '560px' }}>
        {/* Appearance */}
        <div className="card" style={{ marginBottom: '1.5rem' }}>
          <div className="card-header">
            <h3 className="card-title">
              <Monitor size={20} />
              Appearance
            </h3>
            <p className="card-subtitle">Customize your dashboard appearance</p>
          </div>
          <div className="card-body">
            <div className="form-group">
              <label className="form-label">Theme</label>
              <div className="theme-selector">
                <div className="theme-options">
                  <button
                    className={`theme-option ${theme === 'light' ? 'active' : ''}`}
                    onClick={() => setTheme('light')}
                  >
                    <Sun size={20} />
                    <span>Light</span>
                  </button>
                  <button
                    className={`theme-option ${theme === 'dark' ? 'active' : ''}`}
                    onClick={() => setTheme('dark')}
                  >
                    <Moon size={20} />
                    <span>Dark</span>
                  </button>
                </div>
                <p className="text-sm text-muted">
                  Current theme: <strong>{theme === 'light' ? 'Light' : 'Dark'}</strong>
                </p>
              </div>
            </div>
          </div>
        </div>

        {/* Security */}
        <div className="card">
          <div className="card-header">
            <h3 className="card-title">
              <Shield size={20} />
              Security
            </h3>
            <p className="card-subtitle">Manage your password and linked accounts</p>
          </div>
          <div className="card-body">
            <div className="sec-links">
              <a
                href={`${ACCOUNT_URL}/#/security/signingin`}
                target="_blank"
                rel="noopener noreferrer"
                className="sec-link"
              >
                <div className="sec-link-icon">
                  <KeyRound size={18} />
                </div>
                <div className="sec-link-text">
                  <div className="sec-link-title">Change Password</div>
                  <div className="sec-link-desc">Update your account password</div>
                </div>
                <ExternalLink size={14} className="sec-link-arrow" />
              </a>

              <a
                href={`${ACCOUNT_URL}/#/security/linked-accounts`}
                target="_blank"
                rel="noopener noreferrer"
                className="sec-link"
              >
                <div className="sec-link-icon">
                  <Users size={18} />
                </div>
                <div className="sec-link-text">
                  <div className="sec-link-title">Social Logins</div>
                  <div className="sec-link-desc">Manage linked social accounts (Google, GitHub, etc.)</div>
                </div>
                <ExternalLink size={14} className="sec-link-arrow" />
              </a>
            </div>
          </div>
        </div>
      </div>
    </>
  );
};

export default Settings;
