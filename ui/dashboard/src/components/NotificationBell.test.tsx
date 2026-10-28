import { render, screen, fireEvent, waitFor, act } from '@testing-library/react';
import NotificationBell from './NotificationBell';
import { apiService, NotificationDto } from '../services/api';

jest.mock('../services/api', () => ({
  apiService: {
    notifications: { list: jest.fn() },
    invitations: { accept: jest.fn(), decline: jest.fn() },
  },
}));

const mockShowToast = jest.fn();
const mockRefreshWorkspaces = jest.fn();

jest.mock('../contexts/AlertContext', () => ({
  useAlert: () => ({ showToast: mockShowToast }),
}));

jest.mock('../contexts/WorkspaceContext', () => ({
  useWorkspace: () => ({ refreshWorkspaces: mockRefreshWorkspaces }),
}));

const mockList = apiService.notifications.list as jest.Mock;
const mockAccept = apiService.invitations.accept as jest.Mock;
const mockDecline = apiService.invitations.decline as jest.Mock;

function notification(overrides: Partial<NotificationDto> = {}): NotificationDto {
  return {
    id: 'n1',
    type: 'workspace.invite_created',
    payload: { invitation_id: 'inv-1', workspace_name: 'Acme', inviter_name: 'Jane', role: 'Admin' },
    status: 'Unread',
    createdAt: new Date().toISOString(),
    ...overrides,
  };
}

describe('NotificationBell', () => {
  beforeEach(() => {
    jest.clearAllMocks();
    mockList.mockResolvedValue([]);
  });

  it('shows no badge when there are no unread notifications', async () => {
    render(<NotificationBell />);
    fireEvent.click(screen.getByTitle('Notifications'));
    await waitFor(() => expect(screen.getByText("You're all caught up.")).toBeInTheDocument());
    expect(screen.queryByText(/^\d+$|^9\+$/)).not.toBeInTheDocument();
  });

  it('shows the unread count, capped at "9+"', async () => {
    mockList.mockResolvedValue(
      Array.from({ length: 12 }, (_, i) => notification({ id: `n${i}`, status: 'Unread' }))
    );
    render(<NotificationBell />);
    await waitFor(() => expect(screen.getByText('9+')).toBeInTheDocument());
  });

  it('toggles the panel open and closed when clicking the bell', async () => {
    render(<NotificationBell />);
    expect(screen.queryByText('Notifications')).not.toBeInTheDocument();

    fireEvent.click(screen.getByTitle('Notifications'));
    expect(screen.getByText('Notifications')).toBeInTheDocument();

    fireEvent.click(screen.getByTitle('Notifications'));
    expect(screen.queryByText('Notifications')).not.toBeInTheDocument();
  });

  it('closes the panel when clicking outside it', async () => {
    render(
      <div>
        <div data-testid="outside">outside</div>
        <NotificationBell />
      </div>
    );
    fireEvent.click(screen.getByTitle('Notifications'));
    expect(screen.getByText('Notifications')).toBeInTheDocument();

    fireEvent.mouseDown(screen.getByTestId('outside'));
    expect(screen.queryByText('Notifications')).not.toBeInTheDocument();
  });

  it('shows an empty state when there are no notifications', async () => {
    render(<NotificationBell />);
    fireEvent.click(screen.getByTitle('Notifications'));
    await waitFor(() => expect(screen.getByText("You're all caught up.")).toBeInTheDocument());
  });

  it('prefers renderedText, falling back to a generated invite description', async () => {
    mockList.mockResolvedValue([
      notification({ id: 'n1', renderedText: 'Jane invited you to Acme' }),
      notification({ id: 'n2', renderedText: undefined, type: 'WorkspaceInvite', payload: { workspace_name: 'Beta', inviter_name: 'Bob', role: 'Member' } }),
    ]);
    render(<NotificationBell />);
    fireEvent.click(screen.getByTitle('Notifications'));

    await waitFor(() => expect(screen.getByText('Jane invited you to Acme')).toBeInTheDocument());
    // 'Member' maps to the user-facing label 'User'.
    expect(screen.getByText('Bob invited you to join "Beta" as User')).toBeInTheDocument();
  });

  it('falls back to the camelCase payload keys and defaults when snake_case keys are absent', async () => {
    mockList.mockResolvedValue([
      notification({
        id: 'n1',
        renderedText: undefined,
        type: 'WorkspaceInvite',
        payload: { workspaceName: 'Gamma', inviterName: 'Amy' },
      }),
    ]);
    render(<NotificationBell />);
    fireEvent.click(screen.getByTitle('Notifications'));

    await waitFor(() => expect(screen.getByText('Amy invited you to join "Gamma" as Member')).toBeInTheDocument());
  });

  it('falls back to the raw notification type for unrecognized types', async () => {
    mockList.mockResolvedValue([notification({ id: 'n1', renderedText: undefined, type: 'SomeOtherType', payload: {} })]);
    render(<NotificationBell />);
    fireEvent.click(screen.getByTitle('Notifications'));

    await waitFor(() => expect(screen.getByText('SomeOtherType')).toBeInTheDocument());
  });

  it('shows accept/decline actions for a pending invite but not an actioned one', async () => {
    mockList.mockResolvedValue([
      notification({ id: 'n1', status: 'Unread' }),
      notification({ id: 'n2', status: 'Actioned' }),
    ]);
    render(<NotificationBell />);
    fireEvent.click(screen.getByTitle('Notifications'));

    await waitFor(() => expect(screen.getAllByRole('button', { name: /accept/i })).toHaveLength(1));
  });

  it('accepting an invite dismisses it, refreshes workspaces, and shows a success toast', async () => {
    mockList.mockResolvedValue([notification({ id: 'n1' })]);
    mockAccept.mockResolvedValue(undefined);
    mockRefreshWorkspaces.mockResolvedValue(undefined);

    render(<NotificationBell />);
    fireEvent.click(screen.getByTitle('Notifications'));
    await waitFor(() => expect(screen.getByRole('button', { name: /accept/i })).toBeInTheDocument());

    fireEvent.click(screen.getByRole('button', { name: /accept/i }));

    await waitFor(() => expect(mockAccept).toHaveBeenCalledWith('inv-1'));
    await waitFor(() => expect(mockRefreshWorkspaces).toHaveBeenCalled());
    expect(mockShowToast).toHaveBeenCalledWith('Invitation accepted', 'success');
    expect(screen.queryByRole('button', { name: /accept/i })).not.toBeInTheDocument();
  });

  it('shows a server error message when accepting fails', async () => {
    mockList.mockResolvedValue([notification({ id: 'n1' })]);
    mockAccept.mockRejectedValue({ response: { data: { message: 'Invite expired' } } });

    render(<NotificationBell />);
    fireEvent.click(screen.getByTitle('Notifications'));
    await waitFor(() => expect(screen.getByRole('button', { name: /accept/i })).toBeInTheDocument());

    fireEvent.click(screen.getByRole('button', { name: /accept/i }));

    await waitFor(() => expect(mockShowToast).toHaveBeenCalledWith('Invite expired', 'error'));
    // Still present since dismiss() only runs on success.
    expect(screen.getByRole('button', { name: /accept/i })).toBeInTheDocument();
  });

  it('declining an invite dismisses it without refreshing workspaces', async () => {
    mockList.mockResolvedValue([notification({ id: 'n1' })]);
    mockDecline.mockResolvedValue(undefined);

    render(<NotificationBell />);
    fireEvent.click(screen.getByTitle('Notifications'));
    await waitFor(() => expect(screen.getByRole('button', { name: /decline/i })).toBeInTheDocument());

    fireEvent.click(screen.getByRole('button', { name: /decline/i }));

    await waitFor(() => expect(mockDecline).toHaveBeenCalledWith('inv-1'));
    expect(mockRefreshWorkspaces).not.toHaveBeenCalled();
    await waitFor(() => expect(screen.queryByRole('button', { name: /decline/i })).not.toBeInTheDocument());
  });

  it('polls for new notifications on an interval', async () => {
    jest.useFakeTimers();
    render(<NotificationBell />);

    await waitFor(() => expect(mockList).toHaveBeenCalledTimes(1));

    await act(async () => {
      jest.advanceTimersByTime(30000);
    });
    expect(mockList).toHaveBeenCalledTimes(2);

    jest.useRealTimers();
  });
});
