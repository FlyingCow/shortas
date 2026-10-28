import { render, screen, fireEvent } from '@testing-library/react';
import { MemoryRouter, useLocation } from 'react-router-dom';
import Layout from './LayoutUnified';
import { logout, getUserInfo } from '../config/keycloak';

jest.mock('../config/keycloak', () => ({
  logout: jest.fn(),
  getUserInfo: jest.fn(),
}));

const mockToggleTheme = jest.fn();
let mockTheme = 'light';

jest.mock('../contexts/ThemeContext', () => ({
  useTheme: () => ({ theme: mockTheme, toggleTheme: mockToggleTheme }),
}));

const mockSetCurrentWorkspaceId = jest.fn();
let mockWorkspaceState: any = {
  workspaces: [],
  currentWorkspaceId: null,
  setCurrentWorkspaceId: mockSetCurrentWorkspaceId,
  refreshWorkspaces: jest.fn(),
};

jest.mock('../contexts/WorkspaceContext', () => ({
  useWorkspace: () => mockWorkspaceState,
}));

jest.mock('./NotificationBell', () => ({
  __esModule: true,
  default: () => <div data-testid="notification-bell" />,
}));

function LocationProbe() {
  const location = useLocation();
  return <div data-testid="current-path">{location.pathname}</div>;
}

function renderLayout(initialPath = '/dashboard') {
  return render(
    <MemoryRouter initialEntries={[initialPath]}>
      <Layout>
        <div>page content</div>
      </Layout>
      <LocationProbe />
    </MemoryRouter>
  );
}

describe('LayoutUnified', () => {
  beforeEach(() => {
    jest.clearAllMocks();
    mockTheme = 'light';
    mockWorkspaceState = {
      workspaces: [],
      currentWorkspaceId: null,
      setCurrentWorkspaceId: mockSetCurrentWorkspaceId,
      refreshWorkspaces: jest.fn(),
    };
    (getUserInfo as jest.Mock).mockReturnValue({ username: 'jane', email: 'jane@example.com' });
  });

  it('highlights the active nav item for the current route', () => {
    renderLayout('/domains');
    expect(screen.getByRole('link', { name: /domains/i }).className).toContain('active');
    expect(screen.getByRole('link', { name: /routes/i }).className).not.toContain('active');
  });

  it('renders the page content passed as children', () => {
    renderLayout();
    expect(screen.getByText('page content')).toBeInTheDocument();
  });

  it("shows the user's email in the account menu trigger", () => {
    renderLayout();
    expect(screen.getByText('jane@example.com')).toBeInTheDocument();
  });

  it('falls back to username when no email is available', () => {
    (getUserInfo as jest.Mock).mockReturnValue({ username: 'jane' });
    renderLayout();
    expect(screen.getByText('jane')).toBeInTheDocument();
  });

  it("falls back to 'User' when neither email nor username is available", () => {
    (getUserInfo as jest.Mock).mockReturnValue({});
    renderLayout();
    expect(screen.getAllByText('User').length).toBeGreaterThan(0);
  });

  it('opens the user menu and navigates to settings, closing the menu', () => {
    renderLayout();
    fireEvent.click(screen.getByRole('button', { name: /jane@example.com/i }));
    expect(screen.getByRole('button', { name: /^Settings$/i })).toBeInTheDocument();

    fireEvent.click(screen.getByRole('button', { name: /^Settings$/i }));
    expect(screen.getByTestId('current-path')).toHaveTextContent('/settings');
    expect(screen.queryByRole('button', { name: /^Settings$/i })).not.toBeInTheDocument();
  });

  it('logs out and closes the user menu when Logout is clicked', () => {
    renderLayout();
    fireEvent.click(screen.getByRole('button', { name: /jane@example.com/i }));
    fireEvent.click(screen.getByRole('button', { name: /logout/i }));

    expect(logout).toHaveBeenCalledTimes(1);
    expect(screen.queryByRole('button', { name: /logout/i })).not.toBeInTheDocument();
  });

  it('closes the user menu when clicking outside', () => {
    renderLayout();
    fireEvent.click(screen.getByRole('button', { name: /jane@example.com/i }));
    expect(screen.getByRole('button', { name: /logout/i })).toBeInTheDocument();

    fireEvent.mouseDown(screen.getByText('page content'));
    expect(screen.queryByRole('button', { name: /logout/i })).not.toBeInTheDocument();
  });

  it('toggles the theme and reflects the current theme in the sidebar label', () => {
    renderLayout();
    expect(screen.getByText('Dark mode')).toBeInTheDocument();

    fireEvent.click(screen.getByTitle('Switch to dark theme'));
    expect(mockToggleTheme).toHaveBeenCalledTimes(1);
  });

  it('shows "Light mode" when the current theme is dark', () => {
    mockTheme = 'dark';
    renderLayout();
    expect(screen.getByText('Light mode')).toBeInTheDocument();
  });

  it('groups owned workspaces under "My Workspaces" and shared ones under "Shared with Me", labeling Member as User', () => {
    mockWorkspaceState.workspaces = [
      { id: 'w1', name: 'Personal', userRole: 'Owner' },
      { id: 'w2', name: 'Acme', userRole: 'Member' },
      { id: 'w3', name: 'Beta', userRole: 'Admin' },
    ];
    mockWorkspaceState.currentWorkspaceId = 'w1';
    renderLayout();

    fireEvent.click(screen.getByRole('button', { name: /personal/i }));
    expect(screen.getByText('My Workspaces')).toBeInTheDocument();
    expect(screen.getByText('Shared with Me')).toBeInTheDocument();
    expect(screen.getByText('Acme (User)')).toBeInTheDocument();
    expect(screen.getByText('Beta (Admin)')).toBeInTheDocument();
  });

  it('selecting a workspace calls setCurrentWorkspaceId', () => {
    mockWorkspaceState.workspaces = [
      { id: 'w1', name: 'Personal', userRole: 'Owner' },
      { id: 'w2', name: 'Acme', userRole: 'Member' },
    ];
    mockWorkspaceState.currentWorkspaceId = 'w1';
    renderLayout();

    fireEvent.click(screen.getByRole('button', { name: /personal/i }));
    fireEvent.click(screen.getByText('Acme (User)'));
    expect(mockSetCurrentWorkspaceId).toHaveBeenCalledWith('w2');
  });

  describe('keyboard shortcuts', () => {
    it('Alt+1 through Alt+5 navigate to the corresponding nav route', () => {
      renderLayout();
      fireEvent.keyDown(document, { key: '3', altKey: true });
      expect(screen.getByTestId('current-path')).toHaveTextContent('/domains');
    });

    it('Alt+6 navigates to settings', () => {
      renderLayout();
      fireEvent.keyDown(document, { key: '6', altKey: true });
      expect(screen.getByTestId('current-path')).toHaveTextContent('/settings');
    });

    it('ignores Alt+number shortcuts while an input is focused', () => {
      renderLayout();
      const input = document.createElement('input');
      document.body.appendChild(input);
      input.focus();

      fireEvent.keyDown(document, { key: '3', altKey: true });
      expect(screen.getByTestId('current-path')).toHaveTextContent('/dashboard');

      document.body.removeChild(input);
    });

    it('"?" opens the shortcuts help dialog', () => {
      renderLayout();
      expect(screen.queryByRole('dialog', { name: /keyboard shortcuts/i })).not.toBeInTheDocument();

      fireEvent.keyDown(document, { key: '?' });
      expect(screen.getByRole('dialog', { name: /keyboard shortcuts/i })).toBeInTheDocument();
    });

    it('does not open the shortcuts dialog via "?" while an input is focused', () => {
      renderLayout();
      const input = document.createElement('input');
      document.body.appendChild(input);
      input.focus();

      fireEvent.keyDown(document, { key: '?' });
      expect(screen.queryByRole('dialog', { name: /keyboard shortcuts/i })).not.toBeInTheDocument();

      document.body.removeChild(input);
    });

    it('Escape closes the shortcuts dialog when open', () => {
      renderLayout();
      fireEvent.keyDown(document, { key: '?' });
      expect(screen.getByRole('dialog', { name: /keyboard shortcuts/i })).toBeInTheDocument();

      fireEvent.keyDown(document, { key: 'Escape' });
      expect(screen.queryByRole('dialog', { name: /keyboard shortcuts/i })).not.toBeInTheDocument();
    });

    it('Escape closes the user menu when the shortcuts dialog is not open', () => {
      renderLayout();
      fireEvent.click(screen.getByRole('button', { name: /jane@example.com/i }));
      expect(screen.getByRole('button', { name: /logout/i })).toBeInTheDocument();

      fireEvent.keyDown(document, { key: 'Escape' });
      expect(screen.queryByRole('button', { name: /logout/i })).not.toBeInTheDocument();
    });
  });

  describe('shortcuts help dialog', () => {
    it('opens via the sidebar Shortcuts button and closes via the Close button', () => {
      renderLayout();
      fireEvent.click(screen.getByTitle('Keyboard shortcuts (?)'));
      expect(screen.getByRole('dialog', { name: /keyboard shortcuts/i })).toBeInTheDocument();

      fireEvent.click(screen.getByRole('button', { name: /^close$/i }));
      expect(screen.queryByRole('dialog', { name: /keyboard shortcuts/i })).not.toBeInTheDocument();
    });

    it('closes when clicking the overlay but not the panel itself', () => {
      renderLayout();
      fireEvent.click(screen.getByTitle('Keyboard shortcuts (?)'));
      const dialog = screen.getByRole('dialog', { name: /keyboard shortcuts/i });

      fireEvent.click(screen.getByText('Keyboard shortcuts', { selector: 'div' }));
      expect(screen.getByRole('dialog', { name: /keyboard shortcuts/i })).toBeInTheDocument();

      fireEvent.click(dialog);
      expect(screen.queryByRole('dialog', { name: /keyboard shortcuts/i })).not.toBeInTheDocument();
    });
  });
});
