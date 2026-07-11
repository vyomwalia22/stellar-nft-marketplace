import { render, screen } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import { describe, expect, it, vi } from 'vitest';
import { ListingCard } from './ListingCard';

describe('ListingCard', () => {
  it('shows sold state when inactive', () => {
    render(
      <ListingCard image="/image.png" tokenId={1} seller="seller" price="100" active={false} loading={false} onBuy={vi.fn()} />
    );
    expect(screen.getByText('Sold')).toBeInTheDocument();
  });

  it('calls onBuy when active and clicked', async () => {
    const user = userEvent.setup();
    const onBuy = vi.fn();
    render(<ListingCard image="/image.png" tokenId={2} seller="seller" price="200" active={true} loading={false} onBuy={onBuy} />);
    await user.click(screen.getByText('Buy Now'));
    expect(onBuy).toHaveBeenCalled();
  });
});
