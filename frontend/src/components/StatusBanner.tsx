import React from 'react';

export type StatusState = 'idle' | 'pending' | 'success' | 'error';

export function StatusBanner({ state, message, txHash }: { state: StatusState; message: string; txHash?: string }) {
  const color = state === 'success' ? 'bg-emerald-100 text-emerald-900' : state === 'error' ? 'bg-rose-100 text-rose-900' : 'bg-slate-100 text-slate-900';
  return (
    <div className={`rounded border ${color} border-slate-200 p-4`}>
      <p className="text-sm font-medium">{message}</p>
      {txHash ? (
        <a className="mt-2 inline-block text-sm text-slate-700 underline" href={`https://stellar.expert/explorer/testnet/tx/${txHash}`} target="_blank" rel="noreferrer">
          View transaction
        </a>
      ) : null}
    </div>
  );
}
