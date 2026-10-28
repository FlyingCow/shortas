import { render } from '@testing-library/react';
import Logo from './Logo';

describe('Logo', () => {
  it('defaults to size 24 and currentColor', () => {
    const { container } = render(<Logo />);
    const svg = container.querySelector('svg');
    expect(svg).toHaveAttribute('width', '24');
    expect(svg).toHaveAttribute('height', '24');
    expect(svg).toHaveAttribute('stroke', 'currentColor');
  });

  it('applies a custom size, color, and className', () => {
    const { container } = render(<Logo size={48} color="#ff0000" className="my-logo" />);
    const svg = container.querySelector('svg');
    expect(svg).toHaveAttribute('width', '48');
    expect(svg).toHaveAttribute('height', '48');
    expect(svg).toHaveAttribute('stroke', '#ff0000');
    expect(svg).toHaveClass('my-logo');
  });
});
