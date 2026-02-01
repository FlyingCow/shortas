import React, { useState, useEffect } from 'react';
import { X } from 'lucide-react';
import { RouteDto, RoutingPolicy, DomainDto, apiService } from '../services/api';
import PolicyEditor from './PolicyEditor';
import './DesignSystem.css';

interface RouteFormModalProps {
  show: boolean;
  onClose: () => void;
  onSave: (route: Partial<RouteDto>) => void;
  route?: RouteDto | null;
}

// Fields managed by the API — never sent on create/update
const API_MANAGED_FIELDS = ['id', 'status', 'terminal', 'ttl', 'domain'] as const;

const stripApiManagedFields = (data: Partial<RouteDto>): Partial<RouteDto> => {
  const cleaned = { ...data };
  for (const key of API_MANAGED_FIELDS) {
    delete (cleaned as any)[key];
  }
  // Strip routeId from properties — API generates it
  if (cleaned.properties) {
    const { routeId, ownerId, creatorId, ...restProps } = cleaned.properties;
    cleaned.properties = restProps as any;
  }
  return cleaned;
};

export const RouteFormModal: React.FC<RouteFormModalProps> = ({
  show,
  onClose,
  onSave,
  route,
}) => {
  const [domains, setDomains] = useState<DomainDto[]>([]);
  const [formData, setFormData] = useState<Partial<RouteDto>>({
    switch: 'main',
    link: '',
    dest: '',
    destFormat: 'Http',
    code: 302,
    policy: 'Basic',
    domainId: undefined,
  });

  useEffect(() => {
    if (show) {
      fetchDomains();
    }
  }, [show]);

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
        policy: 'Basic',
        domainId: undefined,
      });
    }
  }, [route]);

  const fetchDomains = async () => {
    try {
      const response = await apiService.domains.list({ page: 1, pageSize: 100 });
      setDomains(response.data);
    } catch (err) {
      console.error('Failed to fetch domains:', err);
    }
  };

  const handleSubmit = (e: React.FormEvent) => {
    e.preventDefault();

    if (!formData.domainId || formData.domainId.trim() === '') {
      alert('Domain is required. Please select a domain.');
      return;
    }

    const dataToSave: Partial<RouteDto> = {
      ...formData,
      properties: formData.properties ? {
        ...formData.properties,
        domainId: formData.domainId!,
      } : {
        routeId: '',
        domainId: formData.domainId!,
        ownerId: '',
        scripts: [],
        tags: [],
        custom: {},
        opengraph: false,
        allowDebug: false
      }
    };

    onSave(stripApiManagedFields(dataToSave));
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
                  <label className="label">Domain *</label>
                  <select
                    className="input"
                    value={formData.domainId || ''}
                    onChange={(e) => handleChange('domainId', e.target.value)}
                    required
                  >
                    <option value="">Select a domain...</option>
                    {domains.map((domain) => (
                      <option key={domain.id} value={domain.id}>
                        {domain.name}
                      </option>
                    ))}
                  </select>
                  <p className="text-xs text-muted mt-1">
                    Domain is required for all routes
                  </p>
                </div>

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
                    <label className="label">HTTP Code</label>
                    <select
                      className="input"
                      value={formData.code || 302}
                      onChange={(e) => handleChange('code', parseInt(e.target.value))}
                    >
                      <option value={301}>301 - Permanent</option>
                      <option value={302}>302 - Temporary</option>
                      <option value={307}>307 - Temporary (Preserve Method)</option>
                      <option value={308}>308 - Permanent (Preserve Method)</option>
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
