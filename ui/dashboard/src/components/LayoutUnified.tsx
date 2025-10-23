import React, { useState, useRef, useEffect } from 'react';
import { Link, useLocation, useNavigate } from 'react-router-dom';
import {
  Link as LinkIcon,
  Settings,
  LogOut,
  User,
  Home,
  Activity,
  Sun,
  Moon,
  Globe,
  Briefcase,
  ChevronDown
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
  const navigate = useNavigate();
  const userInfo = getUserInfo();
  const { theme, toggleTheme } = useTheme();
  const [isUserMenuOpen, setIsUserMenuOpen] = useState(false);
  const userMenuRef = useRef<HTMLDivElement>(null);

  const navigation = [
    { name: 'Dashboard', href: '/dashboard', icon: Home },
    { name: 'Routes', href: '/routes', icon: LinkIcon },
    { name: 'Domains', href: '/domains', icon: Globe },
    { name: 'Workspaces', href: '/workspaces', icon: Briefcase },
    { name: 'Clickstream', href: '/clickstream', icon: Activity },
  ];

  const isActive = (path: string) => location.pathname === path;

  // Close dropdown when clicking outside
  useEffect(() => {
    const handleClickOutside = (event: MouseEvent) => {
      if (userMenuRef.current && !userMenuRef.current.contains(event.target as Node)) {
        setIsUserMenuOpen(false);
      }
    };

    document.addEventListener('mousedown', handleClickOutside);
    return () => {
      document.removeEventListener('mousedown', handleClickOutside);
    };
  }, []);

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

          <div className="navbar-user" style={{ display: 'flex', alignItems: 'center', flexDirection: 'row', gap: '0.5rem' }}>
            <button
              className="btn btn-outline btn-sm theme-toggle"
              onClick={toggleTheme}
              title={`Switch to ${theme === 'light' ? 'dark' : 'light'} theme`}
            >
              {theme === 'light' ? <Moon size={16} /> : <Sun size={16} />}
            </button>

            {/* User Dropdown Menu */}
            <div ref={userMenuRef} style={{ position: 'relative' }}>
              <button
                className="btn btn-outline btn-sm"
                onClick={() => setIsUserMenuOpen(!isUserMenuOpen)}
                style={{ display: 'flex', alignItems: 'center', gap: '0.5rem' }}
              >
                <User size={16} />
                <span>{userInfo?.email || userInfo?.username || 'User'}</span>
                <ChevronDown size={14} />
              </button>

              {isUserMenuOpen && (
                <div style={{
                  position: 'absolute',
                  top: 'calc(100% + 0.5rem)',
                  right: 0,
                  backgroundColor: 'var(--bg-primary)',
                  border: '1px solid var(--border-primary)',
                  borderRadius: '4px',
                  boxShadow: '0 4px 6px rgba(0, 0, 0, 0.1)',
                  minWidth: '200px',
                  zIndex: 1000,
                  overflow: 'hidden'
                }}>
                  {/* User Info Header */}
                  <div style={{
                    padding: '0.75rem 1rem',
                    borderBottom: '1px solid var(--border-primary)',
                    backgroundColor: 'var(--bg-secondary)'
                  }}>
                    <div style={{ fontWeight: 600, fontSize: '0.875rem', marginBottom: '0.25rem' }}>
                      {userInfo?.username || 'User'}
                    </div>
                    <div style={{ fontSize: '0.75rem', color: 'var(--text-muted)' }}>
                      {userInfo?.email || ''}
                    </div>
                  </div>

                  {/* Menu Items */}
                  <div style={{ padding: '0.5rem 0' }}>
                    <button
                      onClick={() => {
                        setIsUserMenuOpen(false);
                        navigate('/settings');
                      }}
                      style={{
                        width: '100%',
                        padding: '0.625rem 1rem',
                        display: 'flex',
                        alignItems: 'center',
                        gap: '0.75rem',
                        border: 'none',
                        backgroundColor: 'transparent',
                        color: 'var(--text-primary)',
                        cursor: 'pointer',
                        fontSize: '0.875rem',
                        transition: 'background-color 0.2s'
                      }}
                      onMouseEnter={(e) => e.currentTarget.style.backgroundColor = 'var(--bg-secondary)'}
                      onMouseLeave={(e) => e.currentTarget.style.backgroundColor = 'transparent'}
                    >
                      <Settings size={16} />
                      <span>Settings</span>
                    </button>

                    <button
                      onClick={() => {
                        setIsUserMenuOpen(false);
                        logout();
                      }}
                      style={{
                        width: '100%',
                        padding: '0.625rem 1rem',
                        display: 'flex',
                        alignItems: 'center',
                        gap: '0.75rem',
                        border: 'none',
                        backgroundColor: 'transparent',
                        color: 'var(--error-500)',
                        cursor: 'pointer',
                        fontSize: '0.875rem',
                        transition: 'background-color 0.2s'
                      }}
                      onMouseEnter={(e) => e.currentTarget.style.backgroundColor = 'var(--bg-secondary)'}
                      onMouseLeave={(e) => e.currentTarget.style.backgroundColor = 'transparent'}
                    >
                      <LogOut size={16} />
                      <span>Logout</span>
                    </button>
                  </div>
                </div>
              )}
            </div>
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
