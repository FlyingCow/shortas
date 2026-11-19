import { render, screen, fireEvent } from '@testing-library/react';
import WorldMap, { getTooltipPlacement } from './WorldMap';

// react-simple-maps renders real geography/projection logic over large
// topology data; WorldMap's own logic (color intensity, tooltip content) is
// what we actually want to test, so replace the library with simple stubs
// that hand WorldMap a small, fixed set of fake country features.
const FAKE_FEATURES = [
  { properties: { name: 'united states of america' }, rsmKey: 'geo-us' },
  { properties: { name: 'france' }, rsmKey: 'geo-fr' },
  { properties: { name: 'atlantis' }, rsmKey: 'geo-atlantis' },
];

jest.mock('react-simple-maps', () => ({
  ComposableMap: ({ children }: any) => <svg>{children}</svg>,
  Geographies: ({ children }: any) => children({ geographies: FAKE_FEATURES }),
  Geography: ({ geography, onMouseEnter, onMouseLeave, style }: any) => (
    <path
      data-testid={`geo-${geography.properties.name}`}
      data-fill={style.default.fill}
      onMouseEnter={() => onMouseEnter({ clientX: 100, clientY: 50 })}
      onMouseLeave={onMouseLeave}
    />
  ),
}));

jest.mock('../data/world-110m.json', () => ({}), { virtual: true });

describe('WorldMap', () => {
  it('colors the country with the most clicks the darkest shade', () => {
    render(<WorldMap data={[{ name: 'us', clicks: 100, percentage: 50 }, { name: 'fr', clicks: 20, percentage: 10 }]} />);

    expect(screen.getByTestId('geo-united states of america')).toHaveAttribute('data-fill', '#1e3a8a');
  });

  it('colors a country with exactly 20% intensity using the lightest shade (threshold is exclusive)', () => {
    render(<WorldMap data={[{ name: 'us', clicks: 100, percentage: 50 }, { name: 'fr', clicks: 20, percentage: 10 }]} />);

    expect(screen.getByTestId('geo-france')).toHaveAttribute('data-fill', '#93c5fd');
  });

  it('colors a country with no matching data as neutral gray', () => {
    render(<WorldMap data={[{ name: 'us', clicks: 100, percentage: 50 }]} />);

    expect(screen.getByTestId('geo-atlantis')).toHaveAttribute('data-fill', '#e5e7eb');
  });

  it('shows a tooltip with clicks and percentage on hover, and hides it on mouse leave', () => {
    render(<WorldMap data={[{ name: 'us', clicks: 1234, percentage: 42 }]} />);

    fireEvent.mouseEnter(screen.getByTestId('geo-united states of america'));
    expect(screen.getByText('united states of america')).toBeInTheDocument();
    expect(screen.getByText('1,234')).toBeInTheDocument();
    expect(screen.getByText('42%')).toBeInTheDocument();

    fireEvent.mouseLeave(screen.getByTestId('geo-united states of america'));
    expect(screen.queryByText('united states of america')).not.toBeInTheDocument();
  });

  it('shows 0 clicks/0% in the tooltip for a hovered country with no data', () => {
    render(<WorldMap data={[{ name: 'us', clicks: 100, percentage: 50 }]} />);

    fireEvent.mouseEnter(screen.getByTestId('geo-atlantis'));
    expect(screen.getByText('atlantis')).toBeInTheDocument();
    expect(screen.getByText('0')).toBeInTheDocument();
    expect(screen.getByText('0%')).toBeInTheDocument();
  });

  describe('sizing and tooltip placement on narrow containers', () => {
    it('sizes the map by aspect ratio, capped by the height prop, and clips overflow', () => {
      const { container } = render(<WorldMap data={[]} height={300} />);
      const map = container.querySelector('.world-map-container') as HTMLElement;
      expect(map.style.width).toBe('100%');
      expect(map.style.aspectRatio).toBe('2 / 1');
      expect(map.style.maxHeight).toBe('300px');
      expect(map.style.height).toBe('');
    });

    it('places the tooltip in the quadrant opposite the pointer', () => {
      expect(getTooltipPlacement({ x: 50, y: 20 }, { width: 300, height: 150 })).toEqual({
        left: 'auto', right: 10, top: 'auto', bottom: 10,
      });
      expect(getTooltipPlacement({ x: 250, y: 120 }, { width: 300, height: 150 })).toEqual({
        left: 10, right: 'auto', top: 10, bottom: 'auto',
      });
    });

    it("uses the container's measured height rather than the height prop", () => {
      // A narrow map: 160px wide renders 80px tall even though height=400.
      const widthSpy = jest.spyOn(HTMLElement.prototype, 'clientWidth', 'get').mockReturnValue(160);
      const heightSpy = jest.spyOn(HTMLElement.prototype, 'clientHeight', 'get').mockReturnValue(80);
      try {
        render(<WorldMap data={[{ name: 'us', clicks: 10, percentage: 100 }]} height={400} />);
        // The stubbed Geography reports the pointer at (100, 50); the container rect is at 0,0 in jsdom.
        fireEvent.mouseEnter(screen.getByTestId('geo-united states of america'));
        const tooltip = screen.getByTestId('world-map-tooltip');
        // y=50 is in the bottom half of the measured 80px -> tooltip at the top.
        // (Against the 400px prop it would wrongly count as the top half.)
        expect(tooltip.style.top).toBe('10px');
        expect(tooltip.style.bottom).not.toBe('10px');
        // x=100 is in the right half of 160 -> tooltip on the left.
        expect(tooltip.style.left).toBe('10px');
      } finally {
        widthSpy.mockRestore();
        heightSpy.mockRestore();
      }
    });

    it('caps the tooltip width to the map container', () => {
      render(<WorldMap data={[{ name: 'us', clicks: 10, percentage: 100 }]} />);
      fireEvent.mouseEnter(screen.getByTestId('geo-united states of america'));
      const tooltip = screen.getByTestId('world-map-tooltip');
      expect(tooltip.style.maxWidth).toBe('calc(100% - 20px)');
      expect(tooltip.style.minWidth).toBe('min(180px, calc(100% - 20px))');
    });
  });
});
