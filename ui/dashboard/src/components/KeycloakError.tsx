import React from 'react';
import { AlertTriangle, Server, Settings, Database, RefreshCw } from 'lucide-react';
import './KeycloakError.css';

interface KeycloakErrorProps {
  error: string;
  onRetry: () => void;
  onEnableMockData: () => void;
}

const KeycloakError: React.FC<KeycloakErrorProps> = ({ error, onRetry, onEnableMockData }) => {
  const getErrorType = (errorMessage: string) => {
    if (errorMessage.includes('not reachable') || errorMessage.includes('Failed to fetch')) {
      return 'server';
    } else if (errorMessage.includes('realm') && errorMessage.includes('not found')) {
      return 'realm';
    } else if (errorMessage.includes('client') && errorMessage.includes('not properly configured')) {
      return 'client';
    }
    return 'general';
  };

  const errorType = getErrorType(error);

  const getIcon = () => {
    switch (errorType) {
      case 'server': return <Server className="error-type-icon" />;
      case 'realm': return <Database className="error-type-icon" />;
      case 'client': return <Settings className="error-type-icon" />;
      default: return <AlertTriangle className="error-type-icon" />;
    }
  };

  const getTitle = () => {
    switch (errorType) {
      case 'server': return 'Keycloak Server Not Running';
      case 'realm': return 'Keycloak Realm Not Found';
      case 'client': return 'Keycloak Client Not Configured';
      default: return 'Authentication Error';
    }
  };

  const getSolutions = () => {
    switch (errorType) {
      case 'server':
        return [
          'Start Keycloak server on port 8080',
          'Run: ./test-keycloak.sh to verify connectivity',
          'Or use mock data mode for development'
        ];
      case 'realm':
        return [
          'Create "shortas-dev" realm in Keycloak admin console',
          'Access admin console at http://localhost:8080/admin',
          'Or use mock data mode for development'
        ];
      case 'client':
        return [
          'Create "shortas-dashboard" client in Keycloak',
          'Set client as public OpenID Connect client',
          'Configure redirect URIs: http://localhost:3000/*',
          'Or use mock data mode for development'
        ];
      default:
        return [
          'Check Keycloak server status',
          'Verify realm and client configuration',
          'Or use mock data mode for development'
        ];
    }
  };

  return (
    <div className="auth-page">
      <div className="auth-card" style={{ maxWidth: '600px' }}>
        <div className="auth-card-body">
          <div className="error-header">
            {getIcon()}
            <h2>{getTitle()}</h2>
          </div>

          <div className="error-message">
            <p>{error}</p>
          </div>

          <div className="error-solutions">
            <h3>Solutions:</h3>
            <ul>
              {getSolutions().map((solution, index) => (
                <li key={index}>{solution}</li>
              ))}
            </ul>
          </div>

          <div className="error-actions">
            <button className="btn btn-primary" onClick={onRetry}>
              <RefreshCw size={16} />
              Retry Connection
            </button>
            <button className="btn btn-outline" onClick={onEnableMockData}>
              <Database size={16} />
              Use Mock Data Instead
            </button>
          </div>

          <div className="error-help">
            <div className="help-section">
              <h4>Quick Test:</h4>
              <code>./test-keycloak.sh</code>
              <p>Run this script to test your Keycloak setup</p>
            </div>
            <div className="help-section">
              <h4>Mock Data Mode:</h4>
              <code>REACT_APP_USE_MOCK_DATA=true</code>
              <p>Add this to .env.local to bypass Keycloak</p>
            </div>
          </div>
        </div>
      </div>
    </div>
  );
};

export default KeycloakError;
