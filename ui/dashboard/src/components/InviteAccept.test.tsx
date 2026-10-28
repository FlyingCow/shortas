import { render, screen, fireEvent, waitFor, act } from '@testing-library/react';
import InviteAccept from './InviteAccept';
import { apiService } from '../services/api';
import keycloak from '../config/keycloak';

const mockNavigate = jest.fn();

jest.mock('react-router-dom', () => ({
  useParams: () => ({ token: 'tok-123' }),
  useNavigate: () => mockNavigate,
}));

jest.mock('../services/api', () => ({
  apiService: {
    invitations: {
      previewByToken: jest.fn(),
      acceptByToken: jest.fn(),
    },
  },
}));

jest.mock('../config/keycloak', () => ({
  __esModule: true,
  default: { login: jest.fn() },
}));

const mockPreview = apiService.invitations.previewByToken as jest.Mock;
const mockAccept = apiService.invitations.acceptByToken as jest.Mock;

function previewDto(overrides = {}) {
  return {
    workspaceName: 'Acme',
    inviterName: 'Jane',
    role: 'Admin',
    invitedEmail: 'bob@example.com',
    status: 'Pending',
    ...overrides,
  };
}

describe('InviteAccept', () => {
  beforeEach(() => {
    jest.clearAllMocks();
  });

  it('shows a loading state while the preview is being fetched', () => {
    mockPreview.mockReturnValue(new Promise(() => {})); // never resolves
    render(<InviteAccept authenticated={true} />);
    expect(screen.getByText('Loading invitation...')).toBeInTheDocument();
  });

  it('shows the invitation details once loaded', async () => {
    mockPreview.mockResolvedValue(previewDto());
    render(<InviteAccept authenticated={true} />);

    await waitFor(() => expect(screen.getByText('Join "Acme"')).toBeInTheDocument());
    expect(screen.getByText('bob@example.com')).toBeInTheDocument();
    expect(screen.getByText('Admin')).toBeInTheDocument();
  });

  it('maps the Member role to the user-facing label "User"', async () => {
    mockPreview.mockResolvedValue(previewDto({ role: 'Member' }));
    render(<InviteAccept authenticated={true} />);

    await waitFor(() => expect(screen.getByText('User')).toBeInTheDocument());
  });

  it('shows an invalid-link message for a 404 with no retry option', async () => {
    mockPreview.mockRejectedValue({ response: { status: 404 } });
    render(<InviteAccept authenticated={true} />);

    await waitFor(() => expect(screen.getByText('This invitation link is invalid.')).toBeInTheDocument());
    expect(screen.queryByRole('button', { name: /try again/i })).not.toBeInTheDocument();
  });

  it('shows an already-used message for a 409 with no retry option', async () => {
    mockPreview.mockRejectedValue({ response: { status: 409 } });
    render(<InviteAccept authenticated={true} />);

    await waitFor(() =>
      expect(screen.getByText(/no longer valid/)).toBeInTheDocument()
    );
    expect(screen.queryByRole('button', { name: /try again/i })).not.toBeInTheDocument();
  });

  it('shows a retryable network error when the request never completes', async () => {
    const consoleSpy = jest.spyOn(console, 'error').mockImplementation(() => {});
    mockPreview.mockRejectedValueOnce(new Error('network down'));
    render(<InviteAccept authenticated={true} />);

    await waitFor(() => expect(screen.getByText(/Couldn't reach the server/)).toBeInTheDocument());
    expect(screen.getByRole('button', { name: /try again/i })).toBeInTheDocument();

    mockPreview.mockResolvedValueOnce(previewDto());
    fireEvent.click(screen.getByRole('button', { name: /try again/i }));

    await waitFor(() => expect(screen.getByText('Join "Acme"')).toBeInTheDocument());
    consoleSpy.mockRestore();
  });

  it('shows an unexpected-HTTP-error message with a retry option for other status codes', async () => {
    const consoleSpy = jest.spyOn(console, 'error').mockImplementation(() => {});
    mockPreview.mockRejectedValue({ response: { status: 500 } });
    render(<InviteAccept authenticated={true} />);

    await waitFor(() => expect(screen.getByText(/Unexpected error \(HTTP 500\)/)).toBeInTheDocument());
    expect(screen.getByRole('button', { name: /try again/i })).toBeInTheDocument();
    consoleSpy.mockRestore();
  });

  it('shows a login prompt instead of Accept when not authenticated, and logs in via Keycloak', async () => {
    mockPreview.mockResolvedValue(previewDto());
    render(<InviteAccept authenticated={false} />);

    await waitFor(() => expect(screen.getByRole('button', { name: /log in or sign up/i })).toBeInTheDocument());
    fireEvent.click(screen.getByRole('button', { name: /log in or sign up/i }));

    expect(keycloak.login).toHaveBeenCalledWith({ redirectUri: window.location.href });
  });

  it('accepts the invitation and navigates to /workspaces after a short delay', async () => {
    jest.useFakeTimers();
    mockPreview.mockResolvedValue(previewDto());
    mockAccept.mockResolvedValue(undefined);

    render(<InviteAccept authenticated={true} />);
    await waitFor(() => expect(screen.getByRole('button', { name: /accept invitation/i })).toBeInTheDocument());

    await act(async () => {
      fireEvent.click(screen.getByRole('button', { name: /accept invitation/i }));
    });

    expect(mockAccept).toHaveBeenCalledWith('tok-123');
    expect(screen.getByText("You're in!")).toBeInTheDocument();
    expect(mockNavigate).not.toHaveBeenCalled();

    await act(async () => {
      jest.advanceTimersByTime(1500);
    });
    expect(mockNavigate).toHaveBeenCalledWith('/workspaces');

    jest.useRealTimers();
  });

  it('shows a server error message when accepting fails, without navigating away', async () => {
    const consoleSpy = jest.spyOn(console, 'error').mockImplementation(() => {});
    mockPreview.mockResolvedValue(previewDto());
    mockAccept.mockRejectedValue({ response: { data: { message: 'Invite already used' } } });

    render(<InviteAccept authenticated={true} />);
    await waitFor(() => expect(screen.getByRole('button', { name: /accept invitation/i })).toBeInTheDocument());

    fireEvent.click(screen.getByRole('button', { name: /accept invitation/i }));

    await waitFor(() => expect(screen.getByText('Invite already used')).toBeInTheDocument());
    expect(mockNavigate).not.toHaveBeenCalled();
    consoleSpy.mockRestore();
  });
});
