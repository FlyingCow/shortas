import React, { useState, useEffect, useRef } from 'react';
import { Shuffle, Link2, Globe, ChevronRight, ArrowRightLeft, GitBranch, Tag, Settings } from 'lucide-react';
import { RouteDto, RoutingPolicy, DomainDto, ConditionRouteDto, ConditionalRouting, apiService } from '../services/api';
import { ConditionsEditor } from './ConditionsEditor';
import './DesignSystem.css';
import './RouteForm.css';

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

// Parse existing conditional policy into conditions array.
// Destinations are stored inline in the policy (single-route model).
const parseConditionsFromPolicy = (policy?: RoutingPolicy): ConditionRouteDto[] => {
  if (!policy || typeof policy !== 'object' || !('Conditional' in policy)) {
    return [];
  }
  return (policy.Conditional || []).map((routing: ConditionalRouting) => ({
    dest: routing.dest || '',
    condition: routing.condition,
  }));
};

const HTTP_CODES: { value: number; label: string; desc: string }[] = [
  { value: 301, label: '301', desc: 'Permanent redirect — browsers cache this' },
  { value: 302, label: '302', desc: 'Temporary redirect — default, no caching' },
  { value: 307, label: '307', desc: 'Temporary, preserves request method (POST stays POST)' },
  { value: 308, label: '308', desc: 'Permanent, preserves request method' },
];

const HTTP_CODE_DESCRIPTIONS: Record<number, string> = {
  301: 'Permanent redirect - browsers cache this',
  302: 'Temporary redirect - default, no caching',
  307: 'Temporary, preserves request method',
  308: 'Permanent, preserves request method',
};

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
      // For edit mode, use conditions from route if available, otherwise parse from policy
      const conditions = route.conditions?.length
        ? route.conditions
        : parseConditionsFromPolicy(route.policy);
      return {
        ...route,
        policy: cleanPolicy(route.policy) || 'Basic',
        domainId: route.domainId || route.properties?.domainId || '',
        conditions,
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
      conditions: [],
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
  const [generatingLink, setGeneratingLink] = useState(false);
  const formRef = useRef<HTMLFormElement>(null);

  // Re-initialize when route prop changes
  useEffect(() => {
    setFormData(buildInitialData());
    setErrors({});
    setTouched({});
    setSaveError(null);
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [route]);

  // Auto-select domain when there is exactly one available and creating a new route
  const singleDomain = !isEdit && domains.length === 1;
  useEffect(() => {
    if (singleDomain && !formData.domainId) {
      const domainId = domains[0].id;
      setFormData((prev) => ({ ...prev, domainId }));
    }
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [domains, singleDomain]);

  // Auto-select workspace when there is exactly one available and creating a new route
  const singleWorkspace = !isEdit && (workspaces || []).length === 1;
  useEffect(() => {
    if (singleWorkspace && !formData.properties?.workspaceId) {
      const wsId = workspaces![0].id;
      setFormData((prev) => ({
        ...prev,
        properties: { ...prev.properties, workspaceId: wsId } as any,
      }));
    }
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [workspaces, singleWorkspace]);

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

  const handleGenerateLink = async () => {
    const domainId = formData.domainId?.trim();
    if (!domainId) {
      setErrors((prev) => ({ ...prev, domainId: 'Select a domain first to generate a link' }));
      setTouched((prev) => ({ ...prev, domainId: true }));
      return;
    }
    setGeneratingLink(true);
    try {
      const result = await apiService.routes.suggestLink(domainId);
      handleChange('link', result.link);
      // Clear link error if present
      setErrors((prev) => {
        const next = { ...prev };
        delete next.link;
        return next;
      });
    } catch (err: any) {
      const message =
        err?.response?.data?.message || 'Failed to generate link';
      setSaveError(message);
    } finally {
      setGeneratingLink(false);
    }
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
        switch: 'main',  // Always set switch to 'main' - child routes are managed by API
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

      // Remove policy field - it's auto-determined based on conditions
      delete (dataToSave as any).policy;

      const cleaned = stripApiManagedFields(dataToSave);
      // Link and domain are immutable after creation — don't send them on update
      if (isEdit) {
        delete (cleaned as any).link;
        delete (cleaned as any).domainId;
      }
      await onSave(cleaned);
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
    isEdit && (formData.code && formData.code !== 302);
  const hasConditions = (formData.conditions?.length ?? 0) > 0;
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
    <form
      ref={formRef}
      className="rf-form"
      onSubmit={handleSubmit}
      noValidate
    >
      {/* Header */}
      <div className="rf-header">
        <h3 className="rf-title">
          <Link2 size={18} />
          {isEdit ? 'Edit Route' : 'Create Route'}
        </h3>
        <p className="rf-desc">
          {isEdit ? 'Update your short link settings' : 'Set up a new short link with custom destination'}
        </p>
      </div>

      {saveError && (
        <div className="rf-error-banner">
          <span>{saveError}</span>
          <button type="button" onClick={() => setSaveError(null)}>
            &times;
          </button>
        </div>
      )}

      <div className="rf-body">
        {/* Section 1: Link Setup */}
        <div className="rf-section">
          <div className="rf-section-header">
            <Globe size={14} />
            Link Setup
          </div>
          <div className="rf-section-body">
            {showWorkspace && (
              <div className="rf-control-group">
                <label className="rf-control-label">Workspace</label>
                <select
                  className="rf-select"
                  value={formData.properties?.workspaceId || ''}
                  onChange={(e) => handlePropertyChange('workspaceId', e.target.value)}
                  disabled={saving || isEdit || singleWorkspace}
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

            <div className="rf-control-group">
              <label className="rf-control-label">
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
                disabled={saving || isEdit || singleDomain}
              >
                <option value="">Select a domain...</option>
                {domains.map((d) => (
                  <option key={d.id} value={d.id}>
                    {d.name}{d.isShared ? ' (Shared)' : ''}
                  </option>
                ))}
              </select>
              {fieldError('domainId') || (
                <span className="rf-helper">
                  {isEdit
                    ? 'Domain cannot be changed after creation'
                    : 'Choose the domain for this short link'}
                </span>
              )}
            </div>

            <div className="rf-control-group">
              <label className="rf-control-label">
                Short Link<span className="rf-required">*</span>
              </label>
              <div className="rf-input-with-action">
                <input
                  className={fieldClass('link')}
                  type="text"
                  placeholder="my-link"
                  value={formData.link || ''}
                  onChange={(e) => handleChange('link', e.target.value)}
                  onBlur={() => handleBlur('link')}
                  disabled={saving || isEdit}
                  readOnly={isEdit}
                />
                {!isEdit && (
                  <button
                    type="button"
                    className="rf-btn rf-btn-secondary"
                    onClick={handleGenerateLink}
                    disabled={saving || generatingLink}
                    title="Generate a random short link"
                  >
                    <Shuffle size={14} />
                    {generatingLink ? 'Generating...' : 'Generate'}
                  </button>
                )}
              </div>
              {fieldError('link') || (
                <span className="rf-helper">
                  {isEdit
                    ? 'Link cannot be changed after creation'
                    : 'The path after the domain (e.g. promo, launch-2025) or click Generate'}
                </span>
              )}
            </div>

            <div className="rf-control-group">
              <label className="rf-control-label">
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
          </div>
        </div>

        {/* Section 2: Redirect Options */}
        <details className="rf-section-collapsible" open={hasNonDefaultRedirect || undefined}>
          <summary>
            <ArrowRightLeft size={14} />
            Redirect Options
            <ChevronRight size={14} className="rf-chevron" />
          </summary>
          <div className="rf-section-body">
            <div className="rf-control-group">
              <label className="rf-control-label">HTTP Code</label>
              <div className="rf-code-options">
                {HTTP_CODES.map((c) => (
                  <button
                    key={c.value}
                    type="button"
                    className={`rf-code-btn ${(formData.code ?? 302) === c.value ? 'active' : ''}`}
                    onClick={() => handleChange('code', c.value)}
                    disabled={saving}
                  >
                    {c.label}
                  </button>
                ))}
              </div>
              <p className="rf-code-desc">
                {HTTP_CODE_DESCRIPTIONS[formData.code ?? 302]}
              </p>
            </div>
          </div>
        </details>

        {/* Section 3: Conditional Routes */}
        <details className="rf-section-collapsible" open={hasConditions || undefined}>
          <summary>
            <GitBranch size={14} />
            Conditional Routes
            <ChevronRight size={14} className="rf-chevron" />
          </summary>
          <div className="rf-section-body">
            <ConditionsEditor
              conditions={formData.conditions || []}
              onChange={(conditions) => handleChange('conditions' as any, conditions)}
            />
          </div>
        </details>

        {/* Section 4: Tags & Options */}
        <details className="rf-section-collapsible" open={hasTagsOrFlags || undefined}>
          <summary>
            <Tag size={14} />
            Tags &amp; Options
            <ChevronRight size={14} className="rf-chevron" />
          </summary>
          <div className="rf-section-body">
            <div className="rf-control-group">
              <label className="rf-control-label">Tags (comma-separated)</label>
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

            <div className="rf-control-group">
              <label className="rf-control-label">
                <Settings size={14} />
                Options
              </label>
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
          </div>
        </details>
      </div>

      {/* Actions */}
      <div className="rf-actions">
        <button
          type="button"
          className="rf-btn rf-btn-secondary"
          onClick={onCancel}
          disabled={saving}
        >
          Cancel
        </button>
        <button type="submit" className="rf-btn rf-btn-primary" disabled={saving}>
          {saving ? 'Saving...' : isEdit ? 'Update Route' : 'Create Route'}
        </button>
      </div>
    </form>
  );
};

export default RouteForm;
