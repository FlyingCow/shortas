import { render } from '@testing-library/react';
import * as Icons from './Icons';

const iconNames = Object.keys(Icons) as Array<keyof typeof Icons>;

describe('Icons', () => {
  it('exports at least the icons used elsewhere on the page', () => {
    ['GithubIcon', 'MenuIcon', 'CloseIcon', 'ArrowRightIcon', 'ZapIcon'].forEach((name) => {
      expect(iconNames).toContain(name);
    });
  });

  it.each(iconNames)('%s renders a single svg element', (name) => {
    const Icon = Icons[name];
    const { container } = render(<Icon />);
    const svgs = container.querySelectorAll('svg');
    expect(svgs).toHaveLength(1);
  });

  it('passes through and overrides width/height props', () => {
    const { container } = render(<Icons.GithubIcon width={32} height={32} />);
    const svg = container.querySelector('svg');
    expect(svg).toHaveAttribute('width', '32');
    expect(svg).toHaveAttribute('height', '32');
  });

  it('defaults to a 24x24 viewBox when no size props are given', () => {
    const { container } = render(<Icons.ZapIcon />);
    const svg = container.querySelector('svg');
    expect(svg).toHaveAttribute('width', '24');
    expect(svg).toHaveAttribute('viewBox', '0 0 24 24');
  });
});
