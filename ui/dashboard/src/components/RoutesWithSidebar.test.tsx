import { render, screen, fireEvent, waitFor, act } from '@testing-library/react';
import { MemoryRouter, useLocation } from 'react-router-dom';
import RoutesWithSidebar, { parseRoutesPath, routePath } from './RoutesWithSidebar';
import { apiService } from '../services/api';

jest.mock('../services/api', () => ({
  apiService: {
    routes: {
      list: jest.fn(),
      get: jest.fn(),
      create: jest.fn(),
      update: jest.fn(),
      delete: jest.fn(),
      unblock: jest.fn(),
      search: jest.fn(),
    },
    domains: {
      list: jest.fn(),
      listShared: jest.fn(),
    },
    clickstream: {
      getDailyStats: jest.fn(),
      getGeographicStats: jest.fn(),
      getDeviceStats: jest.fn(),
      getBrowserStats: jest.fn(),
      getTrafficTypeStats: jest.fn(),
    },
  },
}));

jest.mock('../contexts/AlertContext', () => ({
  useAlert: () => ({ showToast: jest.fn(), showConfirm: jest.fn() }),
}));

let mockWorkspaceState: any = { workspaces: [], currentWorkspaceId: 'ws1' };
jest.mock('../contexts/WorkspaceContext', () => ({
  useWorkspace: () => mockWorkspaceState,
}));

jest.mock('../config/runtimeEnv', () => ({
  getRouteImagesBaseUrl: () => 'http://images.test',
}));

jest.mock('./RouteForm', () => ({
  __esModule: true,
  default: ({ route, onSave, onCancel }: any) => (
    <div data-testid="route-form" data-route-id={route?.id ?? ''}>
      <button onClick={() => onSave({ link: 'saved' })}>Save form</button>
      <button onClick={onCancel}>Cancel form</button>
    </div>
  ),
}));

jest.mock('./QRCodeDesigner', () => ({
  __esModule: true,
  default: () => <div data-testid="qr-designer" />,
}));

jest.mock('./WorldMap', () => ({
  __esModule: true,
  default: () => <div data-testid="world-map" />,
}));

jest.mock('recharts', () => {
  const actual = jest.requireActual('recharts');
  return { ...actual, ResponsiveContainer: () => null };
});

const mockedApi = apiService as unknown as {
  routes: Record<string, jest.Mock>;
  domains: Record<string, jest.Mock>;
  clickstream: Record<string, jest.Mock>;
};

const makeRoute = (id: string, link: string): any => ({
  id,
  switch: 'main',
  link,
  dest: `https://example.com/${link}`,
  destFormat: 'Http',
  code: 0,
  ttl: 0,
  status: 'Active',
  terminal: 'External',
  domain: { id: 'd1', name: 'sho.rt', ownerId: 'o1', verificationStatus: 'Verified', verificationReason: '' },
  properties: { routeId: id, ownerId: 'o1', domainId: 'd1', scripts: [], tags: [], custom: {}, opengraph: false, allowDebug: false },
});

const listedRoute = makeRoute('r1', 'first');
const unlistedRoute = makeRoute('r9', 'deep-linked');

function LocationProbe() {
  const location = useLocation();
  return <div data-testid="current-path">{location.pathname}</div>;
}

const currentPath = () => screen.getByTestId('current-path').textContent;

function renderAt(path: string) {
  return render(
    <MemoryRouter initialEntries={[path]}>
      <RoutesWithSidebar />
      <LocationProbe />
    </MemoryRouter>
  );
}

const pageRoot = (container: HTMLElement) =>
  container.querySelector('.routes-with-sidebar') as HTMLElement;

beforeEach(() => {
  jest.clearAllMocks();
  mockWorkspaceState = { workspaces: [], currentWorkspaceId: 'ws1' };
  mockedApi.routes.list.mockResolvedValue({ data: [listedRoute] });
  mockedApi.routes.get.mockRejectedValue(new Error('not found'));
  mockedApi.domains.list.mockResolvedValue({ data: [] });
  mockedApi.domains.listShared.mockResolvedValue([]);
  Object.values(mockedApi.clickstream).forEach((fn) => fn.mockResolvedValue([]));
});

describe('parseRoutesPath', () => {
  it.each([
    ['/routes', { view: 'list', routeId: null }],
    ['/routes/', { view: 'list', routeId: null }],
    ['/routes/new', { view: 'new', routeId: null }],
    ['/routes/abc', { view: 'detail', routeId: 'abc' }],
    ['/routes/abc/edit', { view: 'edit', routeId: 'abc' }],
    ['/routes/abc/qr', { view: 'qr', routeId: 'abc' }],
  ])('maps %s', (path, expected) => {
    expect(parseRoutesPath(path)).toEqual(expected);
  });

  it('rejects unknown sub-paths', () => {
    expect(parseRoutesPath('/routes/abc/nope')).toBeNull();
    expect(parseRoutesPath('/routes/a/b/c')).toBeNull();
  });

  it('builds paths that round-trip', () => {
    expect(parseRoutesPath(routePath('a b', 'edit'))).toEqual({ view: 'edit', routeId: 'a b' });
  });
});

describe('RoutesWithSidebar URL views', () => {
  it('shows the list without a detail pane at /routes', async () => {
    const { container } = renderAt('/routes');
    expect(await screen.findByText('Select a Route')).toBeInTheDocument();
    expect(pageRoot(container)).not.toHaveClass('has-detail');
  });

  it('opens the create form at /routes/new', async () => {
    const { container } = renderAt('/routes/new');
    const form = await screen.findByTestId('route-form');
    expect(form).toHaveAttribute('data-route-id', '');
    expect(screen.getByText('Create New Route')).toBeInTheDocument();
    expect(pageRoot(container)).toHaveClass('has-detail');
  });

  it('shows analytics for a listed route at /routes/:id', async () => {
    const { container } = renderAt('/routes/r1');
    expect(await screen.findByRole('heading', { name: 'first' })).toBeInTheDocument();
    expect(pageRoot(container)).toHaveClass('has-detail');
    expect(mockedApi.routes.get).not.toHaveBeenCalled();
  });

  it('opens the edit form for the route at /routes/:id/edit', async () => {
    renderAt('/routes/r1/edit');
    expect(await screen.findByTestId('route-form')).toHaveAttribute('data-route-id', 'r1');
  });

  it('opens the QR designer at /routes/:id/qr', async () => {
    renderAt('/routes/r1/qr');
    expect(await screen.findByTestId('qr-designer')).toBeInTheDocument();
  });

  it('fetches a deep-linked route that is not in the loaded list', async () => {
    mockedApi.routes.get.mockResolvedValue(unlistedRoute);
    renderAt('/routes/r9');
    expect(await screen.findByRole('heading', { name: 'deep-linked' })).toBeInTheDocument();
    expect(mockedApi.routes.get).toHaveBeenCalledWith('r9');
    expect(currentPath()).toBe('/routes/r9');
  });

  it('redirects to /routes when the route id is unknown', async () => {
    renderAt('/routes/missing');
    await waitFor(() => expect(currentPath()).toBe('/routes'));
  });

  it('redirects unknown sub-paths to /routes', async () => {
    renderAt('/routes/r1/bogus');
    await waitFor(() => expect(currentPath()).toBe('/routes'));
  });
});

describe('RoutesWithSidebar navigation', () => {
  it('navigates to /routes/:id when a route card is selected', async () => {
    renderAt('/routes');
    fireEvent.click(await screen.findByText('first'));
    expect(currentPath()).toBe('/routes/r1');
  });

  it('navigates to /routes/new from the Create Route button', async () => {
    renderAt('/routes');
    fireEvent.click(await screen.findByRole('button', { name: /create route/i }));
    expect(currentPath()).toBe('/routes/new');
  });

  it('navigates to the QR view from the card action', async () => {
    renderAt('/routes');
    fireEvent.click(await screen.findByTitle('QR Code'));
    expect(currentPath()).toBe('/routes/r1/qr');
  });

  it('cancelling an edit returns to the route analytics', async () => {
    renderAt('/routes/r1/edit');
    fireEvent.click(await screen.findByText('Cancel form'));
    expect(currentPath()).toBe('/routes/r1');
  });

  it('Escape on the create form returns to the list', async () => {
    renderAt('/routes/new');
    await screen.findByTestId('route-form');
    fireEvent.keyDown(document, { key: 'Escape' });
    expect(currentPath()).toBe('/routes');
  });

  it('Escape on the QR designer returns to the route analytics', async () => {
    renderAt('/routes/r1/qr');
    await screen.findByTestId('qr-designer');
    fireEvent.keyDown(document, { key: 'Escape' });
    expect(currentPath()).toBe('/routes/r1');
  });

  it.each(['/routes/new', '/routes/r1', '/routes/r1/edit', '/routes/r1/qr'])(
    'the back-to-Routes control on %s returns to the list',
    async (path) => {
      const { container } = renderAt(path);
      fireEvent.click(await screen.findByRole('button', { name: 'Routes', hidden: true }));
      expect(currentPath()).toBe('/routes');
      expect(pageRoot(container)).not.toHaveClass('has-detail');
    }
  );

  it('does not render the back-to-Routes control on the list', async () => {
    renderAt('/routes');
    await screen.findByText('first');
    expect(screen.queryByRole('button', { name: 'Routes', hidden: true })).not.toBeInTheDocument();
  });

  it('Ctrl+N opens the create form', async () => {
    renderAt('/routes');
    await screen.findByText('first');
    fireEvent.keyDown(document, { key: 'n', ctrlKey: true });
    expect(currentPath()).toBe('/routes/new');
  });

  it('after saving an edit, shows that route', async () => {
    mockedApi.routes.update.mockResolvedValue(listedRoute);
    renderAt('/routes/r1/edit');
    fireEvent.click(await screen.findByText('Save form'));
    await waitFor(() => expect(currentPath()).toBe('/routes/r1'));
    expect(mockedApi.routes.update).toHaveBeenCalledWith('r1', { link: 'saved' });
  });

  it('after creating, shows the new route', async () => {
    const created = makeRoute('r2', 'saved');
    mockedApi.routes.create.mockResolvedValue(created);
    renderAt('/routes/new');
    mockedApi.routes.list.mockResolvedValue({ data: [listedRoute, created] });
    fireEvent.click(await screen.findByText('Save form'));
    await waitFor(() => expect(currentPath()).toBe('/routes/r2'));
  });

  it('does not leave a deep link when the workspace resolves initially', async () => {
    mockWorkspaceState = { workspaces: [], currentWorkspaceId: null };
    const { rerender } = renderAt('/routes/r1');
    await screen.findByRole('heading', { name: 'first' });
    mockWorkspaceState = { workspaces: [], currentWorkspaceId: 'ws1' };
    await act(async () => {
      rerender(
        <MemoryRouter initialEntries={['/routes/r1']}>
          <RoutesWithSidebar />
          <LocationProbe />
        </MemoryRouter>
      );
    });
    expect(currentPath()).toBe('/routes/r1');
  });

  it('returns to the list when the user switches workspace', async () => {
    const { rerender } = renderAt('/routes/r1');
    await screen.findByRole('heading', { name: 'first' });
    mockWorkspaceState = { workspaces: [], currentWorkspaceId: 'ws2' };
    await act(async () => {
      rerender(
        <MemoryRouter initialEntries={['/routes/r1']}>
          <RoutesWithSidebar />
          <LocationProbe />
        </MemoryRouter>
      );
    });
    await waitFor(() => expect(currentPath()).toBe('/routes'));
  });
});
