import { render, screen, fireEvent, waitFor } from '@testing-library/react';
import Domains from './Domains';
import { apiService } from '../services/api';

jest.mock('../services/api', () => ({
  apiService: { domains: { list: jest.fn(), delete: jest.fn(), listConversions: jest.fn(), getDnsConfig: jest.fn() } },
}));
const mockShowConfirm = jest.fn();
jest.mock('../contexts/AlertContext', () => ({
  useAlert: () => ({ showToast: jest.fn(), showConfirm: mockShowConfirm }),
}));

const api = apiService.domains as jest.Mocked<typeof apiService.domains>;

async function remove(confirmed: boolean) {
  mockShowConfirm.mockResolvedValue(confirmed);
  render(<Domains />);
  await screen.findByText('go.example.com');
  fireEvent.click(screen.getByTitle(/delete/i));
  await waitFor(() => expect(mockShowConfirm).toHaveBeenCalled());
  return mockShowConfirm.mock.calls[0][0];
}

describe('removing a domain', () => {
  beforeEach(() => {
    jest.resetAllMocks();
    api.list.mockResolvedValue({
      data: [{ id: 'd1', name: 'go.example.com', ownerId: 'u', verificationStatus: 'Verified', verificationReason: '' }],
      pagination: { page: 1, pageSize: 100, totalCount: 1, totalPages: 1 },
    });
  });

  it('lists affected conversions and keeps the domain when declined', async () => {
    api.listConversions.mockResolvedValue([{ name: 'Purchase' }, { name: 'Signup' }] as any);
    expect(await remove(false)).toContain('These conversions will stop receiving events: Purchase, Signup.');
    expect(api.listConversions).toHaveBeenCalledWith('d1');
    expect(api.delete).not.toHaveBeenCalled();
  });

  it('does not mention conversions when there are none', async () => {
    api.listConversions.mockResolvedValue([]);
    expect(await remove(true)).not.toContain('onversions');
    await waitFor(() => expect(api.delete).toHaveBeenCalledWith('d1'));
  });

  it('says so when they cannot be checked', async () => {
    jest.spyOn(console, 'error').mockImplementation(() => {});
    api.listConversions.mockRejectedValue(new Error('x'));
    expect(await remove(false)).toContain('could not be checked');
  });
});
