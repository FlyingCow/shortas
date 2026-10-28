import { render, screen } from '@testing-library/react';
import Hero from './Hero';

describe('Hero', () => {
  it('renders the headline and subtitle', () => {
    render(<Hero />);
    expect(screen.getByRole('heading', { level: 1 })).toHaveTextContent('built for speed');
    expect(screen.getByText(/open-source, self-hostable link shortener/i)).toBeInTheDocument();
  });

  it('renders primary and secondary call-to-action links', () => {
    render(<Hero />);
    const getStarted = screen.getByRole('link', { name: /get started/i });
    expect(getStarted).toHaveAttribute('href', 'https://shortas.work/getting-started/');

    const github = screen.getByRole('link', { name: /view on github/i });
    expect(github).toHaveAttribute('href', 'https://github.com/FlyingCow/shortas');
    expect(github).toHaveAttribute('target', '_blank');
  });

  it('is anchored at #top for the header logo link to target', () => {
    const { container } = render(<Hero />);
    expect(container.querySelector('section#top')).not.toBeNull();
  });
});
