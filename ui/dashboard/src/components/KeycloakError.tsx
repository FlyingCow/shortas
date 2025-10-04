import React from 'react';
import { Container, Row, Col, Card, Button, Alert, Badge } from 'react-bootstrap';
import { AlertTriangle, Server, Settings, Database } from 'lucide-react';
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
    <Container fluid className="vh-100 bg-light d-flex align-items-center justify-content-center">
      <Row className="w-100 justify-content-center">
        <Col lg={8} xl={6}>
          <Card className="shadow">
            <Card.Body className="p-5">
              <div className="text-center mb-4">
                {getIcon()}
                <h2 className="mt-3">{getTitle()}</h2>
              </div>
              
              <Alert variant="danger" className="mb-4">
                {error}
              </Alert>

              <div className="mb-4">
                <h5>Solutions:</h5>
                <ul className="list-unstyled">
                  {getSolutions().map((solution, index) => (
                    <li key={index} className="mb-2">
                      <span className="me-2">💡</span>
                      {solution}
                    </li>
                  ))}
                </ul>
              </div>

              <div className="d-grid gap-2 d-md-flex justify-content-md-center mb-4">
                <Button variant="primary" onClick={onRetry}>
                  <AlertTriangle size={16} className="me-2" />
                  Retry Connection
                </Button>
                
                <Button variant="secondary" onClick={onEnableMockData}>
                  <Database size={16} className="me-2" />
                  Use Mock Data Instead
                </Button>
              </div>

              <Row>
                <Col md={6}>
                  <Card className="bg-light">
                    <Card.Body className="p-3">
                      <h6 className="text-primary mb-2">Quick Test:</h6>
                      <code className="d-block bg-dark text-light p-2 rounded mb-2">
                        ./test-keycloak.sh
                      </code>
                      <p className="small text-muted mb-0">
                        Run this script to test your Keycloak setup
                      </p>
                    </Card.Body>
                  </Card>
                </Col>
                
                <Col md={6}>
                  <Card className="bg-light">
                    <Card.Body className="p-3">
                      <h6 className="text-info mb-2">Mock Data Mode:</h6>
                      <code className="d-block bg-dark text-light p-2 rounded mb-2">
                        REACT_APP_USE_MOCK_DATA=true
                      </code>
                      <p className="small text-muted mb-0">
                        Add this to .env.local to bypass Keycloak
                      </p>
                    </Card.Body>
                  </Card>
                </Col>
              </Row>
            </Card.Body>
          </Card>
        </Col>
      </Row>
    </Container>
  );
};

export default KeycloakError;
