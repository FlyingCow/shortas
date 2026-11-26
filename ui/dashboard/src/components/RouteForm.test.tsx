import { render, screen, fireEvent, waitFor } from '@testing-library/react';
import RouteForm from './RouteForm';
import { RouteDto } from '../services/api';

jest.mock('../services/api', () => ({ apiService: {} }));
jest.mock('../contexts/WorkspaceContext', () => ({ useWorkspace: () => ({ currentWorkspaceId: 'w1' }) }));

const route = (properties: object): RouteDto =>
  ({
    id: 'r1', switch: 'main', link: 'promo', dest: 'https://example.com', destFormat: 'Http', code: 302, ttl: 0,
    status: 'Active', terminal: 'External', policy: 'Basic', domainId: 'd1',
    properties: { routeId: 'r1', domainId: 'd1', ownerId: 'u', scripts: [], tags: [], custom: {}, opengraph: false, allowDebug: false, ...properties },
  }) as RouteDto;

const domains: any[] = [{ id: 'd1', name: 'go.example.com', verificationStatus: 'Verified' }];
const label = 'Exclude from conversion tracking';

function open(properties: object) {
  const onSave = jest.fn().mockResolvedValue(undefined);
  render(<RouteForm route={route(properties)} domains={domains} onSave={onSave} onCancel={jest.fn()} />);
  return onSave;
}

async function save(onSave: jest.Mock) {
  fireEvent.click(screen.getByRole('button', { name: 'Update Route' }));
  await waitFor(() => expect(onSave).toHaveBeenCalled());
  return onSave.mock.calls[0][0].properties.conversionOptOut;
}

describe('conversion opt-out', () => {
  it('is saved when set', async () => {
    const onSave = open({ workspaceId: 'w1' });
    expect(screen.getByLabelText(label)).not.toBeChecked();
    fireEvent.click(screen.getByLabelText(label));
    expect(await save(onSave)).toBe(true);
  });

  it('is displayed and can be cleared', async () => {
    const onSave = open({ workspaceId: 'w1', conversionOptOut: true });
    expect(screen.getByLabelText(label)).toBeChecked();
    fireEvent.click(screen.getByLabelText(label));
    expect(await save(onSave)).toBe(false);
  });

  it('is explained for routes without a workspace', () => {
    open({});
    const explanation = screen.getByText(/belongs to no workspace, so it cannot drive conversions/);
    expect(explanation.closest('details')).toHaveAttribute('open');
    expect(screen.getByLabelText(label)).toBeDisabled();
  });
});
