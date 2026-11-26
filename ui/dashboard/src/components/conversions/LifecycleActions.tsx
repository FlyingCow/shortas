import React, { useState } from 'react';
import { apiService, ConversionDto } from '../../services/api';
import { useAlert } from '../../contexts/AlertContext';
import { errorMessage } from './validation';

type Action = 'activate' | 'pause' | 'archive';

const CONFIRM: Record<Action, [string, string, string]> = {
  activate: [
    'Covered routes will add shortas_cid to their destination URLs. Signed or strict destinations may reject it; exclude those routes in their options first.',
    'Activate conversion',
    'Activate',
  ],
  pause: ['New events will be rejected. Stored events are kept.', 'Pause conversion', 'Pause'],
  archive: [
    'New events will be rejected for good. Stored events are kept. This cannot be undone.',
    'Archive conversion',
    'Archive',
  ],
};

interface Props {
  conversion: ConversionDto;
  onChange: (conversion: ConversionDto) => void;
}

const LifecycleActions: React.FC<Props> = ({ conversion, onChange }) => {
  const { showConfirm, showToast } = useAlert();
  const [busy, setBusy] = useState(false);
  const { status } = conversion;

  const run = async (action: Action) => {
    const [message, title, confirmLabel] = CONFIRM[action];
    const variant = action === 'archive' ? 'danger' : 'primary';
    if (!(await showConfirm(message, title, { confirmLabel, variant }))) return;
    setBusy(true);
    try {
      onChange(await apiService.conversions.changeStatus(conversion.workspaceId, conversion.id, action));
    } catch (err) {
      showToast(errorMessage(err, `Failed to ${action} the conversion`), 'error');
    } finally {
      setBusy(false);
    }
  };

  const button = (action: Action, style: string) => (
    <button className={`btn btn-sm ${style}`} disabled={busy} onClick={() => run(action)}>
      {CONFIRM[action][2]}
    </button>
  );

  if (status === 'Archived') return null;
  return (
    <>
      {(status === 'Draft' || status === 'Paused') && button('activate', 'btn-primary')}
      {(status === 'Activating' || status === 'Active') && button('pause', 'btn-secondary')}
      {button('archive', 'btn-outline-danger')}
    </>
  );
};

export default LifecycleActions;
