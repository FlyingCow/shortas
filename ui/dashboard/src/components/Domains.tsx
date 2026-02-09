import React, { useState, useEffect } from 'react';
import { Plus, Trash2, Search, Globe, X, ExternalLink, Copy, Check, RefreshCw, AlertCircle, CheckCircle, Clock, ChevronDown, ChevronUp, Info } from 'lucide-react';
import { apiService, DomainDto, CreateDomainDto, DnsConfigDto, DomainVerificationStatus } from '../services/api';
import LoadingSpinner from './LoadingSpinner';
import './DesignSystem.css';

const domainStyles = `
.dom-toolbar {
  display: flex;
  align-items: center;
  gap: 0.75rem;
  margin-top: 1.5rem;
  margin-bottom: 1.25rem;
}

.dom-search {
  position: relative;
  flex: 1;
  max-width: 360px;
}

.dom-search svg {
  position: absolute;
  left: 0.75rem;
  top: 50%;
  transform: translateY(-50%);
  color: var(--text-muted);
  pointer-events: none;
}

.dom-search input {
  width: 100%;
  padding: 0.5rem 0.75rem 0.5rem 2.25rem;
  border: 1px solid var(--border-secondary);
  border-radius: var(--radius-md);
  background: var(--bg-primary);
  color: var(--text-primary);
  font-size: 0.8125rem;
  transition: border-color var(--transition-fast), box-shadow var(--transition-fast);
}

.dom-search input:focus {
  outline: none;
  border-color: var(--color-primary);
  box-shadow: 0 0 0 3px var(--color-primary-light);
}

.dom-search input::placeholder {
  color: var(--text-muted);
}

.dom-count {
  font-size: 0.8125rem;
  color: var(--text-muted);
  white-space: nowrap;
  margin-left: auto;
}

.dom-list {
  display: flex;
  flex-direction: column;
  gap: 0.5rem;
}

.dom-item {
  display: flex;
  flex-direction: column;
  background: var(--bg-primary);
  border: 1px solid var(--border-primary);
  border-radius: var(--radius-lg);
  transition: border-color var(--transition-fast), box-shadow var(--transition-fast);
  overflow: hidden;
}

.dom-item:hover {
  border-color: var(--border-secondary);
  box-shadow: 0 1px 4px rgba(0, 0, 0, 0.04);
}

.dom-item-main {
  display: flex;
  align-items: center;
  gap: 1rem;
  padding: 1rem 1.25rem;
}

.dom-icon {
  width: 36px;
  height: 36px;
  display: flex;
  align-items: center;
  justify-content: center;
  background: var(--primary-50, rgba(59, 130, 246, 0.08));
  border-radius: var(--radius-md);
  color: var(--color-primary);
  flex-shrink: 0;
}

.dom-details {
  flex: 1;
  min-width: 0;
  display: flex;
  flex-direction: column;
  gap: 0.25rem;
}

.dom-name-row {
  display: flex;
  align-items: center;
  gap: 0.625rem;
  flex-wrap: wrap;
}

.dom-name {
  font-size: 0.9375rem;
  font-weight: 600;
  color: var(--text-primary);
  word-break: break-all;
}

.dom-id {
  font-size: 0.6875rem;
  color: var(--text-muted);
  font-family: var(--font-mono, monospace);
}

/* Status badges */
.dom-status-badge {
  display: inline-flex;
  align-items: center;
  gap: 0.25rem;
  padding: 0.125rem 0.5rem;
  border-radius: 4px;
  font-size: 0.625rem;
  font-weight: 600;
  text-transform: uppercase;
  letter-spacing: 0.025em;
  white-space: nowrap;
  flex-shrink: 0;
}

.dom-status-verified {
  background: rgba(34, 197, 94, 0.1);
  color: #16a34a;
}

.dom-status-pending {
  background: rgba(234, 179, 8, 0.1);
  color: #ca8a04;
}

.dom-status-failed {
  background: rgba(239, 68, 68, 0.1);
  color: #dc2626;
}

.dom-actions {
  display: flex;
  align-items: center;
  gap: 0.25rem;
  flex-shrink: 0;
}

.dom-action-btn {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  width: 32px;
  height: 32px;
  border: none;
  background: transparent;
  border-radius: var(--radius-sm);
  color: var(--text-muted);
  cursor: pointer;
  transition: all var(--transition-fast);
}

.dom-action-btn:hover {
  background: var(--bg-tertiary);
  color: var(--text-primary);
}

.dom-action-btn.danger:hover {
  background: rgba(239, 68, 68, 0.08);
  color: var(--color-error);
}

.dom-action-btn.copied {
  color: var(--color-success);
}

.dom-action-btn:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.dom-action-btn.spinning svg {
  animation: spin 1s linear infinite;
}

@keyframes spin {
  from { transform: rotate(0deg); }
  to { transform: rotate(360deg); }
}

/* DNS Instructions panel */
.dom-dns-panel {
  border-top: 1px solid var(--border-primary);
  background: var(--bg-secondary);
}

.dom-dns-toggle {
  display: flex;
  align-items: center;
  gap: 0.5rem;
  width: 100%;
  padding: 0.625rem 1.25rem;
  border: none;
  background: transparent;
  color: var(--text-secondary);
  font-size: 0.75rem;
  font-weight: 500;
  cursor: pointer;
  text-align: left;
  transition: color var(--transition-fast);
}

.dom-dns-toggle:hover {
  color: var(--text-primary);
}

.dom-dns-toggle svg {
  flex-shrink: 0;
}

.dom-dns-content {
  padding: 0 1.25rem 1rem;
}

.dom-dns-section {
  margin-bottom: 1rem;
}

.dom-dns-section:last-child {
  margin-bottom: 0;
}

.dom-dns-section h4 {
  margin: 0 0 0.5rem;
  font-size: 0.6875rem;
  font-weight: 600;
  color: var(--text-secondary);
  text-transform: uppercase;
  letter-spacing: 0.05em;
}

.dom-dns-record {
  display: flex;
  flex-direction: column;
  gap: 0.375rem;
  padding: 0.625rem 0.75rem;
  background: var(--bg-primary);
  border: 1px solid var(--border-primary);
  border-radius: var(--radius-md);
  font-family: var(--font-mono, monospace);
  font-size: 0.75rem;
}

.dom-dns-record-row {
  display: flex;
  gap: 0.5rem;
  align-items: baseline;
}

.dom-dns-record-label {
  color: var(--text-muted);
  min-width: 50px;
}

.dom-dns-record-value {
  color: var(--text-primary);
  word-break: break-all;
  flex: 1;
}

.dom-dns-record-copy {
  padding: 0.25rem;
  border: none;
  background: transparent;
  color: var(--text-muted);
  cursor: pointer;
  border-radius: var(--radius-sm);
  display: inline-flex;
  align-items: center;
  justify-content: center;
  transition: all var(--transition-fast);
}

.dom-dns-record-copy:hover {
  background: var(--bg-tertiary);
  color: var(--text-primary);
}

.dom-dns-record-copy.copied {
  color: var(--color-success);
}

.dom-dns-note {
  font-size: 0.6875rem;
  color: var(--text-muted);
  margin-top: 0.25rem;
  display: flex;
  align-items: flex-start;
  gap: 0.375rem;
}

.dom-dns-note svg {
  flex-shrink: 0;
  margin-top: 1px;
}

.dom-verification-reason {
  font-size: 0.6875rem;
  color: var(--text-muted);
  margin-top: 0.125rem;
}

/* Empty state */
.dom-empty {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  padding: 4rem 2rem;
  text-align: center;
}

.dom-empty-icon {
  width: 56px;
  height: 56px;
  display: flex;
  align-items: center;
  justify-content: center;
  background: var(--bg-tertiary);
  border-radius: 50%;
  color: var(--text-muted);
  margin-bottom: 1rem;
}

.dom-empty h3 {
  margin: 0 0 0.375rem 0;
  font-size: 1rem;
  font-weight: 600;
  color: var(--text-primary);
}

.dom-empty p {
  margin: 0 0 1.25rem 0;
  font-size: 0.8125rem;
  color: var(--text-secondary);
  max-width: 320px;
}

/* Modal */
.dom-modal-overlay {
  position: fixed;
  inset: 0;
  background: rgba(0, 0, 0, 0.4);
  backdrop-filter: blur(4px);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 1000;
  padding: 1rem;
}

.dom-modal {
  background: var(--bg-elevated);
  border-radius: var(--radius-xl);
  box-shadow: var(--shadow-xl);
  width: 100%;
  max-width: 520px;
  max-height: 90vh;
  overflow-y: auto;
}

.dom-modal-header {
  padding: 1.25rem 1.5rem;
  border-bottom: 1px solid var(--border-primary);
  display: flex;
  align-items: center;
  justify-content: space-between;
}

.dom-modal-header h2 {
  margin: 0;
  font-size: 1rem;
  font-weight: 600;
  color: var(--text-primary);
}

.dom-modal-close {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 28px;
  height: 28px;
  border: none;
  background: transparent;
  border-radius: var(--radius-sm);
  color: var(--text-muted);
  cursor: pointer;
  transition: all var(--transition-fast);
}

.dom-modal-close:hover {
  background: var(--bg-tertiary);
  color: var(--text-primary);
}

.dom-modal-body {
  padding: 1.5rem;
}

.dom-field {
  display: flex;
  flex-direction: column;
  gap: 0.375rem;
}

.dom-field label {
  font-size: 0.8125rem;
  font-weight: 500;
  color: var(--text-secondary);
}

.dom-field input {
  padding: 0.625rem 0.75rem;
  border: 1px solid var(--border-secondary);
  border-radius: var(--radius-md);
  background: var(--bg-primary);
  color: var(--text-primary);
  font-size: 0.875rem;
  transition: border-color var(--transition-fast), box-shadow var(--transition-fast);
}

.dom-field input:focus {
  outline: none;
  border-color: var(--color-primary);
  box-shadow: 0 0 0 3px var(--color-primary-light);
}

.dom-field input::placeholder {
  color: var(--text-muted);
}

.dom-field .dom-hint {
  font-size: 0.6875rem;
  color: var(--text-muted);
}

.dom-modal-footer {
  padding: 1rem 1.5rem;
  border-top: 1px solid var(--border-primary);
  display: flex;
  gap: 0.5rem;
  justify-content: flex-end;
}

/* DNS Setup in modal */
.dom-setup-info {
  margin-top: 1.5rem;
  padding-top: 1.5rem;
  border-top: 1px solid var(--border-primary);
}

.dom-setup-info h3 {
  margin: 0 0 0.75rem;
  font-size: 0.875rem;
  font-weight: 600;
  color: var(--text-primary);
}

.dom-setup-steps {
  display: flex;
  flex-direction: column;
  gap: 0.75rem;
}

.dom-setup-step {
  display: flex;
  gap: 0.75rem;
}

.dom-setup-step-num {
  width: 20px;
  height: 20px;
  display: flex;
  align-items: center;
  justify-content: center;
  background: var(--color-primary);
  color: white;
  border-radius: 50%;
  font-size: 0.6875rem;
  font-weight: 600;
  flex-shrink: 0;
}

.dom-setup-step-content {
  flex: 1;
}

.dom-setup-step-content p {
  margin: 0;
  font-size: 0.8125rem;
  color: var(--text-secondary);
}

.dom-setup-step-content code {
  background: var(--bg-tertiary);
  padding: 0.125rem 0.375rem;
  border-radius: var(--radius-sm);
  font-family: var(--font-mono, monospace);
  font-size: 0.75rem;
  color: var(--text-primary);
}
`;

const Domains: React.FC = () => {
  const [domains, setDomains] = useState<DomainDto[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const [searchTerm, setSearchTerm] = useState('');
  const [showModal, setShowModal] = useState(false);
  const [formName, setFormName] = useState('');
  const [saving, setSaving] = useState(false);
  const [copiedId, setCopiedId] = useState<string | null>(null);
  const [expandedDomains, setExpandedDomains] = useState<Set<string>>(new Set());
  const [verifyingDomains, setVerifyingDomains] = useState<Set<string>>(new Set());
  const [dnsConfig, setDnsConfig] = useState<DnsConfigDto | null>(null);
  const [dnsConfigLoading, setDnsConfigLoading] = useState(false);
  const [copiedRecord, setCopiedRecord] = useState<string | null>(null);

  useEffect(() => {
    fetchDomains();
  }, []);

  const fetchDomains = async () => {
    try {
      setLoading(true);
      setError(null);
      const response = await apiService.domains.list({ page: 1, pageSize: 100 });
      setDomains(response.data);
    } catch (err) {
      console.error('Failed to fetch domains:', err);
      setError('Failed to load domains. Please try again.');
    } finally {
      setLoading(false);
    }
  };

  const fetchDnsConfig = async () => {
    if (dnsConfig || dnsConfigLoading) return;
    setDnsConfigLoading(true);
    try {
      const config = await apiService.domains.getDnsConfig();
      setDnsConfig(config);
    } catch (err) {
      console.error('Failed to fetch DNS config:', err);
      // Use defaults if not available
      setDnsConfig({
        txtRecordName: '_shortas-domain-challenge',
        allowedIpv4: ['203.0.113.10'],
        allowedIpv6: []
      });
    } finally {
      setDnsConfigLoading(false);
    }
  };

  const handleCreate = () => {
    setFormName('');
    setShowModal(true);
  };

  const handleSave = async () => {
    const name = formName.trim().toLowerCase();
    if (!name) return;

    setSaving(true);
    try {
      await apiService.domains.create({ name });
      await fetchDomains();
      setShowModal(false);
      setFormName('');
    } catch (err: any) {
      console.error('Failed to save domain:', err);
      alert(err.response?.data?.message || 'Failed to create domain.');
    } finally {
      setSaving(false);
    }
  };

  const handleDelete = async (domain: DomainDto) => {
    if (!window.confirm(`Delete "${domain.name}"? Routes using this domain will be affected.`)) return;
    try {
      await apiService.domains.delete(domain.id);
      await fetchDomains();
    } catch (err: any) {
      console.error('Failed to delete domain:', err);
      alert(err.response?.data?.message || 'Failed to delete domain.');
    }
  };

  const handleVerify = async (domain: DomainDto) => {
    setVerifyingDomains(prev => new Set(prev).add(domain.id));
    try {
      await apiService.domains.triggerVerification(domain.id);
      await fetchDomains();
    } catch (err: any) {
      console.error('Failed to verify domain:', err);
      alert(err.response?.data?.message || 'Failed to verify domain.');
    } finally {
      setVerifyingDomains(prev => {
        const next = new Set(prev);
        next.delete(domain.id);
        return next;
      });
    }
  };

  const handleCopy = (domain: DomainDto) => {
    navigator.clipboard.writeText(domain.name);
    setCopiedId(domain.id);
    setTimeout(() => setCopiedId(null), 1500);
  };

  const handleCopyRecord = (value: string, key: string) => {
    navigator.clipboard.writeText(value);
    setCopiedRecord(key);
    setTimeout(() => setCopiedRecord(null), 1500);
  };

  const toggleDnsPanel = (domainId: string) => {
    const isExpanding = !expandedDomains.has(domainId);
    setExpandedDomains(prev => {
      const next = new Set(prev);
      if (next.has(domainId)) {
        next.delete(domainId);
      } else {
        next.add(domainId);
      }
      return next;
    });
    if (isExpanding) {
      fetchDnsConfig();
    }
  };

  const getStatusBadge = (status: DomainVerificationStatus) => {
    switch (status) {
      case 'Verified':
        return (
          <span className="dom-status-badge dom-status-verified">
            <CheckCircle size={10} />
            Verified
          </span>
        );
      case 'Failed':
        return (
          <span className="dom-status-badge dom-status-failed">
            <AlertCircle size={10} />
            Failed
          </span>
        );
      case 'Pending':
      default:
        return (
          <span className="dom-status-badge dom-status-pending">
            <Clock size={10} />
            Pending
          </span>
        );
    }
  };

  const formatVerificationReason = (reason: string): string => {
    const reasonMap: Record<string, string> = {
      'not_checked': 'Verification pending',
      'txt_record_valid': 'TXT record verified',
      'txt_record_missing': 'TXT record not found',
      'txt_record_mismatch': 'TXT record value incorrect',
      'a_record_valid': 'A record verified',
      'a_record_invalid': 'A record points to wrong IP',
      'a_record_missing': 'A record not found',
      'aaaa_record_invalid': 'AAAA record points to wrong IP',
      'dns_timeout': 'DNS lookup timed out',
    };
    return reasonMap[reason] || reason;
  };

  const filteredDomains = domains.filter(d =>
    d.name.toLowerCase().includes(searchTerm.toLowerCase())
  );

  if (loading) return <LoadingSpinner />;

  if (error) {
    return (
      <div className="container" style={{ paddingTop: '1.5rem' }}>
        <div className="alert alert-error">
          <h3>Error Loading Domains</h3>
          <p>{error}</p>
          <button className="btn btn-primary" onClick={fetchDomains}>Retry</button>
        </div>
      </div>
    );
  }

  return (
    <>
      <style>{domainStyles}</style>
      <div className="container">
        {/* Toolbar */}
        <div className="dom-toolbar">
          <div className="dom-search">
            <Search size={15} />
            <input
              type="text"
              placeholder="Search domains..."
              value={searchTerm}
              onChange={(e) => setSearchTerm(e.target.value)}
            />
          </div>
          <span className="dom-count">{filteredDomains.length} domain{filteredDomains.length !== 1 ? 's' : ''}</span>
          <button className="btn btn-primary btn-sm" onClick={handleCreate}>
            <Plus size={16} />
            Add Domain
          </button>
        </div>

        {/* Domain list */}
        {filteredDomains.length > 0 ? (
          <div className="dom-list">
            {filteredDomains.map((domain) => (
              <div key={domain.id} className="dom-item">
                <div className="dom-item-main">
                  <div className="dom-icon">
                    <Globe size={18} />
                  </div>
                  <div className="dom-details">
                    <div className="dom-name-row">
                      <span className="dom-name">{domain.name}</span>
                      {getStatusBadge(domain.verificationStatus)}
                    </div>
                    <div className="dom-id">{domain.id}</div>
                    {domain.verificationStatus !== 'Verified' && domain.verificationReason && (
                      <div className="dom-verification-reason">
                        {formatVerificationReason(domain.verificationReason)}
                      </div>
                    )}
                  </div>
                  <div className="dom-actions">
                    <button
                      className={`dom-action-btn ${verifyingDomains.has(domain.id) ? 'spinning' : ''}`}
                      onClick={() => handleVerify(domain)}
                      title="Verify domain"
                      disabled={verifyingDomains.has(domain.id)}
                    >
                      <RefreshCw size={15} />
                    </button>
                    <button
                      className={`dom-action-btn ${copiedId === domain.id ? 'copied' : ''}`}
                      onClick={() => handleCopy(domain)}
                      title="Copy domain name"
                    >
                      {copiedId === domain.id ? <Check size={15} /> : <Copy size={15} />}
                    </button>
                    <button
                      className="dom-action-btn"
                      onClick={() => window.open(`https://${domain.name}`, '_blank')}
                      title="Open in browser"
                    >
                      <ExternalLink size={15} />
                    </button>
                    <button
                      className="dom-action-btn danger"
                      onClick={() => handleDelete(domain)}
                      title="Delete domain"
                    >
                      <Trash2 size={15} />
                    </button>
                  </div>
                </div>

                {/* DNS Instructions Panel */}
                <div className="dom-dns-panel">
                  <button
                    className="dom-dns-toggle"
                    onClick={() => toggleDnsPanel(domain.id)}
                  >
                    <Info size={14} />
                    <span>DNS Configuration Instructions</span>
                    {expandedDomains.has(domain.id) ? <ChevronUp size={14} /> : <ChevronDown size={14} />}
                  </button>

                  {expandedDomains.has(domain.id) && dnsConfigLoading && !dnsConfig && (
                    <div className="dom-dns-content" style={{ textAlign: 'center', padding: '1rem', color: 'var(--text-muted)', fontSize: '0.8125rem' }}>
                      Loading DNS configuration...
                    </div>
                  )}
                  {expandedDomains.has(domain.id) && dnsConfig && (
                    <div className="dom-dns-content">
                      <div className="dom-dns-section">
                        <h4>1. Ownership Verification (TXT Record)</h4>
                        <div className="dom-dns-record">
                          <div className="dom-dns-record-row">
                            <span className="dom-dns-record-label">Name:</span>
                            <span className="dom-dns-record-value">{dnsConfig.txtRecordName}.{domain.name}</span>
                            <button
                              className={`dom-dns-record-copy ${copiedRecord === `txt-name-${domain.id}` ? 'copied' : ''}`}
                              onClick={() => handleCopyRecord(`${dnsConfig.txtRecordName}.${domain.name}`, `txt-name-${domain.id}`)}
                              title="Copy"
                            >
                              {copiedRecord === `txt-name-${domain.id}` ? <Check size={12} /> : <Copy size={12} />}
                            </button>
                          </div>
                          <div className="dom-dns-record-row">
                            <span className="dom-dns-record-label">Type:</span>
                            <span className="dom-dns-record-value">TXT</span>
                          </div>
                          <div className="dom-dns-record-row">
                            <span className="dom-dns-record-label">Value:</span>
                            <span className="dom-dns-record-value">{domain.id}</span>
                            <button
                              className={`dom-dns-record-copy ${copiedRecord === `txt-value-${domain.id}` ? 'copied' : ''}`}
                              onClick={() => handleCopyRecord(domain.id, `txt-value-${domain.id}`)}
                              title="Copy"
                            >
                              {copiedRecord === `txt-value-${domain.id}` ? <Check size={12} /> : <Copy size={12} />}
                            </button>
                          </div>
                        </div>
                        <p className="dom-dns-note">
                          <Info size={12} />
                          This TXT record proves you own the domain.
                        </p>
                      </div>

                      <div className="dom-dns-section">
                        <h4>2. Point Domain to Our Servers (A Record)</h4>
                        <div className="dom-dns-record">
                          <div className="dom-dns-record-row">
                            <span className="dom-dns-record-label">Name:</span>
                            <span className="dom-dns-record-value">{domain.name}</span>
                          </div>
                          <div className="dom-dns-record-row">
                            <span className="dom-dns-record-label">Type:</span>
                            <span className="dom-dns-record-value">A</span>
                          </div>
                          {dnsConfig.allowedIpv4.map((ip, idx) => (
                            <div key={idx} className="dom-dns-record-row">
                              <span className="dom-dns-record-label">Value:</span>
                              <span className="dom-dns-record-value">{ip}</span>
                              <button
                                className={`dom-dns-record-copy ${copiedRecord === `a-${domain.id}-${idx}` ? 'copied' : ''}`}
                                onClick={() => handleCopyRecord(ip, `a-${domain.id}-${idx}`)}
                                title="Copy"
                              >
                                {copiedRecord === `a-${domain.id}-${idx}` ? <Check size={12} /> : <Copy size={12} />}
                              </button>
                            </div>
                          ))}
                        </div>
                        <p className="dom-dns-note">
                          <Info size={12} />
                          This A record routes traffic to our servers.
                        </p>
                      </div>

                      {dnsConfig.allowedIpv6.length > 0 && (
                        <div className="dom-dns-section">
                          <h4>3. IPv6 Support (AAAA Record - Optional)</h4>
                          <div className="dom-dns-record">
                            <div className="dom-dns-record-row">
                              <span className="dom-dns-record-label">Name:</span>
                              <span className="dom-dns-record-value">{domain.name}</span>
                            </div>
                            <div className="dom-dns-record-row">
                              <span className="dom-dns-record-label">Type:</span>
                              <span className="dom-dns-record-value">AAAA</span>
                            </div>
                            {dnsConfig.allowedIpv6.map((ip, idx) => (
                              <div key={idx} className="dom-dns-record-row">
                                <span className="dom-dns-record-label">Value:</span>
                                <span className="dom-dns-record-value">{ip}</span>
                                <button
                                  className={`dom-dns-record-copy ${copiedRecord === `aaaa-${domain.id}-${idx}` ? 'copied' : ''}`}
                                  onClick={() => handleCopyRecord(ip, `aaaa-${domain.id}-${idx}`)}
                                  title="Copy"
                                >
                                  {copiedRecord === `aaaa-${domain.id}-${idx}` ? <Check size={12} /> : <Copy size={12} />}
                                </button>
                              </div>
                            ))}
                          </div>
                        </div>
                      )}
                    </div>
                  )}
                </div>
              </div>
            ))}
          </div>
        ) : (
          <div className="dom-empty">
            <div className="dom-empty-icon">
              <Globe size={24} />
            </div>
            <h3>{searchTerm ? 'No matches' : 'No domains yet'}</h3>
            <p>
              {searchTerm
                ? 'Try a different search term.'
                : 'Add your first custom domain to start creating short links.'}
            </p>
            {!searchTerm && (
              <button className="btn btn-primary btn-sm" onClick={handleCreate}>
                <Plus size={16} />
                Add Domain
              </button>
            )}
          </div>
        )}

        {/* Create modal */}
        {showModal && (
          <div className="dom-modal-overlay" onClick={() => !saving && setShowModal(false)}>
            <div className="dom-modal" onClick={(e) => e.stopPropagation()}>
              <div className="dom-modal-header">
                <h2>Add Domain</h2>
                <button className="dom-modal-close" onClick={() => !saving && setShowModal(false)}>
                  <X size={16} />
                </button>
              </div>
              <div className="dom-modal-body">
                <div className="dom-field">
                  <label>Domain name</label>
                  <input
                    type="text"
                    value={formName}
                    onChange={(e) => setFormName(e.target.value.toLowerCase())}
                    onKeyDown={(e) => e.key === 'Enter' && handleSave()}
                    placeholder="links.example.com"
                    autoFocus
                    disabled={saving}
                  />
                  <span className="dom-hint">Enter the fully qualified domain name. It will be lowercased automatically.</span>
                </div>

                {/* DNS Setup Instructions */}
                <div className="dom-setup-info">
                  <h3>After adding your domain</h3>
                  <div className="dom-setup-steps">
                    <div className="dom-setup-step">
                      <span className="dom-setup-step-num">1</span>
                      <div className="dom-setup-step-content">
                        <p>Add a <code>TXT</code> record with your domain ID to verify ownership.</p>
                      </div>
                    </div>
                    <div className="dom-setup-step">
                      <span className="dom-setup-step-num">2</span>
                      <div className="dom-setup-step-content">
                        <p>Add an <code>A</code> record pointing to our servers.</p>
                      </div>
                    </div>
                    <div className="dom-setup-step">
                      <span className="dom-setup-step-num">3</span>
                      <div className="dom-setup-step-content">
                        <p>Click the refresh button to verify your DNS configuration.</p>
                      </div>
                    </div>
                  </div>
                </div>
              </div>
              <div className="dom-modal-footer">
                <button className="btn btn-outline btn-sm" onClick={() => setShowModal(false)} disabled={saving}>
                  Cancel
                </button>
                <button className="btn btn-primary btn-sm" onClick={handleSave} disabled={saving || !formName.trim()}>
                  {saving ? 'Adding...' : 'Add Domain'}
                </button>
              </div>
            </div>
          </div>
        )}
      </div>
    </>
  );
};

export default Domains;
