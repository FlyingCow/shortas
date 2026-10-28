import { render, screen, fireEvent } from '@testing-library/react';
import Dropdown, { DropdownOption } from './Dropdown';

const options: DropdownOption[] = [
  { value: 'us', label: 'United States', group: 'Americas' },
  { value: 'ca', label: 'Canada', group: 'Americas' },
  { value: 'fr', label: 'France', group: 'Europe' },
  { value: 'de', label: 'Germany', group: 'Europe' },
];

describe('Dropdown', () => {
  it('shows the placeholder when no option matches the current value', () => {
    render(<Dropdown value="" options={options} onChange={jest.fn()} placeholder="Pick one" />);
    expect(screen.getByText('Pick one')).toBeInTheDocument();
  });

  it("shows the selected option's label", () => {
    render(<Dropdown value="fr" options={options} onChange={jest.fn()} />);
    expect(screen.getByText('France')).toBeInTheDocument();
  });

  it('opens the menu when the trigger is clicked, and renders options in a portal', () => {
    render(<Dropdown value="us" options={options} onChange={jest.fn()} />);
    expect(screen.queryByText('Germany')).not.toBeInTheDocument();

    fireEvent.click(screen.getByRole('button', { name: /united states/i }));
    expect(screen.getByText('Germany')).toBeInTheDocument();
  });

  it('shows group headers only when the group changes from the previous option', () => {
    render(<Dropdown value="us" options={options} onChange={jest.fn()} />);
    fireEvent.click(screen.getByRole('button', { name: /united states/i }));

    expect(screen.getAllByText('Americas')).toHaveLength(1);
    expect(screen.getAllByText('Europe')).toHaveLength(1);
  });

  it('calls onChange and closes the menu when an option is selected', () => {
    const onChange = jest.fn();
    render(<Dropdown value="us" options={options} onChange={onChange} />);
    fireEvent.click(screen.getByRole('button', { name: /united states/i }));

    fireEvent.click(screen.getByText('Germany'));

    expect(onChange).toHaveBeenCalledWith('de');
    expect(screen.queryByText('France')).not.toBeInTheDocument();
  });

  it('marks the currently selected option with the active class', () => {
    render(<Dropdown value="fr" options={options} onChange={jest.fn()} />);
    fireEvent.click(screen.getByRole('button', { name: /france/i }));

    const activeOption = document.querySelector('.app-select-item-active');
    expect(activeOption).toHaveTextContent('France');
  });

  it('does not open when disabled', () => {
    render(<Dropdown value="us" options={options} onChange={jest.fn()} disabled />);
    fireEvent.click(screen.getByRole('button', { name: /united states/i }));
    expect(screen.queryByText('Germany')).not.toBeInTheDocument();
  });

  it('shows the empty message when there are no options', () => {
    render(<Dropdown value="" options={[]} onChange={jest.fn()} emptyMessage="Nothing here" />);
    fireEvent.click(screen.getByRole('button'));
    expect(screen.getByText('Nothing here')).toBeInTheDocument();
  });

  it('applies the error styling class to the trigger', () => {
    render(<Dropdown value="us" options={options} onChange={jest.fn()} error />);
    expect(screen.getByRole('button', { name: /united states/i }).className).toContain('app-select-error');
  });

  it('closes the menu on Escape', () => {
    render(<Dropdown value="us" options={options} onChange={jest.fn()} />);
    fireEvent.click(screen.getByRole('button', { name: /united states/i }));
    expect(screen.getByText('Germany')).toBeInTheDocument();

    fireEvent.keyDown(document, { key: 'Escape' });
    expect(screen.queryByText('Germany')).not.toBeInTheDocument();
  });

  it('closes the menu when clicking outside', () => {
    render(
      <div>
        <div data-testid="outside">outside</div>
        <Dropdown value="us" options={options} onChange={jest.fn()} />
      </div>
    );
    fireEvent.click(screen.getByRole('button', { name: /united states/i }));
    expect(screen.getByText('Germany')).toBeInTheDocument();

    fireEvent.mouseDown(screen.getByTestId('outside'));
    expect(screen.queryByText('Germany')).not.toBeInTheDocument();
  });

  it('closes on window resize', () => {
    render(<Dropdown value="us" options={options} onChange={jest.fn()} />);
    fireEvent.click(screen.getByRole('button', { name: /united states/i }));
    expect(screen.getByText('Germany')).toBeInTheDocument();

    fireEvent(window, new Event('resize'));
    expect(screen.queryByText('Germany')).not.toBeInTheDocument();
  });

  it('closes on an outside scroll but stays open for a scroll inside the menu', () => {
    render(<Dropdown value="us" options={options} onChange={jest.fn()} />);
    fireEvent.click(screen.getByRole('button', { name: /united states/i }));
    const menu = screen.getByText('Germany').closest('.app-select-menu')!;

    fireEvent.scroll(menu);
    expect(screen.getByText('Germany')).toBeInTheDocument();

    fireEvent.scroll(window);
    expect(screen.queryByText('Germany')).not.toBeInTheDocument();
  });
});
