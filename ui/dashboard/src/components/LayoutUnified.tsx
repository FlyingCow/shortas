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
  ChevronDown,
  Keyboard
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
  const [showShortcutHelp, setShowShortcutHelp] = useState(false);
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

  // Global keyboard shortcuts
  useEffect(() => {
    const isInputFocused = () => {
      const el = document.activeElement;
      if (!el || el === document.body) return false;
      const tag = (el as HTMLElement).tagName;
      const role = (el as HTMLElement).getAttribute?.('role');
      const editable = (el as HTMLElement).isContentEditable;
      return tag === 'INPUT' || tag === 'TEXTAREA' || tag === 'SELECT' || role === 'textbox' || editable;
    };

    const onKeyDown = (e: KeyboardEvent) => {
      if (e.key === 'Escape') {
        if (showShortcutHelp) {
          setShowShortcutHelp(false);
          e.preventDefault();
        } else if (isUserMenuOpen) {
          setIsUserMenuOpen(false);
        }
        return;
      }
      if (e.key === '?' && !e.ctrlKey && !e.metaKey && !e.altKey) {
        if (!isInputFocused()) {
          setShowShortcutHelp((v) => !v);
          e.preventDefault();
        }
        return;
      }
      if (e.altKey && !e.ctrlKey && !e.metaKey && !isInputFocused()) {
        const n = parseInt(e.key, 10);
        if (e.key === '6' || n === 6) {
          navigate('/settings');
          e.preventDefault();
        } else if (n >= 1 && n <= 5) {
          const item = navigation[n - 1];
          if (item) {
            navigate(item.href);
            e.preventDefault();
          }
        }
      }
    };

    document.addEventListener('keydown', onKeyDown);
    return () => document.removeEventListener('keydown', onKeyDown);
  }, [navigate, isUserMenuOpen, showShortcutHelp, navigation]);

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
              onClick={() => setShowShortcutHelp(true)}
              title="Keyboard shortcuts (?)"
              style={{ border: 'none', background: 'none', cursor: 'pointer', textAlign: 'left' }}
            >
              <Keyboard size={18} />
              <span className="sidebar-nav-label">Shortcuts</span>
            </button>
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

      {/* Keyboard shortcuts help */}
      {showShortcutHelp && (
        <div
          style={{
            position: 'fixed', inset: 0, background: 'var(--theme-bg-overlay, rgba(0,0,0,0.5))',
            display: 'flex', alignItems: 'center', justifyContent: 'center', zIndex: 1060, padding: 16
          }}
          onClick={() => setShowShortcutHelp(false)}
          role="dialog"
          aria-label="Keyboard shortcuts"
        >
          <div
            style={{
              background: 'var(--theme-bg-elevated)', borderRadius: 8, boxShadow: 'var(--theme-shadow-xl)',
              maxWidth: 360, width: '100%', overflow: 'hidden'
            }}
            onClick={(e) => e.stopPropagation()}
          >
            <div style={{ padding: '0.75rem 1rem', borderBottom: '1px solid var(--theme-border-primary)', display: 'flex', alignItems: 'center', gap: 8, fontSize: '0.9375rem', fontWeight: 600 }}>
              <Keyboard size={18} />
              Keyboard shortcuts
            </div>
            <div style={{ padding: 16, fontSize: '0.8125rem' }}>
              <table style={{ width: '100%', borderCollapse: 'collapse' }}>
                <tbody>
                  <tr><td style={{ padding: '0.35rem 0.5rem 0.35rem 0' }}><kbd style={{ padding: '0.15rem 0.4rem', background: 'var(--bg-tertiary)', borderRadius: 4 }}>Alt</kbd> + <kbd style={{ padding: '0.15rem 0.4rem', background: 'var(--bg-tertiary)', borderRadius: 4 }}>1</kbd></td><td style={{ padding: '0.35rem 0' }}>Dashboard</td></tr>
                  <tr><td style={{ padding: '0.35rem 0.5rem 0.35rem 0' }}><kbd style={{ padding: '0.15rem 0.4rem', background: 'var(--bg-tertiary)', borderRadius: 4 }}>Alt</kbd> + <kbd style={{ padding: '0.15rem 0.4rem', background: 'var(--bg-tertiary)', borderRadius: 4 }}>2</kbd></td><td style={{ padding: '0.35rem 0' }}>Routes</td></tr>
                  <tr><td style={{ padding: '0.35rem 0.5rem 0.35rem 0' }}><kbd style={{ padding: '0.15rem 0.4rem', background: 'var(--bg-tertiary)', borderRadius: 4 }}>Alt</kbd> + <kbd style={{ padding: '0.15rem 0.4rem', background: 'var(--bg-tertiary)', borderRadius: 4 }}>3</kbd></td><td style={{ padding: '0.35rem 0' }}>Domains</td></tr>
                  <tr><td style={{ padding: '0.35rem 0.5rem 0.35rem 0' }}><kbd style={{ padding: '0.15rem 0.4rem', background: 'var(--bg-tertiary)', borderRadius: 4 }}>Alt</kbd> + <kbd style={{ padding: '0.15rem 0.4rem', background: 'var(--bg-tertiary)', borderRadius: 4 }}>4</kbd></td><td style={{ padding: '0.35rem 0' }}>Workspaces</td></tr>
                  <tr><td style={{ padding: '0.35rem 0.5rem 0.35rem 0' }}><kbd style={{ padding: '0.15rem 0.4rem', background: 'var(--bg-tertiary)', borderRadius: 4 }}>Alt</kbd> + <kbd style={{ padding: '0.15rem 0.4rem', background: 'var(--bg-tertiary)', borderRadius: 4 }}>5</kbd></td><td style={{ padding: '0.35rem 0' }}>Clickstream</td></tr>
                  <tr><td style={{ padding: '0.35rem 0.5rem 0.35rem 0' }}><kbd style={{ padding: '0.15rem 0.4rem', background: 'var(--bg-tertiary)', borderRadius: 4 }}>Alt</kbd> + <kbd style={{ padding: '0.15rem 0.4rem', background: 'var(--bg-tertiary)', borderRadius: 4 }}>6</kbd></td><td style={{ padding: '0.35rem 0' }}>Settings</td></tr>
                  <tr><td colSpan={2} style={{ padding: '0.5rem 0 0.25rem 0', borderTop: '1px solid var(--border-primary)' }}></td></tr>
                  <tr><td style={{ padding: '0.35rem 0.5rem 0.35rem 0' }}><kbd style={{ padding: '0.15rem 0.4rem', background: 'var(--bg-tertiary)', borderRadius: 4 }}>?</kbd></td><td style={{ padding: '0.35rem 0' }}>This help</td></tr>
                  <tr><td style={{ padding: '0.35rem 0.5rem 0.35rem 0' }}><kbd style={{ padding: '0.15rem 0.4rem', background: 'var(--bg-tertiary)', borderRadius: 4 }}>Esc</kbd></td><td style={{ padding: '0.35rem 0' }}>Close modal / cancel</td></tr>
                </tbody>
              </table>
            </div>
            <div style={{ padding: '0.5rem 1rem 0.75rem', borderTop: '1px solid var(--theme-border-primary)', display: 'flex', justifyContent: 'flex-end' }}>
              <button type="button" className="btn btn-outline" onClick={() => setShowShortcutHelp(false)}>Close</button>
            </div>
          </div>
        </div>
      )}
    </>
  );
};

export default Layout;
