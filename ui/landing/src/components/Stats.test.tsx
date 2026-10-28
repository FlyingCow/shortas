import { render, screen } from '@testing-library/react';
import Stats from './Stats';

describe('Stats', () => {
  it('renders all four stat cards with their numbers and labels', () => {
    render(<Stats />);
    expect(screen.getByText('10+')).toBeInTheDocument();
    expect(screen.getByText('Rust Microservices')).toBeInTheDocument();
    expect(screen.getByText('100%')).toBeInTheDocument();
    expect(screen.getByText('Open Source')).toBeInTheDocument();
    expect(screen.getByText('17')).toBeInTheDocument();
    expect(screen.getByText('Vendor Lock-In')).toBeInTheDocument();
  });

  it('links to the architecture doc and the GitHub repo', () => {
    render(<Stats />);
    expect(screen.getByRole('link', { name: /read the architecture/i })).toHaveAttribute(
      'href',
      'https://shortas.work/architecture/'
    );
    expect(screen.getByRole('link', { name: /star on github/i })).toHaveAttribute(
      'href',
      'https://github.com/FlyingCow/shortas'
    );
  });
});
