import React from 'react';
import { LogIn, Home, Shield } from 'lucide-react';
import Logo from './Logo';
import './KeycloakLogin.css';

interface KeycloakLoginProps {
  onLogin: () => void;
  onGoToLanding: () => void;
}

const KeycloakLogin: React.FC<KeycloakLoginProps> = ({ onLogin, onGoToLanding }) => {
  return (
    <div className="auth-page">
      <div className="auth-card">
        <div className="auth-card-body">
          <div className="auth-header">
            <Logo size={32} />
            <h2>Sign In</h2>
            <p>Access your Shortas dashboard</p>
          </div>

          <p className="auth-description">
            Use your organization credentials to sign in securely
            through our Keycloak authentication system.
          </p>

          <div className="auth-actions">
            <button className="btn btn-primary btn-block" onClick={onLogin}>
              <LogIn size={16} />
              Sign In with Keycloak
            </button>

            <button className="btn btn-outline btn-block" onClick={onGoToLanding}>
              <Home size={16} />
              Visit Landing Page
            </button>
          </div>

          <div className="auth-badges">
            <span className="auth-badge auth-badge-success">
              <Shield size={12} />
              Secure
            </span>
            <span className="auth-badge auth-badge-info">
              <LogIn size={12} />
              SSO
            </span>
          </div>
        </div>
      </div>
    </div>
  );
};

export default KeycloakLogin;
