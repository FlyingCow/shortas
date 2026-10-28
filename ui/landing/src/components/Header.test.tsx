import { render, screen, fireEvent } from '@testing-library/react';
import Header from './Header';

describe('Header', () => {
  it('renders the logo and primary nav links', () => {
    render(<Header />);
    expect(screen.getByText('Shortas')).toBeInTheDocument();
    expect(screen.getByRole('link', { name: 'Features' })).toHaveAttribute('href', '#features');
    expect(screen.getByRole('link', { name: 'How It Works' })).toHaveAttribute('href', '#how-it-works');
  });

  it('toggles the mobile menu open and closed', () => {
    render(<Header />);
    const toggle = screen.getByRole('button', { name: /toggle menu/i });
    expect(toggle).toHaveAttribute('aria-expanded', 'false');

    fireEvent.click(toggle);
    expect(toggle).toHaveAttribute('aria-expanded', 'true');

    fireEvent.click(toggle);
    expect(toggle).toHaveAttribute('aria-expanded', 'false');
  });

  it('closes the mobile menu when a nav link is clicked', () => {
    render(<Header />);
    const toggle = screen.getByRole('button', { name: /toggle menu/i });
    fireEvent.click(toggle);
    expect(toggle).toHaveAttribute('aria-expanded', 'true');

    fireEvent.click(screen.getByRole('link', { name: 'Features' }));
    expect(toggle).toHaveAttribute('aria-expanded', 'false');
  });

  it('links to GitHub with safe external attributes', () => {
    render(<Header />);
    const githubLink = screen.getByRole('link', { name: /view source on github/i });
    expect(githubLink).toHaveAttribute('href', 'https://github.com/FlyingCow/shortas');
    expect(githubLink).toHaveAttribute('target', '_blank');
    expect(githubLink).toHaveAttribute('rel', 'noopener noreferrer');
  });
});
