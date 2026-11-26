import React, { useState } from 'react';
import { apiService, ConversionDto } from '../../services/api';
import { useAlert } from '../../contexts/AlertContext';
import Dropdown from '../Dropdown';
import SecretDialog from './SecretDialog';
import Snippet from './Snippet';
import { refundSnippet, scriptSnippet, serverSnippet, SECRET_PLACEHOLDER, stepSnippet } from './snippets';
import { errorMessage } from './validation';

interface Props {
  conversion: ConversionDto;
  /** Undefined while the conversion has none. */
  domain?: string;
  canManage: boolean;
  onChange: (conversion: ConversionDto) => void;
  onTestSent: () => void;
}

function testFailure(err: any): string {
  switch (err?.response?.status) {
    case 400:
      return 'The short domain does not know this conversion or step yet. Try again in a minute.';
    case 429:
    case 503:
      return 'The short domain is busy. Try again shortly.';
    default:
      return 'The short domain could not be reached. Check that it is set up.';
  }
}

const InstallView: React.FC<Props> = ({ conversion, domain, canManage, onChange, onTestSent }) => {
  const { showConfirm, showToast } = useAlert();
  const [step, setStep] = useState(conversion.steps[0]?.key ?? '');
  const [sending, setSending] = useState(false);
  const [rotating, setRotating] = useState(false);
  const [secret, setSecret] = useState<string | null>(null);

  const sendTest = async () => {
    if (!domain) return;
    setSending(true);
    try {
      await apiService.conversions.sendTestEvent(domain, conversion.publicKey, step);
      showToast('Test event sent', 'success');
      onTestSent();
    } catch (err) {
      showToast(testFailure(err), 'error');
    } finally {
      setSending(false);
    }
  };

  const rotate = async () => {
    const confirmed = await showConfirm(
      'The current secret key will stop working within minutes. Servers need the new one.',
      'Rotate secret key',
      { confirmLabel: 'Rotate', variant: 'danger' }
    );
    if (!confirmed) return;
    setRotating(true);
    try {
      const { secretKey, ...rotated } = await apiService.conversions.rotateSecret(conversion.workspaceId, conversion.id);
      setSecret(secretKey);
      onChange(rotated);
    } catch (err) {
      showToast(errorMessage(err, 'Failed to rotate the secret key'), 'error');
    } finally {
      setRotating(false);
    }
  };

  const archived = conversion.status === 'Archived';
  return (
    <div className="cv-form">
      {!domain && (
        <div className="alert alert-warning">{conversion.domainId
            ? 'The short domain’s name could not be loaded. Replace YOUR-SHORT-DOMAIN.'
            : 'Choose a short domain to complete these snippets.'}</div>
      )}
      <section className="cv-card">
        <h2>1. Script</h2>
        <p className="cv-help">Put it on the landing page and on every page that reports a step.</p>
        <Snippet label="script" code={scriptSnippet(conversion, domain)} />
        <p className="cv-help">
          Add data-consent="required" to the second element to send nothing until shortas.consent(true).
        </p>
      </section>
      <section className="cv-card">
        <h2>2. Steps</h2>
        <p className="cv-help">Call each when its step happens.</p>
        <Snippet label="step calls" code={stepSnippet(conversion)} />
      </section>
      <section className="cv-card">
        <h2>3. Server</h2>
        <p className="cv-help">
          Replace CLICK_ID with shortas.clickId("{conversion.publicKey}"), passed to your server with the form or order, and {SECRET_PLACEHOLDER} with the secret key.
        </p>
        <Snippet label="server request" code={serverSnippet(conversion, domain)} />
        <Snippet label="refund request" code={refundSnippet(conversion, domain)} />
        {canManage && !archived && (
          <button className="btn btn-sm btn-outline-danger" disabled={rotating} onClick={rotate}>Rotate secret key</button>
        )}
      </section>
      <section className="cv-card">
        <h2>Test</h2>
        <p className="cv-help">
          Sends a browser event from this page. It appears in the event log marked as a test and is not counted.
        </p>
        <div className="cv-actions" style={{ justifyContent: 'flex-start', marginTop: '0.5rem' }}>
          <Dropdown
            value={step}
            onChange={setStep}
            options={conversion.steps.map((s) => ({ value: s.key, label: s.name }))}
          />
          <button className="btn btn-sm btn-secondary" disabled={!domain || archived || sending} onClick={sendTest}>
            {sending ? 'Sending...' : 'Send test event'}
          </button>
        </div>
      </section>
      {secret && <SecretDialog secret={secret} onClose={() => setSecret(null)} />}
    </div>
  );
};

export default InstallView;
