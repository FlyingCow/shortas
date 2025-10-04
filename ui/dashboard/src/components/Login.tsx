import React, { useState } from 'react';
import { Container, Row, Col, Card, Button, Form, Alert, Spinner } from 'react-bootstrap';
import { 
  Link as LinkIcon, 
  Shield, 
  ArrowRight,
  Eye,
  EyeOff,
  User,
  Lock
} from 'lucide-react';
import keycloak, { keycloakLoginOptions } from '../config/keycloak';
import './Login.css';

interface LoginProps {
  onLogin: () => void;
  error?: string | null;
}

const Login: React.FC<LoginProps> = ({ onLogin, error }) => {
  const [isLoading, setIsLoading] = useState(false);
  const [showPassword, setShowPassword] = useState(false);
  const [formData, setFormData] = useState({
    username: '',
    password: '',
    rememberMe: false,
  });

  const handleKeycloakLogin = async () => {
    try {
      setIsLoading(true);
      await keycloak.login(keycloakLoginOptions);
    } catch (error) {
      console.error('Login failed:', error);
      setIsLoading(false);
    }
  };

  const handleInputChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    const { name, value, type, checked } = e.target;
    setFormData(prev => ({
      ...prev,
      [name]: type === 'checkbox' ? checked : value,
    }));
  };

  const handleFormSubmit = (e: React.FormEvent) => {
    e.preventDefault();
    handleKeycloakLogin();
  };

  return (
    <Container fluid className="vh-100 bg-light">
      <Row className="h-100">
        {/* Left Side - Branding */}
        <Col lg={6} className="d-none d-lg-flex align-items-center justify-content-center bg-primary">
          <div className="text-center text-white p-5">
            <div className="mb-4">
              <LinkIcon size={64} className="mb-3" />
              <h1 className="display-4 fw-bold">SHORTAS</h1>
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

        {/* Right Side - Login Form */}
        <Col lg={6} className="d-flex align-items-center justify-content-center">
          <div className="w-100" style={{ maxWidth: '400px' }}>
            <Card className="shadow">
              <Card.Body className="p-5">
                <div className="text-center mb-4">
                  <LinkIcon size={40} className="text-primary mb-3 d-lg-none" />
                  <h2 className="h3 mb-1">Sign In</h2>
                  <p className="text-muted">Access your Shortas dashboard</p>
                </div>

                <Alert variant="info" className="small">
                  <strong>Demo Form:</strong> This form is for UI demonstration. 
                  Click "Continue with Keycloak SSO" below for actual authentication.
                </Alert>

                {error && (
                  <Alert variant="danger" className="d-flex align-items-center">
                    <Shield size={16} className="me-2" />
                    <div>
                      <strong>Authentication Error</strong>
                      <div className="small">{error}</div>
                    </div>
                  </Alert>
                )}

                <Form onSubmit={handleFormSubmit}>
                  <Form.Group className="mb-3">
                    <Form.Label>Username or Email</Form.Label>
                    <Form.Control
                      type="text"
                      name="username"
                      value={formData.username}
                      onChange={handleInputChange}
                      placeholder="Enter your username or email"
                      disabled={isLoading}
                    />
                  </Form.Group>

                  <Form.Group className="mb-3">
                    <Form.Label>Password</Form.Label>
                    <div className="position-relative">
                      <Form.Control
                        type={showPassword ? 'text' : 'password'}
                        name="password"
                        value={formData.password}
                        onChange={handleInputChange}
                        placeholder="Enter your password"
                        disabled={isLoading}
                      />
                      <Button
                        variant="link"
                        className="position-absolute end-0 top-50 translate-middle-y border-0"
                        style={{ zIndex: 5 }}
                        onClick={() => setShowPassword(!showPassword)}
                        disabled={isLoading}
                      >
                        {showPassword ? <EyeOff size={16} /> : <Eye size={16} />}
                      </Button>
                    </div>
                  </Form.Group>

                  <div className="d-flex justify-content-between align-items-center mb-3">
                    <Form.Check
                      type="checkbox"
                      name="rememberMe"
                      checked={formData.rememberMe}
                      onChange={handleInputChange}
                      disabled={isLoading}
                      label="Remember me"
                    />
                    <Button variant="link" className="p-0 text-decoration-none">
                      Forgot password?
                    </Button>
                  </div>

                  <Button
                    type="submit"
                    variant="outline-primary"
                    size="lg"
                    className="w-100 mb-3"
                    disabled={isLoading}
                  >
                    {isLoading ? (
                      <>
                        <Spinner animation="border" size="sm" className="me-2" />
                        Signing in...
                      </>
                    ) : (
                      <>
                        Sign In with Keycloak
                        <ArrowRight size={16} className="ms-2" />
                      </>
                    )}
                  </Button>
                </Form>

                <div className="text-center mb-3">
                  <span className="text-muted">or</span>
                </div>

                <Button
                  variant="primary"
                  size="lg"
                  className="w-100"
                  onClick={handleKeycloakLogin}
                  disabled={isLoading}
                >
                  {isLoading ? (
                    <>
                      <Spinner animation="border" size="sm" className="me-2" />
                      Connecting to Keycloak...
                    </>
                  ) : (
                    <>
                      <Shield size={16} className="me-2" />
                      Continue with Keycloak SSO
                    </>
                  )}
                </Button>

                <div className="text-center mt-4">
                  <p className="small text-muted">
                    Powered by <strong>Keycloak</strong> for secure authentication
                  </p>
                  <div className="d-flex justify-content-center gap-3">
                    <span className="badge bg-success">
                      <Shield size={12} className="me-1" />
                      OpenID Connect
                    </span>
                    <span className="badge bg-info">
                      <Lock size={12} className="me-1" />
                      PKCE Security
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

export default Login;
