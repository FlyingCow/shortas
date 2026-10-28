import { render, screen, fireEvent } from '@testing-library/react';
import Settings from './SettingsUnified';

const mockSetTheme = jest.fn();
let mockTheme = 'light';

jest.mock('../contexts/ThemeContext', () => ({
  useTheme: () => ({ theme: mockTheme, setTheme: mockSetTheme }),
}));

describe('SettingsUnified', () => {
  beforeEach(() => {
    mockSetTheme.mockClear();
    mockTheme = 'light';
  });

  it('marks the active theme option and shows the current theme label', () => {
    mockTheme = 'dark';
    render(<Settings />);

    expect(screen.getByRole('button', { name: /dark/i }).className).toContain('active');
    expect(screen.getByRole('button', { name: /light/i }).className).not.toContain('active');
    expect(screen.getByText('Dark', { selector: 'strong' })).toBeInTheDocument();
  });

  it('calls setTheme when a theme option is clicked', () => {
    render(<Settings />);
    fireEvent.click(screen.getByRole('button', { name: /dark/i }));
    expect(mockSetTheme).toHaveBeenCalledWith('dark');

    fireEvent.click(screen.getByRole('button', { name: /light/i }));
    expect(mockSetTheme).toHaveBeenCalledWith('light');
  });

  it('links to the Keycloak account console for password and social login management', () => {
    render(<Settings />);

    const changePassword = screen.getByRole('link', { name: /change password/i });
    expect(changePassword).toHaveAttribute('href', expect.stringContaining('/realms/shortas-dev/account/#/security/signingin'));
    expect(changePassword).toHaveAttribute('target', '_blank');
    expect(changePassword).toHaveAttribute('rel', 'noopener noreferrer');

    const socialLogins = screen.getByRole('link', { name: /social logins/i });
    expect(socialLogins).toHaveAttribute('href', expect.stringContaining('/realms/shortas-dev/account/#/security/linked-accounts'));
  });
});
