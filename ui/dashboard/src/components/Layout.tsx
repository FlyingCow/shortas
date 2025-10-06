import React from 'react';
import { Link, useLocation } from 'react-router-dom';
import { Navbar, Nav, Container, Row, Col, Button, Dropdown } from 'react-bootstrap';
import { 
  BarChart3, 
  Link as LinkIcon, 
  Settings, 
  LogOut, 
  User,
  Home,
  Activity
} from 'lucide-react';
import { logout, getUserInfo } from '../config/keycloak';
import DevelopmentBanner from './DevelopmentBanner';
import './DesignSystem.css';

interface LayoutProps {
  children: React.ReactNode;
}

const Layout: React.FC<LayoutProps> = ({ children }) => {
  const location = useLocation();
  const userInfo = getUserInfo();

  const navigation = [
    { name: 'Dashboard', href: '/dashboard', icon: Home },
    { name: 'Routes', href: '/routes', icon: LinkIcon },
    { name: 'Analytics', href: '/analytics', icon: BarChart3 },
    { name: 'Clickstream', href: '/clickstream', icon: Activity },
    { name: 'Settings', href: '/settings', icon: Settings },
  ];

  const isActive = (path: string) => location.pathname === path;

  return (
    <>
      <DevelopmentBanner />
      
      {/* Top Navigation */}
      <Navbar bg="dark" variant="dark" expand="lg" className="mb-0">
        <Container fluid>
          <Navbar.Brand as={Link} to="/dashboard" className="d-flex align-items-center">
            <LinkIcon className="me-2" size={24} />
            <span className="fw-bold">SHORTAS</span>
          </Navbar.Brand>
          
          <Navbar.Toggle aria-controls="basic-navbar-nav" />
          <Navbar.Collapse id="basic-navbar-nav">
            <Nav className="me-auto">
              {navigation.map((item) => {
                const Icon = item.icon;
                return (
                  <Nav.Link
                    key={item.name}
                    as={Link}
                    to={item.href}
                    className={`d-flex align-items-center ${isActive(item.href) ? 'active' : ''}`}
                  >
                    <Icon className="me-2" size={18} />
                    {item.name}
                  </Nav.Link>
                );
              })}
            </Nav>
            
            <Nav>
              <Dropdown>
                <Dropdown.Toggle variant="outline-light" id="user-dropdown" className="d-flex align-items-center">
                  <User className="me-2" size={18} />
                  {userInfo.username}
                </Dropdown.Toggle>
                
                <Dropdown.Menu>
                  <Dropdown.Item disabled>
                    <div className="small">
                      <div>{userInfo.name || userInfo.username}</div>
                      <div className="text-muted">{userInfo.email}</div>
                    </div>
                  </Dropdown.Item>
                  <Dropdown.Divider />
                  <Dropdown.Item onClick={logout}>
                    <LogOut className="me-2" size={16} />
                    Logout
                  </Dropdown.Item>
                </Dropdown.Menu>
              </Dropdown>
            </Nav>
          </Navbar.Collapse>
        </Container>
      </Navbar>

      {/* Main Content */}
      <Container fluid className="px-0">
        <Row className="g-0">
          <Col>
            <div className="p-4">
              <div className="d-flex justify-content-between align-items-center mb-4">
                <h1 className="h3 mb-0">
                  {navigation.find(item => isActive(item.href))?.name || 'Dashboard'}
                </h1>
              </div>
              
              <main>
                {children}
              </main>
            </div>
          </Col>
        </Row>
      </Container>
    </>
  );
};

export default Layout;
