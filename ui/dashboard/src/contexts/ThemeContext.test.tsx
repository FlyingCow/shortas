import { renderHook, act } from '@testing-library/react';
import { ThemeProvider, useTheme } from './ThemeContext';

function mockMatchMedia(prefersDark: boolean) {
  const listeners: Array<(e: MediaQueryListEvent) => void> = [];
  window.matchMedia = jest.fn().mockImplementation((query: string) => ({
    matches: prefersDark,
    media: query,
    addEventListener: (_: string, cb: (e: MediaQueryListEvent) => void) => listeners.push(cb),
    removeEventListener: jest.fn(),
  })) as any;
  return {
    fireChange: (matches: boolean) => listeners.forEach((cb) => cb({ matches } as MediaQueryListEvent)),
  };
}

describe('ThemeContext', () => {
  beforeEach(() => {
    localStorage.clear();
    document.documentElement.removeAttribute('data-theme');
    document.documentElement.className = '';
  });

  it('throws when useTheme is used outside a ThemeProvider', () => {
    // Suppress the expected React error-boundary console noise for this one case.
    const spy = jest.spyOn(console, 'error').mockImplementation(() => {});
    expect(() => renderHook(() => useTheme())).toThrow(/must be used within a ThemeProvider/);
    spy.mockRestore();
  });

  it('defaults to the saved localStorage theme when present', () => {
    localStorage.setItem('theme', 'dark');
    mockMatchMedia(false);
    const { result } = renderHook(() => useTheme(), { wrapper: ThemeProvider });
    expect(result.current.theme).toBe('dark');
  });

  it('falls back to system preference when no saved theme exists', () => {
    mockMatchMedia(true);
    const { result } = renderHook(() => useTheme(), { wrapper: ThemeProvider });
    expect(result.current.theme).toBe('dark');
  });

  it('defaults to light when no saved theme and no system preference', () => {
    mockMatchMedia(false);
    const { result } = renderHook(() => useTheme(), { wrapper: ThemeProvider });
    expect(result.current.theme).toBe('light');
  });

  it('toggleTheme flips the theme and persists it, updating the DOM', () => {
    mockMatchMedia(false);
    const { result } = renderHook(() => useTheme(), { wrapper: ThemeProvider });

    act(() => result.current.toggleTheme());

    expect(result.current.theme).toBe('dark');
    expect(localStorage.getItem('theme')).toBe('dark');
    expect(document.documentElement.getAttribute('data-theme')).toBe('dark');
    expect(document.documentElement.classList.contains('theme-dark')).toBe(true);
  });

  it('setTheme sets an explicit theme', () => {
    mockMatchMedia(false);
    const { result } = renderHook(() => useTheme(), { wrapper: ThemeProvider });

    act(() => result.current.setTheme('dark'));
    expect(result.current.theme).toBe('dark');

    act(() => result.current.setTheme('light'));
    expect(result.current.theme).toBe('light');
    expect(localStorage.getItem('theme')).toBe('light');
  });

  it('follows system theme changes only when the user has not explicitly set one', () => {
    const { fireChange } = mockMatchMedia(false);
    const { result } = renderHook(() => useTheme(), { wrapper: ThemeProvider });

    expect(result.current.theme).toBe('light');

    act(() => fireChange(true));
    expect(result.current.theme).toBe('dark');
  });

  it('does not follow system theme changes once the user has set an explicit theme', () => {
    const { fireChange } = mockMatchMedia(false);
    const { result } = renderHook(() => useTheme(), { wrapper: ThemeProvider });

    act(() => result.current.setTheme('light'));
    act(() => fireChange(true));

    // User explicitly chose light, so the system "dark" change should be ignored.
    expect(result.current.theme).toBe('light');
  });
});
