import { render, screen } from '@testing-library/react';
import Features from './Features';

describe('Features', () => {
  it('renders the section heading', () => {
    render(<Features />);
    expect(screen.getByRole('heading', { name: /everything a short link needs/i })).toBeInTheDocument();
  });

  it('renders all six feature cards with titles', () => {
    render(<Features />);
    const titles = [
      'Built for Speed',
      'Real-Time Analytics',
      'Conditional Routing',
      'Custom Domains',
      'Safe by Default',
      'Open Source',
    ];
    titles.forEach((title) => {
      expect(screen.getByRole('heading', { name: title })).toBeInTheDocument();
    });
  });

  it('is anchored at #features for the header nav link to target', () => {
    const { container } = render(<Features />);
    expect(container.querySelector('section#features')).not.toBeNull();
  });
});
