import React, { useCallback, useEffect, useRef, useState } from 'react';
import { apiService, ConversionDto, ConversionLogItem } from '../../services/api';
import LoadingSpinner from '../LoadingSpinner';

const PAGE = 50;

const STATUS: Record<string, string> = {
  attributed: 'badge-success',
  refunded: 'badge-primary',
  pending: 'badge-warning',
  unattributed: 'badge-secondary',
  rejected: 'badge-error',
  invalid: 'badge-error',
};

const REASONS: Record<string, string> = {
  no_click_id: 'No click ID',
  click_not_found: 'Click not found',
  click_not_trackable: 'Click was not trackable',
  click_not_eligible: 'Click cannot be used',
  route_not_covered: 'Route is not covered',
  before_click: 'Before the click',
  outside_window: 'After the attribution window',
  not_active: 'Conversion was not active',
  origin_not_allowed: 'Origin is not allowed',
  invalid: 'Invalid',
  original_not_found: 'Refunded event not found',
  currency_mismatch: 'Other currency than the refunded event',
  exceeds_original: 'More than was left to refund',
};

function reason(item: ConversionLogItem): string {
  if (item.status === 'pending') return 'Waiting for its click, up to 15 minutes';
  if (item.reason === 'invalid' && item.field) return `Invalid ${item.field}`;
  return REASONS[item.reason] ?? item.reason;
}

function value(item: ConversionLogItem): string {
  if (!item.value) return '';
  const text = `${item.kind === 'refund' ? '-' : ''}${item.value} ${item.currency}`;
  return item.applied && Number(item.applied) !== Number(item.value) ? `${text} (${item.applied} applied)` : text;
}

const EventLog: React.FC<{ conversion: ConversionDto }> = ({ conversion }) => {
  const { workspaceId, id } = conversion;
  const [items, setItems] = useState<ConversionLogItem[] | null>(null);
  const [hasMore, setHasMore] = useState(false);
  const [error, setError] = useState(false);
  const [busy, setBusy] = useState(false);
  // Only the latest request may change the list.
  const latest = useRef(0);

  const load = useCallback(async (offset: number) => {
    const request = ++latest.current;
    setBusy(true);
    try {
      const log = await apiService.conversions.events(workspaceId, id, PAGE, offset);
      if (request !== latest.current) return;
      setItems((previous) => {
        if (!offset || !previous) return log.items;
        // Events arriving meanwhile shift the pages.
        const seen = new Set(previous.map((i) => i.id));
        return [...previous, ...log.items.filter((i) => !seen.has(i.id))];
      });
      setHasMore(log.hasMore);
      setError(false);
    } catch (err) {
      if (request !== latest.current) return;
      console.error('Failed to load events:', err);
      setError(true);
    } finally {
      if (request === latest.current) setBusy(false);
    }
  }, [workspaceId, id]);

  useEffect(() => {
    load(0);
    return () => {
      latest.current += 1;
    };
  }, [load]);

  const stepName = (key: string) =>
    [...conversion.steps, ...conversion.archivedSteps].find((s) => s.key === key)?.name ?? key;

  return (
    <div>
      <div className="cv-actions" style={{ marginBottom: '0.75rem' }}>
        <button className="btn btn-sm btn-secondary" disabled={busy} onClick={() => load(0)}>Refresh</button>
      </div>
      {error && <div className="alert alert-error">Failed to load events.</div>}
      {!items && !error && <LoadingSpinner message="Loading events..." />}
      {items?.length === 0 && (
        <div className="empty-state">
          <div className="empty-state-title">No events yet</div>
          <div className="empty-state-description">Rejected events are kept for 30 days.</div>
        </div>
      )}
      {!!items?.length && (
        <div className="table-container">
          <div className="table-wrapper">
            <table className="unified-table">
              <thead>
                <tr>
                  <th>Received</th>
                  <th>Step</th>
                  <th>Status</th>
                  <th>Reason</th>
                  <th>Value</th>
                  <th>Source</th>
                  <th>Reference</th>
                </tr>
              </thead>
              <tbody>
                {items.map((item) => (
                  <tr key={item.id}>
                    <td title={item.receivedAt}>{new Date(item.receivedAt).toLocaleString()}</td>
                    <td>{item.kind === 'refund' ? 'Refund' : stepName(item.step)}</td>
                    <td>
                      <span className="cv-badges">
                        <span className={`badge ${STATUS[item.status] ?? 'badge-secondary'}`}>{item.status}</span>
                        {item.test && <span className="badge badge-primary">test</span>}
                      </span>
                    </td>
                    <td>{reason(item)}</td>
                    <td>{value(item)}</td>
                    <td>{item.source}</td>
                    <td>{item.externalId || item.clickId}</td>
                  </tr>
                ))}
              </tbody>
            </table>
          </div>
        </div>
      )}
      {hasMore && (
        <button className="btn btn-sm btn-secondary" style={{ marginTop: '0.75rem' }} disabled={busy} onClick={() => load(items?.length ?? 0)}>
          Load more
        </button>
      )}
    </div>
  );
};

export default EventLog;
