import React from 'react';
import Snippet from './Snippet';

/** The only place a secret key is shown. It is kept in memory until closed. */
const SecretDialog: React.FC<{ secret: string; onClose: () => void }> = ({ secret, onClose }) => (
  <div className="modal-overlay">
    <div className="modal" role="dialog" aria-label="Secret key" style={{ maxWidth: 560 }}>
      <div className="modal-header"><h3>Secret key</h3></div>
      <div className="modal-body">
        <div className="alert alert-warning">
          Copy it now. It is shown only once. Use it on your server and never in a page or app.
        </div>
        <Snippet label="secret key" code={secret} />
      </div>
      <div className="modal-footer">
        <button className="btn btn-primary btn-sm" onClick={onClose}>I have copied it</button>
      </div>
    </div>
  </div>
);

export default SecretDialog;
