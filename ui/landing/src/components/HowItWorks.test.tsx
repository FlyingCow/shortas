import { render, screen } from '@testing-library/react';
import HowItWorks from './HowItWorks';

describe('HowItWorks', () => {
  it('renders all three steps in order', () => {
    render(<HowItWorks />);
    expect(screen.getByRole('heading', { name: 'Deploy It' })).toBeInTheDocument();
    expect(screen.getByRole('heading', { name: 'Add a Route' })).toBeInTheDocument();
    expect(screen.getByRole('heading', { name: 'Share & Track' })).toBeInTheDocument();
  });

  it('does not render a connector after the last step', () => {
    const { container } = render(<HowItWorks />);
    const steps = container.querySelectorAll('.step');
    expect(steps).toHaveLength(3);

    const connectors = container.querySelectorAll('.step-connector');
    expect(connectors).toHaveLength(2);
    expect(steps[2].querySelector('.step-connector')).toBeNull();
  });

  it('links to the getting-started guide', () => {
    render(<HowItWorks />);
    const link = screen.getByRole('link', { name: /read the getting started guide/i });
    expect(link).toHaveAttribute('href', 'https://shortas.work/getting-started/');
  });
});
