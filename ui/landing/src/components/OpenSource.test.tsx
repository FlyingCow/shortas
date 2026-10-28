import { render, screen } from '@testing-library/react';
import OpenSource from './OpenSource';

describe('OpenSource', () => {
  it('renders the three open-source feature callouts', () => {
    render(<OpenSource />);
    expect(screen.getByRole('heading', { name: 'Fully Open Source' })).toBeInTheDocument();
    expect(screen.getByRole('heading', { name: 'Community Driven' })).toBeInTheDocument();
    expect(screen.getByRole('heading', { name: 'Self-Hostable' })).toBeInTheDocument();
  });

  it('links to the source repo and the contributing guide', () => {
    render(<OpenSource />);
    expect(screen.getByRole('link', { name: /view source code/i })).toHaveAttribute(
      'href',
      'https://github.com/FlyingCow/shortas'
    );
    expect(screen.getByRole('link', { name: /start contributing/i })).toHaveAttribute(
      'href',
      'https://shortas.work/development/'
    );
  });

  it('is anchored at #open-source for the header nav link to target', () => {
    const { container } = render(<OpenSource />);
    expect(container.querySelector('section#open-source')).not.toBeNull();
  });
});
