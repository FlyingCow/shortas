import { render, screen, fireEvent } from '@testing-library/react';
import WorldMap from './WorldMap';

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
});
