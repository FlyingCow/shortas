import React from 'react';
import { Container, Row, Col, Card, Button } from 'react-bootstrap';
import { 
  Shield, 
  ArrowRight,
  Home,
  LogIn
} from 'lucide-react';
import Logo from './Logo';
import './KeycloakLogin.css';

interface KeycloakLoginProps {
  onLogin: () => void;
  onGoToLanding: () => void;
}

const KeycloakLogin: React.FC<KeycloakLoginProps> = ({ onLogin, onGoToLanding }) => {
  return (
    <Container fluid className="vh-100 keycloak-login-container">
      <Row className="h-100">
        {/* Left Side - Branding */}
        <Col lg={6} className="d-none d-lg-flex align-items-center justify-content-center keycloak-login-branding">
          <div className="text-center text-white p-5 keycloak-login-content">
            <div className="mb-4">
              <Logo size={64} className="mb-3" color="white" />
              <h1 className="display-4 fw-bold keycloak-login-title">SHORTAS</h1>
            </div>
            
            <div className="mb-5">
              <h2 className="h3 mb-3">Welcome to Shortas Dashboard</h2>
              <p className="lead">
                Manage your shortened URLs with powerful analytics, 
                real-time tracking, and enterprise-grade security.
              </p>
            </div>

            <div className="text-start">
              <div className="d-flex align-items-center mb-3">
                <Shield size={24} className="me-3" />
                <div>
                  <h5 className="mb-1">Secure Authentication</h5>
                  <small>Protected by Keycloak OpenID Connect</small>
                </div>
              </div>
              
              <div className="d-flex align-items-center mb-3">
                <LogIn size={24} className="me-3" />
                <div>
                  <h5 className="mb-1">Single Sign-On</h5>
                  <small>Seamless authentication experience</small>
                </div>
              </div>
              
              <div className="d-flex align-items-center">
                <ArrowRight size={24} className="me-3" />
                <div>
                  <h5 className="mb-1">Enterprise Security</h5>
                  <small>Advanced security features and compliance</small>
                </div>
              </div>
            </div>
          </div>
        </Col>

        {/* Right Side - Login Form */}
        <Col lg={6} className="d-flex align-items-center justify-content-center">
          <div className="w-100" style={{ maxWidth: '400px' }}>
            <Card className="shadow keycloak-login-card">
              <Card.Body className="p-5 keycloak-login-form-content">
                <div className="text-center mb-4">
                  <Logo size={40} className="text-muted mb-3 d-lg-none" />
                  <h2 className="h3 mb-1 keycloak-login-form-title">Sign In</h2>
                  <p className="keycloak-login-form-subtitle">Access your Shortas dashboard</p>
                </div>

                <div className="text-center mb-4">
                  <p className="text-muted">
                    Use your organization credentials to sign in securely 
                    through our Keycloak authentication system.
                  </p>
                </div>

                <div className="d-grid gap-3">
                  <Button
                    variant="primary"
                    size="lg"
                    onClick={onLogin}
                    className="keycloak-login-button keycloak-login-button-primary d-flex align-items-center justify-content-center"
                  >
                    <LogIn size={16} className="me-2" />
                    Sign In with Keycloak
                  </Button>

                  <Button
                    variant="outline-primary"
                    size="lg"
                    onClick={onGoToLanding}
                    className="keycloak-login-button keycloak-login-button-secondary d-flex align-items-center justify-content-center"
                  >
                    <Home size={16} className="me-2" />
                    Visit Landing Page
                  </Button>
                </div>

                <div className="text-center mt-4">
                  <div className="keycloak-login-badges">
                    <span className="keycloak-login-badge keycloak-login-badge-success">
                      <Shield size={14} className="me-1" />
                      Secure
                    </span>
                    <span className="keycloak-login-badge keycloak-login-badge-info">
                      <LogIn size={14} className="me-1" />
                      SSO
                    </span>
                  </div>
                </div>
              </Card.Body>
            </Card>
          </div>
        </Col>
      </Row>
    </Container>
  );
};

export default KeycloakLogin;

