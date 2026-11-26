import { render, screen, fireEvent, waitFor, within } from '@testing-library/react';
import { MemoryRouter, useLocation } from 'react-router-dom';
import Conversions from './Conversions';
import { refundSnippet, scriptSnippet, serverSnippet, stepSnippet } from './snippets';
import { validate } from './validation';
import { apiService, ConversionDto, ConversionLogItem } from '../../services/api';

jest.mock('../../services/api', () => ({
  apiService: {
    conversions: {
      list: jest.fn(),
      get: jest.fn(),
      create: jest.fn(),
      update: jest.fn(),
      changeStatus: jest.fn(),
      rotateSecret: jest.fn(),
      domains: jest.fn(),
      events: jest.fn(),
      sendTestEvent: jest.fn(),
    },
    routes: { list: jest.fn() },
  },
}));

const mockShowConfirm = jest.fn();
const mockShowToast = jest.fn();
jest.mock('../../contexts/AlertContext', () => ({
  useAlert: () => ({ showToast: mockShowToast, showConfirm: mockShowConfirm }),
}));

let mockRole: string;
let mockWorkspaces: any[] | null;
jest.mock('../../contexts/WorkspaceContext', () => ({
  useWorkspace: () => ({
    workspaces: mockWorkspaces ?? [{ id: 'w1', name: 'W', userRole: mockRole }],
    loading: false,
    currentWorkspaceId: 'w1',
  }),
}));

const api = apiService.conversions as jest.Mocked<typeof apiService.conversions>;
const SECRET = 'sk_abc_verysecret';

const conversion = (change: Partial<ConversionDto> = {}): ConversionDto => ({
  id: 'c1',
  workspaceId: 'w1',
  name: 'Purchase',
  status: 'Draft',
  version: 3,
  publicKey: 'pk_abc',
  archivedSteps: [],
  createdAt: '',
  updatedAt: '',
  currency: 'EUR',
  windowDays: 30,
  counting: 'Every',
  domainId: 'd1',
  allowedOrigins: [],
  serverValuesOnly: false,
  steps: [
    { key: 'cart', name: 'Cart' },
    { key: 'sale', name: 'Sale', defaultValue: '5' },
  ],
  routeIds: null,
  ...change,
});

function Path() {
  return <div data-testid="path">{useLocation().pathname}</div>;
}

// Mounted like App does, under a splat route.
function open(path: string) {
  const { Route, Routes } = require('react-router-dom');
  return render(
    <MemoryRouter initialEntries={[path]}>
      <Routes>
        <Route path="/conversions/*" element={<Conversions />} />
      </Routes>
      <Path />
    </MemoryRouter>
  );
}

const path = () => screen.getByTestId('path').textContent;
const button = (name: string | RegExp) => screen.getByRole('button', { name });
const type = (label: string, value: string) =>
  fireEvent.change(screen.getByLabelText(label), { target: { value } });

beforeEach(() => {
  jest.resetAllMocks();
  mockRole = 'Admin';
  mockWorkspaces = null;
  mockShowConfirm.mockResolvedValue(true);
  api.list.mockResolvedValue([conversion()]);
  api.get.mockResolvedValue(conversion());
  api.domains.mockResolvedValue([{ id: 'd1', name: 'go.example.com', isShared: false, verified: true }]);
  api.events.mockResolvedValue({ items: [], hasMore: false });
  (apiService.routes.list as jest.Mock).mockResolvedValue({
    data: [{ id: 'r1', link: 'promo', domain: { name: 'go.example.com' } }],
    pagination: { page: 1, pageSize: 50, totalCount: 1, totalPages: 1 },
  });
});

describe('list', () => {
  it('shows conversions and the actions their status allows', async () => {
    api.list.mockResolvedValue([
      conversion({ id: 'a', name: 'A', status: 'Draft' }),
      conversion({ id: 'b', name: 'B', status: 'Active', routeIds: ['r1', 'r2'] }),
      conversion({ id: 'c', name: 'C', status: 'Paused' }),
      conversion({ id: 'd', name: 'D', status: 'Archived' }),
    ]);
    open('/conversions');
    const actions = (name: string) =>
      within(screen.getByText(name).closest('tr')!)
        .queryAllByRole('button')
        .map((b) => b.textContent);
    await screen.findByText('A');
    expect(actions('A')).toEqual(['Activate', 'Archive']);
    expect(actions('B')).toEqual(['Pause', 'Archive']);
    expect(actions('C')).toEqual(['Activate', 'Archive']);
    expect(actions('D')).toEqual([]);
    expect(screen.getAllByText('Cart → Sale')).toHaveLength(4);
  });

  it('activates after warning about destination URLs', async () => {
    api.changeStatus.mockResolvedValue(conversion({ status: 'Activating' }));
    open('/conversions');
    fireEvent.click(await screen.findByRole('button', { name: 'Activate' }));
    expect(await screen.findByText('Activating')).toBeInTheDocument();
    expect(mockShowConfirm.mock.calls[0][0]).toContain('shortas_cid');
    expect(api.changeStatus).toHaveBeenCalledWith('w1', 'c1', 'activate');
  });

  it('does nothing when the confirmation is declined', async () => {
    mockShowConfirm.mockResolvedValue(false);
    open('/conversions');
    fireEvent.click(await screen.findByRole('button', { name: 'Archive' }));
    await waitFor(() => expect(mockShowConfirm).toHaveBeenCalled());
    expect(api.changeStatus).not.toHaveBeenCalled();
  });

  it('states why activation was refused', async () => {
    api.changeStatus.mockRejectedValue({ response: { status: 422, data: { message: 'The short domain is not verified' } } });
    open('/conversions');
    fireEvent.click(await screen.findByRole('button', { name: 'Activate' }));
    await waitFor(() => expect(mockShowToast).toHaveBeenCalledWith('The short domain is not verified', 'error'));
  });

  it('refreshes until an activating conversion is active', async () => {
    jest.useFakeTimers();
    try {
      api.list.mockResolvedValueOnce([conversion({ status: 'Activating' })]);
      api.list.mockResolvedValue([conversion({ status: 'Active' })]);
      open('/conversions');
      await screen.findByText('Activating');
      jest.advanceTimersByTime(4000);
      expect(await screen.findByText('Active')).toBeInTheDocument();
    } finally {
      jest.useRealTimers();
    }
  });

  it('lets Members look but not change', async () => {
    mockRole = 'Member';
    open('/conversions');
    expect(await screen.findByText('Purchase')).toBeInTheDocument();
    expect(screen.queryByRole('button')).toBeNull();
    expect(screen.queryByText('New Conversion')).toBeNull();
    expect(screen.queryByText('Edit')).toBeNull();
  });

  it.each(['/conversions/new', '/conversions/c1/edit'])('sends Members away from %s', async (url) => {
    mockRole = 'Member';
    open(url);
    await waitFor(() => expect(path()).toBe('/conversions'));
    await screen.findByText('Purchase');
  });

  it('reports a failure, and asks for a workspace when there is none', async () => {
    api.list.mockRejectedValue(new Error('x'));
    jest.spyOn(console, 'error').mockImplementation(() => {});
    const { unmount } = open('/conversions');
    expect(await screen.findByText('Failed to load conversions.')).toBeInTheDocument();
    unmount();
    mockWorkspaces = [];
    open('/conversions');
    expect(screen.getByText('No workspace')).toBeInTheDocument();
  });
});

describe('editor', () => {
  it('names invalid fields and sends nothing', async () => {
    open('/conversions/new');
    await screen.findByText('New conversion');
    type('Attribution window (days)', '91');
    type('Currency', 'E');
    type('Step 1 key', 'Bad Key');
    type('Step 1 default value', '1.23456');
    type('Allowed origins, one per line', 'https://shop.example/path');
    fireEvent.click(button('Create'));
    expect(screen.getAllByRole('alert').map((a) => a.textContent)).toEqual([
      'Enter a name of up to 100 characters',
      'Enter a currency code, such as EUR',
      'Enter 1 to 90 days',
      'Enter a name',
      'Use a-z, 0-9, - and _',
      'Enter a number with up to 4 decimals',
      'Enter up to 20 origins such as https://shop.example, without a path',
    ]);
    expect(api.create).not.toHaveBeenCalled();
  });

  it('validates what the API validates', () => {
    const base = conversion();
    const fields = (change: Partial<ConversionDto>) => Object.keys(validate({ ...base, ...change }));
    expect(fields({})).toEqual([]);
    expect(fields({ name: ' ' })).toEqual(['name']);
    expect(fields({ steps: [] })).toEqual(['steps']);
    expect(fields({ steps: Array.from({ length: 21 }, (_, i) => ({ key: `k${i}`, name: 'n' })) })).toEqual(['steps']);
    expect(fields({ steps: [{ key: 'a', name: 'A' }, { key: 'a', name: 'B' }] })).toEqual(['steps[1].key']);
    expect(fields({ windowDays: 0 })).toEqual(['windowDays']);
    expect(fields({ windowDays: 1.5 })).toEqual(['windowDays']);
    expect(fields({ windowDays: NaN })).toEqual(['windowDays']);
    expect(fields({ routeIds: [] })).toEqual(['routeIds']);
    expect(fields({ allowedOrigins: ['ftp://a.example'] })).toEqual(['allowedOrigins']);
    expect(fields({ allowedOrigins: ['http://localhost:3000'] })).toEqual([]);
    expect(fields({ steps: [{ key: 'a', name: 'A', defaultValue: '-1' }] })).toEqual(['steps[0].defaultValue']);
  });

  it('creates with ordered steps and a route filter, then shows the secret once', async () => {
    api.create.mockResolvedValue({ ...conversion(), secretKey: SECRET });
    open('/conversions/new');
    await screen.findByText('New conversion');
    type('Name', ' Purchase ');
    type('Step 1 name', 'Sale');
    type('Step 1 key', 'sale');
    type('Step 1 default value', '5.25');
    fireEvent.click(button('Add step'));
    type('Step 2 name', 'Cart');
    type('Step 2 key', 'cart');
    fireEvent.click(button('Move step 2 up'));
    fireEvent.click(screen.getByLabelText('Only chosen routes'));
    fireEvent.click(await screen.findByLabelText('go.example.com/promo'));
    type('Allowed origins, one per line', 'https://shop.example\n\n');
    fireEvent.click(button('Create'));

    expect(await screen.findByText(SECRET)).toBeInTheDocument();
    expect(api.create).toHaveBeenCalledWith('w1', {
      name: 'Purchase',
      currency: 'EUR',
      windowDays: 30,
      counting: 'OncePerClick',
      domainId: null,
      allowedOrigins: ['https://shop.example'],
      serverValuesOnly: false,
      steps: [{ key: 'cart', name: 'Cart' }, { key: 'sale', name: 'Sale', defaultValue: '5.25' }],
      routeIds: ['r1'],
    });
    fireEvent.click(button('I have copied it'));
    await waitFor(() => expect(path()).toBe('/conversions/c1'));
    await screen.findByText('1. Script');
    expect(document.body.textContent).not.toContain(SECRET);
  });

  it('keeps saved keys, sends the edited version and chooses a domain', async () => {
    api.get.mockResolvedValue(conversion({ domainId: null }));
    api.update.mockResolvedValue(conversion());
    open('/conversions/c1/edit');
    expect(await screen.findByLabelText('Step 1 key')).toBeDisabled();
    fireEvent.click(button('Add step'));
    expect(screen.getByLabelText('Step 3 key')).toBeEnabled();
    fireEvent.click(button('Remove step 3'));
    fireEvent.click(button(/Choose later/));
    fireEvent.click(await screen.findByText('go.example.com'));
    fireEvent.click(button('Save'));
    await waitFor(() => expect(path()).toBe('/conversions/c1'));
    const [workspace, id, version, input] = api.update.mock.calls[0];
    expect([workspace, id, version]).toEqual(['w1', 'c1', 3]);
    expect(input).toMatchObject({ domainId: 'd1', routeIds: null });
    expect(input.steps).toEqual([{ key: 'cart', name: 'Cart' }, { key: 'sale', name: 'Sale', defaultValue: '5' }]);
    await screen.findByText('1. Script');
  });

  it('shows the fields and conflicts the API reports', async () => {
    api.update.mockRejectedValueOnce({
      response: { status: 400, data: { message: 'Invalid fields: currency', details: 'currency' } },
    });
    open('/conversions/c1/edit');
    fireEvent.click(await screen.findByRole('button', { name: 'Save' }));
    expect(await screen.findByText('Invalid value')).toBeInTheDocument();
    api.update.mockRejectedValueOnce({ response: { status: 409, data: { message: 'Changed by someone else' } } });
    fireEvent.click(button('Save'));
    await waitFor(() => expect(mockShowToast).toHaveBeenCalledWith('Changed by someone else', 'error'));
    expect(path()).toBe('/conversions/c1/edit');
  });

  it('does not edit archived conversions', async () => {
    api.get.mockResolvedValue(conversion({ status: 'Archived' }));
    open('/conversions/c1/edit');
    expect(await screen.findByText('An archived conversion cannot be changed.')).toBeInTheDocument();
  });
});

describe('install view', () => {
  it('uses the domain and public key, and no snippet contains the secret', async () => {
    api.rotateSecret.mockResolvedValue({ ...conversion({ version: 4 }), secretKey: SECRET });
    open('/conversions/c1');
    await waitFor(() => expect(screen.getByLabelText('script').textContent).toContain(
      'https://go.example.com/.well-known/shortas/c/pk_abc.js'
    ));
    expect(screen.getByLabelText('step calls').textContent).toContain('shortas.track("pk_abc", "cart");');

    fireEvent.click(button('Rotate secret key'));
    expect(await screen.findByText(SECRET)).toBeInTheDocument();
    // Also while the new key is on the screen.
    for (const label of ['script', 'step calls', 'server request', 'refund request']) {
      const code = screen.getByLabelText(label).textContent!;
      expect(code).not.toContain(SECRET);
      expect(code).not.toContain('sk_');
    }
    fireEvent.click(button('I have copied it'));
    expect(document.body.textContent).not.toContain(SECRET);
    expect(JSON.stringify({ ...localStorage, ...sessionStorage })).not.toContain(SECRET);
  });

  it('never puts a secret in a snippet, whatever it is given', () => {
    const withSecret = { ...conversion(), secretKey: SECRET } as ConversionDto;
    for (const code of [
      scriptSnippet(withSecret, 'go.example.com'),
      stepSnippet(withSecret),
      serverSnippet(withSecret, 'go.example.com'),
      refundSnippet(withSecret, 'go.example.com'),
    ]) {
      expect(code).not.toContain(SECRET);
    }
    expect(stepSnippet(conversion({ serverValuesOnly: true }))).not.toContain('value:');
  });

  it('sends a test event and shows the log', async () => {
    api.sendTestEvent.mockResolvedValue();
    open('/conversions/c1');
    await screen.findByText('1. Script');
    await waitFor(() => expect(button('Send test event')).toBeEnabled());
    fireEvent.click(button('Send test event'));
    expect(await screen.findByText('No events yet')).toBeInTheDocument();
    expect(api.sendTestEvent).toHaveBeenCalledWith('go.example.com', 'pk_abc', 'cart');
  });

  it('sends tests whatever origins are allowed', async () => {
    api.get.mockResolvedValue(conversion({ allowedOrigins: ['https://shop.example'] }));
    api.sendTestEvent.mockResolvedValue();
    open('/conversions/c1');
    await waitFor(() => expect(button('Send test event')).toBeEnabled());
    expect(screen.queryByText(/allowed origins|other origins/)).toBeNull();
    fireEvent.click(button('Send test event'));
    await waitFor(() => expect(api.sendTestEvent).toHaveBeenCalled());
  });

  it('asks for a domain, and Members cannot rotate', async () => {
    mockRole = 'Member';
    api.get.mockResolvedValue(conversion({ domainId: null }));
    open('/conversions/c1');
    expect(await screen.findByText(/Choose a short domain/)).toBeInTheDocument();
    expect(button('Send test event')).toBeDisabled();
    for (const name of ['Rotate secret key', 'Activate', 'Archive']) {
      expect(screen.queryByRole('button', { name })).toBeNull();
    }
  });
});

describe('event log', () => {
  const item = (id: string, change: Partial<ConversionLogItem>): ConversionLogItem => ({
    id,
    kind: 'event',
    status: 'attributed',
    reason: '',
    field: '',
    source: 'browser',
    test: false,
    step: 'sale',
    eventId: id,
    externalId: id,
    refundId: '',
    clickId: '',
    routeId: '',
    value: '49.5',
    applied: null,
    currency: 'EUR',
    occurredAt: '2026-11-25T19:00:00Z',
    receivedAt: '2026-11-25T19:00:00Z',
    ...change,
  });
  const row = (id: string) => screen.getByText(id).closest('tr')!.textContent;

  async function openLog() {
    open('/conversions/c1');
    fireEvent.click(await screen.findByRole('tab', { name: 'Events' }));
  }

  it('shows attributed, unattributed, rejected, pending, refunded and test rows', async () => {
    api.events.mockResolvedValue({
      hasMore: false,
      items: [
        item('ev1', {}),
        item('ev2', { status: 'unattributed', reason: 'no_click_id', source: 'server' }),
        item('ev3', { status: 'rejected', reason: 'origin_not_allowed' }),
        item('ev4', { status: 'rejected', reason: 'invalid', field: 'value', value: null }),
        item('ev5', { status: 'pending' }),
        item('ev6', { status: 'unattributed', reason: 'no_click_id', test: true }),
        item('ev7', { kind: 'refund', status: 'invalid', reason: 'exceeds_original', value: '40', applied: '29.5' }),
        item('ev8', { status: 'rejected', reason: 'something_new' }),
      ],
    });
    await openLog();
    await screen.findByText('ev1');
    expect(row('ev1')).toMatch(/Saleattributed49.5 EURbrowser/);
    expect(row('ev2')).toMatch(/unattributedNo click ID49.5 EURserver/);
    expect(row('ev3')).toContain('rejectedOrigin is not allowed');
    expect(row('ev4')).toContain('rejectedInvalid value');
    expect(row('ev5')).toContain('pendingWaiting for its click');
    expect(row('ev6')).toContain('unattributedtest');
    expect(row('ev1')).not.toContain('test');
    expect(row('ev7')).toContain('RefundinvalidMore than was left to refund-40 EUR (29.5 applied)');
    expect(row('ev8')).toContain('something_new');
  });

  it('loads more without repeating rows', async () => {
    api.events.mockResolvedValueOnce({ hasMore: true, items: [item('ev1', {}), item('ev2', {})] });
    await openLog();
    await screen.findByText('ev2');
    api.events.mockResolvedValueOnce({ hasMore: false, items: [item('ev2', {}), item('ev3', {})] });
    fireEvent.click(button('Load more'));
    await screen.findByText('ev3');
    expect(api.events).toHaveBeenLastCalledWith('w1', 'c1', 50, 2);
    expect(screen.getAllByText(/^ev/)).toHaveLength(3);
    expect(screen.queryByText('Load more')).toBeNull();
  });

  it('reports a failure', async () => {
    jest.spyOn(console, 'error').mockImplementation(() => {});
    api.events.mockRejectedValue(new Error('x'));
    await openLog();
    expect(await screen.findByText('Failed to load events.')).toBeInTheDocument();
  });
});
