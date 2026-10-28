import { render, screen, fireEvent, waitFor } from '@testing-library/react';
import QRCodeDesigner from './QRCodeDesigner';
import { apiService } from '../services/api';
import QRCodeStyling from 'qr-code-styling';

let mockQrInstance: any;

// CRA's Jest config sets `resetMocks: true`, which wipes any
// `mockImplementation` set inside a `jest.mock(...)` factory before the
// first test even runs - so the constructor mock's behavior is (re)applied
// in `beforeEach` instead of baked into the factory here.
jest.mock('qr-code-styling', () => jest.fn());

const mockShowToast = jest.fn();
jest.mock('../contexts/AlertContext', () => ({
  useAlert: () => ({ showToast: mockShowToast }),
}));

jest.mock('../services/api', () => ({
  apiService: {
    routes: {
      qr: {
        getSettings: jest.fn(),
        updateSettings: jest.fn(),
        getUploadUrl: jest.fn(),
        getLogoUploadUrl: jest.fn(),
      },
    },
  },
}));

jest.mock('../config/runtimeEnv', () => ({
  getRouteImagesBaseUrl: () => 'https://images.example.com',
}));

const mockGetSettings = apiService.routes.qr.getSettings as jest.Mock;
const mockUpdateSettings = apiService.routes.qr.updateSettings as jest.Mock;
const mockGetUploadUrl = apiService.routes.qr.getUploadUrl as jest.Mock;
const mockGetLogoUploadUrl = apiService.routes.qr.getLogoUploadUrl as jest.Mock;

describe('QRCodeDesigner', () => {
  beforeEach(() => {
    mockQrInstance = undefined;
    (QRCodeStyling as unknown as jest.Mock).mockImplementation(() => {
      mockQrInstance = {
        append: jest.fn(),
        update: jest.fn(),
        getRawData: jest.fn().mockResolvedValue(new Blob(['<svg></svg>'], { type: 'image/svg+xml' })),
        download: jest.fn().mockResolvedValue(undefined),
      };
      return mockQrInstance;
    });
    global.URL.createObjectURL = jest.fn(() => 'blob:fake-url');
    global.URL.revokeObjectURL = jest.fn();
    global.fetch = jest.fn().mockResolvedValue({ ok: true });
  });

  it('defaults content to the example URL, editable when no url prop is given', () => {
    render(<QRCodeDesigner />);
    const input = screen.getByPlaceholderText('https://...') as HTMLInputElement;
    expect(input.value).toBe('https://example.com');
    expect(input).not.toHaveAttribute('readonly');

    fireEvent.change(input, { target: { value: 'https://custom.example.com' } });
    expect(input.value).toBe('https://custom.example.com');
  });

  it('uses the provided url as fixed, read-only content', () => {
    render(<QRCodeDesigner url="https://fixed.example.com" />);
    const input = screen.getByPlaceholderText('https://...') as HTMLInputElement;
    expect(input.value).toBe('https://fixed.example.com');
    expect(input).toHaveAttribute('readonly');
  });

  it('marks the selected dot style as active', () => {
    render(<QRCodeDesigner />);
    expect(screen.getByRole('button', { name: 'Rounded' }).className).toContain('active');

    fireEvent.click(screen.getByRole('button', { name: 'Dots' }));
    expect(screen.getByRole('button', { name: 'Dots' }).className).toContain('active');
    expect(screen.getByRole('button', { name: 'Rounded' }).className).not.toContain('active');
  });

  it('clicking a preset color updates the foreground color text input', () => {
    render(<QRCodeDesigner />);
    fireEvent.click(screen.getByTitle('#3b82f6'));

    const colorTextInputs = screen.getAllByDisplayValue('#3b82f6');
    expect(colorTextInputs.length).toBeGreaterThan(0);
  });

  it('updates the size label when the slider changes', () => {
    render(<QRCodeDesigner />);
    expect(screen.getByText('280px')).toBeInTheDocument();

    const slider = document.querySelector('.qr-size-slider') as HTMLInputElement;
    fireEvent.change(slider, { target: { value: '400' } });
    expect(screen.getByText('400px')).toBeInTheDocument();
  });

  it('rejects a non-image file with a warning toast', () => {
    render(<QRCodeDesigner />);
    const fileInput = document.querySelector('.qr-file-input') as HTMLInputElement;
    const textFile = new File(['hello'], 'notes.txt', { type: 'text/plain' });

    fireEvent.change(fileInput, { target: { files: [textFile] } });

    expect(mockShowToast).toHaveBeenCalledWith('Please select an image file (PNG, JPG, SVG, etc.).', 'warning');
    expect(screen.queryByText('notes.txt')).not.toBeInTheDocument();
  });

  it('accepts an image file, shows its name and a Remove button, and clears it on Remove', () => {
    render(<QRCodeDesigner />);
    const fileInput = document.querySelector('.qr-file-input') as HTMLInputElement;
    const imageFile = new File(['fake'], 'logo.png', { type: 'image/png' });

    fireEvent.change(fileInput, { target: { files: [imageFile] } });

    expect(screen.getByText('logo.png')).toBeInTheDocument();
    expect(global.URL.createObjectURL).toHaveBeenCalledWith(imageFile);

    fireEvent.click(screen.getByRole('button', { name: /remove/i }));
    expect(global.URL.revokeObjectURL).toHaveBeenCalledWith('blob:fake-url');
    expect(screen.queryByText('logo.png')).not.toBeInTheDocument();
  });

  it('has no Save button when no routeId is given', () => {
    render(<QRCodeDesigner />);
    expect(screen.queryByRole('button', { name: /^save$/i })).not.toBeInTheDocument();
  });

  it('loads existing settings for a routeId and shows a loading state while in flight', async () => {
    let resolveSettings: (v: any) => void;
    mockGetSettings.mockReturnValue(new Promise((resolve) => { resolveSettings = resolve; }));

    render(<QRCodeDesigner routeId="r1" />);
    expect(screen.getByText('Loading settings...')).toBeInTheDocument();

    resolveSettings!({ dotStyle: 'dots', fgColor: '#111111', bgColor: '#eeeeee', size: 320 });
    await waitFor(() => expect(screen.queryByText('Loading settings...')).not.toBeInTheDocument());

    expect(screen.getByRole('button', { name: 'Dots' }).className).toContain('active');
    expect(screen.getByText('320px')).toBeInTheDocument();
  });

  it('falls back to defaults when loading settings fails', async () => {
    mockGetSettings.mockRejectedValue(new Error('not found'));
    render(<QRCodeDesigner routeId="r1" />);

    await waitFor(() => expect(screen.queryByText('Loading settings...')).not.toBeInTheDocument());
    expect(screen.getByRole('button', { name: 'Rounded' }).className).toContain('active');
  });

  it('saves settings (no logo change): uploads the QR SVG and calls updateSettings, showing a success toast', async () => {
    mockGetSettings.mockResolvedValue({ dotStyle: 'rounded', fgColor: '#000000', bgColor: '#ffffff', size: 280 });
    mockGetUploadUrl.mockResolvedValue({ url: 'https://upload.example.com/qr', key: 'qr-key' });
    mockUpdateSettings.mockResolvedValue(undefined);

    render(<QRCodeDesigner routeId="r1" />);
    await waitFor(() => expect(screen.queryByText('Loading settings...')).not.toBeInTheDocument());

    fireEvent.click(screen.getByRole('button', { name: /^save$/i }));

    await waitFor(() => expect(mockUpdateSettings).toHaveBeenCalled());
    expect(mockGetLogoUploadUrl).not.toHaveBeenCalled();
    expect(global.fetch).toHaveBeenCalledWith(
      'https://upload.example.com/qr',
      expect.objectContaining({ method: 'PUT' })
    );
    expect(mockUpdateSettings).toHaveBeenCalledWith('r1', expect.objectContaining({
      dotStyle: 'rounded',
      fgColor: '#000000',
      bgColor: '#ffffff',
      size: 280,
    }));
    expect(mockShowToast).toHaveBeenCalledWith('QR code settings saved successfully', 'success');
  });

  it('saves settings with a changed logo: uploads the logo first and derives centerImageUrl from the upload key', async () => {
    mockGetSettings.mockResolvedValue({ dotStyle: 'rounded', fgColor: '#000000', bgColor: '#ffffff', size: 280 });
    mockGetLogoUploadUrl.mockResolvedValue({ url: 'https://upload.example.com/logo', key: 'owner1/route1/logo.png' });
    mockGetUploadUrl.mockResolvedValue({ url: 'https://upload.example.com/qr', key: 'qr-key' });
    mockUpdateSettings.mockResolvedValue(undefined);

    render(<QRCodeDesigner routeId="r1" />);
    await waitFor(() => expect(screen.queryByText('Loading settings...')).not.toBeInTheDocument());

    const fileInput = document.querySelector('.qr-file-input') as HTMLInputElement;
    const imageFile = new File(['fake'], 'logo.png', { type: 'image/png' });
    fireEvent.change(fileInput, { target: { files: [imageFile] } });

    fireEvent.click(screen.getByRole('button', { name: /^save$/i }));

    await waitFor(() => expect(mockUpdateSettings).toHaveBeenCalled());
    expect(mockGetLogoUploadUrl).toHaveBeenCalledWith('r1', 'image/png');
    expect(global.fetch).toHaveBeenCalledWith(
      'https://upload.example.com/logo',
      expect.objectContaining({ method: 'PUT', body: imageFile })
    );
    expect(mockUpdateSettings).toHaveBeenCalledWith(
      'r1',
      expect.objectContaining({ centerImageUrl: 'https://images.example.com/route-images/owner1/route1/logo.png' })
    );
  });

  it('shows an error toast and logs when saving fails', async () => {
    const consoleSpy = jest.spyOn(console, 'error').mockImplementation(() => {});
    mockGetSettings.mockResolvedValue({ dotStyle: 'rounded', fgColor: '#000000', bgColor: '#ffffff', size: 280 });
    mockGetUploadUrl.mockRejectedValue(new Error('upload failed'));

    render(<QRCodeDesigner routeId="r1" />);
    await waitFor(() => expect(screen.queryByText('Loading settings...')).not.toBeInTheDocument());

    fireEvent.click(screen.getByRole('button', { name: /^save$/i }));

    await waitFor(() => expect(mockShowToast).toHaveBeenCalledWith('Failed to save QR code settings', 'error'));
    expect(consoleSpy).toHaveBeenCalled();
    consoleSpy.mockRestore();
  });

  it('download button triggers the QR instance download', async () => {
    render(<QRCodeDesigner />);
    await waitFor(() => expect(mockQrInstance).toBeDefined());

    fireEvent.click(screen.getByRole('button', { name: /download svg/i }));
    await waitFor(() => expect(mockQrInstance.download).toHaveBeenCalledWith({ name: 'qrcode', extension: 'svg' }));
  });
});
