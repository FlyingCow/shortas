import { render, screen, fireEvent, waitFor, act } from '@testing-library/react';
import { renderHook } from '@testing-library/react';
import { AlertProvider, useAlert } from './AlertContext';

function Harness({ onConfirmResult }: { onConfirmResult?: (result: boolean) => void }) {
  const { showAlert, showConfirm, showToast } = useAlert();
  return (
    <div>
      <button onClick={() => showAlert('Something happened', 'Heads up')}>trigger-alert</button>
      <button
        onClick={async () => {
          const result = await showConfirm('Are you sure?', 'Confirm it', { variant: 'danger' });
          onConfirmResult?.(result);
        }}
      >
        trigger-confirm
      </button>
      <button onClick={() => showToast('Saved successfully', 'success')}>trigger-toast</button>
    </div>
  );
}

describe('AlertContext', () => {
  it('throws when useAlert is used outside an AlertProvider', () => {
    const spy = jest.spyOn(console, 'error').mockImplementation(() => {});
    expect(() => renderHook(() => useAlert())).toThrow(/must be used within AlertProvider/);
    spy.mockRestore();
  });

  it('showAlert renders a dialog with the given title and message, closed by OK', () => {
    render(
      <AlertProvider>
        <Harness />
      </AlertProvider>
    );

    fireEvent.click(screen.getByText('trigger-alert'));

    expect(screen.getByRole('dialog')).toBeInTheDocument();
    expect(screen.getByText('Heads up')).toBeInTheDocument();
    expect(screen.getByText('Something happened')).toBeInTheDocument();

    fireEvent.click(screen.getByRole('button', { name: 'OK' }));
    expect(screen.queryByRole('dialog')).not.toBeInTheDocument();
  });

  it('clicking the overlay closes an alert but not a confirm dialog', () => {
    render(
      <AlertProvider>
        <Harness />
      </AlertProvider>
    );

    fireEvent.click(screen.getByText('trigger-alert'));
    fireEvent.click(screen.getByRole('dialog'));
    expect(screen.queryByRole('dialog')).not.toBeInTheDocument();

    fireEvent.click(screen.getByText('trigger-confirm'));
    fireEvent.click(screen.getByRole('dialog'));
    expect(screen.getByRole('dialog')).toBeInTheDocument();
  });

  it('showConfirm resolves true when Confirm is clicked, using custom labels and danger styling', async () => {
    const onConfirmResult = jest.fn();
    render(
      <AlertProvider>
        <Harness onConfirmResult={onConfirmResult} />
      </AlertProvider>
    );

    fireEvent.click(screen.getByText('trigger-confirm'));

    const confirmButton = screen.getByRole('button', { name: 'Confirm' });
    expect(confirmButton.className).toContain('btn-danger');

    fireEvent.click(confirmButton);

    await waitFor(() => expect(onConfirmResult).toHaveBeenCalledWith(true));
    expect(screen.queryByRole('dialog')).not.toBeInTheDocument();
  });

  it('showConfirm resolves false when Cancel is clicked', async () => {
    const onConfirmResult = jest.fn();
    render(
      <AlertProvider>
        <Harness onConfirmResult={onConfirmResult} />
      </AlertProvider>
    );

    fireEvent.click(screen.getByText('trigger-confirm'));
    fireEvent.click(screen.getByRole('button', { name: 'Cancel' }));

    await waitFor(() => expect(onConfirmResult).toHaveBeenCalledWith(false));
  });

  it('Escape key closes the dialog and resolves confirm false', async () => {
    const onConfirmResult = jest.fn();
    render(
      <AlertProvider>
        <Harness onConfirmResult={onConfirmResult} />
      </AlertProvider>
    );

    fireEvent.click(screen.getByText('trigger-confirm'));
    fireEvent.keyDown(document, { key: 'Escape' });

    await waitFor(() => expect(onConfirmResult).toHaveBeenCalledWith(false));
    expect(screen.queryByRole('dialog')).not.toBeInTheDocument();
  });

  it('Enter key confirms the dialog', async () => {
    const onConfirmResult = jest.fn();
    render(
      <AlertProvider>
        <Harness onConfirmResult={onConfirmResult} />
      </AlertProvider>
    );

    fireEvent.click(screen.getByText('trigger-confirm'));
    fireEvent.keyDown(document, { key: 'Enter' });

    await waitFor(() => expect(onConfirmResult).toHaveBeenCalledWith(true));
  });

  it('showToast renders a toast that auto-dismisses after 4 seconds', () => {
    jest.useFakeTimers();
    render(
      <AlertProvider>
        <Harness />
      </AlertProvider>
    );

    fireEvent.click(screen.getByText('trigger-toast'));
    expect(screen.getByText('Saved successfully')).toBeInTheDocument();

    act(() => {
      jest.advanceTimersByTime(4000);
    });
    expect(screen.queryByText('Saved successfully')).not.toBeInTheDocument();

    jest.useRealTimers();
  });

  it('toast close button dismisses it immediately', () => {
    render(
      <AlertProvider>
        <Harness />
      </AlertProvider>
    );

    fireEvent.click(screen.getByText('trigger-toast'));
    fireEvent.click(screen.getByRole('button', { name: 'Close' }));

    expect(screen.queryByText('Saved successfully')).not.toBeInTheDocument();
  });
});
