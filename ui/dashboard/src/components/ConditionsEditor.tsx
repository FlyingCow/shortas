import React, { useState, forwardRef, useImperativeHandle, useMemo } from 'react';
import { Plus, Trash2, ChevronDown, ChevronUp, GripVertical } from 'lucide-react';
import {
  Expression,
  StringCondition,
  NumericCondition,
  DateCondition,
  ConditionRouteDto,
} from '../services/api';
import Dropdown from './Dropdown';
import './DesignSystem.css';
import './PolicyEditor.css';
import './ConditionsEditor.css';

export interface ConditionsEditorHandle {
  /** Touches every invalid field so its error becomes visible, expands any
   *  collapsed block that has one, and reports whether everything is valid.
   *  Call this from the parent form's own validate() before submitting. */
  validate: () => boolean;
}

interface ConditionsEditorProps {
  conditions: ConditionRouteDto[];
  onChange: (conditions: ConditionRouteDto[]) => void;
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

const CONDITION_TYPE_LABELS: Record<string, string> = {
  ua: 'User Agent',
  os: 'Operating System',
  device: 'Device',
  country: 'Country',
  lang: 'Language',
  day_of_week: 'Day of Week',
  day_of_month: 'Day of Month',
  month: 'Month',
  date: 'Date',
  rnd: 'Random %',
};

// Inclusive numeric ranges for fields a user can type a raw number into
// (eq is dropdown-constrained for month/day_of_week, but gt/lt/in are
// still free-typed, so the range has to be enforced here regardless of
// which operator is active).
const NUMERIC_RANGES: Record<string, [number, number]> = {
  rnd: [0, 100],
  day_of_week: [0, 6],
  day_of_month: [1, 31],
  month: [1, 12],
};

/** Error key format: "<conditionIndex>.dest" or "<conditionIndex>.<field>". */
const computeConditionErrors = (conditions: ConditionRouteDto[]): Record<string, string> => {
  const errors: Record<string, string> = {};

  conditions.forEach((cond, index) => {
    if (!cond.dest?.trim()) {
      errors[`${index}.dest`] = 'Destination URL is required';
    }

    const condition = cond.condition || {};
    (Object.keys(condition) as (keyof Expression)[]).forEach((field) => {
      if (field === 'default_operator') return;
      const key = `${index}.${field}`;

      if (field === 'date') {
        const dc = condition.date as DateCondition;
        if (dc.eq !== undefined && !dc.eq) errors[key] = 'Date is required';
        else if (dc.gt !== undefined && !dc.gt) errors[key] = 'Date is required';
        else if (dc.lt !== undefined && !dc.lt) errors[key] = 'Date is required';
        else if (dc.in !== undefined && dc.in.length === 0) errors[key] = 'Add at least one date';
        return;
      }

      const range = NUMERIC_RANGES[field as string];
      if (range) {
        const [min, max] = range;
        const outOfRange = (v: number | undefined) => v !== undefined && (Number.isNaN(v) || v < min || v > max);
        const nc = condition[field] as NumericCondition;
        if (outOfRange(nc.eq) || outOfRange(nc.gt) || outOfRange(nc.lt)) {
          errors[key] = `Must be between ${min} and ${max}`;
        } else if (nc.in !== undefined) {
          if (nc.in.length === 0) errors[key] = 'Add at least one value';
          else if (nc.in.some(outOfRange)) errors[key] = `Values must be between ${min} and ${max}`;
        }
        return;
      }

      // Remaining fields (ua, os, device, country, lang) are StringConditions.
      const sc = condition[field] as StringCondition;
      if (sc.eq !== undefined && !sc.eq.trim()) errors[key] = 'Value is required';
      else if (sc.starts !== undefined && !sc.starts.trim()) errors[key] = 'Value is required';
      else if (sc.ends !== undefined && !sc.ends.trim()) errors[key] = 'Value is required';
      else if (sc.in !== undefined && sc.in.length === 0) errors[key] = 'Add at least one value';
    });
  });

  return errors;
};

export const ConditionsEditor = forwardRef<ConditionsEditorHandle, ConditionsEditorProps>(
  ({ conditions, onChange }, ref) => {
    const [expandedConditions, setExpandedConditions] = useState<Set<number>>(new Set());
    const [dragOverIndex, setDragOverIndex] = useState<number | null>(null);
    const [touched, setTouched] = useState<Set<string>>(new Set());

    const errors = useMemo(() => computeConditionErrors(conditions), [conditions]);

    const touch = (key: string) => {
      setTouched((prev) => (prev.has(key) ? prev : new Set(prev).add(key)));
    };

    useImperativeHandle(ref, () => ({
      validate: () => {
        const errorKeys = Object.keys(errors);
        if (errorKeys.length > 0) {
          setTouched((prev) => new Set([...Array.from(prev), ...errorKeys]));
          const blocksWithErrors = new Set(errorKeys.map((k) => Number(k.split('.')[0])));
          setExpandedConditions((prev) => new Set([...Array.from(prev), ...Array.from(blocksWithErrors)]));
        }
        return errorKeys.length === 0;
      },
    }), [errors]);

    const toggleExpanded = (index: number) => {
      const next = new Set(expandedConditions);
      if (next.has(index)) next.delete(index);
      else next.add(index);
      setExpandedConditions(next);
    };

    const addCondition = () => {
      onChange([...conditions, { dest: '', condition: {} }]);
      setExpandedConditions(new Set(Array.from(expandedConditions).concat(conditions.length)));
    };

    const updateCondition = (index: number, updated: ConditionRouteDto) => {
      const next = [...conditions];
      next[index] = updated;
      onChange(next);
    };

    const removeCondition = (index: number) => {
      onChange(conditions.filter((_, i) => i !== index));
      setExpandedConditions(new Set(Array.from(expandedConditions).filter(i => i !== index).map(i => i > index ? i - 1 : i)));
    };

    const moveCondition = (fromIndex: number, toIndex: number) => {
      if (fromIndex === toIndex) return;
      const next = conditions.slice();
      const [item] = next.splice(fromIndex, 1);
      next.splice(toIndex, 0, item);
      onChange(next);
      const oldToNew = (o: number): number => {
        if (o === fromIndex) return toIndex;
        if (fromIndex < toIndex) {
          if (o < fromIndex) return o;
          if (o > fromIndex && o <= toIndex) return o - 1;
          return o;
        } else {
          if (o < toIndex) return o;
          if (o >= toIndex && o < fromIndex) return o + 1;
          return o;
        }
      };
      setExpandedConditions(new Set(Array.from(expandedConditions).map(oldToNew)));
    };

    const handleDragStart = (e: React.DragEvent, index: number) => {
      e.dataTransfer.effectAllowed = 'move';
      e.dataTransfer.setData('text/plain', String(index));
      e.dataTransfer.setData('application/json', JSON.stringify({ index }));
    };

    const handleDragOver = (e: React.DragEvent, index: number) => {
      e.preventDefault();
      e.dataTransfer.dropEffect = 'move';
      setDragOverIndex(index);
    };

    const handleDragLeave = (e: React.DragEvent) => {
      const related = e.relatedTarget as Node | null;
      if (!related || !e.currentTarget.contains(related)) {
        setDragOverIndex(null);
      }
    };

    const handleDrop = (e: React.DragEvent, dropIndex: number) => {
      e.preventDefault();
      setDragOverIndex(null);
      const raw = e.dataTransfer.getData('text/plain');
      const dragIndex = raw !== '' ? parseInt(raw, 10) : null;
      if (dragIndex == null || isNaN(dragIndex) || dragIndex === dropIndex) return;
      moveCondition(dragIndex, dropIndex);
    };

    const handleDragEnd = () => {
      setDragOverIndex(null);
    };

    return (
      <div className="ce-root conditions-editor">
        <div className="ce-header">
          <button type="button" className="ce-add-btn" onClick={addCondition}>
            <Plus size={16} />
            Add condition
          </button>
        </div>

        {conditions.length === 0 && (
          <div className="ce-empty">
            No conditional routes. The main destination will always be used. Add a condition to redirect based on device, country, time, etc.
          </div>
        )}

        {conditions.map((cond, index) => {
          const destKey = `${index}.dest`;
          const destError = touched.has(destKey) ? errors[destKey] : undefined;
          const blockHasErrors = Object.keys(errors).some((k) => k.startsWith(`${index}.`) && touched.has(k));

          return (
            <div
              key={index}
              className={`ce-block ${dragOverIndex === index ? 'ce-block--drag-over' : ''} ${blockHasErrors ? 'ce-block--error' : ''}`}
              onDragOver={(e) => handleDragOver(e, index)}
              onDragLeave={handleDragLeave}
              onDrop={(e) => handleDrop(e, index)}
              onDragEnd={handleDragEnd}
            >
              <div className="ce-block-head">
                <div className="ce-block-head-left">
                  <span className="ce-block-num">{index + 1}</span>
                  <div className="ce-block-dest-wrap">
                    <input
                      type="text"
                      className={`ce-block-dest ${destError ? 'ce-input-error' : ''}`}
                      placeholder="Destination URL"
                      value={cond.dest}
                      onChange={e => updateCondition(index, { ...cond, dest: e.target.value })}
                      onBlur={() => touch(destKey)}
                    />
                    {destError && <span className="ce-error-text">{destError}</span>}
                  </div>
                </div>
                <div
                  className="ce-block-drag-handle"
                  draggable
                  onDragStart={(e) => handleDragStart(e, index)}
                  title="Drag to reorder"
                  aria-label="Drag to reorder"
                >
                  <GripVertical size={18} />
                </div>
                <div className="ce-block-actions">
                  <button
                    type="button"
                    className="ce-block-btn"
                    onClick={() => toggleExpanded(index)}
                    aria-label={expandedConditions.has(index) ? 'Collapse' : 'Expand'}
                  >
                    {expandedConditions.has(index) ? <ChevronUp size={18} /> : <ChevronDown size={18} />}
                  </button>
                  <button
                    type="button"
                    className="ce-block-btn ce-block-btn--danger"
                    onClick={() => removeCondition(index)}
                    aria-label="Remove condition"
                  >
                    <Trash2 size={16} />
                  </button>
                </div>
              </div>

              {expandedConditions.has(index) && (
                <div className="ce-block-body">
                  <div className="ce-when-label">When (all must match)</div>
                  <ConditionRulesEditor
                    condition={cond.condition}
                    onChange={c => updateCondition(index, { ...cond, condition: c })}
                    blockIndex={index}
                    errors={errors}
                    touched={touched}
                    onTouch={touch}
                  />
                </div>
              )}

              {!expandedConditions.has(index) && Object.keys(cond.condition).length > 0 && (
                <div className="ce-block-body">
                  <p className="ce-collapsed-hint">
                    {Object.keys(cond.condition).length} rule(s) · Expand to edit
                  </p>
                </div>
              )}
            </div>
          );
        })}
      </div>
    );
  }
);

ConditionsEditor.displayName = 'ConditionsEditor';

interface ConditionRulesEditorProps {
  condition: Expression;
  onChange: (condition: Expression) => void;
  blockIndex: number;
  errors: Record<string, string>;
  touched: Set<string>;
  onTouch: (key: string) => void;
}

const ConditionRulesEditor: React.FC<ConditionRulesEditorProps> = ({ condition, onChange, blockIndex, errors, touched, onTouch }) => {
  const addConditionType = (type: string) => {
    const next = { ...condition };
    switch (type) {
      case 'ua':
      case 'os':
      case 'device':
      case 'country':
      case 'lang':
        next[type] = { eq: '' };
        break;
      case 'day_of_week':
      case 'day_of_month':
      case 'month':
      case 'rnd':
        next[type] = { eq: 0 };
        break;
      case 'date':
        next.date = { eq: '' };
        break;
    }
    onChange(next);
  };

  const updateStringCondition = (field: keyof Expression, operator: keyof StringCondition, value: string | string[]) => {
    onChange({ ...condition, [field]: { [operator]: value } } as Expression);
  };

  const updateNumericCondition = (field: keyof Expression, operator: keyof NumericCondition, value: number | number[]) => {
    onChange({ ...condition, [field]: { [operator]: value } } as Expression);
  };

  const updateDateCondition = (op: 'eq' | 'gt' | 'lt' | 'in', value: string | string[]) => {
    onChange({ ...condition, date: { [op]: value } } as Expression);
  };

  const removeConditionType = (type: keyof Expression) => {
    const next = { ...condition };
    delete next[type];
    onChange(next);
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

  const stringOperators = [
    { key: 'eq', label: 'equals' },
    { key: 'in', label: 'is one of' },
    { key: 'starts', label: 'starts with' },
    { key: 'ends', label: 'ends with' },
  ] as const;

  const numericOperators = [
    { key: 'eq', label: 'equals' },
    { key: 'gt', label: 'greater than' },
    { key: 'lt', label: 'less than' },
    { key: 'in', label: 'is one of' },
  ] as const;

  const dateOperators = [
    { key: 'eq', label: 'equals' },
    { key: 'gt', label: 'after' },
    { key: 'lt', label: 'before' },
    { key: 'in', label: 'in dates' },
  ] as const;

  const renderStringRule = (field: keyof Expression, label: string) => {
    const cond = condition[field] as StringCondition | undefined;
    if (!cond) return null;
    const options = getOptionsForField(field);
    const hasOptions = options.length > 0;
    const op = cond.eq !== undefined ? 'eq' : cond.in !== undefined ? 'in' : cond.starts !== undefined ? 'starts' : 'ends';
    const errorKey = `${blockIndex}.${field}`;
    const error = touched.has(errorKey) ? errors[errorKey] : undefined;

    return (
      <div key={String(field)} className="ce-rule-row-wrap">
        <div className={`ce-rule-row ${error ? 'ce-rule-row--error' : ''}`}>
          <span className="ce-rule-type">{label}</span>
          <Dropdown
            compact
            style={{ flexShrink: 0, minWidth: 100 }}
            value={op}
            onChange={v => {
              const k = v as keyof StringCondition;
              if (k === 'eq') updateStringCondition(field, 'eq', '');
              if (k === 'in') updateStringCondition(field, 'in', []);
              if (k === 'starts') updateStringCondition(field, 'starts', '');
              if (k === 'ends') updateStringCondition(field, 'ends', '');
              onTouch(errorKey);
            }}
            options={stringOperators.map(o => ({ value: o.key, label: o.label }))}
          />
          {op === 'eq' && (
            hasOptions ? (
              <Dropdown
                compact
                style={{ flexShrink: 0, minWidth: 120 }}
                value={cond.eq ?? ''}
                onChange={v => { updateStringCondition(field, 'eq', v); onTouch(errorKey); }}
                placeholder="Select…"
                options={options.map(opt => ({ value: opt, label: opt }))}
              />
            ) : (
              <input
                type="text"
                className={`ce-rule-value ${error ? 'ce-input-error' : ''}`}
                value={cond.eq}
                onChange={e => updateStringCondition(field, 'eq', e.target.value)}
                onBlur={() => onTouch(errorKey)}
                placeholder={`${label} value`}
              />
            )
          )}
          {op === 'in' && (
            <div className="ce-tags">
              {(cond.in || []).map((val, idx) => (
                <span key={idx} className="ce-tag">
                  {val}
                  <button type="button" className="ce-tag-remove" onClick={() => { updateStringCondition(field, 'in', (cond.in || []).filter((_, i) => i !== idx)); onTouch(errorKey); }} aria-label="Remove">×</button>
                </span>
              ))}
              {hasOptions ? (
                <Dropdown
                  compact
                  style={{ maxWidth: 140 }}
                  value=""
                  onChange={v => { if (v) updateStringCondition(field, 'in', [...(cond.in || []), v]); onTouch(errorKey); }}
                  placeholder="+ Add"
                  options={options.filter(o => !cond.in?.includes(o)).map(o => ({ value: o, label: o }))}
                />
              ) : (
                <input
                  type="text"
                  className={`ce-rule-value ${error ? 'ce-input-error' : ''}`}
                  style={{ minWidth: 100 }}
                  placeholder="a, b, c"
                  value={(cond.in || []).join(', ')}
                  onChange={e => updateStringCondition(field, 'in', e.target.value.split(',').map(s => s.trim()).filter(Boolean))}
                  onBlur={() => onTouch(errorKey)}
                />
              )}
            </div>
          )}
          {(op === 'starts' || op === 'ends') && (
            <input
              type="text"
              className={`ce-rule-value ${error ? 'ce-input-error' : ''}`}
              value={op === 'starts' ? cond.starts : cond.ends}
              onChange={e => op === 'starts' ? updateStringCondition(field, 'starts', e.target.value) : updateStringCondition(field, 'ends', e.target.value)}
              onBlur={() => onTouch(errorKey)}
              placeholder={op === 'starts' ? 'Prefix…' : 'Suffix…'}
            />
          )}
          <button type="button" className="ce-rule-remove" onClick={() => removeConditionType(field)} aria-label="Remove rule">
            <Trash2 size={14} />
          </button>
        </div>
        {error && <span className="ce-error-text">{error}</span>}
      </div>
    );
  };

  const renderNumericRule = (field: 'day_of_week' | 'day_of_month' | 'month' | 'rnd', label: string) => {
    const cond = condition[field] as NumericCondition | undefined;
    if (!cond) return null;
    const op = cond.eq !== undefined ? 'eq' : cond.gt !== undefined ? 'gt' : cond.lt !== undefined ? 'lt' : 'in';
    const isMonth = field === 'month';
    const isDayOfWeek = field === 'day_of_week';
    const isRnd = field === 'rnd';
    const errorKey = `${blockIndex}.${field}`;
    const error = touched.has(errorKey) ? errors[errorKey] : undefined;

    return (
      <div key={field} className="ce-rule-row-wrap">
        <div className={`ce-rule-row ${error ? 'ce-rule-row--error' : ''}`}>
          <span className="ce-rule-type">{label}</span>
          <Dropdown
            compact
            style={{ flexShrink: 0, minWidth: 100 }}
            value={op}
            onChange={v => {
              const k = v as keyof NumericCondition;
              if (k === 'eq') updateNumericCondition(field, 'eq', isRnd ? 0 : isMonth ? 1 : isDayOfWeek ? 0 : 1);
              if (k === 'gt') updateNumericCondition(field, 'gt', 0);
              if (k === 'lt') updateNumericCondition(field, 'lt', 0);
              if (k === 'in') updateNumericCondition(field, 'in', []);
              onTouch(errorKey);
            }}
            options={numericOperators.map(o => ({ value: o.key, label: o.label }))}
          />
          {op === 'eq' && (
            isMonth ? (
              <Dropdown
                compact
                style={{ flexShrink: 0, minWidth: 120 }}
                value={String(cond.eq)}
                onChange={v => { updateNumericCondition(field, 'eq', parseInt(v)); onTouch(errorKey); }}
                placeholder="Month…"
                options={MONTHS.map((m, i) => ({ value: String(i + 1), label: m }))}
              />
            ) : isDayOfWeek ? (
              <Dropdown
                compact
                style={{ flexShrink: 0, minWidth: 120 }}
                value={String(cond.eq)}
                onChange={v => { updateNumericCondition(field, 'eq', parseInt(v)); onTouch(errorKey); }}
                placeholder="Day…"
                options={DAYS_OF_WEEK.map((d, i) => ({ value: String(i), label: d }))}
              />
            ) : (
              <input
                type="number"
                className={`ce-rule-value ${error ? 'ce-input-error' : ''}`}
                value={cond.eq}
                onChange={e => updateNumericCondition(field, 'eq', parseInt(e.target.value) || 0)}
                onBlur={() => onTouch(errorKey)}
                min={isRnd ? 0 : undefined}
                max={isRnd ? 100 : field === 'day_of_month' ? 31 : undefined}
                placeholder={isRnd ? '0–100' : 'Value'}
              />
            )
          )}
          {(op === 'gt' || op === 'lt') && (
            <input
              type="number"
              className={`ce-rule-value ${error ? 'ce-input-error' : ''}`}
              value={op === 'gt' ? cond.gt : cond.lt}
              onChange={e => updateNumericCondition(field, op, parseInt(e.target.value) || 0)}
              onBlur={() => onTouch(errorKey)}
              min={isRnd ? 0 : undefined}
              max={isRnd ? 100 : undefined}
            />
          )}
          {op === 'in' && (
            <input
              type="text"
              className={`ce-rule-value ${error ? 'ce-input-error' : ''}`}
              value={(cond.in || []).join(', ')}
              onChange={e => updateNumericCondition(field, 'in', e.target.value.split(',').map(s => parseInt(s.trim())).filter(n => !isNaN(n)))}
              onBlur={() => onTouch(errorKey)}
              placeholder="e.g. 1, 2, 3"
            />
          )}
          <button type="button" className="ce-rule-remove" onClick={() => removeConditionType(field)} aria-label="Remove rule">
            <Trash2 size={14} />
          </button>
        </div>
        {error && <span className="ce-error-text">{error}</span>}
      </div>
    );
  };

  const renderDateRule = () => {
    const cond = condition.date;
    if (!cond) return null;
    const op = cond.eq !== undefined ? 'eq' : cond.gt !== undefined ? 'gt' : cond.lt !== undefined ? 'lt' : 'in';
    const errorKey = `${blockIndex}.date`;
    const error = touched.has(errorKey) ? errors[errorKey] : undefined;

    return (
      <div key="date" className="ce-rule-row-wrap">
        <div className={`ce-rule-row ${error ? 'ce-rule-row--error' : ''}`}>
          <span className="ce-rule-type">Date</span>
          <Dropdown
            compact
            style={{ flexShrink: 0, minWidth: 100 }}
            value={op}
            onChange={v => {
              const k = v as 'eq' | 'gt' | 'lt' | 'in';
              if (k === 'eq') updateDateCondition('eq', '');
              if (k === 'gt') updateDateCondition('gt', '');
              if (k === 'lt') updateDateCondition('lt', '');
              if (k === 'in') updateDateCondition('in', []);
              onTouch(errorKey);
            }}
            options={dateOperators.map(o => ({ value: o.key, label: o.label }))}
          />
          {(op === 'eq' || op === 'gt' || op === 'lt') && (
            <input
              type="date"
              className={`ce-rule-value ${error ? 'ce-input-error' : ''}`}
              value={op === 'eq' ? cond.eq : op === 'gt' ? cond.gt : cond.lt}
              onChange={e => updateDateCondition(op, e.target.value)}
              onBlur={() => onTouch(errorKey)}
            />
          )}
          {op === 'in' && (
            <div className="ce-tags">
              {(cond.in || []).map((d, idx) => (
                <span key={idx} className="ce-tag">
                  {d}
                  <button type="button" className="ce-tag-remove" onClick={() => { updateDateCondition('in', (cond.in || []).filter((_, i) => i !== idx)); onTouch(errorKey); }} aria-label="Remove">×</button>
                </span>
              ))}
              <input
                type="date"
                className="ce-rule-value"
                style={{ maxWidth: 140 }}
                onChange={e => { if (e.target.value) updateDateCondition('in', [...(cond.in || []), e.target.value]); onTouch(errorKey); }}
                onBlur={() => onTouch(errorKey)}
              />
            </div>
          )}
          <button type="button" className="ce-rule-remove" onClick={() => removeConditionType('date')} aria-label="Remove rule">
            <Trash2 size={14} />
          </button>
        </div>
        {error && <span className="ce-error-text">{error}</span>}
      </div>
    );
  };

  const ruleTypes: { key: string; label: string }[] = [
    { key: 'ua', label: 'User Agent (Browser)' },
    { key: 'os', label: 'Operating System' },
    { key: 'device', label: 'Device Type' },
    { key: 'country', label: 'Country' },
    { key: 'lang', label: 'Language' },
    { key: 'day_of_week', label: 'Day of Week' },
    { key: 'day_of_month', label: 'Day of Month' },
    { key: 'month', label: 'Month' },
    { key: 'date', label: 'Date' },
    { key: 'rnd', label: 'Random (A/B)' },
  ];

  return (
    <div className="ce-rules-wrap">
      <div className="ce-rules">
        {condition.ua && renderStringRule('ua', CONDITION_TYPE_LABELS.ua)}
        {condition.os && renderStringRule('os', CONDITION_TYPE_LABELS.os)}
        {condition.device && renderStringRule('device', CONDITION_TYPE_LABELS.device)}
        {condition.country && renderStringRule('country', CONDITION_TYPE_LABELS.country)}
        {condition.lang && renderStringRule('lang', CONDITION_TYPE_LABELS.lang)}
        {condition.day_of_week && renderNumericRule('day_of_week', CONDITION_TYPE_LABELS.day_of_week)}
        {condition.day_of_month && renderNumericRule('day_of_month', CONDITION_TYPE_LABELS.day_of_month)}
        {condition.month && renderNumericRule('month', CONDITION_TYPE_LABELS.month)}
        {condition.rnd && renderNumericRule('rnd', CONDITION_TYPE_LABELS.rnd)}
        {condition.date && renderDateRule()}
      </div>
      {Object.keys(condition).length === 0 && <p className="ce-no-rules">No rules. Add one below.</p>}
      <div className="ce-add-rule-wrap">
        <Dropdown
          compact
          value=""
          onChange={v => v && addConditionType(v)}
          placeholder="+ Add rule"
          options={ruleTypes.filter(({ key }) => !(key in condition)).map(({ key, label }) => ({ value: key, label }))}
        />
      </div>
    </div>
  );
};

export default ConditionsEditor;
