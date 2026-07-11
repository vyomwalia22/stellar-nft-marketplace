import { useEffect, useState } from 'react';
import { WalletConnect } from './components/WalletConnect';
import { StatusBanner } from './components/StatusBanner';
import { ListingCard } from './components/ListingCard';
import { NFT_CONTRACT_ID, MARKETPLACE_CONTRACT_ID, NATIVE_TOKEN_CONTRACT_ID } from './lib/config';
import { buildInvocationXdr, submitSignedTx } from './lib/contracts';
import { signTransactionXdr } from './lib/wallet';

interface Listing {
  id: number;
  tokenId: number;
  seller: string;
  price: string;
  active: boolean;
  image: string;
}

export default function App() {
  const [address, setAddress] = useState('');
  const [nftUri, setNftUri] = useState('');
  const [price, setPrice] = useState('');
  const [status, setStatus] = useState<'idle' | 'pending' | 'success' | 'error'>('idle');
  const [statusMessage, setStatusMessage] = useState('');
  const [txHash, setTxHash] = useState('');
  const [listings, setListings] = useState<Listing[]>([]);
  const [buying, setBuying] = useState<Record<number, boolean>>({});

  const canSubmit = !!address && nftUri && price;

  const handleConnected = (publicKey: string) => {
    setAddress(publicKey);
  };

  const loadListings = async () => {
    const raw = await fetch('/listings.json');
    const data = await raw.json();
    setListings(data as Listing[]);
  };

  const mintNft = async () => {
    if (!address) return;
    setStatus('pending');
    setStatusMessage('Minting NFT');
    try {
      const xdr = await buildInvocationXdr(NFT_CONTRACT_ID, 'mint', [address, nftUri], address);
      const signed = await signTransactionXdr(xdr);
      const hash = await submitSignedTx(signed);
      setTxHash(hash);
      setStatus('success');
      setStatusMessage('NFT minted successfully');
    } catch (error) {
      setStatus('error');
      setStatusMessage((error instanceof Error ? error.message : 'Mint failed') as string);
    }
  };

  const listForSale = async () => {
    if (!address) return;
    setStatus('pending');
    setStatusMessage('Listing NFT for sale');
    try {
      const xdr = await buildInvocationXdr(MARKETPLACE_CONTRACT_ID, 'list_item', [address, NFT_CONTRACT_ID, 1, NATIVE_TOKEN_CONTRACT_ID, parseInt(price, 10)], address);
      const signed = await signTransactionXdr(xdr);
      const hash = await submitSignedTx(signed);
      setTxHash(hash);
      setStatus('success');
      setStatusMessage('NFT listed successfully');
    } catch (error) {
      setStatus('error');
      setStatusMessage((error instanceof Error ? error.message : 'Listing failed') as string);
    }
  };

  const buyItem = async (listingId: number) => {
    if (!address) return;
    setBuying((prev) => ({ ...prev, [listingId]: true }));
    try {
      const xdr = await buildInvocationXdr(MARKETPLACE_CONTRACT_ID, 'buy_item', [address, listingId], address);
      const signed = await signTransactionXdr(xdr);
      const hash = await submitSignedTx(signed);
      setTxHash(hash);
      setStatus('success');
      setStatusMessage('Purchase completed');
      await loadListings();
    } catch (error) {
      setStatus('error');
      setStatusMessage((error instanceof Error ? error.message : 'Purchase failed') as string);
    } finally {
      setBuying((prev) => ({ ...prev, [listingId]: false }));
    }
  };

  useEffect(() => {
    loadListings();
  }, []);

  return (
    <div className="min-h-screen bg-slate-50 py-8 px-4 sm:px-8">
      <div className="mx-auto max-w-6xl space-y-8">
        <header className="flex flex-col gap-4 rounded-3xl bg-white p-6 shadow-sm sm:flex-row sm:items-center sm:justify-between">
          <div>
            <h1 className="text-3xl font-semibold text-slate-900">Stellar Soroban NFT Marketplace</h1>
            <p className="mt-2 text-slate-600">Mint, list, and buy NFTs with atomic escrow settlement.</p>
          </div>
          <WalletConnect onConnected={handleConnected} />
        </header>

        {status !== 'idle' && <StatusBanner state={status} message={statusMessage} txHash={txHash} />}

        <section className="grid gap-6 lg:grid-cols-2">
          <div className="rounded-3xl bg-white p-6 shadow-sm">
            <h2 className="text-xl font-semibold text-slate-900">Mint NFT</h2>
            <label className="mt-4 block text-sm text-slate-700">Token URI</label>
            <input value={nftUri} onChange={(event) => setNftUri(event.target.value)} className="mt-2 w-full rounded-xl border border-slate-200 px-4 py-3" placeholder="https://example.com/nft/metadata.json" />
            <label className="mt-4 block text-sm text-slate-700">Price (XLM)</label>
            <input value={price} onChange={(event) => setPrice(event.target.value)} className="mt-2 w-full rounded-xl border border-slate-200 px-4 py-3" placeholder="100" type="number" />
            <button disabled={!canSubmit || !address} onClick={mintNft} className="mt-6 w-full rounded-xl bg-slate-900 px-4 py-3 text-white hover:bg-slate-700 disabled:cursor-not-allowed disabled:opacity-50">
              Mint NFT
            </button>
          </div>

          <div className="rounded-3xl bg-white p-6 shadow-sm">
            <h2 className="text-xl font-semibold text-slate-900">List for sale</h2>
            <p className="mt-2 text-sm text-slate-600">Use a minted token ID and your wallet address to list the NFT onto the marketplace escrow.</p>
            <button disabled={!address} onClick={listForSale} className="mt-6 w-full rounded-xl bg-slate-900 px-4 py-3 text-white hover:bg-slate-700 disabled:cursor-not-allowed disabled:opacity-50">
              List on Marketplace
            </button>
          </div>
        </section>

        <section>
          <div className="mb-4 flex items-center justify-between">
            <h2 className="text-xl font-semibold text-slate-900">Marketplace Listings</h2>
            <button onClick={loadListings} className="rounded-full bg-slate-100 px-4 py-2 text-slate-700 hover:bg-slate-200">Refresh</button>
          </div>

          <div className="grid gap-6 sm:grid-cols-2 xl:grid-cols-3">
            {listings.map((listing) => (
              <ListingCard
                key={listing.id}
                image={listing.image}
                tokenId={listing.tokenId}
                seller={listing.seller}
                price={listing.price}
                active={listing.active}
                loading={buying[listing.id] ?? false}
                onBuy={() => buyItem(listing.id)}
              />
            ))}
          </div>
        </section>
      </div>
    </div>
  );
}
