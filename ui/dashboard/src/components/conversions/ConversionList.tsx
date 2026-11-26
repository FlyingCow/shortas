import React, { useCallback, useEffect, useState } from 'react';
import { Link } from 'react-router-dom';
import { Plus, Target } from 'lucide-react';
import { apiService, ConversionDto } from '../../services/api';
import LoadingSpinner from '../LoadingSpinner';
import LifecycleActions from './LifecycleActions';
import StatusBadge from './StatusBadge';
import { ConversionPageProps } from './Conversions';

export const ACTIVATING_POLL_MS = 4000;

const ConversionList: React.FC<ConversionPageProps> = ({ workspaceId, canManage }) => {
  const [conversions, setConversions] = useState<ConversionDto[] | null>(null);
  const [error, setError] = useState(false);

  const load = useCallback(async () => {
    try {
      setConversions(await apiService.conversions.list(workspaceId));
      setError(false);
    } catch (err) {
      console.error('Failed to load conversions:', err);
      setError(true);
    }
  }, [workspaceId]);

  useEffect(() => { load(); }, [load]);

  const activating = conversions?.some((c) => c.status === 'Activating');
  useEffect(() => {
    if (!activating) return;
    const timer = setInterval(load, ACTIVATING_POLL_MS);
    return () => clearInterval(timer);
  }, [activating, load]);

  const replace = (changed: ConversionDto) =>
    setConversions((list) => list && list.map((c) => (c.id === changed.id ? changed : c)));

  return (
    <div className="container">
      <div className="page-header">
        <div className="page-title-section">
          <h1 className="page-title">Conversions</h1>
          <p className="page-subtitle">Track signups, orders and other goals reached from your short links</p>
        </div>
        {canManage && (
          <div className="page-actions">
            <Link className="btn btn-primary btn-sm" to="/conversions/new">
              <Plus size={16} />
              New Conversion
            </Link>
          </div>
        )}
      </div>

      {error && <div className="alert alert-error">Failed to load conversions.</div>}
      {!conversions && !error && <LoadingSpinner message="Loading conversions..." />}
      {conversions?.length === 0 && (
        <div className="empty-state">
          <div className="empty-state-icon"><Target size={24} /></div>
          <div className="empty-state-title">No conversions</div>
          <div className="empty-state-description">
            {canManage
              ? 'Create one to see which links lead to results.'
              : 'An Owner or Admin of this workspace can create them.'}
          </div>
        </div>
      )}
      {!!conversions?.length && (
        <div className="table-container">
          <div className="table-wrapper">
            <table className="unified-table">
              <thead>
                <tr>
                  <th>Name</th>
                  <th>Status</th>
                  <th>Steps</th>
                  <th>Routes</th>
                  <th>Window</th>
                  {canManage && <th aria-label="Actions" />}
                </tr>
              </thead>
              <tbody>
                {conversions.map((c) => (
                  <tr key={c.id}>
                    <td><Link className="cv-name" to={`/conversions/${c.id}`}>{c.name}</Link></td>
                    <td><StatusBadge status={c.status} /></td>
                    <td>{c.steps.map((s) => s.name).join(' → ')}</td>
                    <td>{c.routeIds ? c.routeIds.length : 'All'}</td>
                    <td>{c.windowDays} days</td>
                    {canManage && (
                      <td>
                        <div className="cv-actions">
                          {c.status !== 'Archived' && (
                            <Link className="btn btn-sm btn-secondary" to={`/conversions/${c.id}/edit`}>Edit</Link>
                          )}
                          <LifecycleActions conversion={c} onChange={replace} />
                        </div>
                      </td>
                    )}
                  </tr>
                ))}
              </tbody>
            </table>
          </div>
        </div>
      )}
    </div>
  );
};

export default ConversionList;
