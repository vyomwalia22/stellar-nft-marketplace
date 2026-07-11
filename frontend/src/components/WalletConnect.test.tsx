import { render, screen, fireEvent } from '@testing-library/react';
import { describe, expect, it, vi } from 'vitest';
import { WalletConnect } from './WalletConnect';

describe('WalletConnect', () => {
  it('renders connect button', () => {
    const onConnected = vi.fn();
    render(<WalletConnect onConnected={onConnected} />);
    expect(screen.getByText('Connect Wallet')).toBeInTheDocument();
  });

  it('disables button while connecting', () => {
    const onConnected = vi.fn();
    render(<WalletConnect onConnected={onConnected} />);
    const button = screen.getByRole('button');
    fireEvent.click(button);
    expect(button).toBeDisabled();
  });
});
