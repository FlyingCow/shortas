import React, { useState, useEffect } from 'react';
import { Container, Row, Col, Card, Form, Button, Alert, Badge, ListGroup } from 'react-bootstrap';
import { 
  User, 
  Shield, 
  Bell, 
  Database,
  Key,
  Save,
  RefreshCw
} from 'lucide-react';
import { apiService } from '../services/api';
import { getUserInfo } from '../config/keycloak';
import LoadingSpinner from './LoadingSpinner';
import './Settings.css';

interface UserSettings {
  email: string;
  status: string;
  debug: boolean;
  overflow: boolean;
  skip_tracking: string[];
  allowed_request_params: string[];
  allowed_destination_params: string[];
}

const Settings: React.FC = () => {
  const [userSettings, setUserSettings] = useState<UserSettings | null>(null);
  const [loading, setLoading] = useState(true);
  const [saving, setSaving] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [successMessage, setSuccessMessage] = useState<string | null>(null);

  const userInfo = getUserInfo();

  useEffect(() => {
    fetchUserSettings();
  }, []);

  const fetchUserSettings = async () => {
    try {
      setLoading(true);
      setError(null);
      
      if (userInfo.sub) {
        const settings = await apiService.userSettings.get(userInfo.sub);
        setUserSettings(settings);
      }
    } catch (err) {
      console.error('Failed to fetch user settings:', err);
      setError('Failed to load user settings. Please try again.');
    } finally {
      setLoading(false);
    }
  };

  const saveSettings = async () => {
    if (!userSettings || !userInfo.sub) return;

    try {
      setSaving(true);
      setError(null);
      setSuccessMessage(null);

      await apiService.userSettings.update(userInfo.sub, userSettings);
      setSuccessMessage('Settings saved successfully!');
      
      // Clear success message after 3 seconds
      setTimeout(() => setSuccessMessage(null), 3000);
    } catch (err) {
      console.error('Failed to save settings:', err);
      setError('Failed to save settings. Please try again.');
    } finally {
      setSaving(false);
    }
  };

  const updateSetting = (key: keyof UserSettings, value: any) => {
    if (!userSettings) return;
    
    setUserSettings({
      ...userSettings,
      [key]: value,
    });
  };

  const addArrayItem = (key: 'skip_tracking' | 'allowed_request_params' | 'allowed_destination_params', value: string) => {
    if (!userSettings || !value.trim()) return;
    
    const currentArray = userSettings[key];
    if (!currentArray.includes(value.trim())) {
      updateSetting(key, [...currentArray, value.trim()]);
    }
  };

  const removeArrayItem = (key: 'skip_tracking' | 'allowed_request_params' | 'allowed_destination_params', index: number) => {
    if (!userSettings) return;
    
    const currentArray = userSettings[key];
    updateSetting(key, currentArray.filter((_, i) => i !== index));
  };

  if (loading) {
    return <LoadingSpinner message="Loading settings..." />;
  }

  if (error && !userSettings) {
    return (
      <div className="error-message">
        <p>{error}</p>
        <button onClick={fetchUserSettings} className="btn">
          Retry
        </button>
      </div>
    );
  }

  return (
    <div className="settings-page">
      {/* Header */}
      <div className="settings-header">
        <div className="settings-title">
          <h2>Settings</h2>
          <p>Manage your account and application preferences</p>
        </div>
        <div className="settings-actions">
          <button 
            onClick={fetchUserSettings}
            className="btn btn-secondary"
            disabled={loading}
          >
            <RefreshCw className="btn-icon" />
            Refresh
          </button>
          <button 
            onClick={saveSettings}
            className="btn"
            disabled={saving || !userSettings}
          >
            <Save className="btn-icon" />
            {saving ? 'Saving...' : 'Save Changes'}
          </button>
        </div>
      </div>

      {/* Messages */}
      {error && (
        <div className="alert alert-error">
          {error}
        </div>
      )}

      {successMessage && (
        <div className="alert alert-success">
          {successMessage}
        </div>
      )}

      <div className="settings-content">
        {/* Profile Section */}
        <div className="settings-section">
          <div className="section-header">
            <User className="section-icon" />
            <div>
              <h3>Profile Information</h3>
              <p>Your account details and basic information</p>
            </div>
          </div>
          
          <div className="settings-grid">
            <div className="setting-item">
              <label>Username</label>
              <input
                type="text"
                value={userInfo.username || ''}
                disabled
                className="setting-input disabled"
              />
              <small>Username cannot be changed</small>
            </div>

            <div className="setting-item">
              <label>Email</label>
              <input
                type="email"
                value={userSettings?.email || ''}
                onChange={(e) => updateSetting('email', e.target.value)}
                className="setting-input"
              />
            </div>

            <div className="setting-item">
              <label>Full Name</label>
              <input
                type="text"
                value={userInfo.name || ''}
                disabled
                className="setting-input disabled"
              />
              <small>Name is managed by your identity provider</small>
            </div>

            <div className="setting-item">
              <label>Status</label>
              <select
                value={userSettings?.status || 'active'}
                onChange={(e) => updateSetting('status', e.target.value)}
                className="setting-select"
              >
                <option value="active">Active</option>
                <option value="inactive">Inactive</option>
                <option value="suspended">Suspended</option>
              </select>
            </div>
          </div>
        </div>

        {/* Application Settings */}
        <div className="settings-section">
          <div className="section-header">
            <Database className="section-icon" />
            <div>
              <h3>Application Settings</h3>
              <p>Configure how the application behaves for your account</p>
            </div>
          </div>
          
          <div className="settings-grid">
            <div className="setting-item">
              <div className="setting-toggle">
                <input
                  type="checkbox"
                  id="debug-mode"
                  checked={userSettings?.debug || false}
                  onChange={(e) => updateSetting('debug', e.target.checked)}
                />
                <label htmlFor="debug-mode">Debug Mode</label>
              </div>
              <small>Enable debug information in responses</small>
            </div>

            <div className="setting-item">
              <div className="setting-toggle">
                <input
                  type="checkbox"
                  id="overflow-mode"
                  checked={userSettings?.overflow || false}
                  onChange={(e) => updateSetting('overflow', e.target.checked)}
                />
                <label htmlFor="overflow-mode">Overflow Handling</label>
              </div>
              <small>Allow overflow handling for high traffic</small>
            </div>
          </div>
        </div>

        {/* Tracking Settings */}
        <div className="settings-section">
          <div className="section-header">
            <Shield className="section-icon" />
            <div>
              <h3>Privacy & Tracking</h3>
              <p>Control what data is tracked and processed</p>
            </div>
          </div>
          
          <div className="setting-item">
            <label>Skip Tracking Parameters</label>
            <div className="array-setting">
              <div className="array-items">
                {userSettings?.skip_tracking?.map((item, index) => (
                  <div key={index} className="array-item">
                    <span>{item}</span>
                    <button
                      onClick={() => removeArrayItem('skip_tracking', index)}
                      className="remove-button"
                    >
                      ×
                    </button>
                  </div>
                ))}
              </div>
              <div className="array-input">
                <input
                  type="text"
                  placeholder="Add parameter to skip..."
                  onKeyPress={(e) => {
                    if (e.key === 'Enter') {
                      addArrayItem('skip_tracking', e.currentTarget.value);
                      e.currentTarget.value = '';
                    }
                  }}
                  className="setting-input"
                />
              </div>
            </div>
            <small>Parameters that should not be tracked in analytics</small>
          </div>
        </div>

        {/* Parameter Settings */}
        <div className="settings-section">
          <div className="section-header">
            <Key className="section-icon" />
            <div>
              <h3>Parameter Management</h3>
              <p>Configure allowed request and destination parameters</p>
            </div>
          </div>
          
          <div className="settings-grid">
            <div className="setting-item">
              <label>Allowed Request Parameters</label>
              <div className="array-setting">
                <div className="array-items">
                  {userSettings?.allowed_request_params?.map((item, index) => (
                    <div key={index} className="array-item">
                      <span>{item}</span>
                      <button
                        onClick={() => removeArrayItem('allowed_request_params', index)}
                        className="remove-button"
                      >
                        ×
                      </button>
                    </div>
                  ))}
                </div>
                <div className="array-input">
                  <input
                    type="text"
                    placeholder="Add allowed parameter..."
                    onKeyPress={(e) => {
                      if (e.key === 'Enter') {
                        addArrayItem('allowed_request_params', e.currentTarget.value);
                        e.currentTarget.value = '';
                      }
                    }}
                    className="setting-input"
                  />
                </div>
              </div>
              <small>Parameters allowed in incoming requests</small>
            </div>

            <div className="setting-item">
              <label>Allowed Destination Parameters</label>
              <div className="array-setting">
                <div className="array-items">
                  {userSettings?.allowed_destination_params?.map((item, index) => (
                    <div key={index} className="array-item">
                      <span>{item}</span>
                      <button
                        onClick={() => removeArrayItem('allowed_destination_params', index)}
                        className="remove-button"
                      >
                        ×
                      </button>
                    </div>
                  ))}
                </div>
                <div className="array-input">
                  <input
                    type="text"
                    placeholder="Add allowed parameter..."
                    onKeyPress={(e) => {
                      if (e.key === 'Enter') {
                        addArrayItem('allowed_destination_params', e.currentTarget.value);
                        e.currentTarget.value = '';
                      }
                    }}
                    className="setting-input"
                  />
                </div>
              </div>
              <small>Parameters allowed in destination URLs</small>
            </div>
          </div>
        </div>
      </div>
    </div>
  );
};

export default Settings;
