import React from 'react';
import { Link, useLocation } from 'react-router-dom';
import { 
  BarChart3, 
  Link as LinkIcon, 
  Settings, 
  LogOut, 
  User,
  Home,
  Activity,
  Sun,
  Moon
} from 'lucide-react';
import { logout, getUserInfo } from '../config/keycloak';
import { useTheme } from '../contexts/ThemeContext';
import DevelopmentBanner from './DevelopmentBanner';
import Footer from './Footer';
import Logo from './Logo';
import './DesignSystem.css';

interface LayoutProps {
  children: React.ReactNode;
}

const Layout: React.FC<LayoutProps> = ({ children }) => {
  const location = useLocation();
  const userInfo = getUserInfo();
  const { theme, toggleTheme } = useTheme();

  const navigation = [
    { name: 'Dashboard', href: '/dashboard', icon: Home },
    { name: 'Routes', href: '/routes', icon: LinkIcon },
    { name: 'Analytics', href: '/analytics', icon: BarChart3 },
    { name: 'Clickstream', href: '/clickstream', icon: Activity },
    { name: 'Settings', href: '/settings', icon: Settings },
  ];

  const isActive = (path: string) => location.pathname === path;

  return (
    <div className="layout-wrapper">
      <DevelopmentBanner />
      
      {/* Top Navigation */}
      <nav className="navbar">
        <div className="navbar-container" style={{ display: 'flex', alignItems: 'center', justifyContent: 'space-between', flexDirection: 'row' }}>
          <Link to="/dashboard" className="navbar-brand" style={{ display: 'flex', alignItems: 'center', flexDirection: 'row' }}>
            <Logo size={24} />
            <span>SHORTAS</span>
          </Link>
          
          <div className="navbar-nav" style={{ display: 'flex', alignItems: 'center', flexDirection: 'row', gap: '0.5rem' }}>
            {navigation.map((item) => {
              const Icon = item.icon;
              return (
                <Link
                  key={item.name}
                  to={item.href}
                  className={`navbar-link ${isActive(item.href) ? 'active' : ''}`}
                  style={{ display: 'flex', alignItems: 'center', flexDirection: 'row', whiteSpace: 'nowrap' }}
                >
                  <Icon size={18} />
                  {item.name}
                </Link>
              );
            })}
          </div>

          <div className="navbar-user" style={{ display: 'flex', alignItems: 'center', flexDirection: 'row' }}>
            <button 
              className="btn btn-outline btn-sm theme-toggle"
              onClick={toggleTheme}
              title={`Switch to ${theme === 'light' ? 'dark' : 'light'} theme`}
            >
              {theme === 'light' ? <Moon size={16} /> : <Sun size={16} />}
            </button>
            <div className="user-info" style={{ display: 'flex', alignItems: 'center', flexDirection: 'row' }}>
              <User size={16} />
              <span>{userInfo?.username || 'User'}</span>
            </div>
            <button 
              className="btn btn-outline btn-sm"
              onClick={() => logout()}
            >
              <LogOut size={16} />
              Logout
            </button>
          </div>
        </div>
      </nav>

      {/* Main Content */}
      <main className="main-content">
        {children}
      </main>

      {/* Footer */}
      <Footer />
    </div>
  );
};

export default Layout;
