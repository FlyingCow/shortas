import React, { useState } from 'react';
import { Plus, Trash2, ChevronDown, ChevronUp } from 'lucide-react';
import {
  RoutingPolicy,
  ConditionalRouting,
  Expression,
  StringCondition,
  NumericCondition,
} from '../services/api';
import './DesignSystem.css';
import './PolicyEditor.css';

interface PolicyEditorProps {
  policy?: RoutingPolicy;
  onChange: (policy: RoutingPolicy) => void;
}

export const PolicyEditor: React.FC<PolicyEditorProps> = ({ policy, onChange }) => {
  const [expanded, setExpanded] = useState(false);
  const [policyType, setPolicyType] = useState<string>(
    !policy || policy === 'Basic' || policy === 'Mirroring'
      ? (policy as string) || 'Basic'
      : typeof policy === 'object' && 'Conditional' in policy
      ? 'Conditional'
      : typeof policy === 'object' && 'Challenge' in policy
      ? 'Challenge'
      : typeof policy === 'object' && 'File' in policy
      ? 'File'
      : 'Basic'
  );

  const handlePolicyTypeChange = (type: string) => {
    setPolicyType(type);
    switch (type) {
      case 'Basic':
        onChange('Basic');
        break;
      case 'Mirroring':
        onChange('Mirroring');
        break;
      case 'Conditional':
        onChange({ Conditional: [] });
        break;
      case 'Challenge':
        onChange({ Challenge: {} });
        break;
      case 'File':
        onChange({ File: {} });
        break;
    }
  };

  const renderConditionalEditor = () => {
    if (typeof policy !== 'object' || !('Conditional' in policy)) return null;

    const conditions = policy.Conditional || [];

    const addCondition = () => {
      onChange({
        Conditional: [
          ...conditions,
          {
            key: '',
            condition: {},
          },
        ],
      });
    };

    const updateCondition = (index: number, updated: ConditionalRouting) => {
      const newConditions = [...conditions];
      newConditions[index] = updated;
      onChange({ Conditional: newConditions });
    };

    const removeCondition = (index: number) => {
      const newConditions = conditions.filter((_, i) => i !== index);
      onChange({ Conditional: newConditions });
    };

    return (
      <div>
        <div className="d-flex justify-content-between align-items-center mb-3">
          <h6 className="mb-0 fw-semibold">Conditions</h6>
          <button type="button" className="btn btn-sm btn-primary" onClick={addCondition}>
            <Plus size={14} className="me-1" />
            Add Condition
          </button>
        </div>

        {conditions.length === 0 && (
          <div className="alert alert-warning">
            <small>No conditions defined. Add a condition to enable conditional routing.</small>
          </div>
        )}

        {conditions.map((cond, index) => (
          <div key={index} className="card mb-3 shadow-sm">
            <div className="card-body">
              <div className="d-flex justify-content-between align-items-center mb-3">
                <h6 className="mb-0">
                  <span className="badge bg-secondary">Condition {index + 1}</span>
                </h6>
                <button
                  type="button"
                  className="btn btn-sm btn-outline-danger"
                  onClick={() => removeCondition(index)}
                >
                  <Trash2 size={14} />
                </button>
              </div>

              <div className="mb-3">
                <label className="form-label">Route Key</label>
                <input
                  type="text"
                  className="form-control"
                  placeholder="e.g., mobile-users"
                  value={cond.key}
                  onChange={(e) =>
                    updateCondition(index, { ...cond, key: e.target.value })
                  }
                />
                <div className="form-text">
                  The key of the route to redirect to when this condition matches
                </div>
              </div>

              <ConditionEditor
                condition={cond.condition}
                onChange={(condition) =>
                  updateCondition(index, { ...cond, condition })
                }
              />
            </div>
          </div>
        ))}
      </div>
    );
  };

  return (
    <div className="policy-editor mb-3">
      <div className="d-flex justify-content-between align-items-center mb-2 policy-header">
        <label className="form-label fw-semibold mb-0">Routing Policy</label>
        <button
          type="button"
          className="btn btn-sm btn-outline-secondary"
          onClick={() => setExpanded(!expanded)}
        >
          {expanded ? <ChevronUp size={14} /> : <ChevronDown size={14} />}
          <span className="ms-1">{expanded ? 'Collapse' : 'Expand'}</span>
        </button>
      </div>

      <select
        className="form-select"
        value={policyType}
        onChange={(e) => handlePolicyTypeChange(e.target.value)}
      >
        <option value="Basic">Basic - Simple redirect</option>
        <option value="Conditional">Conditional - Route based on conditions</option>
        <option value="Challenge">Challenge - Show challenge before redirect</option>
        <option value="File">File - Serve a file</option>
        <option value="Mirroring">Mirroring - Mirror destination</option>
      </select>

      {expanded && (
        <div className="mt-3">
          {policyType === 'Conditional' && renderConditionalEditor()}
          {policyType === 'Basic' && (
            <div className="alert alert-info mb-0">
              <small><strong>Basic policy:</strong> Simple redirect to the destination URL.</small>
            </div>
          )}
          {policyType === 'Mirroring' && (
            <div className="alert alert-info mb-0">
              <small><strong>Mirroring policy:</strong> Mirror the destination website content.</small>
            </div>
          )}
        </div>
      )}
    </div>
  );
};

interface ConditionEditorProps {
  condition: Expression;
  onChange: (condition: Expression) => void;
}

const COUNTRIES = [
  'US', 'UK', 'CA', 'AU', 'DE', 'FR', 'IT', 'ES', 'NL', 'BE', 'CH', 'AT', 'SE', 'NO', 'DK', 'FI',
  'JP', 'CN', 'KR', 'IN', 'BR', 'MX', 'AR', 'CL', 'CO', 'PE', 'RU', 'PL', 'CZ', 'HU', 'GR', 'PT',
  'IE', 'NZ', 'SG', 'MY', 'TH', 'ID', 'PH', 'VN', 'ZA', 'EG', 'NG', 'KE', 'IL', 'SA', 'AE', 'TR'
];

const BROWSERS = ['Chrome', 'Firefox', 'Safari', 'Edge', 'Opera', 'Brave', 'Chromium'];
const OPERATING_SYSTEMS = ['Windows', 'macOS', 'Linux', 'iOS', 'Android', 'ChromeOS'];
const DEVICES = ['Desktop', 'Mobile', 'Tablet', 'Smartphone'];
const LANGUAGES = ['en', 'es', 'fr', 'de', 'it', 'pt', 'ru', 'ja', 'zh', 'ko', 'ar'];
const DAYS_OF_WEEK = ['Sunday', 'Monday', 'Tuesday', 'Wednesday', 'Thursday', 'Friday', 'Saturday'];
const MONTHS = ['January', 'February', 'March', 'April', 'May', 'June', 'July', 'August', 'September', 'October', 'November', 'December'];

const ConditionEditor: React.FC<ConditionEditorProps> = ({ condition, onChange }) => {
  const addConditionType = (type: string) => {
    const newCondition = { ...condition };

    switch (type) {
      case 'ua':
      case 'os':
      case 'device':
      case 'country':
      case 'lang':
        newCondition[type] = { eq: '' };
        break;
      case 'day_of_week':
      case 'day_of_month':
      case 'month':
      case 'rnd':
        newCondition[type] = { eq: 0 };
        break;
      case 'date':
        newCondition[type] = { eq: '' };
        break;
    }

    onChange(newCondition);
  };

  const updateStringCondition = (
    field: keyof Expression,
    operator: keyof StringCondition,
    value: string | string[]
  ) => {
    const newCondition = { ...condition };
    newCondition[field] = { [operator]: value } as any;
    onChange(newCondition);
  };

  const updateNumericCondition = (
    field: keyof Expression,
    operator: keyof NumericCondition,
    value: number | number[]
  ) => {
    const newCondition = { ...condition };
    newCondition[field] = { [operator]: value } as any;
    onChange(newCondition);
  };

  const updateDateCondition = (
    operator: 'eq' | 'gt' | 'lt' | 'in',
    value: string | string[]
  ) => {
    const newCondition = { ...condition };
    newCondition.date = { [operator]: value } as any;
    onChange(newCondition);
  };

  const removeConditionType = (type: keyof Expression) => {
    const newCondition = { ...condition };
    delete newCondition[type];
    onChange(newCondition);
  };

  const getOptionsForField = (field: keyof Expression): string[] => {
    switch (field) {
      case 'ua': return BROWSERS;
      case 'os': return OPERATING_SYSTEMS;
      case 'device': return DEVICES;
      case 'country': return COUNTRIES;
      case 'lang': return LANGUAGES;
      default: return [];
    }
  };

  const renderStringCondition = (field: keyof Expression, label: string) => {
    const cond = condition[field] as StringCondition | undefined;
    if (!cond) return null;

    const options = getOptionsForField(field);
    const hasOptions = options.length > 0;

    return (
      <div className="card condition-card condition-card-primary mb-2">
        <div className="card-body p-3">
          <div className="d-flex justify-content-between align-items-center mb-2">
            <span className="badge condition-badge-primary">{label}</span>
            <button
              type="button"
              className="btn btn-sm btn-outline-danger"
              onClick={() => removeConditionType(field)}
            >
              <Trash2 size={12} />
            </button>
          </div>

          <div className="vstack gap-2">
            {cond.eq !== undefined && (
              <div>
                <label className="form-label small mb-1">Equals</label>
                {hasOptions ? (
                  <select
                    className="form-select form-select-sm"
                    value={cond.eq}
                    onChange={(e) => updateStringCondition(field, 'eq', e.target.value)}
                  >
                    <option value="">Select {label.toLowerCase()}</option>
                    {options.map(opt => (
                      <option key={opt} value={opt}>{opt}</option>
                    ))}
                  </select>
                ) : (
                  <input
                    type="text"
                    className="form-control form-control-sm"
                    value={cond.eq}
                    onChange={(e) => updateStringCondition(field, 'eq', e.target.value)}
                    placeholder={`Enter ${label.toLowerCase()}`}
                  />
                )}
              </div>
            )}

            {cond.in !== undefined && (
              <div>
                <label className="form-label small mb-1">In list</label>
                {hasOptions ? (
                  <>
                    <div className="d-flex flex-wrap gap-1 mb-2">
                      {(cond.in || []).map((val, idx) => (
                        <span key={idx} className="badge value-badge-info d-inline-flex align-items-center">
                          {val}
                          <button
                            type="button"
                            className="btn-close btn-close-white ms-1"
                            style={{ fontSize: '0.6rem', padding: '0.25rem' }}
                            onClick={() => {
                              const newList = cond.in?.filter((_, i) => i !== idx) || [];
                              updateStringCondition(field, 'in', newList);
                            }}
                            aria-label="Remove"
                          />
                        </span>
                      ))}
                    </div>
                    <select
                      className="form-select form-select-sm"
                      value=""
                      onChange={(e) => {
                        if (e.target.value && cond.in && !cond.in.includes(e.target.value)) {
                          updateStringCondition(field, 'in', [...cond.in, e.target.value]);
                        }
                      }}
                    >
                      <option value="">+ Add {label.toLowerCase()}</option>
                      {options.filter(opt => !cond.in?.includes(opt)).map(opt => (
                        <option key={opt} value={opt}>{opt}</option>
                      ))}
                    </select>
                  </>
                ) : (
                  <input
                    type="text"
                    className="form-control form-control-sm"
                    value={(cond.in || []).join(', ')}
                    onChange={(e) =>
                      updateStringCondition(
                        field,
                        'in',
                        e.target.value.split(',').map((s) => s.trim()).filter(Boolean)
                      )
                    }
                    placeholder="e.g., value1, value2, value3"
                  />
                )}
              </div>
            )}

            {cond.starts !== undefined && (
              <div>
                <label className="form-label small mb-1">Starts with</label>
                <input
                  type="text"
                  className="form-control form-control-sm"
                  value={cond.starts}
                  onChange={(e) => updateStringCondition(field, 'starts', e.target.value)}
                  placeholder="Starts with..."
                />
              </div>
            )}

            {cond.ends !== undefined && (
              <div>
                <label className="form-label small mb-1">Ends with</label>
                <input
                  type="text"
                  className="form-control form-control-sm"
                  value={cond.ends}
                  onChange={(e) => updateStringCondition(field, 'ends', e.target.value)}
                  placeholder="Ends with..."
                />
              </div>
            )}

            <div className="operator-radio-group">
              <label className="form-label small mb-2">Operators:</label>
              <div className="d-flex flex-wrap gap-2">
                <button
                  type="button"
                  className={`btn btn-sm ${cond.eq !== undefined ? 'btn-primary' : 'btn-outline-primary'}`}
                  onClick={() => updateStringCondition(field, 'eq', '')}
                  disabled={cond.eq !== undefined}
                >
                  Equals
                </button>
                <button
                  type="button"
                  className={`btn btn-sm ${cond.in !== undefined ? 'btn-primary' : 'btn-outline-primary'}`}
                  onClick={() => updateStringCondition(field, 'in', [])}
                  disabled={cond.in !== undefined}
                >
                  In list
                </button>
                <button
                  type="button"
                  className={`btn btn-sm ${cond.starts !== undefined ? 'btn-primary' : 'btn-outline-primary'}`}
                  onClick={() => updateStringCondition(field, 'starts', '')}
                  disabled={cond.starts !== undefined}
                >
                  Starts with
                </button>
                <button
                  type="button"
                  className={`btn btn-sm ${cond.ends !== undefined ? 'btn-primary' : 'btn-outline-primary'}`}
                  onClick={() => updateStringCondition(field, 'ends', '')}
                  disabled={cond.ends !== undefined}
                >
                  Ends with
                </button>
              </div>
            </div>
          </div>
        </div>
      </div>
    );
  };

  const renderNumericCondition = (field: 'day_of_week' | 'day_of_month' | 'month' | 'rnd', label: string) => {
    const cond = condition[field] as NumericCondition | undefined;
    if (!cond) return null;

    const isMonth = field === 'month';
    const isDayOfWeek = field === 'day_of_week';
    const isRnd = field === 'rnd';

    return (
      <div className="card condition-card condition-card-success mb-2">
        <div className="card-body p-3">
          <div className="d-flex justify-content-between align-items-center mb-2">
            <span className="badge condition-badge-success">{label}</span>
            <button
              type="button"
              className="btn btn-sm btn-outline-danger"
              onClick={() => removeConditionType(field)}
            >
              <Trash2 size={12} />
            </button>
          </div>

          <div className="vstack gap-2">
            {cond.eq !== undefined && (
              <div>
                <label className="form-label small mb-1">Equals</label>
                {isMonth ? (
                  <select
                    className="form-select form-select-sm"
                    value={cond.eq}
                    onChange={(e) => updateNumericCondition(field, 'eq', parseInt(e.target.value))}
                  >
                    <option value="">Select month</option>
                    {MONTHS.map((month, idx) => (
                      <option key={idx} value={idx + 1}>{month}</option>
                    ))}
                  </select>
                ) : isDayOfWeek ? (
                  <select
                    className="form-select form-select-sm"
                    value={cond.eq}
                    onChange={(e) => updateNumericCondition(field, 'eq', parseInt(e.target.value))}
                  >
                    <option value="">Select day</option>
                    {DAYS_OF_WEEK.map((day, idx) => (
                      <option key={idx} value={idx}>{day}</option>
                    ))}
                  </select>
                ) : (
                  <input
                    type="number"
                    className="form-control form-control-sm"
                    value={cond.eq}
                    onChange={(e) => updateNumericCondition(field, 'eq', parseInt(e.target.value))}
                    placeholder={isRnd ? "0-100" : "Enter value"}
                    min={isRnd ? 0 : undefined}
                    max={isRnd ? 100 : field === 'day_of_month' ? 31 : undefined}
                  />
                )}
              </div>
            )}

            {cond.gt !== undefined && (
              <div>
                <label className="form-label small mb-1">Greater than</label>
                <input
                  type="number"
                  className="form-control form-control-sm"
                  value={cond.gt}
                  onChange={(e) => updateNumericCondition(field, 'gt', parseInt(e.target.value))}
                />
              </div>
            )}

            {cond.lt !== undefined && (
              <div>
                <label className="form-label small mb-1">Less than</label>
                <input
                  type="number"
                  className="form-control form-control-sm"
                  value={cond.lt}
                  onChange={(e) => updateNumericCondition(field, 'lt', parseInt(e.target.value))}
                />
              </div>
            )}

            {cond.in !== undefined && (
              <div>
                <label className="form-label small mb-1">In list</label>
                {isMonth ? (
                  <>
                    <div className="d-flex flex-wrap gap-1 mb-2">
                      {(cond.in || []).map((val, idx) => (
                        <span key={idx} className="badge value-badge-success d-inline-flex align-items-center">
                          {MONTHS[val - 1]}
                          <button
                            type="button"
                            className="btn-close btn-close-white ms-1"
                            style={{ fontSize: '0.6rem', padding: '0.25rem' }}
                            onClick={() => {
                              const newList = cond.in?.filter((_, i) => i !== idx) || [];
                              updateNumericCondition(field, 'in', newList);
                            }}
                            aria-label="Remove"
                          />
                        </span>
                      ))}
                    </div>
                    <select
                      className="form-select form-select-sm"
                      value=""
                      onChange={(e) => {
                        const val = parseInt(e.target.value);
                        if (val && cond.in && !cond.in.includes(val)) {
                          updateNumericCondition(field, 'in', [...cond.in, val]);
                        }
                      }}
                    >
                      <option value="">+ Add month</option>
                      {MONTHS.map((month, idx) => (
                        <option key={idx} value={idx + 1}>{month}</option>
                      ))}
                    </select>
                  </>
                ) : isDayOfWeek ? (
                  <>
                    <div className="d-flex flex-wrap gap-1 mb-2">
                      {(cond.in || []).map((val, idx) => (
                        <span key={idx} className="badge value-badge-success d-inline-flex align-items-center">
                          {DAYS_OF_WEEK[val]}
                          <button
                            type="button"
                            className="btn-close btn-close-white ms-1"
                            style={{ fontSize: '0.6rem', padding: '0.25rem' }}
                            onClick={() => {
                              const newList = cond.in?.filter((_, i) => i !== idx) || [];
                              updateNumericCondition(field, 'in', newList);
                            }}
                            aria-label="Remove"
                          />
                        </span>
                      ))}
                    </div>
                    <select
                      className="form-select form-select-sm"
                      value=""
                      onChange={(e) => {
                        const val = parseInt(e.target.value);
                        if (!isNaN(val) && cond.in && !cond.in.includes(val)) {
                          updateNumericCondition(field, 'in', [...cond.in, val]);
                        }
                      }}
                    >
                      <option value="">+ Add day</option>
                      {DAYS_OF_WEEK.map((day, idx) => (
                        <option key={idx} value={idx}>{day}</option>
                      ))}
                    </select>
                  </>
                ) : (
                  <input
                    type="text"
                    className="form-control form-control-sm"
                    value={(cond.in || []).join(', ')}
                    onChange={(e) =>
                      updateNumericCondition(
                        field,
                        'in',
                        e.target.value.split(',').map((s) => parseInt(s.trim())).filter(n => !isNaN(n))
                      )
                    }
                    placeholder="e.g., 1, 15, 30"
                  />
                )}
              </div>
            )}

            <div className="operator-radio-group">
              <label className="form-label small mb-2">Operators:</label>
              <div className="d-flex flex-wrap gap-2">
                <button
                  type="button"
                  className={`btn btn-sm ${cond.eq !== undefined ? 'btn-primary' : 'btn-outline-primary'}`}
                  onClick={() => updateNumericCondition(field, 'eq', 0)}
                  disabled={cond.eq !== undefined}
                >
                  Equals
                </button>
                <button
                  type="button"
                  className={`btn btn-sm ${cond.gt !== undefined ? 'btn-primary' : 'btn-outline-primary'}`}
                  onClick={() => updateNumericCondition(field, 'gt', 0)}
                  disabled={cond.gt !== undefined}
                >
                  Greater than
                </button>
                <button
                  type="button"
                  className={`btn btn-sm ${cond.lt !== undefined ? 'btn-primary' : 'btn-outline-primary'}`}
                  onClick={() => updateNumericCondition(field, 'lt', 0)}
                  disabled={cond.lt !== undefined}
                >
                  Less than
                </button>
                <button
                  type="button"
                  className={`btn btn-sm ${cond.in !== undefined ? 'btn-primary' : 'btn-outline-primary'}`}
                  onClick={() => updateNumericCondition(field, 'in', [])}
                  disabled={cond.in !== undefined}
                >
                  In list
                </button>
              </div>
            </div>
          </div>
        </div>
      </div>
    );
  };

  const renderDateCondition = () => {
    const cond = condition.date;
    if (!cond) return null;

    return (
      <div className="card condition-card condition-card-warning mb-2">
        <div className="card-body p-3">
          <div className="d-flex justify-content-between align-items-center mb-2">
            <span className="badge condition-badge-warning">Date</span>
            <button
              type="button"
              className="btn btn-sm btn-outline-danger"
              onClick={() => removeConditionType('date')}
            >
              <Trash2 size={12} />
            </button>
          </div>

          <div className="vstack gap-2">
            {cond.eq !== undefined && (
              <div>
                <label className="form-label small mb-1">Equals (YYYY-MM-DD)</label>
                <input
                  type="date"
                  className="form-control form-control-sm"
                  value={cond.eq}
                  onChange={(e) => updateDateCondition('eq', e.target.value)}
                />
              </div>
            )}

            {cond.gt !== undefined && (
              <div>
                <label className="form-label small mb-1">After date (YYYY-MM-DD)</label>
                <input
                  type="date"
                  className="form-control form-control-sm"
                  value={cond.gt}
                  onChange={(e) => updateDateCondition('gt', e.target.value)}
                />
              </div>
            )}

            {cond.lt !== undefined && (
              <div>
                <label className="form-label small mb-1">Before date (YYYY-MM-DD)</label>
                <input
                  type="date"
                  className="form-control form-control-sm"
                  value={cond.lt}
                  onChange={(e) => updateDateCondition('lt', e.target.value)}
                />
              </div>
            )}

            {cond.in !== undefined && (
              <div>
                <label className="form-label small mb-1">In dates</label>
                <div className="d-flex flex-wrap gap-1 mb-2">
                  {(cond.in || []).map((date, idx) => (
                    <span key={idx} className="badge value-badge-warning d-inline-flex align-items-center">
                      {date}
                      <button
                        type="button"
                        className="btn-close ms-1"
                        style={{ fontSize: '0.6rem', padding: '0.25rem' }}
                        onClick={() => {
                          const newList = cond.in?.filter((_, i) => i !== idx) || [];
                          updateDateCondition('in', newList);
                        }}
                        aria-label="Remove"
                      />
                    </span>
                  ))}
                </div>
                <input
                  type="date"
                  className="form-control form-control-sm"
                  onChange={(e) => {
                    if (e.target.value && cond.in && !cond.in.includes(e.target.value)) {
                      updateDateCondition('in', [...cond.in, e.target.value]);
                    }
                  }}
                />
              </div>
            )}

            <div className="operator-radio-group">
              <label className="form-label small mb-2">Operators:</label>
              <div className="d-flex flex-wrap gap-2">
                <button
                  type="button"
                  className={`btn btn-sm ${cond.eq !== undefined ? 'btn-primary' : 'btn-outline-primary'}`}
                  onClick={() => updateDateCondition('eq', '')}
                  disabled={cond.eq !== undefined}
                >
                  Equals
                </button>
                <button
                  type="button"
                  className={`btn btn-sm ${cond.gt !== undefined ? 'btn-primary' : 'btn-outline-primary'}`}
                  onClick={() => updateDateCondition('gt', '')}
                  disabled={cond.gt !== undefined}
                >
                  After date
                </button>
                <button
                  type="button"
                  className={`btn btn-sm ${cond.lt !== undefined ? 'btn-primary' : 'btn-outline-primary'}`}
                  onClick={() => updateDateCondition('lt', '')}
                  disabled={cond.lt !== undefined}
                >
                  Before date
                </button>
                <button
                  type="button"
                  className={`btn btn-sm ${cond.in !== undefined ? 'btn-primary' : 'btn-outline-primary'}`}
                  onClick={() => updateDateCondition('in', [])}
                  disabled={cond.in !== undefined}
                >
                  In dates
                </button>
              </div>
            </div>
          </div>
        </div>
      </div>
    );
  };

  return (
    <div className="mb-3">
      <label className="form-label fw-semibold">Condition Rules</label>

      {condition.ua && renderStringCondition('ua', 'User Agent (Browser)')}
      {condition.os && renderStringCondition('os', 'Operating System')}
      {condition.device && renderStringCondition('device', 'Device Type')}
      {condition.country && renderStringCondition('country', 'Country')}
      {condition.lang && renderStringCondition('lang', 'Language')}
      {condition.day_of_week && renderNumericCondition('day_of_week', 'Day of Week')}
      {condition.day_of_month && renderNumericCondition('day_of_month', 'Day of Month')}
      {condition.month && renderNumericCondition('month', 'Month')}
      {condition.rnd && renderNumericCondition('rnd', 'Random (A/B Testing)')}
      {condition.date && renderDateCondition()}

      <select
        className="form-select form-select-sm mt-2"
        value=""
        onChange={(e) => {
          if (e.target.value) {
            addConditionType(e.target.value);
          }
        }}
      >
        <option value="">+ Add condition type</option>
        {!condition.ua && <option value="ua">User Agent (Browser)</option>}
        {!condition.os && <option value="os">Operating System</option>}
        {!condition.device && <option value="device">Device Type</option>}
        {!condition.country && <option value="country">Country</option>}
        {!condition.lang && <option value="lang">Language</option>}
        {!condition.day_of_week && <option value="day_of_week">Day of Week</option>}
        {!condition.day_of_month && <option value="day_of_month">Day of Month</option>}
        {!condition.month && <option value="month">Month</option>}
        {!condition.date && <option value="date">Date</option>}
        {!condition.rnd && <option value="rnd">Random (A/B Testing)</option>}
      </select>

      {Object.keys(condition).length === 0 && (
        <div className="alert alert-warning mt-2">
          <small>No conditions defined. Add a condition type above.</small>
        </div>
      )}
    </div>
  );
};

export default PolicyEditor;
