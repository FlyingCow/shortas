import React, { useCallback, useEffect, useState } from 'react';
import { Link, useParams } from 'react-router-dom';
import { apiService, ConversionDomainDto, ConversionDto } from '../../services/api';
import LoadingSpinner from '../LoadingSpinner';
import { ACTIVATING_POLL_MS } from './ConversionList';
import { ConversionPageProps } from './Conversions';
import EventLog from './EventLog';
import InstallView from './InstallView';
import LifecycleActions from './LifecycleActions';
import StatusBadge from './StatusBadge';
import { errorMessage } from './validation';

const ConversionDetail: React.FC<ConversionPageProps> = ({ workspaceId, canManage }) => {
  const { id = '' } = useParams<{ id: string }>();
  const [conversion, setConversion] = useState<ConversionDto | null>(null);
  const [domains, setDomains] = useState<ConversionDomainDto[]>([]);
  const [error, setError] = useState<string | null>(null);
  const [tab, setTab] = useState<'install' | 'events'>('install');

  const load = useCallback(async () => {
    try {
      setConversion(await apiService.conversions.get(workspaceId, id));
      setError(null);
    } catch (err) {
      setError(errorMessage(err, 'Failed to load the conversion.'));
    }
  }, [workspaceId, id]);

  useEffect(() => {
    load();
    apiService.conversions
      .domains(workspaceId)
      .then(setDomains)
      .catch((err) => console.error('Failed to load domains:', err));
  }, [load, workspaceId]);

  const activating = conversion?.status === 'Activating';
  useEffect(() => {
    if (!activating) return;
    const timer = setInterval(load, ACTIVATING_POLL_MS);
    return () => clearInterval(timer);
  }, [activating, load]);

  if (!conversion) {
    return error ? <div className="container"><div className="alert alert-error">{error}</div></div> : <LoadingSpinner message="Loading conversion..." />;
  }

  const domain = domains.find((d) => d.id === conversion.domainId)?.name;
  return (
    <div className="container">
      <div className="page-header">
        <div className="page-title-section">
          <h1 className="page-title">{conversion.name}</h1>
          <p className="page-subtitle">
            <StatusBadge status={conversion.status} />{' '}
            {activating && 'Covered routes are being updated.'}
          </p>
        </div>
        {canManage && conversion.status !== 'Archived' && (
          <div className="page-actions">
            <Link className="btn btn-sm btn-secondary" to={`/conversions/${conversion.id}/edit`}>Edit</Link>
            <LifecycleActions conversion={conversion} onChange={setConversion} />
          </div>
        )}
      </div>

      <div className="cv-tabs" role="tablist">
        {(['install', 'events'] as const).map((t) => (
          <button key={t} role="tab" aria-selected={tab === t} className={`btn btn-sm ${tab === t ? 'btn-primary' : 'btn-secondary'}`} onClick={() => setTab(t)}>
            {t === 'install' ? 'Install' : 'Events'}
          </button>
        ))}
      </div>

      {tab === 'install' ? (
        <InstallView conversion={conversion} domain={domain} canManage={canManage} onChange={setConversion} onTestSent={() => setTab('events')} />
      ) : (
        <EventLog conversion={conversion} />
      )}
    </div>
  );
};

export default ConversionDetail;
