import React, { useState, useRef, useEffect, useLayoutEffect, useCallback } from 'react';
import { createPortal } from 'react-dom';
import { ChevronDown, Check } from 'lucide-react';

export interface DropdownOption {
  value: string;
  label: string;
}

interface DropdownProps {
  value: string;
  options: DropdownOption[];
  onChange: (value: string) => void;
  icon?: React.ReactNode;
  placeholder?: string;
  disabled?: boolean;
  error?: boolean;
  /** Smaller footprint for dense contexts (e.g. the conditional-routing rule builder). */
  compact?: boolean;
  /** Stretch to fill its container, like a native block-level <select>. */
  block?: boolean;
  align?: 'left' | 'right';
  className?: string;
  style?: React.CSSProperties;
  emptyMessage?: string;
}

const MENU_MAX_HEIGHT = 320;
const MENU_GAP = 6;

interface MenuPosition {
  top?: number;
  bottom?: number;
  left?: number;
  right?: number;
  minWidth: number;
}

/**
 * Styled replacement for a native <select>, matching the topbar workspace
 * switcher's look everywhere: a button trigger (icon + current label +
 * chevron) opening a floating menu with a checkmark on the active option.
 *
 * The menu is rendered in a portal (document.body) and positioned with
 * `position: fixed` from the trigger's actual screen coordinates, so it's
 * never clipped by a scrollable/overflow-constrained ancestor (e.g. a
 * modal body or a form panel) the way a plain `position: absolute` child
 * would be.
 */
const Dropdown: React.FC<DropdownProps> = ({
  value,
  options,
  onChange,
  icon,
  placeholder = 'Select…',
  disabled,
  error,
  compact,
  block,
  align = 'left',
  className,
  style,
  emptyMessage = 'No options',
}) => {
  const [open, setOpen] = useState(false);
  const [menuPos, setMenuPos] = useState<MenuPosition | null>(null);
  const wrapperRef = useRef<HTMLDivElement>(null);
  const triggerRef = useRef<HTMLButtonElement>(null);
  const menuRef = useRef<HTMLDivElement>(null);
  const current = options.find((o) => o.value === value);

  const updatePosition = useCallback(() => {
    const btn = triggerRef.current;
    if (!btn) return;
    const rect = btn.getBoundingClientRect();
    // Flip upward when there isn't room below but there is above — anchor
    // via `bottom` in that case so the menu grows up from the trigger
    // instead of overlapping it (we cap height with CSS, not a measured
    // actual height, so we can't just offset `top` by the menu's height).
    const openUpward = rect.bottom + MENU_GAP + MENU_MAX_HEIGHT > window.innerHeight && rect.top > MENU_MAX_HEIGHT;
    const base: MenuPosition = { minWidth: rect.width };
    if (openUpward) {
      base.bottom = window.innerHeight - rect.top + MENU_GAP;
    } else {
      base.top = rect.bottom + MENU_GAP;
    }
    if (align === 'right') {
      base.right = window.innerWidth - rect.right;
    } else {
      base.left = rect.left;
    }
    setMenuPos(base);
  }, [align]);

  useLayoutEffect(() => {
    if (open) updatePosition();
  }, [open, updatePosition]);

  useEffect(() => {
    const onClickOutside = (e: MouseEvent) => {
      const target = e.target as Node;
      const insideTrigger = wrapperRef.current?.contains(target);
      const insideMenu = menuRef.current?.contains(target);
      if (!insideTrigger && !insideMenu) {
        setOpen(false);
      }
    };
    document.addEventListener('mousedown', onClickOutside);
    return () => document.removeEventListener('mousedown', onClickOutside);
  }, []);

  useEffect(() => {
    if (!open) return;
    const onKeyDown = (e: KeyboardEvent) => {
      if (e.key === 'Escape') setOpen(false);
    };
    // Close on scroll/resize rather than trying to track a moving trigger —
    // simpler and matches how most floating menus behave. Scrolling the
    // menu's own option list is not an "outside" scroll, though — a long
    // list (e.g. the country picker) needs to stay open while scrolled.
    const onScroll = (e: Event) => {
      if (menuRef.current && e.target instanceof Node && menuRef.current.contains(e.target)) {
        return;
      }
      setOpen(false);
    };
    const onResize = () => setOpen(false);
    document.addEventListener('keydown', onKeyDown);
    window.addEventListener('scroll', onScroll, true);
    window.addEventListener('resize', onResize);
    return () => {
      document.removeEventListener('keydown', onKeyDown);
      window.removeEventListener('scroll', onScroll, true);
      window.removeEventListener('resize', onResize);
    };
  }, [open]);

  const sizeClass = compact ? 'btn-xs' : 'btn-sm';
  const iconSize = compact ? 12 : 14;

  const menu = open && menuPos && createPortal(
    <div
      ref={menuRef}
      className="app-select-menu"
      style={{
        position: 'fixed',
        top: menuPos.top,
        bottom: menuPos.bottom,
        left: menuPos.left,
        right: menuPos.right,
        minWidth: menuPos.minWidth,
      }}
    >
      {options.length === 0 ? (
        <div className="app-select-empty">{emptyMessage}</div>
      ) : (
        options.map((opt) => (
          <button
            type="button"
            key={opt.value}
            className={`app-select-item ${opt.value === value ? 'app-select-item-active' : ''}`}
            onClick={() => {
              onChange(opt.value);
              setOpen(false);
            }}
          >
            <span>{opt.label}</span>
            {opt.value === value && <Check size={14} />}
          </button>
        ))
      )}
    </div>,
    document.body
  );

  return (
    <div
      ref={wrapperRef}
      className={`app-select ${block ? 'app-select-block' : ''} ${className || ''}`}
      style={style}
    >
      <button
        ref={triggerRef}
        type="button"
        className={`btn btn-outline ${sizeClass} app-select-trigger ${error ? 'app-select-error' : ''}`}
        onClick={() => !disabled && setOpen((o) => !o)}
        disabled={disabled}
      >
        {icon}
        <span className={`app-select-trigger-label ${!current ? 'app-select-placeholder' : ''}`}>
          {current?.label ?? placeholder}
        </span>
        <ChevronDown size={iconSize} className={`app-select-chevron ${open ? 'app-select-chevron-open' : ''}`} />
      </button>

      {menu}
    </div>
  );
};

export default Dropdown;
