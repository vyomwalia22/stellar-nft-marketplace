import StellarSdk from '@stellar/stellar-sdk';

export class ContractCallError extends Error {
  constructor(message: string) {
    super(message);
    this.name = 'ContractCallError';
  }
}

export async function buildInvocationXdr(contractId: string, method: string, args: any[], sourceAddress: string) {
  const server = new StellarSdk.Server(import.meta.env.VITE_RPC_URL || 'https://rpc.testnet.soroban.stellar.org');
  const account = await server.loadAccount(sourceAddress);
  const op = StellarSdk.Operation.invokeHostFunction({
    function: 'InvokeContract',
    contractId,
    method,
    args,
  } as any);
  const tx = new StellarSdk.TransactionBuilder(account, {
    fee: '100',
    networkPassphrase: import.meta.env.VITE_NETWORK_PASSPHRASE || 'Test SDF Network ; September 2015',
  })
    .addOperation(op)
    .setTimeout(30)
    .build();

  return tx.toXDR();
}

export async function submitSignedTx(signedXdr: string) {
  const server = new StellarSdk.Server(import.meta.env.VITE_RPC_URL || 'https://rpc.testnet.soroban.stellar.org');
  const response = await server.submitTransactionXDR(signedXdr);
  const hash = response.hash;
  const result = await server.getTransaction(hash);
  if (result.status !== 'success') {
    throw new ContractCallError(`Transaction failed: ${result.status}`);
  }
  return hash;
}

export async function readContract(contractId: string, method: string, args: any[], sourceAddress: string) {
  const server = new StellarSdk.Server(import.meta.env.VITE_RPC_URL || 'https://rpc.testnet.soroban.stellar.org');
  const response = await server.callContract(contractId, method, args, sourceAddress);
  if (response.status !== 'success') {
    throw new ContractCallError('Read-only contract call failed');
  }
  return response;
}
