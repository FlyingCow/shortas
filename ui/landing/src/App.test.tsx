import React from 'react';
import { render, screen } from '@testing-library/react';
import App from './App';

test('renders the Shortas hero heading', () => {
  render(<App />);
  const heading = screen.getByRole('heading', { level: 1 });
  expect(heading).toHaveTextContent(/built for speed/i);
});
