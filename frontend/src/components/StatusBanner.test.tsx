import { render, screen } from '@testing-library/react';
import { describe, expect, it } from 'vitest';
import { StatusBanner } from './StatusBanner';

describe('StatusBanner', () => {
  it('renders success state with link', () => {
    render(<StatusBanner state="success" message="Done" txHash="abc123" />);
    expect(screen.getByText('Done')).toBeInTheDocument();
    expect(screen.getByText('View transaction')).toBeInTheDocument();
  });

  it('renders error state without link', () => {
    render(<StatusBanner state="error" message="Failed" />);
    expect(screen.getByText('Failed')).toBeInTheDocument();
    expect(screen.queryByText('View transaction')).toBeNull();
  });
});
