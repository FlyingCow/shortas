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

interface AlertContextValue {
  showAlert: (message: string, title?: string) => void;
  showConfirm: (message: string, title?: string, options?: ConfirmOptions) => Promise<boolean>;
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

  const value: AlertContextValue = { showAlert, showConfirm };

  useEffect(() => {
    if (!state) return;
    const onKey = (e: KeyboardEvent) => {
      if (e.key === 'Escape') {
        handleClose();
        e.preventDefault();
        e.stopImmediatePropagation();
      }
    };
    document.addEventListener('keydown', onKey);
    return () => document.removeEventListener('keydown', onKey);
  }, [state, handleClose]);

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
    </AlertContext.Provider>
  );
}
