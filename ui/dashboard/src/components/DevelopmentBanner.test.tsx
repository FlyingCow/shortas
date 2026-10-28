import { render, screen } from '@testing-library/react';

// `isDevelopment`/`useMockData` are read once at module-load time from
// process.env, so each scenario needs a fresh module graph.
function renderBannerWithEnv(nodeEnv: string, useMockData?: string) {
  jest.resetModules();
  process.env.NODE_ENV = nodeEnv;
  if (useMockData === undefined) {
    delete process.env.REACT_APP_USE_MOCK_DATA;
  } else {
    process.env.REACT_APP_USE_MOCK_DATA = useMockData;
  }
  // eslint-disable-next-line @typescript-eslint/no-var-requires
  const DevelopmentBanner = require('./DevelopmentBanner').default;
  return render(<DevelopmentBanner />);
}

describe('DevelopmentBanner', () => {
  const originalEnv = { ...process.env };

  afterEach(() => {
    process.env = { ...originalEnv };
  });

  it('renders nothing outside development mode', () => {
    renderBannerWithEnv('production');
    expect(screen.queryByText('Development Mode')).not.toBeInTheDocument();
  });

  it('shows the banner in development without the mock-data hint when mock data is off', () => {
    renderBannerWithEnv('development', 'false');
    expect(screen.getByText('Development Mode')).toBeInTheDocument();
    expect(screen.queryByText(/Using mock data/)).not.toBeInTheDocument();
    expect(screen.queryByText(/REACT_APP_USE_MOCK_DATA=false/)).not.toBeInTheDocument();
  });

  it('shows the mock-data hint when mock data is enabled', () => {
    renderBannerWithEnv('development', 'true');
    expect(screen.getByText('Development Mode')).toBeInTheDocument();
    expect(screen.getByText(/Using mock data/)).toBeInTheDocument();
    expect(screen.getByText(/REACT_APP_USE_MOCK_DATA=false/)).toBeInTheDocument();
  });
});
