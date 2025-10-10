import React from 'react';
import { Container, Row, Col, Card, Button } from 'react-bootstrap';
import { 
  Link as LinkIcon, 
  Shield, 
  ArrowRight,
  Home,
  LogIn
} from 'lucide-react';
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
    // Navigate to landing page or external site
    const landingUrl = process.env.REACT_APP_LANDING_URL || 'https://shortas.com';
    window.location.href = landingUrl;
  };

  return (
    <Container fluid className="vh-100 logged-out-container">
      <Row className="h-100">
        {/* Left Side - Branding */}
        <Col lg={6} className="d-none d-lg-flex align-items-center justify-content-center logged-out-branding">
          <div className="text-center text-white p-5 logged-out-content">
            <div className="mb-4">
              <Logo size={64} className="mb-3" color="white" />
              <h1 className="display-4 fw-bold logged-out-title">SHORTAS</h1>
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
                <LinkIcon size={24} className="me-3" />
                <div>
                  <h5 className="mb-1">URL Management</h5>
                  <small>Create, edit, and manage your short links</small>
                </div>
              </div>
              
              <div className="d-flex align-items-center">
                <ArrowRight size={24} className="me-3" />
                <div>
                  <h5 className="mb-1">Real-time Analytics</h5>
                  <small>Track clicks, locations, and performance</small>
                </div>
              </div>
            </div>
          </div>
        </Col>

        {/* Right Side - Logged Out Options */}
        <Col lg={6} className="d-flex align-items-center justify-content-center">
          <div className="w-100" style={{ maxWidth: '400px' }}>
            <Card className="shadow logged-out-card">
              <Card.Body className="p-5 logged-out-form-content">
                <div className="text-center mb-4">
                  <LinkIcon size={40} className="text-muted mb-3 d-lg-none" />
                  <h2 className="h3 mb-1 logged-out-form-title">You're Logged Out</h2>
                  <p className="logged-out-form-subtitle">Sign in to access your dashboard</p>
                </div>

                <div className="text-center mb-4">
                  <p className="text-muted">
                    You have been logged out of your account. 
                    Sign in again to access your dashboard or visit our landing page.
                  </p>
                </div>

                <div className="d-grid gap-3">
                  <Button
                    variant="primary"
                    size="lg"
                    onClick={handleKeycloakLogin}
                    className="logged-out-button logged-out-button-primary d-flex align-items-center justify-content-center"
                  >
                    <LogIn size={16} className="me-2" />
                    Sign In to Dashboard
                  </Button>

                  <Button
                    variant="primary"
                    size="lg"
                    onClick={handleGoToLanding}
                    className="logged-out-button logged-out-button-secondary d-flex align-items-center justify-content-center"
                  >
                    <Home size={16} className="me-2" />
                    Visit Landing Page
                  </Button>
                </div>

                <div className="text-center mt-4">
                  <p className="small text-muted">
                    Powered by <strong>Keycloak</strong> for secure authentication
                  </p>
                </div>
              </Card.Body>
            </Card>
          </div>
        </Col>
      </Row>
    </Container>
  );
};

export default LoggedOut;
