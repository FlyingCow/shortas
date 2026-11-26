import React, { useEffect, useState } from 'react';
import { useNavigate, useParams } from 'react-router-dom';
import { ArrowDown, ArrowUp, Plus, Trash2 } from 'lucide-react';
import {
  apiService,
  ConversionDomainDto,
  ConversionDto,
  ConversionInputDto,
  ConversionStepDto,
  CountingMode,
} from '../../services/api';
import { useAlert } from '../../contexts/AlertContext';
import Dropdown from '../Dropdown';
import LoadingSpinner from '../LoadingSpinner';
import RoutePicker from './RoutePicker';
import SecretDialog from './SecretDialog';
import { apiErrors, errorMessage, MAX_STEPS, validate } from './validation';
import { ConversionPageProps } from './Conversions';

/** The key of a saved step cannot change. */
interface FormStep extends ConversionStepDto {
  saved?: boolean;
}

interface Form extends Omit<ConversionInputDto, 'allowedOrigins' | 'windowDays' | 'routeIds' | 'steps'> {
  steps: FormStep[];
  origins: string;
  windowDays: string;
  filtered: boolean;
  routeIds: string[];
}

const EMPTY: Form = {
  name: '',
  currency: 'EUR',
  windowDays: '30',
  counting: 'OncePerClick',
  domainId: null,
  origins: '',
  serverValuesOnly: false,
  steps: [{ key: '', name: '' }],
  filtered: false,
  routeIds: [],
};

const toForm = (c: ConversionDto): Form => ({
  name: c.name,
  currency: c.currency,
  windowDays: String(c.windowDays),
  counting: c.counting,
  domainId: c.domainId ?? null,
  origins: c.allowedOrigins.join('\n'),
  serverValuesOnly: c.serverValuesOnly,
  steps: c.steps.map((s) => ({ ...s, saved: true })),
  filtered: !!c.routeIds,
  routeIds: c.routeIds ?? [],
});

const toInput = (f: Form): ConversionInputDto => ({
  name: f.name.trim(),
  currency: f.currency.trim().toUpperCase(),
  windowDays: Number(f.windowDays),
  counting: f.counting,
  domainId: f.domainId,
  allowedOrigins: f.origins.split('\n').map((o) => o.trim()).filter(Boolean),
  serverValuesOnly: f.serverValuesOnly,
  steps: f.steps.map((s) => ({
    key: s.key,
    name: s.name.trim(),
    ...(s.defaultValue ? { defaultValue: s.defaultValue } : {}),
  })),
  routeIds: f.filtered ? f.routeIds : null,
});

const ConversionEditor: React.FC<ConversionPageProps> = ({ workspaceId }) => {
  const { id } = useParams<{ id: string }>();
  const navigate = useNavigate();
  const { showToast } = useAlert();
  const [existing, setExisting] = useState<ConversionDto | null>(null);
  const [form, setForm] = useState<Form>(EMPTY);
  const [domains, setDomains] = useState<ConversionDomainDto[]>([]);
  const [domainsFailed, setDomainsFailed] = useState(false);
  const [loading, setLoading] = useState(!!id);
  const [loadError, setLoadError] = useState<string | null>(null);
  const [errors, setErrors] = useState<Record<string, string>>({});
  const [saving, setSaving] = useState(false);
  const [created, setCreated] = useState<{ id: string; secret: string } | null>(null);

  useEffect(() => {
    let current = true;
    apiService.conversions
      .domains(workspaceId)
      .then((list) => current && setDomains(list))
      .catch(() => current && setDomainsFailed(true));
    if (id) {
      apiService.conversions
        .get(workspaceId, id)
        .then((c) => {
          if (!current) return;
          setExisting(c);
          setForm(toForm(c));
        })
        .catch((err) => current && setLoadError(errorMessage(err, 'Failed to load the conversion.')))
        .finally(() => current && setLoading(false));
    }
    return () => {
      current = false;
    };
  }, [workspaceId, id]);

  const set = (change: Partial<Form>) => setForm((f) => ({ ...f, ...change }));

  const setStep = (index: number, change: Partial<ConversionStepDto>) =>
    set({ steps: form.steps.map((s, i) => (i === index ? { ...s, ...change } : s)) });

  const moveStep = (index: number, by: number) => {
    const steps = [...form.steps];
    [steps[index], steps[index + by]] = [steps[index + by], steps[index]];
    set({ steps });
    setErrors({});
  };

  const removeStep = (index: number) => {
    set({ steps: form.steps.filter((_, i) => i !== index) });
    setErrors({});
  };

  const save = async (e: React.FormEvent) => {
    e.preventDefault();
    const input = toInput(form);
    const found = validate(input);
    setErrors(found);
    if (Object.keys(found).length) return;

    setSaving(true);
    try {
      if (existing) {
        await apiService.conversions.update(workspaceId, existing.id, existing.version, input);
        navigate(`/conversions/${existing.id}`);
      } else {
        const result = await apiService.conversions.create(workspaceId, input);
        setCreated({ id: result.id, secret: result.secretKey });
      }
    } catch (err) {
      setErrors(apiErrors(err));
      showToast(errorMessage(err, 'Failed to save the conversion'), 'error');
    } finally {
      setSaving(false);
    }
  };

  if (loading) return <LoadingSpinner message="Loading conversion..." />;
  if (loadError) return <div className="container"><div className="alert alert-error">{loadError}</div></div>;
  if (existing?.status === 'Archived') {
    return <div className="container"><div className="alert alert-info">An archived conversion cannot be changed.</div></div>;
  }

  const error = (field: string) =>
    errors[field] && <span className="cv-error" role="alert">{errors[field]}</span>;
  // A domain that is no longer offered still has to be shown.
  const unknownDomain = form.domainId && !domains.some((d) => d.id === form.domainId);

  return (
    <div className="container">
      <div className="page-header">
        <div className="page-title-section">
          <h1 className="page-title">{existing ? 'Edit conversion' : 'New conversion'}</h1>
        </div>
      </div>

      <form className="cv-form" onSubmit={save} noValidate>
        <section className="cv-card">
          <h2>Basics</h2>
          <div className="cv-grid">
            <div className="form-group">
              <label className="form-label" htmlFor="cv-name">Name</label>
              <input id="cv-name" className="form-control" value={form.name} onChange={(e) => set({ name: e.target.value })} />
              {error('name')}
            </div>
            <div className="form-group">
              <label className="form-label">Short domain</label>
              <Dropdown
                block
                value={form.domainId ?? ''}
                onChange={(v) => set({ domainId: v || null })}
                error={!!errors.domainId}
                placeholder="Choose later"
                options={[
                  { value: '', label: 'Choose later' },
                  ...domains.map((d) => ({
                    value: d.id,
                    label: `${d.name}${d.isShared ? ' (Shared)' : ''}${d.verified ? '' : ' (not verified)'}`,
                  })),
                  ...(unknownDomain ? [{ value: form.domainId!, label: 'Current domain' }] : []),
                ]}
              />
              {domainsFailed && <span className="cv-error">Failed to load domains.</span>}
              {error('domainId') || (
                <span className="cv-help">Serves the script and receives events. Needed for activation.</span>
              )}
            </div>
            <div className="form-group">
              <label className="form-label" htmlFor="cv-currency">Currency</label>
              <input id="cv-currency" className="form-control" maxLength={3} value={form.currency} onChange={(e) => set({ currency: e.target.value.toUpperCase() })} />
              {error('currency')}
            </div>
            <div className="form-group">
              <label className="form-label" htmlFor="cv-window">Attribution window (days)</label>
              <input id="cv-window" className="form-control" inputMode="numeric" value={form.windowDays} onChange={(e) => set({ windowDays: e.target.value })} />
              {error('windowDays')}
            </div>
            <div className="form-group">
              <label className="form-label">Counting</label>
              <Dropdown
                block
                value={form.counting}
                onChange={(v) => set({ counting: v as CountingMode })}
                options={[
                  { value: 'OncePerClick', label: 'Once per click' },
                  { value: 'Every', label: 'Every conversion' },
                ]}
              />
              <span className="cv-help">
                {form.counting === 'Every'
                  ? 'Every distinct event counts. Good for orders.'
                  : 'Each step counts once per click. Good for signups.'}
              </span>
            </div>
          </div>
        </section>

        <section className="cv-card">
          <h2>Steps</h2>
          <p className="cv-help">In order. The last one is the conversion, and its values are the revenue.</p>
          {form.steps.map((step, i) => (
            <div className="cv-step" key={i}>
              <span className="cv-step-number">{i + 1}</span>
              <div>
                <input className="form-control" aria-label={`Step ${i + 1} name`} placeholder="Name" value={step.name} onChange={(e) => setStep(i, { name: e.target.value })} />
                {error(`steps[${i}].name`)}
              </div>
              <div>
                <input className="form-control" aria-label={`Step ${i + 1} key`} placeholder="key" value={step.key} disabled={step.saved} onChange={(e) => setStep(i, { key: e.target.value })} />
                {error(`steps[${i}].key`)}
              </div>
              <div>
                <input className="form-control" aria-label={`Step ${i + 1} default value`} placeholder="Default value" inputMode="decimal" value={step.defaultValue ?? ''} onChange={(e) => setStep(i, { defaultValue: e.target.value.trim() })} />
                {error(`steps[${i}].defaultValue`)}
              </div>
              <div className="cv-step-buttons">
                <button type="button" className="btn btn-xs btn-secondary" aria-label={`Move step ${i + 1} up`} disabled={i === 0} onClick={() => moveStep(i, -1)}><ArrowUp size={14} /></button>
                <button type="button" className="btn btn-xs btn-secondary" aria-label={`Move step ${i + 1} down`} disabled={i === form.steps.length - 1} onClick={() => moveStep(i, 1)}><ArrowDown size={14} /></button>
                <button type="button" className="btn btn-xs btn-outline-danger" aria-label={`Remove step ${i + 1}`} disabled={form.steps.length === 1} onClick={() => removeStep(i)}><Trash2 size={14} /></button>
              </div>
            </div>
          ))}
          {error('steps')}
          {!!existing?.archivedSteps.length && (
            <p className="cv-help">
              Archived: {existing.archivedSteps.map((s) => `${s.name} (${s.key})`).join(', ')}. Add a step with the same key to bring one back.
            </p>
          )}
          {existing && existing.status !== 'Draft' && (
            <p className="cv-help">Removed steps are archived: their events are kept and new ones rejected.</p>
          )}
          <button type="button" className="btn btn-sm btn-secondary" style={{ marginTop: '0.75rem' }} disabled={form.steps.length >= MAX_STEPS} onClick={() => set({ steps: [...form.steps, { key: '', name: '' }] })}>
            <Plus size={14} /> Add step
          </button>
        </section>

        <section className="cv-card">
          <h2>Routes</h2>
          <label className="cv-check">
            <input type="radio" name="cv-routes" checked={!form.filtered} onChange={() => set({ filtered: false })} />
            Every route in this workspace
          </label>
          <label className="cv-check">
            <input type="radio" name="cv-routes" checked={form.filtered} onChange={() => set({ filtered: true })} />
            Only chosen routes
          </label>
          {form.filtered && (
            <RoutePicker workspaceId={workspaceId} selected={form.routeIds} onChange={(routeIds) => set({ routeIds })} />
          )}
          {error('routeIds')}
        </section>

        <section className="cv-card">
          <h2>Trust</h2>
          <label className="cv-check">
            <input type="checkbox" checked={form.serverValuesOnly} onChange={(e) => set({ serverValuesOnly: e.target.checked })} />
            Accept values only from servers
          </label>
          <div className="form-group" style={{ marginTop: '0.75rem' }}>
            <label className="form-label" htmlFor="cv-origins">Allowed origins, one per line</label>
            <textarea id="cv-origins" className="form-control" rows={3} placeholder="https://shop.example" value={form.origins} onChange={(e) => set({ origins: e.target.value })} />
            {error('allowedOrigins') || (
              <span className="cv-help">Leave empty to allow any site. Browser events can be forged, so use server events when values matter.</span>
            )}
          </div>
        </section>

        <div className="cv-actions">
          <button type="button" className="btn btn-sm btn-secondary" disabled={saving} onClick={() => navigate(existing ? `/conversions/${existing.id}` : '/conversions')}>Cancel</button>
          <button type="submit" className="btn btn-sm btn-primary" disabled={saving}>
            {saving ? 'Saving...' : existing ? 'Save' : 'Create'}
          </button>
        </div>
      </form>

      {created && <SecretDialog secret={created.secret} onClose={() => navigate(`/conversions/${created.id}`)} />}
    </div>
  );
};

export default ConversionEditor;
