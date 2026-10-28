import { render, screen, fireEvent, waitFor } from '@testing-library/react';
import RouteFormModal from './RouteFormModal';
import { apiService } from '../services/api';

jest.mock('../services/api', () => ({
  apiService: {
    domains: {
      list: jest.fn(),
      listShared: jest.fn(),
    },
  },
}));

jest.mock('./RouteForm', () => ({
  __esModule: true,
  default: ({ domains, onCancel }: any) => (
    <div data-testid="route-form">
      <span data-testid="domain-count">{domains.length}</span>
      {domains.map((d: any) => (
        <span key={d.id} data-testid="domain-name">{d.name}</span>
      ))}
      <button onClick={onCancel}>cancel-form</button>
    </div>
  ),
}));

const mockList = apiService.domains.list as jest.Mock;
const mockListShared = apiService.domains.listShared as jest.Mock;

describe('RouteFormModal', () => {
  beforeEach(() => {
    mockList.mockReset();
    mockListShared.mockReset();
  });

  it('renders nothing when show is false', () => {
    render(<RouteFormModal show={false} onClose={jest.fn()} onSave={jest.fn()} />);
    expect(screen.queryByTestId('route-form')).not.toBeInTheDocument();
  });

  it('fetches and combines shared domains before user domains', async () => {
    mockList.mockResolvedValue({ data: [{ id: 'd1', name: 'mine.com' }], pagination: {} as any });
    mockListShared.mockResolvedValue([{ id: 'd2', name: 'shared.com' }]);

    render(<RouteFormModal show={true} onClose={jest.fn()} onSave={jest.fn()} />);

    await waitFor(() => expect(screen.getByTestId('domain-count')).toHaveTextContent('2'));
    const names = screen.getAllByTestId('domain-name').map((el) => el.textContent);
    expect(names).toEqual(['shared.com', 'mine.com']);
  });

  it('treats a failed shared-domains fetch as an empty list rather than failing entirely', async () => {
    mockList.mockResolvedValue({ data: [{ id: 'd1', name: 'mine.com' }], pagination: {} as any });
    mockListShared.mockRejectedValue(new Error('forbidden'));

    render(<RouteFormModal show={true} onClose={jest.fn()} onSave={jest.fn()} />);

    await waitFor(() => expect(screen.getByTestId('domain-count')).toHaveTextContent('1'));
    expect(screen.getByTestId('domain-name')).toHaveTextContent('mine.com');
  });

  it('logs and shows no domains when the user-domains fetch itself fails', async () => {
    const consoleSpy = jest.spyOn(console, 'error').mockImplementation(() => {});
    mockList.mockRejectedValue(new Error('network down'));
    mockListShared.mockResolvedValue([]);

    render(<RouteFormModal show={true} onClose={jest.fn()} onSave={jest.fn()} />);

    await waitFor(() => expect(consoleSpy).toHaveBeenCalledWith('Failed to fetch domains:', expect.any(Error)));
    expect(screen.getByTestId('domain-count')).toHaveTextContent('0');
    consoleSpy.mockRestore();
  });

  it('closes when the overlay is clicked but not when the content is clicked', async () => {
    mockList.mockResolvedValue({ data: [], pagination: {} as any });
    mockListShared.mockResolvedValue([]);
    const onClose = jest.fn();

    const { container } = render(<RouteFormModal show={true} onClose={onClose} onSave={jest.fn()} />);
    await waitFor(() => expect(screen.getByTestId('route-form')).toBeInTheDocument());

    fireEvent.click(screen.getByTestId('route-form'));
    expect(onClose).not.toHaveBeenCalled();

    fireEvent.click(container.querySelector('.modal-overlay')!);
    expect(onClose).toHaveBeenCalledTimes(1);
  });

  it('passes onClose through to the form as onCancel', async () => {
    mockList.mockResolvedValue({ data: [], pagination: {} as any });
    mockListShared.mockResolvedValue([]);
    const onClose = jest.fn();

    render(<RouteFormModal show={true} onClose={onClose} onSave={jest.fn()} />);
    await waitFor(() => expect(screen.getByTestId('route-form')).toBeInTheDocument());

    fireEvent.click(screen.getByText('cancel-form'));
    expect(onClose).toHaveBeenCalledTimes(1);
  });
});
