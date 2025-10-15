import React, { useState, useEffect } from 'react';
import { X } from 'lucide-react';
import { RouteDto, RoutingPolicy } from '../services/api';
import PolicyEditor from './PolicyEditor';
import './DesignSystem.css';

interface RouteFormModalProps {
  show: boolean;
  onClose: () => void;
  onSave: (route: Partial<RouteDto>) => void;
  route?: RouteDto | null;
}

export const RouteFormModal: React.FC<RouteFormModalProps> = ({
  show,
  onClose,
  onSave,
  route,
}) => {
  const [formData, setFormData] = useState<Partial<RouteDto>>({
    switch: 'main',
    link: '',
    dest: '',
    destFormat: 'Http',
    code: 302,
    ttl: 0,
    status: 'Active',
    terminal: 'External',
    policy: 'Basic',
  });

  useEffect(() => {
    if (route) {
      setFormData(route);
    } else {
      setFormData({
        switch: 'main',
        link: '',
        dest: '',
        destFormat: 'Http',
        code: 302,
        ttl: 0,
        status: 'Active',
        terminal: 'External',
        policy: 'Basic',
      });
    }
  }, [route]);

  const handleSubmit = (e: React.FormEvent) => {
    e.preventDefault();
    onSave(formData);
  };

  const handleChange = (field: keyof RouteDto, value: any) => {
    setFormData((prev) => ({ ...prev, [field]: value }));
  };

  const handlePolicyChange = (policy: RoutingPolicy) => {
    setFormData((prev) => ({ ...prev, policy }));
  };

  if (!show) return null;

  return (
    <div className="modal-overlay" onClick={onClose}>
      <div className="modal-content" onClick={(e) => e.stopPropagation()}>
        <div className="modal-header">
          <h2 className="modal-title">
            {route ? 'Edit Route' : 'Create New Route'}
          </h2>
          <button className="modal-close" onClick={onClose}>
            <X size={20} />
          </button>
        </div>

        <form onSubmit={handleSubmit}>
          <div className="modal-body">
            <div className="space-y-md">
              {/* Basic Information */}
              <div className="space-y-sm">
                <h3 className="text-lg font-semibold">Basic Information</h3>

                <div>
                  <label className="label">Short Link *</label>
                  <input
                    type="text"
                    className="input"
                    placeholder="example.com/mylink"
                    value={formData.link || ''}
                    onChange={(e) => handleChange('link', e.target.value)}
                    required
                  />
                  <p className="text-xs text-muted mt-1">
                    Format: domain/path (e.g., example.com/promo)
                  </p>
                </div>

                <div>
                  <label className="label">Destination URL *</label>
                  <input
                    type="text"
                    className="input"
                    placeholder="https://example.com/destination"
                    value={formData.dest || ''}
                    onChange={(e) => handleChange('dest', e.target.value)}
                    required
                  />
                </div>

                <div className="grid grid-cols-2 gap-sm">
                  <div>
                    <label className="label">Switch</label>
                    <input
                      type="text"
                      className="input"
                      placeholder="main"
                      value={formData.switch || 'main'}
                      onChange={(e) => handleChange('switch', e.target.value)}
                    />
                  </div>

                  <div>
                    <label className="label">Status</label>
                    <select
                      className="input"
                      value={formData.status || 'Active'}
                      onChange={(e) => handleChange('status', e.target.value)}
                    >
                      <option value="Active">Active</option>
                      <option value="Inactive">Inactive</option>
                      <option value="Pending">Pending</option>
                    </select>
                  </div>
                </div>

                <div className="grid grid-cols-3 gap-sm">
                  <div>
                    <label className="label">HTTP Code</label>
                    <input
                      type="number"
                      className="input"
                      placeholder="302"
                      value={formData.code || 302}
                      onChange={(e) => handleChange('code', parseInt(e.target.value))}
                    />
                  </div>

                  <div>
                    <label className="label">TTL (seconds)</label>
                    <input
                      type="number"
                      className="input"
                      placeholder="0"
                      value={formData.ttl || 0}
                      onChange={(e) => handleChange('ttl', parseInt(e.target.value))}
                    />
                  </div>

                  <div>
                    <label className="label">Terminal</label>
                    <select
                      className="input"
                      value={formData.terminal || 'External'}
                      onChange={(e) => handleChange('terminal', e.target.value)}
                    >
                      <option value="External">External</option>
                      <option value="Internal">Internal</option>
                    </select>
                  </div>
                </div>
              </div>

              {/* Routing Policy */}
              <div className="space-y-sm">
                <h3 className="text-lg font-semibold">Routing Policy</h3>
                <PolicyEditor
                  policy={formData.policy || 'Basic'}
                  onChange={handlePolicyChange}
                />
              </div>

              {/* Properties */}
              <div className="space-y-sm">
                <h3 className="text-lg font-semibold">Properties (Optional)</h3>

                <div className="grid grid-cols-2 gap-sm">
                  <div>
                    <label className="label">Route ID</label>
                    <input
                      type="text"
                      className="input"
                      placeholder="my-route-001"
                      value={formData.properties?.routeId || ''}
                      onChange={(e) =>
                        handleChange('properties', {
                          ...formData.properties,
                          routeId: e.target.value,
                        })
                      }
                    />
                  </div>

                  <div>
                    <label className="label">Domain ID</label>
                    <input
                      type="text"
                      className="input"
                      placeholder="example-com"
                      value={formData.properties?.domainId || ''}
                      onChange={(e) =>
                        handleChange('properties', {
                          ...formData.properties,
                          domainId: e.target.value,
                        })
                      }
                    />
                  </div>
                </div>

                <div>
                  <label className="label">Tags (comma-separated)</label>
                  <input
                    type="text"
                    className="input"
                    placeholder="marketing, campaign, promo"
                    value={formData.properties?.tags?.join(', ') || ''}
                    onChange={(e) =>
                      handleChange('properties', {
                        ...formData.properties,
                        tags: e.target.value.split(',').map((s) => s.trim()).filter(Boolean),
                      })
                    }
                  />
                </div>

                <div className="flex gap-md">
                  <label className="flex items-center gap-xs">
                    <input
                      type="checkbox"
                      checked={formData.properties?.opengraph || false}
                      onChange={(e) =>
                        handleChange('properties', {
                          ...formData.properties,
                          opengraph: e.target.checked,
                        })
                      }
                    />
                    <span className="text-sm">Enable OpenGraph</span>
                  </label>

                  <label className="flex items-center gap-xs">
                    <input
                      type="checkbox"
                      checked={formData.properties?.allowDebug || false}
                      onChange={(e) =>
                        handleChange('properties', {
                          ...formData.properties,
                          allowDebug: e.target.checked,
                        })
                      }
                    />
                    <span className="text-sm">Allow Debug</span>
                  </label>
                </div>
              </div>
            </div>
          </div>

          <div className="modal-footer">
            <button type="button" className="btn btn-outline" onClick={onClose}>
              Cancel
            </button>
            <button type="submit" className="btn btn-primary">
              {route ? 'Update Route' : 'Create Route'}
            </button>
          </div>
        </form>
      </div>
    </div>
  );
};

export default RouteFormModal;
