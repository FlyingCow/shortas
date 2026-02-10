import React, { useState } from 'react';
import { Plus, Trash2, ChevronDown, ChevronUp, GripVertical } from 'lucide-react';
import {
  Expression,
  StringCondition,
  NumericCondition,
  ConditionRouteDto,
} from '../services/api';
import './DesignSystem.css';
import './PolicyEditor.css';
import './ConditionsEditor.css';

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

export const ConditionsEditor: React.FC<ConditionsEditorProps> = ({ conditions, onChange }) => {
  const [expandedConditions, setExpandedConditions] = useState<Set<number>>(new Set());
  const [dragOverIndex, setDragOverIndex] = useState<number | null>(null);

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

      {conditions.map((cond, index) => (
        <div
          key={index}
          className={`ce-block ${dragOverIndex === index ? 'ce-block--drag-over' : ''}`}
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
                  className="ce-block-dest"
                  placeholder="Destination URL"
                  value={cond.dest}
                  onChange={e => updateCondition(index, { ...cond, dest: e.target.value })}
                />
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
      ))}
    </div>
  );
};

interface ConditionRulesEditorProps {
  condition: Expression;
  onChange: (condition: Expression) => void;
}

const ConditionRulesEditor: React.FC<ConditionRulesEditorProps> = ({ condition, onChange }) => {
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

    return (
      <div key={String(field)} className="ce-rule-row">
        <span className="ce-rule-type">{label}</span>
        <select
          className="ce-rule-op"
          value={op}
          onChange={e => {
            const k = e.target.value as keyof StringCondition;
            if (k === 'eq') updateStringCondition(field, 'eq', '');
            if (k === 'in') updateStringCondition(field, 'in', []);
            if (k === 'starts') updateStringCondition(field, 'starts', '');
            if (k === 'ends') updateStringCondition(field, 'ends', '');
          }}
        >
          {stringOperators.map(o => (
            <option key={o.key} value={o.key}>{o.label}</option>
          ))}
        </select>
        {op === 'eq' && (
          hasOptions ? (
            <select
              className="ce-rule-value"
              value={cond.eq}
              onChange={e => updateStringCondition(field, 'eq', e.target.value)}
            >
              <option value="">Select…</option>
              {options.map(opt => <option key={opt} value={opt}>{opt}</option>)}
            </select>
          ) : (
            <input
              type="text"
              className="ce-rule-value"
              value={cond.eq}
              onChange={e => updateStringCondition(field, 'eq', e.target.value)}
              placeholder={`${label} value`}
            />
          )
        )}
        {op === 'in' && (
          <div className="ce-tags">
            {(cond.in || []).map((val, idx) => (
              <span key={idx} className="ce-tag">
                {val}
                <button type="button" className="ce-tag-remove" onClick={() => updateStringCondition(field, 'in', (cond.in || []).filter((_, i) => i !== idx))} aria-label="Remove">×</button>
              </span>
            ))}
            {hasOptions ? (
              <select
                className="ce-rule-value"
                style={{ maxWidth: 140 }}
                value=""
                onChange={e => e.target.value && updateStringCondition(field, 'in', [...(cond.in || []), e.target.value])}
              >
                <option value="">+ Add</option>
                {options.filter(o => !cond.in?.includes(o)).map(o => <option key={o} value={o}>{o}</option>)}
              </select>
            ) : (
              <input
                type="text"
                className="ce-rule-value"
                style={{ minWidth: 100 }}
                placeholder="a, b, c"
                value={(cond.in || []).join(', ')}
                onChange={e => updateStringCondition(field, 'in', e.target.value.split(',').map(s => s.trim()).filter(Boolean))}
              />
            )}
          </div>
        )}
        {(op === 'starts' || op === 'ends') && (
          <input
            type="text"
            className="ce-rule-value"
            value={op === 'starts' ? cond.starts : cond.ends}
            onChange={e => op === 'starts' ? updateStringCondition(field, 'starts', e.target.value) : updateStringCondition(field, 'ends', e.target.value)}
            placeholder={op === 'starts' ? 'Prefix…' : 'Suffix…'}
          />
        )}
        <button type="button" className="ce-rule-remove" onClick={() => removeConditionType(field)} aria-label="Remove rule">
          <Trash2 size={14} />
        </button>
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

    return (
      <div key={field} className="ce-rule-row">
        <span className="ce-rule-type">{label}</span>
        <select
          className="ce-rule-op"
          value={op}
          onChange={e => {
            const k = e.target.value as keyof NumericCondition;
            if (k === 'eq') updateNumericCondition(field, 'eq', isRnd ? 0 : isMonth ? 1 : isDayOfWeek ? 0 : 1);
            if (k === 'gt') updateNumericCondition(field, 'gt', 0);
            if (k === 'lt') updateNumericCondition(field, 'lt', 0);
            if (k === 'in') updateNumericCondition(field, 'in', []);
          }}
        >
          {numericOperators.map(o => (
            <option key={o.key} value={o.key}>{o.label}</option>
          ))}
        </select>
        {op === 'eq' && (
          isMonth ? (
            <select className="ce-rule-value" value={cond.eq} onChange={e => updateNumericCondition(field, 'eq', parseInt(e.target.value))}>
              <option value="">Month…</option>
              {MONTHS.map((m, i) => <option key={i} value={i + 1}>{m}</option>)}
            </select>
          ) : isDayOfWeek ? (
            <select className="ce-rule-value" value={cond.eq} onChange={e => updateNumericCondition(field, 'eq', parseInt(e.target.value))}>
              <option value="">Day…</option>
              {DAYS_OF_WEEK.map((d, i) => <option key={i} value={i}>{d}</option>)}
            </select>
          ) : (
            <input
              type="number"
              className="ce-rule-value"
              value={cond.eq}
              onChange={e => updateNumericCondition(field, 'eq', parseInt(e.target.value) || 0)}
              min={isRnd ? 0 : undefined}
              max={isRnd ? 100 : field === 'day_of_month' ? 31 : undefined}
              placeholder={isRnd ? '0–100' : 'Value'}
            />
          )
        )}
        {(op === 'gt' || op === 'lt') && (
          <input
            type="number"
            className="ce-rule-value"
            value={op === 'gt' ? cond.gt : cond.lt}
            onChange={e => updateNumericCondition(field, op, parseInt(e.target.value) || 0)}
            min={isRnd ? 0 : undefined}
            max={isRnd ? 100 : undefined}
          />
        )}
        {op === 'in' && (
          <input
            type="text"
            className="ce-rule-value"
            value={(cond.in || []).join(', ')}
            onChange={e => updateNumericCondition(field, 'in', e.target.value.split(',').map(s => parseInt(s.trim())).filter(n => !isNaN(n)))}
            placeholder="e.g. 1, 2, 3"
          />
        )}
        <button type="button" className="ce-rule-remove" onClick={() => removeConditionType(field)} aria-label="Remove rule">
          <Trash2 size={14} />
        </button>
      </div>
    );
  };

  const renderDateRule = () => {
    const cond = condition.date;
    if (!cond) return null;
    const op = cond.eq !== undefined ? 'eq' : cond.gt !== undefined ? 'gt' : cond.lt !== undefined ? 'lt' : 'in';

    return (
      <div key="date" className="ce-rule-row">
        <span className="ce-rule-type">Date</span>
        <select
          className="ce-rule-op"
          value={op}
          onChange={e => {
            const k = e.target.value as 'eq' | 'gt' | 'lt' | 'in';
            if (k === 'eq') updateDateCondition('eq', '');
            if (k === 'gt') updateDateCondition('gt', '');
            if (k === 'lt') updateDateCondition('lt', '');
            if (k === 'in') updateDateCondition('in', []);
          }}
        >
          {dateOperators.map(o => (
            <option key={o.key} value={o.key}>{o.label}</option>
          ))}
        </select>
        {(op === 'eq' || op === 'gt' || op === 'lt') && (
          <input
            type="date"
            className="ce-rule-value"
            value={op === 'eq' ? cond.eq : op === 'gt' ? cond.gt : cond.lt}
            onChange={e => updateDateCondition(op, e.target.value)}
          />
        )}
        {op === 'in' && (
          <div className="ce-tags">
            {(cond.in || []).map((d, idx) => (
              <span key={idx} className="ce-tag">
                {d}
                <button type="button" className="ce-tag-remove" onClick={() => updateDateCondition('in', (cond.in || []).filter((_, i) => i !== idx))} aria-label="Remove">×</button>
              </span>
            ))}
            <input
              type="date"
              className="ce-rule-value"
              style={{ maxWidth: 140 }}
              onChange={e => e.target.value && updateDateCondition('in', [...(cond.in || []), e.target.value])}
            />
          </div>
        )}
        <button type="button" className="ce-rule-remove" onClick={() => removeConditionType('date')} aria-label="Remove rule">
          <Trash2 size={14} />
        </button>
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
        <select
          className="ce-add-rule"
          value=""
          onChange={e => e.target.value && addConditionType(e.target.value)}
        >
          <option value="">+ Add rule</option>
          {ruleTypes.filter(({ key }) => !(key in condition)).map(({ key, label }) => (
            <option key={key} value={key}>{label}</option>
          ))}
        </select>
      </div>
    </div>
  );
};

export default ConditionsEditor;
