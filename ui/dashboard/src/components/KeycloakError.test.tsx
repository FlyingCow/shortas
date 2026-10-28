import { render, screen, fireEvent } from '@testing-library/react';
import KeycloakError from './KeycloakError';

describe('KeycloakError', () => {
  it('classifies a network-unreachable error as a server issue', () => {
    render(
      <KeycloakError
        error="Keycloak server is not reachable at http://localhost:8080"
        onRetry={jest.fn()}
        onEnableMockData={jest.fn()}
      />
    );
    expect(screen.getByText('Keycloak Server Not Running')).toBeInTheDocument();
    expect(screen.getByText(/Start Keycloak server on port 8080/)).toBeInTheDocument();
  });

  it('classifies a missing-realm error', () => {
    render(
      <KeycloakError
        error="Keycloak realm 'shortas-dev' not found"
        onRetry={jest.fn()}
        onEnableMockData={jest.fn()}
      />
    );
    expect(screen.getByText('Keycloak Realm Not Found')).toBeInTheDocument();
    expect(screen.getByText(/Create "shortas-dev" realm/)).toBeInTheDocument();
  });

  it('classifies a misconfigured-client error', () => {
    render(
      <KeycloakError
        error="Keycloak client 'shortas-dashboard' is not properly configured"
        onRetry={jest.fn()}
        onEnableMockData={jest.fn()}
      />
    );
    expect(screen.getByText('Keycloak Client Not Configured')).toBeInTheDocument();
    expect(screen.getByText(/Create "shortas-dashboard" client/)).toBeInTheDocument();
  });

  it('falls back to a general authentication error for unrecognized messages', () => {
    render(<KeycloakError error="Something unexpected happened" onRetry={jest.fn()} onEnableMockData={jest.fn()} />);
    expect(screen.getByText('Authentication Error')).toBeInTheDocument();
    expect(screen.getByText(/Check Keycloak server status/)).toBeInTheDocument();
  });

  it('always shows the original error message verbatim', () => {
    render(<KeycloakError error="Something unexpected happened" onRetry={jest.fn()} onEnableMockData={jest.fn()} />);
    expect(screen.getByText('Something unexpected happened')).toBeInTheDocument();
  });

  it('wires up Retry Connection and Use Mock Data Instead buttons', () => {
    const onRetry = jest.fn();
    const onEnableMockData = jest.fn();
    render(<KeycloakError error="Something unexpected happened" onRetry={onRetry} onEnableMockData={onEnableMockData} />);

    fireEvent.click(screen.getByRole('button', { name: /retry connection/i }));
    expect(onRetry).toHaveBeenCalledTimes(1);

    fireEvent.click(screen.getByRole('button', { name: /use mock data instead/i }));
    expect(onEnableMockData).toHaveBeenCalledTimes(1);
  });
});
