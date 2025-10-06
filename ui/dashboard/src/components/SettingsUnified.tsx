import React, { useState, useEffect } from 'react';
import { 
  User, 
  Bell, 
  Shield, 
  Globe,
  Save,
  Eye,
  EyeOff,
  Sun,
  Moon,
  Monitor
} from 'lucide-react';
// Removed Bootstrap Dropdown import - using unified controls
import { apiService } from '../services/api';
import { useTheme } from '../contexts/ThemeContext';
import LoadingSpinner from './LoadingSpinner';
import './DesignSystem.css';

interface UserSettings {
  email: string;
  name: string;
  timezone: string;
  language: string;
  notifications: {
    email: boolean;
    push: boolean;
    sms: boolean;
  };
  privacy: {
    profilePublic: boolean;
    analyticsSharing: boolean;
    dataRetention: string;
  };
  security: {
    twoFactor: boolean;
    sessionTimeout: number;
    loginAlerts: boolean;
  };
}

const Settings: React.FC = () => {
  const { theme, toggleTheme, setTheme } = useTheme();
  const [settings, setSettings] = useState<UserSettings | null>(null);
  const [loading, setLoading] = useState(true);
  const [saving, setSaving] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [success, setSuccess] = useState<string | null>(null);
  const [showPassword, setShowPassword] = useState(false);

  useEffect(() => {
    fetchUserSettings();
  }, []);

  const fetchUserSettings = async () => {
    try {
      setLoading(true);
      setError(null);
      
      // Mock settings data
      const mockSettings: UserSettings = {
        email: 'user@example.com',
        name: 'John Doe',
        timezone: 'UTC',
        language: 'en',
        notifications: {
          email: true,
          push: true,
          sms: false
        },
        privacy: {
          profilePublic: false,
          analyticsSharing: true,
          dataRetention: '1year'
        },
        security: {
          twoFactor: false,
          sessionTimeout: 30,
          loginAlerts: true
        }
      };
      
      setSettings(mockSettings);
    } catch (err) {
      console.error('Failed to fetch settings:', err);
      setError('Failed to load settings. Please try again.');
    } finally {
      setLoading(false);
    }
  };

  const handleSave = async () => {
    try {
      setSaving(true);
      setError(null);
      setSuccess(null);
      
      // Simulate API call
      await new Promise(resolve => setTimeout(resolve, 1000));
      
      setSuccess('Settings saved successfully!');
      setTimeout(() => setSuccess(null), 3000);
    } catch (err) {
      console.error('Failed to save settings:', err);
      setError('Failed to save settings. Please try again.');
    } finally {
      setSaving(false);
    }
  };

  const updateSettings = (path: string, value: any) => {
    if (!settings) return;
    
    const keys = path.split('.');
    const newSettings = { ...settings };
    let current = newSettings as any;
    
    for (let i = 0; i < keys.length - 1; i++) {
      current = current[keys[i]];
    }
    
    current[keys[keys.length - 1]] = value;
    setSettings(newSettings);
  };

  if (loading) {
    return <LoadingSpinner message="Loading settings..." />;
  }

  if (error && !settings) {
    return (
      <div className="alert alert-error">
        <h3>Error Loading Settings</h3>
        <p>{error}</p>
        <button className="btn btn-primary" onClick={fetchUserSettings}>
          Retry
        </button>
      </div>
    );
  }

  return (
    <div className="container">
      {/* Page Header */}
      <div className="page-header">
        <h1 className="page-title">Settings</h1>
        <p className="page-subtitle">Manage your account preferences and security settings</p>
      </div>

      {/* Success/Error Messages */}
      {success && (
        <div className="alert alert-success">
          <p>{success}</p>
        </div>
      )}
      
      {error && (
        <div className="alert alert-error">
          <p>{error}</p>
        </div>
      )}

      <div className="grid grid-cols-1 lg:grid-cols-3 gap-lg">
        {/* Profile Settings */}
        <div className="card">
          <div className="card-header">
            <h3 className="card-title">
              <User size={20} />
              Profile
            </h3>
            <p className="card-subtitle">Basic account information</p>
          </div>
          <div className="card-body">
            <div className="form-group">
              <label className="form-label">Full Name</label>
              <input
                type="text"
                className="form-control"
                value={settings?.name || ''}
                onChange={(e) => updateSettings('name', e.target.value)}
              />
            </div>
            
            <div className="form-group">
              <label className="form-label">Email Address</label>
              <input
                type="email"
                className="form-control"
                value={settings?.email || ''}
                onChange={(e) => updateSettings('email', e.target.value)}
              />
            </div>
            
            <div className="form-group">
              <label className="form-label">Timezone</label>
              <div className="control-select">
                <select
                  value={settings?.timezone || 'UTC'}
                  onChange={(e) => updateSettings('timezone', e.target.value)}
                >
                  <option value="UTC">UTC</option>
                  <option value="America/New_York">Eastern Time</option>
                  <option value="America/Chicago">Central Time</option>
                  <option value="America/Denver">Mountain Time</option>
                  <option value="America/Los_Angeles">Pacific Time</option>
                  <option value="Europe/London">London</option>
                  <option value="Europe/Paris">Paris</option>
                  <option value="Asia/Tokyo">Tokyo</option>
                </select>
              </div>
            </div>
            
            <div className="form-group">
              <label className="form-label">Language</label>
              <div className="control-select">
                <select
                  value={settings?.language || 'en'}
                  onChange={(e) => updateSettings('language', e.target.value)}
                >
                  <option value="en">English</option>
                  <option value="es">Spanish</option>
                  <option value="fr">French</option>
                  <option value="de">German</option>
                  <option value="ja">Japanese</option>
                </select>
              </div>
            </div>
          </div>
        </div>

        {/* Theme Settings */}
        <div className="card">
          <div className="card-header">
            <h3 className="card-title">
              <Monitor size={20} />
              Appearance
            </h3>
            <p className="card-subtitle">Customize your dashboard appearance</p>
          </div>
          <div className="card-body">
            <div className="form-group">
              <label className="form-label">Theme</label>
              <div className="theme-selector">
                <div className="theme-options">
                  <button
                    className={`theme-option ${theme === 'light' ? 'active' : ''}`}
                    onClick={() => setTheme('light')}
                  >
                    <Sun size={20} />
                    <span>Light</span>
                  </button>
                  <button
                    className={`theme-option ${theme === 'dark' ? 'active' : ''}`}
                    onClick={() => setTheme('dark')}
                  >
                    <Moon size={20} />
                    <span>Dark</span>
                  </button>
                </div>
                <p className="text-sm text-muted">
                  Current theme: <strong>{theme === 'light' ? 'Light' : 'Dark'}</strong>
                </p>
              </div>
            </div>
          </div>
        </div>

        {/* Notification Settings */}
        <div className="card">
          <div className="card-header">
            <h3 className="card-title">
              <Bell size={20} />
              Notifications
            </h3>
            <p className="card-subtitle">Choose how you want to be notified</p>
          </div>
          <div className="card-body">
            <div className="form-group">
              <label className="form-label">
                <input
                  type="checkbox"
                  checked={settings?.notifications.email || false}
                  onChange={(e) => updateSettings('notifications.email', e.target.checked)}
                  style={{ marginRight: '0.5rem' }}
                />
                Email Notifications
              </label>
              <p className="text-sm text-muted">Receive updates via email</p>
            </div>
            
            <div className="form-group">
              <label className="form-label">
                <input
                  type="checkbox"
                  checked={settings?.notifications.push || false}
                  onChange={(e) => updateSettings('notifications.push', e.target.checked)}
                  style={{ marginRight: '0.5rem' }}
                />
                Push Notifications
              </label>
              <p className="text-sm text-muted">Browser push notifications</p>
            </div>
            
            <div className="form-group">
              <label className="form-label">
                <input
                  type="checkbox"
                  checked={settings?.notifications.sms || false}
                  onChange={(e) => updateSettings('notifications.sms', e.target.checked)}
                  style={{ marginRight: '0.5rem' }}
                />
                SMS Notifications
              </label>
              <p className="text-sm text-muted">Text message alerts</p>
            </div>
          </div>
        </div>

        {/* Security Settings */}
        <div className="card">
          <div className="card-header">
            <h3 className="card-title">
              <Shield size={20} />
              Security
            </h3>
            <p className="card-subtitle">Account security and privacy</p>
          </div>
          <div className="card-body">
            <div className="form-group">
              <label className="form-label">
                <input
                  type="checkbox"
                  checked={settings?.security.twoFactor || false}
                  onChange={(e) => updateSettings('security.twoFactor', e.target.checked)}
                  style={{ marginRight: '0.5rem' }}
                />
                Two-Factor Authentication
              </label>
              <p className="text-sm text-muted">Add an extra layer of security</p>
            </div>
            
            <div className="form-group">
              <label className="form-label">Session Timeout (minutes)</label>
              <input
                type="number"
                className="form-control"
                value={settings?.security.sessionTimeout || 30}
                onChange={(e) => updateSettings('security.sessionTimeout', parseInt(e.target.value))}
                min="5"
                max="480"
              />
            </div>
            
            <div className="form-group">
              <label className="form-label">
                <input
                  type="checkbox"
                  checked={settings?.security.loginAlerts || false}
                  onChange={(e) => updateSettings('security.loginAlerts', e.target.checked)}
                  style={{ marginRight: '0.5rem' }}
                />
                Login Alerts
              </label>
              <p className="text-sm text-muted">Get notified of new logins</p>
            </div>
          </div>
        </div>
      </div>

      {/* Privacy Settings */}
      <div className="card">
        <div className="card-header">
          <h3 className="card-title">
            <Globe size={20} />
            Privacy & Data
          </h3>
          <p className="card-subtitle">Control your data and privacy settings</p>
        </div>
        <div className="card-body">
          <div className="grid grid-cols-1 md:grid-cols-2 gap-lg">
            <div className="form-group">
              <label className="form-label">
                <input
                  type="checkbox"
                  checked={settings?.privacy.profilePublic || false}
                  onChange={(e) => updateSettings('privacy.profilePublic', e.target.checked)}
                  style={{ marginRight: '0.5rem' }}
                />
                Public Profile
              </label>
              <p className="text-sm text-muted">Make your profile visible to others</p>
            </div>
            
            <div className="form-group">
              <label className="form-label">
                <input
                  type="checkbox"
                  checked={settings?.privacy.analyticsSharing || false}
                  onChange={(e) => updateSettings('privacy.analyticsSharing', e.target.checked)}
                  style={{ marginRight: '0.5rem' }}
                />
                Share Analytics Data
              </label>
              <p className="text-sm text-muted">Help improve our service</p>
            </div>
            
            <div className="form-group">
              <label className="form-label">Data Retention</label>
              <div className="control-select">
                <select
                  value={settings?.privacy.dataRetention || '30days'}
                  onChange={(e) => updateSettings('privacy.dataRetention', e.target.value)}
                >
                  <option value="30days">30 Days</option>
                  <option value="6months">6 Months</option>
                  <option value="1year">1 Year</option>
                  <option value="2years">2 Years</option>
                  <option value="indefinite">Indefinite</option>
                </select>
              </div>
            </div>
          </div>
        </div>
      </div>

      {/* Save Button */}
      <div className="card">
        <div className="card-body">
          <div className="flex items-center justify-between">
            <div>
              <h4>Save Changes</h4>
              <p className="text-muted">Your settings will be saved to your account</p>
            </div>
            <button 
              className="btn btn-primary"
              onClick={handleSave}
              disabled={saving}
            >
              {saving ? (
                <>
                  <div className="loading-spinner" style={{ width: '16px', height: '16px' }}></div>
                  Saving...
                </>
              ) : (
                <>
                  <Save size={16} />
                  Save Settings
                </>
              )}
            </button>
          </div>
        </div>
      </div>
    </div>
  );
};

export default Settings;
