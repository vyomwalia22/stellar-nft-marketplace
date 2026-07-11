import { StellarWalletsKit } from '@creit.tech/stellar-wallets-kit/sdk';
import { Networks } from '@creit.tech/stellar-wallets-kit/types';
import { RabetModule } from '@creit.tech/stellar-wallets-kit/modules/rabet';

const NETWORK = Networks.TESTNET;

StellarWalletsKit.init({
  modules: [new RabetModule()],
  network: NETWORK,
  authModal: {
    hideUnsupportedWallets: true,
  },
});

export class WalletError extends Error {
  code: 'NOT_FOUND' | 'REJECTED' | 'NO_ACCOUNT' | 'UNKNOWN';

  constructor(message: string, code: WalletError['code']) {
    super(message);
    this.code = code;
  }
}

export async function openWalletModal() {
  try {
    const result = await StellarWalletsKit.authModal({});
    if (!result || typeof result.address !== 'string') {
      throw new WalletError('No wallet was selected', 'NOT_FOUND');
    }
    return result.address;
  } catch (error: unknown) {
    const message = error instanceof Error ? error.message : String(error);
    if (message.includes('User rejected') || message.includes('denied')) {
      throw new WalletError(message, 'REJECTED');
    }
    if (message.includes('No account') || message.includes('no account')) {
      throw new WalletError(message, 'NO_ACCOUNT');
    }
    throw new WalletError(message, 'UNKNOWN');
  }
}

export async function signTransactionXdr(xdr: string) {
  try {
    const result = await StellarWalletsKit.signTransaction(xdr, {
      networkPassphrase: NETWORK,
    });
    return typeof result === 'string' ? result : result.signedTxXdr;
  } catch (error: unknown) {
    const message = error instanceof Error ? error.message : String(error);
    if (message.includes('rejected') || message.includes('denied')) {
      throw new WalletError(message, 'REJECTED');
    }
    throw new WalletError(message, 'UNKNOWN');
  }
}
