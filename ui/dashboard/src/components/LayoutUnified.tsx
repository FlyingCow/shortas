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

  const getCurrentPageTitle = () => {
    const match = navigation.find((item) => isActive(item.href));
    if (match) return match.name;
    if (location.pathname.startsWith('/settings')) return 'Settings';
    return 'Dashboard';
  };

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
    <>
      <DevelopmentBanner />
      <div className="app-layout">
        {/* Sidebar */}
        <aside className="app-sidebar">
          <Link to="/dashboard" className="sidebar-logo">
            <Logo size={22} />
            <span className="sidebar-logo-text">SHORTAS</span>
          </Link>

          <nav className="sidebar-nav">
            {navigation.map((item) => {
              const Icon = item.icon;
              return (
                <Link
                  key={item.name}
                  to={item.href}
                  className={`sidebar-nav-item ${isActive(item.href) ? 'active' : ''}`}
                >
                  <Icon size={18} />
                  <span className="sidebar-nav-label">{item.name}</span>
                </Link>
              );
            })}
          </nav>

          <div className="sidebar-footer">
            <Link
              to="/settings"
              className={`sidebar-nav-item ${isActive('/settings') ? 'active' : ''}`}
            >
              <Settings size={18} />
              <span className="sidebar-nav-label">Settings</span>
            </Link>
            <button
              className="sidebar-nav-item"
              onClick={toggleTheme}
              title={`Switch to ${theme === 'light' ? 'dark' : 'light'} theme`}
              style={{ border: 'none', background: 'none', cursor: 'pointer', textAlign: 'left' }}
            >
              {theme === 'light' ? <Moon size={18} /> : <Sun size={18} />}
              <span className="sidebar-nav-label">
                {theme === 'light' ? 'Dark mode' : 'Light mode'}
              </span>
            </button>
          </div>
        </aside>

        {/* Main content wrapper */}
        <div className="app-main-wrapper">
          {/* Top bar */}
          <header className="app-topbar">
            <div className="topbar-left">
              <span className="topbar-title">{getCurrentPageTitle()}</span>
            </div>
            <div className="topbar-right">
              {/* User Dropdown */}
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
                    backgroundColor: 'var(--bg-secondary)',
                    border: '1px solid var(--border-primary)',
                    borderRadius: '4px',
                    boxShadow: 'var(--shadow-md)',
                    minWidth: '200px',
                    zIndex: 1000,
                    overflow: 'hidden'
                  }}>
                    {/* User Info Header */}
                    <div style={{
                      padding: '0.75rem 1rem',
                      borderBottom: '1px solid var(--border-primary)',
                      backgroundColor: 'var(--bg-tertiary)'
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
                        onMouseEnter={(e) => e.currentTarget.style.backgroundColor = 'var(--bg-tertiary)'}
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
                          color: 'var(--color-error)',
                          cursor: 'pointer',
                          fontSize: '0.875rem',
                          transition: 'background-color 0.2s'
                        }}
                        onMouseEnter={(e) => e.currentTarget.style.backgroundColor = 'var(--bg-tertiary)'}
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
          </header>

          {/* Main Content */}
          <main className="app-main">
            {children}
          </main>

          {/* Footer */}
          <Footer />
        </div>
      </div>
    </>
  );
};

export default Layout;
