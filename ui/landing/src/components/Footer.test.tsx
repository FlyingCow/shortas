import { render, screen } from '@testing-library/react';
import Footer from './Footer';

describe('Footer', () => {
  it('renders the current year in the copyright line', () => {
    render(<Footer />);
    const year = new Date().getFullYear().toString();
    expect(screen.getByText(new RegExp(`© ${year} Shortas`))).toBeInTheDocument();
  });

  it('renders a link to the GitHub repository', () => {
    render(<Footer />);
    const githubLinks = screen.getAllByRole('link', { name: /github|source code/i });
    expect(
      githubLinks.some((link) => link.getAttribute('href') === 'https://github.com/FlyingCow/shortas')
    ).toBe(true);
  });

  it('renders each footer section heading', () => {
    render(<Footer />);
    expect(screen.getByRole('heading', { name: 'Documentation' })).toBeInTheDocument();
    expect(screen.getByRole('heading', { name: 'Project' })).toBeInTheDocument();
    expect(screen.getByRole('heading', { name: 'About' })).toBeInTheDocument();
  });

  it('opens external links in a new tab safely', () => {
    render(<Footer />);
    const issueLink = screen.getByRole('link', { name: /report an issue/i });
    expect(issueLink).toHaveAttribute('target', '_blank');
    expect(issueLink).toHaveAttribute('rel', 'noopener noreferrer');
  });
});
