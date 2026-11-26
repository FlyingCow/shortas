import React, { useState } from 'react';

const Snippet: React.FC<{ label: string; code: string }> = ({ label, code }) => {
  const [copied, setCopied] = useState(false);

  const copy = async () => {
    try {
      await navigator.clipboard.writeText(code);
      setCopied(true);
      setTimeout(() => setCopied(false), 1500);
    } catch {
      // Not allowed here; the text can still be selected.
    }
  };

  return (
    <div className="cv-snippet">
      <pre aria-label={label}>{code}</pre>
      <button type="button" className="btn btn-xs btn-secondary" onClick={copy} aria-label={`Copy ${label}`}>
        {copied ? 'Copied' : 'Copy'}
      </button>
    </div>
  );
};

export default Snippet;
