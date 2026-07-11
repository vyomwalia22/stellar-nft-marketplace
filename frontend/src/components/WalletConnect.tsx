import { useState } from 'react';
import { openWalletModal, WalletError } from '../lib/wallet';

export function WalletConnect({ onConnected }: { onConnected: (address: string) => void }) {
  const [loading, setLoading] = useState(false);
  const [address, setAddress] = useState<string | null>(null);

  const connect = async () => {
    setLoading(true);
    try {
      const account = await openWalletModal();
      setAddress(account);
      onConnected(account);
    } catch (error) {
      if (error instanceof WalletError) {
        console.error('Wallet error', error.code, error.message);
      } else {
        console.error(error);
      }
    } finally {
      setLoading(false);
    }
  };

  const disconnect = () => {
    setAddress(null);
    onConnected('');
  };

  return (
    <div className="flex items-center gap-3">
      {address ? (
        <>
          <span className="rounded-full bg-slate-200 px-3 py-1 text-sm text-slate-700">{address.slice(0, 8)}…{address.slice(-8)}</span>
          <button className="rounded bg-slate-900 px-4 py-2 text-white hover:bg-slate-700" onClick={disconnect}>Disconnect</button>
        </>
      ) : (
        <button className="rounded bg-slate-900 px-4 py-2 text-white hover:bg-slate-700" onClick={connect} disabled={loading}>
          {loading ? 'Connecting…' : 'Connect Wallet'}
        </button>
      )}
    </div>
  );
}
