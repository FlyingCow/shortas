import { renderHook, waitFor, act } from '@testing-library/react';
import { WorkspaceProvider, useWorkspace } from './WorkspaceContext';
import { apiService, WorkspaceDto } from '../services/api';

jest.mock('../services/api', () => ({
  apiService: {
    workspaces: {
      list: jest.fn(),
    },
  },
}));

const mockList = apiService.workspaces.list as jest.Mock;

const STORAGE_KEY = 'shortas.currentWorkspaceId';

function workspace(id: string, overrides: Partial<WorkspaceDto> = {}): WorkspaceDto {
  return {
    id,
    name: `Workspace ${id}`,
    description: '',
    workspaceType: 'User',
    isSystem: false,
    ...overrides,
  };
}

describe('WorkspaceContext', () => {
  beforeEach(() => {
    localStorage.clear();
    mockList.mockReset();
  });

  it('throws when useWorkspace is used outside a WorkspaceProvider', () => {
    const spy = jest.spyOn(console, 'error').mockImplementation(() => {});
    expect(() => renderHook(() => useWorkspace())).toThrow(/must be used within a WorkspaceProvider/);
    spy.mockRestore();
  });

  it('loads workspaces on mount and selects the first one when nothing is stored', async () => {
    mockList.mockResolvedValue([workspace('ws-1'), workspace('ws-2')]);

    const { result } = renderHook(() => useWorkspace(), { wrapper: WorkspaceProvider });

    expect(result.current.loading).toBe(true);

    await waitFor(() => expect(result.current.loading).toBe(false));

    expect(result.current.workspaces).toHaveLength(2);
    expect(result.current.currentWorkspaceId).toBe('ws-1');
  });

  it('keeps the persisted selection when it is still a valid workspace', async () => {
    localStorage.setItem(STORAGE_KEY, 'ws-2');
    mockList.mockResolvedValue([workspace('ws-1'), workspace('ws-2')]);

    const { result } = renderHook(() => useWorkspace(), { wrapper: WorkspaceProvider });
    await waitFor(() => expect(result.current.loading).toBe(false));

    expect(result.current.currentWorkspaceId).toBe('ws-2');
  });

  it('falls back to the first workspace when the persisted selection no longer exists', async () => {
    localStorage.setItem(STORAGE_KEY, 'deleted-workspace');
    mockList.mockResolvedValue([workspace('ws-1'), workspace('ws-2')]);

    const { result } = renderHook(() => useWorkspace(), { wrapper: WorkspaceProvider });
    await waitFor(() => expect(result.current.loading).toBe(false));

    expect(result.current.currentWorkspaceId).toBe('ws-1');
  });

  it('sets currentWorkspaceId to null when the user has no workspaces', async () => {
    mockList.mockResolvedValue([]);

    const { result } = renderHook(() => useWorkspace(), { wrapper: WorkspaceProvider });
    await waitFor(() => expect(result.current.loading).toBe(false));

    expect(result.current.currentWorkspaceId).toBeNull();
    expect(result.current.workspaces).toEqual([]);
  });

  it('stops loading and logs when the API call fails', async () => {
    const consoleSpy = jest.spyOn(console, 'error').mockImplementation(() => {});
    mockList.mockRejectedValue(new Error('network down'));

    const { result } = renderHook(() => useWorkspace(), { wrapper: WorkspaceProvider });
    await waitFor(() => expect(result.current.loading).toBe(false));

    expect(result.current.workspaces).toEqual([]);
    expect(consoleSpy).toHaveBeenCalledWith('Failed to load workspaces:', expect.any(Error));
    consoleSpy.mockRestore();
  });

  it('setCurrentWorkspaceId updates state and persists to localStorage', async () => {
    mockList.mockResolvedValue([workspace('ws-1'), workspace('ws-2')]);

    const { result } = renderHook(() => useWorkspace(), { wrapper: WorkspaceProvider });
    await waitFor(() => expect(result.current.loading).toBe(false));

    act(() => result.current.setCurrentWorkspaceId('ws-2'));

    expect(result.current.currentWorkspaceId).toBe('ws-2');
    expect(localStorage.getItem(STORAGE_KEY)).toBe('ws-2');
  });
});
