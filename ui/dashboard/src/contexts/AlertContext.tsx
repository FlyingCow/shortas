import React, { createContext, useCallback, useContext, useEffect, useRef, useState } from 'react';

export interface ConfirmOptions {
  confirmLabel?: string;
  cancelLabel?: string;
  variant?: 'danger' | 'primary';
}

interface AlertState {
  type: 'alert' | 'confirm';
  title: string;
  message: string;
  confirmLabel: string;
  cancelLabel: string;
  variant: 'danger' | 'primary';
}

export type ToastVariant = 'success' | 'error' | 'info' | 'warning';

interface Toast {
  id: string;
  message: string;
  variant: ToastVariant;
}

interface AlertContextValue {
  showAlert: (message: string, title?: string) => void;
  showConfirm: (message: string, title?: string, options?: ConfirmOptions) => Promise<boolean>;
  showToast: (message: string, variant?: ToastVariant) => void;
}

const AlertContext = createContext<AlertContextValue | null>(null);

export function useAlert(): AlertContextValue {
  const ctx = useContext(AlertContext);
  if (!ctx) throw new Error('useAlert must be used within AlertProvider');
  return ctx;
}

interface AlertProviderProps {
  children: React.ReactNode;
}

export function AlertProvider({ children }: AlertProviderProps) {
  const [state, setState] = useState<AlertState | null>(null);
  const resolveRef = useRef<((value: boolean) => void) | null>(null);
  const [toasts, setToasts] = useState<Toast[]>([]);
  const toastIdRef = useRef(0);

  const showToast = useCallback((message: string, variant: ToastVariant = 'info') => {
    const id = `toast-${++toastIdRef.current}`;
    setToasts(prev => [...prev, { id, message, variant }]);
    setTimeout(() => {
      setToasts(prev => prev.filter(t => t.id !== id));
    }, 4000);
  }, []);

  const removeToast = useCallback((id: string) => {
    setToasts(prev => prev.filter(t => t.id !== id));
  }, []);

  const showAlert = useCallback((message: string, title = 'Notice') => {
    setState({
      type: 'alert',
      title,
      message,
      confirmLabel: 'OK',
      cancelLabel: 'Cancel',
      variant: 'primary',
    });
  }, []);

  const showConfirm = useCallback((
    message: string,
    title = 'Confirm',
    options?: ConfirmOptions
  ): Promise<boolean> => {
    return new Promise((resolve) => {
      resolveRef.current = resolve;
      setState({
        type: 'confirm',
        title,
        message,
        confirmLabel: options?.confirmLabel ?? 'Confirm',
        cancelLabel: options?.cancelLabel ?? 'Cancel',
        variant: options?.variant ?? 'primary',
      });
    });
  }, []);

  const handleClose = useCallback(() => {
    setState(null);
    if (resolveRef.current) {
      resolveRef.current(false);
      resolveRef.current = null;
    }
  }, []);

  const handleConfirm = useCallback(() => {
    if (state?.type === 'alert') {
      setState(null);
      return;
    }
    if (resolveRef.current) {
      resolveRef.current(true);
      resolveRef.current = null;
    }
    setState(null);
  }, [state?.type]);

  const value: AlertContextValue = { showAlert, showConfirm, showToast };

  useEffect(() => {
    if (!state) return;
    const onKey = (e: KeyboardEvent) => {
      if (e.key === 'Escape') {
        handleClose();
        e.preventDefault();
        e.stopImmediatePropagation();
      } else if (e.key === 'Enter' && !e.shiftKey) {
        handleConfirm();
        e.preventDefault();
        e.stopImmediatePropagation();
      }
    };
    document.addEventListener('keydown', onKey);
    return () => document.removeEventListener('keydown', onKey);
  }, [state, handleClose, handleConfirm]);

  return (
    <AlertContext.Provider value={value}>
      {children}
      {state && (
        <>
          <style>{`
            .alert-msgbox-overlay {
              position: fixed;
              inset: 0;
              background: var(--theme-bg-overlay, rgba(0,0,0,0.5));
              display: flex;
              align-items: center;
              justify-content: center;
              z-index: 1050;
              padding: 1rem;
            }
            .alert-msgbox {
              background: var(--theme-bg-elevated, #fff);
              border-radius: var(--radius-lg, 8px);
              box-shadow: var(--theme-shadow-xl, 0 8px 24px rgba(0,0,0,0.15));
              max-width: 380px;
              width: 100%;
              overflow: hidden;
            }
            .alert-msgbox-header {
              padding: 0.75rem 1rem;
              border-bottom: 1px solid var(--theme-border-primary, #e4e4e7);
              font-size: 0.9375rem;
              font-weight: 600;
              color: var(--theme-text-primary, #09090b);
            }
            .alert-msgbox-body {
              padding: 0.75rem 1rem;
              font-size: 0.875rem;
              color: var(--theme-text-secondary, #3f3f46);
              line-height: 1.45;
            }
            .alert-msgbox-footer {
              padding: 0.5rem 1rem 0.75rem;
              display: flex;
              gap: 0.5rem;
              justify-content: flex-end;
            }
            .alert-msgbox-footer .btn { font-size: 0.8125rem; padding: 0.375rem 0.75rem; }
          `}</style>
          <div
            className="alert-msgbox-overlay"
            onClick={state.type === 'alert' ? handleClose : undefined}
            role="dialog"
            aria-modal="true"
            aria-labelledby="alert-dialog-title"
          >
            <div className="alert-msgbox" onClick={(e) => e.stopPropagation()}>
              <div id="alert-dialog-title" className="alert-msgbox-header">
                {state.title}
              </div>
              <div className="alert-msgbox-body">
                {state.message}
              </div>
              <div className="alert-msgbox-footer">
                {state.type === 'confirm' && (
                  <button type="button" className="btn btn-outline" onClick={handleClose}>
                    {state.cancelLabel}
                  </button>
                )}
                <button
                  type="button"
                  className={state.variant === 'danger' ? 'btn btn-danger' : 'btn btn-primary'}
                  onClick={handleConfirm}
                >
                  {state.type === 'alert' ? 'OK' : state.confirmLabel}
                </button>
              </div>
            </div>
          </div>
        </>
      )}
      {/* Toast notifications */}
      {toasts.length > 0 && (
        <>
          <style>{`
            .toast-container {
              position: fixed;
              top: 1rem;
              right: 1rem;
              z-index: 1060;
              display: flex;
              flex-direction: column;
              gap: 0.5rem;
              pointer-events: none;
            }
            .toast {
              display: flex;
              align-items: center;
              gap: 0.75rem;
              padding: 0.75rem 1rem;
              background: var(--theme-bg-elevated, #fff);
              border-radius: var(--radius-md, 6px);
              box-shadow: 0 4px 12px rgba(0, 0, 0, 0.15);
              min-width: 280px;
              max-width: 400px;
              pointer-events: auto;
              animation: toast-slide-in 0.2s ease-out;
            }
            @keyframes toast-slide-in {
              from {
                opacity: 0;
                transform: translateX(1rem);
              }
              to {
                opacity: 1;
                transform: translateX(0);
              }
            }
            .toast-icon {
              width: 20px;
              height: 20px;
              flex-shrink: 0;
              display: flex;
              align-items: center;
              justify-content: center;
            }
            .toast-success .toast-icon { color: #16a34a; }
            .toast-error .toast-icon { color: #dc2626; }
            .toast-warning .toast-icon { color: #ca8a04; }
            .toast-info .toast-icon { color: #2563eb; }
            .toast-message {
              flex: 1;
              font-size: 0.875rem;
              color: var(--theme-text-primary, #09090b);
              line-height: 1.4;
            }
            .toast-close {
              width: 24px;
              height: 24px;
              display: flex;
              align-items: center;
              justify-content: center;
              border: none;
              background: transparent;
              color: var(--theme-text-muted, #a1a1aa);
              cursor: pointer;
              border-radius: var(--radius-sm, 4px);
              flex-shrink: 0;
              transition: all 0.15s;
            }
            .toast-close:hover {
              background: var(--theme-bg-tertiary, #f4f4f5);
              color: var(--theme-text-primary, #09090b);
            }
          `}</style>
          <div className="toast-container">
            {toasts.map((toast) => (
              <div key={toast.id} className={`toast toast-${toast.variant}`}>
                <span className="toast-icon">
                  {toast.variant === 'success' && (
                    <svg width="20" height="20" viewBox="0 0 20 20" fill="none" xmlns="http://www.w3.org/2000/svg">
                      <path d="M16.667 5L7.5 14.167 3.333 10" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round"/>
                    </svg>
                  )}
                  {toast.variant === 'error' && (
                    <svg width="20" height="20" viewBox="0 0 20 20" fill="none" xmlns="http://www.w3.org/2000/svg">
                      <circle cx="10" cy="10" r="7.5" stroke="currentColor" strokeWidth="2"/>
                      <path d="M12.5 7.5l-5 5M7.5 7.5l5 5" stroke="currentColor" strokeWidth="2" strokeLinecap="round"/>
                    </svg>
                  )}
                  {toast.variant === 'warning' && (
                    <svg width="20" height="20" viewBox="0 0 20 20" fill="none" xmlns="http://www.w3.org/2000/svg">
                      <path d="M10 7v3M10 13h.01" stroke="currentColor" strokeWidth="2" strokeLinecap="round"/>
                      <path d="M8.57 3.814L2.07 15.092a1.5 1.5 0 001.287 2.258h13.286a1.5 1.5 0 001.287-2.258L11.43 3.814a1.5 1.5 0 00-2.86 0z" stroke="currentColor" strokeWidth="2"/>
                    </svg>
                  )}
                  {toast.variant === 'info' && (
                    <svg width="20" height="20" viewBox="0 0 20 20" fill="none" xmlns="http://www.w3.org/2000/svg">
                      <circle cx="10" cy="10" r="7.5" stroke="currentColor" strokeWidth="2"/>
                      <path d="M10 9v4M10 7h.01" stroke="currentColor" strokeWidth="2" strokeLinecap="round"/>
                    </svg>
                  )}
                </span>
                <span className="toast-message">{toast.message}</span>
                <button className="toast-close" onClick={() => removeToast(toast.id)} aria-label="Close">
                  <svg width="14" height="14" viewBox="0 0 14 14" fill="none" xmlns="http://www.w3.org/2000/svg">
                    <path d="M10.5 3.5l-7 7M3.5 3.5l7 7" stroke="currentColor" strokeWidth="1.5" strokeLinecap="round"/>
                  </svg>
                </button>
              </div>
            ))}
          </div>
        </>
      )}
    </AlertContext.Provider>
  );
}
