import React from 'react';
import { LogIn, Home } from 'lucide-react';
import keycloak, { keycloakLoginOptions } from '../config/keycloak';
import Logo from './Logo';
import './LoggedOut.css';

interface LoggedOutProps {
  onLogin: () => void;
}

const LoggedOut: React.FC<LoggedOutProps> = ({ onLogin }) => {
  const handleKeycloakLogin = async () => {
    try {
      await keycloak.login(keycloakLoginOptions);
    } catch (error) {
      console.error('Login failed:', error);
    }
  };

  const handleGoToLanding = () => {
    const landingUrl = process.env.REACT_APP_LANDING_URL || 'https://shortas.com';
    window.location.href = landingUrl;
  };

  return (
    <div className="auth-page">
      <div className="auth-card">
        <div className="auth-card-body">
          <div className="auth-header">
            <Logo size={32} />
            <h2>You're Logged Out</h2>
            <p>Sign in to access your dashboard</p>
          </div>

          <p className="auth-description">
            You have been logged out of your account.
            Sign in again to access your dashboard or visit our landing page.
          </p>

          <div className="auth-actions">
            <button className="btn btn-primary btn-block" onClick={handleKeycloakLogin}>
              <LogIn size={16} />
              Sign In to Dashboard
            </button>

            <button className="btn btn-outline btn-block" onClick={handleGoToLanding}>
              <Home size={16} />
              Visit Landing Page
            </button>
          </div>

          <p className="auth-footer-text">
            Powered by <strong>Keycloak</strong> for secure authentication
          </p>
        </div>
      </div>
    </div>
  );
};

export default LoggedOut;
