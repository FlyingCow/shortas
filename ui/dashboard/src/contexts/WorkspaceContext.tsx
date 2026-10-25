import React, { createContext, useContext, useState, useEffect, useCallback, ReactNode } from 'react';
import { apiService, WorkspaceDto } from '../services/api';

const STORAGE_KEY = 'shortas.currentWorkspaceId';
const ALL_SENTINEL = 'all';

interface WorkspaceContextType {
  workspaces: WorkspaceDto[];
  loading: boolean;
  /** null means "All Workspaces" */
  currentWorkspaceId: string | null;
  setCurrentWorkspaceId: (id: string | null) => void;
  refreshWorkspaces: () => Promise<void>;
}

const WorkspaceContext = createContext<WorkspaceContextType | undefined>(undefined);

interface WorkspaceProviderProps {
  children: ReactNode;
}

export const WorkspaceProvider: React.FC<WorkspaceProviderProps> = ({ children }) => {
  const [workspaces, setWorkspaces] = useState<WorkspaceDto[]>([]);
  const [loading, setLoading] = useState(true);
  const [currentWorkspaceId, setCurrentWorkspaceIdState] = useState<string | null>(null);

  const setCurrentWorkspaceId = useCallback((id: string | null) => {
    setCurrentWorkspaceIdState(id);
    localStorage.setItem(STORAGE_KEY, id === null ? ALL_SENTINEL : id);
  }, []);

  const refreshWorkspaces = useCallback(async () => {
    try {
      setLoading(true);
      const data = await apiService.workspaces.list();
      setWorkspaces(data);

      // Reconcile the persisted selection against what's actually available:
      // honor an explicit "All Workspaces" choice, keep a still-valid
      // selection, or fall back to the user's first (usually their
      // auto-created Personal) workspace if nothing valid is stored.
      const stored = localStorage.getItem(STORAGE_KEY);
      if (stored === ALL_SENTINEL) {
        setCurrentWorkspaceIdState(null);
      } else if (stored && data.some((ws) => ws.id === stored)) {
        setCurrentWorkspaceIdState(stored);
      } else {
        setCurrentWorkspaceIdState(data[0]?.id ?? null);
      }
    } catch (err) {
      console.error('Failed to load workspaces:', err);
    } finally {
      setLoading(false);
    }
  }, []);

  useEffect(() => { refreshWorkspaces(); }, [refreshWorkspaces]);

  const value: WorkspaceContextType = {
    workspaces,
    loading,
    currentWorkspaceId,
    setCurrentWorkspaceId,
    refreshWorkspaces,
  };

  return (
    <WorkspaceContext.Provider value={value}>
      {children}
    </WorkspaceContext.Provider>
  );
};

export const useWorkspace = (): WorkspaceContextType => {
  const context = useContext(WorkspaceContext);
  if (context === undefined) {
    throw new Error('useWorkspace must be used within a WorkspaceProvider');
  }
  return context;
};
