import { render, screen, fireEvent, waitFor } from '@testing-library/react';
import LoggedOut from './LoggedOut';
import keycloak from '../config/keycloak';

jest.mock('../config/keycloak', () => ({
  __esModule: true,
  default: { login: jest.fn() },
  keycloakLoginOptions: { redirectUri: 'http://localhost/dashboard' },
}));

describe('LoggedOut', () => {
  afterEach(() => {
    jest.clearAllMocks();
  });

  it('logs in via Keycloak with the configured login options when clicking Sign In', async () => {
    (keycloak.login as jest.Mock).mockResolvedValue(undefined);
    render(<LoggedOut onLogin={jest.fn()} />);

    fireEvent.click(screen.getByRole('button', { name: /sign in to dashboard/i }));

    await waitFor(() =>
      expect(keycloak.login).toHaveBeenCalledWith({ redirectUri: 'http://localhost/dashboard' })
    );
  });

  it('logs an error when Keycloak login fails', async () => {
    const consoleSpy = jest.spyOn(console, 'error').mockImplementation(() => {});
    (keycloak.login as jest.Mock).mockRejectedValue(new Error('popup blocked'));

    render(<LoggedOut onLogin={jest.fn()} />);
    fireEvent.click(screen.getByRole('button', { name: /sign in to dashboard/i }));

    await waitFor(() => expect(consoleSpy).toHaveBeenCalledWith('Login failed:', expect.any(Error)));
    consoleSpy.mockRestore();
  });

  it('navigates to the landing page when clicking Visit Landing Page', () => {
    const original = window.location;
    // @ts-expect-error - jsdom location isn't directly reassignable
    delete window.location;
    window.location = { ...original, href: '' } as Location;

    render(<LoggedOut onLogin={jest.fn()} />);
    fireEvent.click(screen.getByRole('button', { name: /visit landing page/i }));

    expect(window.location.href).toBe('https://shortas.com');
    window.location = original;
  });
});
