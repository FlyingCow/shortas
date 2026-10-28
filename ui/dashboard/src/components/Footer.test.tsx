import { render, screen } from '@testing-library/react';
import Footer from './Footer';

describe('Footer', () => {
  it('renders the current year in the copyright line', () => {
    render(<Footer />);
    const year = new Date().getFullYear().toString();
    expect(screen.getByText(`© ${year} Shortas`)).toBeInTheDocument();
  });

  it('links to GitHub safely', () => {
    render(<Footer />);
    const githubLink = screen.getByRole('link', { name: 'GitHub' });
    expect(githubLink).toHaveAttribute('href', 'https://github.com');
    expect(githubLink).toHaveAttribute('target', '_blank');
    expect(githubLink).toHaveAttribute('rel', 'noopener noreferrer');
  });
});
