import React from 'react';

export interface ListingCardProps {
  image: string;
  tokenId: number;
  seller: string;
  price: string;
  active: boolean;
  loading: boolean;
  onBuy: () => Promise<void>;
}

export function ListingCard({ image, tokenId, seller, price, active, loading, onBuy }: ListingCardProps) {
  return (
    <div className="rounded-xl border border-slate-200 bg-white p-4 shadow-sm">
      <img src={image} alt={`NFT ${tokenId}`} className="mb-4 h-48 w-full rounded-xl object-cover" />
      <div className="space-y-2">
        <div className="flex items-center justify-between">
          <span className="font-semibold">Token #{tokenId}</span>
          <span className="rounded-full bg-slate-100 px-3 py-1 text-sm text-slate-700">{price} XLM</span>
        </div>
        <p className="text-sm text-slate-600">Seller: {seller.slice(0, 6)}…{seller.slice(-6)}</p>
        <button
          className="mt-4 w-full rounded bg-slate-900 px-4 py-2 text-white hover:bg-slate-700 disabled:cursor-not-allowed disabled:opacity-50"
          onClick={onBuy}
          disabled={!active || loading}
        >
          {loading ? 'Buying…' : active ? 'Buy Now' : 'Sold'}
        </button>
      </div>
    </div>
  );
}
