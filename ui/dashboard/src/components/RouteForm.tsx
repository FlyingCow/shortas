import React, { useState, useEffect, useRef } from 'react';
import { RouteDto, RoutingPolicy, DomainDto } from '../services/api';
import PolicyEditor from './PolicyEditor';
import './DesignSystem.css';

interface RouteFormProps {
  route?: RouteDto | null;
  domains: DomainDto[];
  workspaces?: any[];
  onSave: (data: Partial<RouteDto>) => Promise<void>;
  onCancel: () => void;
  showWorkspace?: boolean;
}

// Fields managed by the API — never sent on create/update
const API_MANAGED_FIELDS = ['id', 'status', 'terminal', 'ttl', 'domain'] as const;

const stripApiManagedFields = (data: Partial<RouteDto>): Partial<RouteDto> => {
  const cleaned = { ...data };
  for (const key of API_MANAGED_FIELDS) {
    delete (cleaned as any)[key];
  }
  if (cleaned.properties) {
    const { routeId, ownerId, creatorId, ...restProps } = cleaned.properties;
    cleaned.properties = restProps as any;
  }
  return cleaned;
};

const cleanCondition = (condition: any): any => {
  if (!condition || typeof condition !== 'object') return condition;

  const cleaned: any = {};
  for (const [key, value] of Object.entries(condition)) {
    if (value === null || value === undefined) continue;

    if (typeof value === 'object' && !Array.isArray(value)) {
      const cleanedNested: any = {};
      for (const [nestedKey, nestedValue] of Object.entries(value as object)) {
        if (nestedValue === null || nestedValue === undefined || nestedValue === '') continue;
        if (Array.isArray(nestedValue) && nestedValue.length === 0) continue;
        cleanedNested[nestedKey] = nestedValue;
      }
      if (Object.keys(cleanedNested).length > 0) {
        cleaned[key] = cleanedNested;
      }
    } else if (Array.isArray(value)) {
      if (value.length > 0) {
        cleaned[key] = value.map((item) =>
          typeof item === 'object' ? cleanCondition(item) : item
        );
      }
    } else {
      cleaned[key] = value;
    }
  }
  return cleaned;
};

const cleanPolicy = (policy: any): any => {
  if (!policy || policy === 'Basic' || policy === 'Mirroring') return policy;
  if (typeof policy === 'object' && 'Conditional' in policy) {
    return {
      Conditional: (policy.Conditional || []).map((cond: any) => ({
        key: cond.key,
        condition: cleanCondition(cond.condition),
      })),
    };
  }
  return policy;
};

const getPolicyType = (policy?: RoutingPolicy): string => {
  if (!policy || policy === 'Basic') return 'Basic';
  if (policy === 'Mirroring') return 'Mirroring';
  if (typeof policy === 'object') {
    if ('Conditional' in policy) return 'Conditional';
    if ('Challenge' in policy) return 'Challenge';
    if ('File' in policy) return 'File';
  }
  return 'Basic';
};

const HTTP_CODES: { value: number; label: string; desc: string }[] = [
  { value: 301, label: '301', desc: 'Permanent redirect — browsers cache this' },
  { value: 302, label: '302', desc: 'Temporary redirect — default, no caching' },
  { value: 307, label: '307', desc: 'Temporary, preserves request method (POST stays POST)' },
  { value: 308, label: '308', desc: 'Permanent, preserves request method' },
];

const routeFormStyles = `
.rf-form {
  display: flex;
  flex-direction: column;
  gap: var(--space-md);
}
.rf-section {
  border: 1px solid var(--border-primary);
  border-radius: var(--radius-md);
  background: var(--bg-primary);
}
.rf-section summary {
  padding: var(--space-sm) var(--space-md);
  font-weight: 600;
  font-size: 0.9375rem;
  color: var(--text-primary);
  cursor: pointer;
  user-select: none;
  list-style: none;
  display: flex;
  align-items: center;
  justify-content: space-between;
  border-bottom: 1px solid transparent;
  transition: border-color var(--transition-fast);
}
.rf-section summary::-webkit-details-marker {
  display: none;
}
.rf-section summary::after {
  content: '\\25B6';
  font-size: 0.625rem;
  color: var(--text-muted);
  transition: transform var(--transition-fast);
}
.rf-section[open] > summary::after {
  transform: rotate(90deg);
}
.rf-section[open] > summary {
  border-bottom-color: var(--border-primary);
}
.rf-section-body {
  padding: var(--space-md);
  display: flex;
  flex-direction: column;
  gap: var(--space-sm);
}
.rf-section-always {
  border: 1px solid var(--border-primary);
  border-radius: var(--radius-md);
  background: var(--bg-primary);
}
.rf-section-always .rf-section-header {
  padding: var(--space-sm) var(--space-md);
  font-weight: 600;
  font-size: 0.9375rem;
  color: var(--text-primary);
  border-bottom: 1px solid var(--border-primary);
}
.rf-field {
  display: flex;
  flex-direction: column;
  gap: 0.25rem;
}
.rf-label {
  font-size: 0.8125rem;
  font-weight: 500;
  color: var(--text-secondary);
}
.rf-label .rf-required {
  color: var(--color-error);
  margin-left: 2px;
}
.rf-input,
.rf-select {
  padding: 0.5rem 0.75rem;
  border: 1px solid var(--border-secondary);
  border-radius: var(--radius-sm);
  font-size: 0.875rem;
  background: var(--bg-primary);
  color: var(--text-primary);
  transition: border-color var(--transition-fast), box-shadow var(--transition-fast);
}
.rf-input:focus,
.rf-select:focus {
  outline: none;
  border-color: var(--color-primary);
  box-shadow: 0 0 0 3px var(--color-primary-light);
}
.rf-input:disabled,
.rf-select:disabled {
  opacity: 0.6;
  cursor: not-allowed;
  background: var(--bg-tertiary);
}
.rf-input.rf-input-error,
.rf-select.rf-input-error {
  border-color: var(--color-error);
}
.rf-input.rf-input-error:focus,
.rf-select.rf-input-error:focus {
  box-shadow: 0 0 0 3px rgba(239, 68, 68, 0.15);
}
.rf-helper {
  font-size: 0.75rem;
  color: var(--text-muted);
}
.rf-error-text {
  font-size: 0.75rem;
  color: var(--color-error);
}
.rf-row {
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: var(--space-sm);
}
.rf-checkbox-group {
  display: flex;
  gap: var(--space-md);
  flex-wrap: wrap;
}
.rf-checkbox-label {
  display: flex;
  align-items: center;
  gap: 0.375rem;
  font-size: 0.875rem;
  color: var(--text-primary);
  cursor: pointer;
}
.rf-checkbox-label input[type="checkbox"] {
  accent-color: var(--color-primary);
}
.rf-actions {
  display: flex;
  justify-content: flex-end;
  gap: var(--space-sm);
  padding-top: var(--space-sm);
}
.rf-error-banner {
  padding: var(--space-sm) var(--space-md);
  background: var(--color-error-light);
  color: var(--color-error);
  border: 1px solid var(--color-error);
  border-radius: var(--radius-sm);
  font-size: 0.875rem;
  display: flex;
  align-items: center;
  justify-content: space-between;
}
.rf-error-banner button {
  background: none;
  border: none;
  color: var(--color-error);
  cursor: pointer;
  font-size: 1.125rem;
  line-height: 1;
  padding: 0 0.25rem;
}
.rf-code-option-desc {
  font-size: 0.6875rem;
  color: var(--text-muted);
  margin-top: 1px;
}
@media (max-width: 600px) {
  .rf-row {
    grid-template-columns: 1fr;
  }
}
`;

const RouteForm: React.FC<RouteFormProps> = ({
  route,
  domains,
  workspaces,
  onSave,
  onCancel,
  showWorkspace,
}) => {
  const isEdit = !!route;

  const buildInitialData = (): Partial<RouteDto> => {
    if (route) {
      return {
        ...route,
        policy: cleanPolicy(route.policy) || 'Basic',
        domainId: route.domainId || route.properties?.domainId || '',
      };
    }
    return {
      switch: 'main',
      link: '',
      dest: '',
      destFormat: 'Http',
      code: 302,
      policy: 'Basic',
      domainId: '',
      properties: {
        routeId: '',
        domainId: '',
        ownerId: '',
        scripts: [],
        tags: [],
        custom: {},
        opengraph: false,
        allowDebug: false,
      },
    };
  };

  const [formData, setFormData] = useState<Partial<RouteDto>>(buildInitialData);
  const [errors, setErrors] = useState<Record<string, string>>({});
  const [saving, setSaving] = useState(false);
  const [saveError, setSaveError] = useState<string | null>(null);
  const [touched, setTouched] = useState<Record<string, boolean>>({});
  const formRef = useRef<HTMLFormElement>(null);

  // Re-initialize when route prop changes
  useEffect(() => {
    setFormData(buildInitialData());
    setErrors({});
    setTouched({});
    setSaveError(null);
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [route]);

  // Escape key handler
  useEffect(() => {
    const handleKeyDown = (e: KeyboardEvent) => {
      if (e.key === 'Escape') {
        onCancel();
      }
    };
    document.addEventListener('keydown', handleKeyDown);
    return () => document.removeEventListener('keydown', handleKeyDown);
  }, [onCancel]);

  const handleChange = (field: keyof RouteDto, value: any) => {
    setFormData((prev) => ({ ...prev, [field]: value }));
    // Clear error when user types
    if (errors[field]) {
      setErrors((prev) => {
        const next = { ...prev };
        delete next[field];
        return next;
      });
    }
  };

  const handlePropertyChange = (key: string, value: any) => {
    setFormData((prev) => ({
      ...prev,
      properties: { ...prev.properties, [key]: value } as any,
    }));
  };

  const handleBlur = (field: string) => {
    setTouched((prev) => ({ ...prev, [field]: true }));
    validateField(field);
  };

  const validateField = (field: string): string | null => {
    let error: string | null = null;
    switch (field) {
      case 'domainId':
        if (!formData.domainId?.trim()) error = 'Domain is required';
        break;
      case 'link':
        if (!formData.link?.trim()) error = 'Short link is required';
        break;
      case 'dest':
        if (!formData.dest?.trim()) error = 'Destination URL is required';
        break;
    }
    setErrors((prev) => {
      const next = { ...prev };
      if (error) next[field] = error;
      else delete next[field];
      return next;
    });
    return error;
  };

  const validate = (): boolean => {
    const newErrors: Record<string, string> = {};
    if (!formData.domainId?.trim()) newErrors.domainId = 'Domain is required';
    if (!formData.link?.trim()) newErrors.link = 'Short link is required';
    if (!formData.dest?.trim()) newErrors.dest = 'Destination URL is required';
    setErrors(newErrors);
    setTouched({ domainId: true, link: true, dest: true });
    return Object.keys(newErrors).length === 0;
  };

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    if (!validate()) return;

    setSaving(true);
    setSaveError(null);

    try {
      const dataToSave: Partial<RouteDto> = {
        ...formData,
        properties: formData.properties
          ? { ...formData.properties, domainId: formData.domainId! }
          : {
              routeId: '',
              domainId: formData.domainId!,
              ownerId: '',
              scripts: [],
              tags: [],
              custom: {},
              opengraph: false,
              allowDebug: false,
            },
      };

      await onSave(stripApiManagedFields(dataToSave));
    } catch (err: any) {
      const message =
        err?.response?.data?.message ||
        err?.message ||
        'Failed to save route. Please try again.';
      setSaveError(message);
    } finally {
      setSaving(false);
    }
  };

  // Determine which collapsible sections should default open
  const hasNonDefaultRedirect =
    isEdit && ((formData.code && formData.code !== 302) || (formData.switch && formData.switch !== 'main'));
  const hasNonBasicPolicy = isEdit && getPolicyType(formData.policy as RoutingPolicy) !== 'Basic';
  const hasTags = (formData.properties?.tags?.length ?? 0) > 0;
  const hasFlags = formData.properties?.opengraph || formData.properties?.allowDebug;
  const hasTagsOrFlags = isEdit && (hasTags || hasFlags);

  const fieldError = (field: string) =>
    touched[field] && errors[field] ? (
      <span className="rf-error-text">{errors[field]}</span>
    ) : null;

  const fieldClass = (field: string) =>
    touched[field] && errors[field] ? 'rf-input rf-input-error' : 'rf-input';

  const selectClass = (field: string) =>
    touched[field] && errors[field] ? 'rf-select rf-input-error' : 'rf-select';

  return (
    <>
      <style>{routeFormStyles}</style>
      <form
        ref={formRef}
        className="rf-form"
        onSubmit={handleSubmit}
        noValidate
      >
        {saveError && (
          <div className="rf-error-banner">
            <span>{saveError}</span>
            <button type="button" onClick={() => setSaveError(null)}>
              &times;
            </button>
          </div>
        )}

        {/* Section 1: Link Setup (always open) */}
        <div className="rf-section-always">
          <div className="rf-section-header">Link Setup</div>
          <div className="rf-section-body">
            <div className="rf-field">
              <label className="rf-label">
                Domain<span className="rf-required">*</span>
              </label>
              <select
                className={selectClass('domainId')}
                value={formData.domainId || ''}
                onChange={(e) => {
                  handleChange('domainId', e.target.value);
                  handlePropertyChange('domainId', e.target.value);
                }}
                onBlur={() => handleBlur('domainId')}
                disabled={saving}
              >
                <option value="">Select a domain...</option>
                {domains.map((d) => (
                  <option key={d.id} value={d.id}>
                    {d.name}
                  </option>
                ))}
              </select>
              {fieldError('domainId') || (
                <span className="rf-helper">Choose the domain for this short link</span>
              )}
            </div>

            <div className="rf-field">
              <label className="rf-label">
                Short Link<span className="rf-required">*</span>
              </label>
              <input
                className={fieldClass('link')}
                type="text"
                placeholder="my-link"
                value={formData.link || ''}
                onChange={(e) => handleChange('link', e.target.value)}
                onBlur={() => handleBlur('link')}
                disabled={saving}
              />
              {fieldError('link') || (
                <span className="rf-helper">The path after the domain (e.g. promo, launch-2025)</span>
              )}
            </div>

            <div className="rf-field">
              <label className="rf-label">
                Destination URL<span className="rf-required">*</span>
              </label>
              <input
                className={fieldClass('dest')}
                type="text"
                placeholder="https://example.com/destination"
                value={formData.dest || ''}
                onChange={(e) => handleChange('dest', e.target.value)}
                onBlur={() => handleBlur('dest')}
                disabled={saving}
              />
              {fieldError('dest')}
            </div>

            {showWorkspace && (
              <div className="rf-field">
                <label className="rf-label">Workspace</label>
                <select
                  className="rf-select"
                  value={formData.properties?.workspaceId || ''}
                  onChange={(e) => handlePropertyChange('workspaceId', e.target.value)}
                  disabled={saving || isEdit}
                >
                  <option value="">Select a workspace...</option>
                  {(workspaces || []).map((ws: any) => (
                    <option key={ws.id} value={ws.id}>
                      {ws.name}
                    </option>
                  ))}
                </select>
                <span className="rf-helper">
                  {isEdit
                    ? 'Workspace cannot be changed after creation'
                    : 'Workspace cannot be changed later'}
                </span>
              </div>
            )}
          </div>
        </div>

        {/* Section 2: Redirect Options */}
        <details className="rf-section" open={hasNonDefaultRedirect || undefined}>
          <summary>Redirect Options</summary>
          <div className="rf-section-body">
            <div className="rf-field">
              <label className="rf-label">HTTP Code</label>
              <select
                className="rf-select"
                value={formData.code ?? 302}
                onChange={(e) => handleChange('code', parseInt(e.target.value))}
                disabled={saving}
              >
                {HTTP_CODES.map((c) => (
                  <option key={c.value} value={c.value}>
                    {c.label} - {c.desc}
                  </option>
                ))}
              </select>
            </div>

            <div className="rf-field">
              <label className="rf-label">Switch</label>
              <input
                className="rf-input"
                type="text"
                placeholder="main"
                value={formData.switch || 'main'}
                onChange={(e) => handleChange('switch', e.target.value)}
                disabled={saving}
              />
              <span className="rf-helper">
                Named variant of this route (default: main)
              </span>
            </div>
          </div>
        </details>

        {/* Section 3: Routing Policy */}
        <details className="rf-section" open={hasNonBasicPolicy || undefined}>
          <summary>Routing Policy</summary>
          <div className="rf-section-body">
            <PolicyEditor
              policy={(formData.policy as RoutingPolicy) || 'Basic'}
              onChange={(policy) => handleChange('policy' as any, policy)}
            />
          </div>
        </details>

        {/* Section 4: Tags & Options */}
        <details className="rf-section" open={hasTagsOrFlags || undefined}>
          <summary>Tags &amp; Options</summary>
          <div className="rf-section-body">
            <div className="rf-field">
              <label className="rf-label">Tags (comma-separated)</label>
              <input
                className="rf-input"
                type="text"
                placeholder="marketing, campaign, promo"
                value={formData.properties?.tags?.join(', ') || ''}
                onChange={(e) =>
                  handlePropertyChange(
                    'tags',
                    e.target.value
                      .split(',')
                      .map((s) => s.trim())
                      .filter(Boolean)
                  )
                }
                disabled={saving}
              />
            </div>

            <div className="rf-checkbox-group">
              <label className="rf-checkbox-label">
                <input
                  type="checkbox"
                  checked={formData.properties?.opengraph || false}
                  onChange={(e) => handlePropertyChange('opengraph', e.target.checked)}
                  disabled={saving}
                />
                Enable OpenGraph
              </label>

              <label className="rf-checkbox-label">
                <input
                  type="checkbox"
                  checked={formData.properties?.allowDebug || false}
                  onChange={(e) => handlePropertyChange('allowDebug', e.target.checked)}
                  disabled={saving}
                />
                Allow Debug
              </label>
            </div>
          </div>
        </details>

        {/* Actions */}
        <div className="rf-actions">
          <button
            type="button"
            className="btn btn-outline"
            onClick={onCancel}
            disabled={saving}
          >
            Cancel
          </button>
          <button type="submit" className="btn btn-primary" disabled={saving}>
            {saving ? 'Saving...' : isEdit ? 'Update Route' : 'Create Route'}
          </button>
        </div>
      </form>
    </>
  );
};

export default RouteForm;
