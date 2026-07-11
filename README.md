# Stellar NFT Marketplace

A complete Stellar Soroban NFT marketplace with two Soroban smart contracts and a React + TypeScript frontend.

## Architecture

This repository is a Cargo workspace with two smart contracts and a dedicated frontend.

- `contracts/nft` contains the NFT contract for minting, ownership, metadata, and balances.
- `contracts/marketplace` contains the marketplace contract that escrows NFTs, lists items, and executes atomic purchases.

The marketplace contract uses cross-contract calls to the NFT contract so that payment and NFT transfer happen in one atomic transaction.

## Build commands

### Contracts

```bash
cargo test --workspace
cargo build --workspace --release --target wasm32-unknown-unknown
```

### Frontend

```bash
cd frontend
npm ci
npm run lint
npm test -- --run
npm run build
```

## Deployment

1. Build WASM artifacts:
   ```bash
   cargo build --workspace --release --target wasm32-unknown-unknown
   ```
2. Deploy contracts to Soroban Testnet:
   ```bash
   soroban contract deploy --wasm target/wasm32-unknown-unknown/release/nft-contract.wasm --network testnet
   soroban contract deploy --wasm target/wasm32-unknown-unknown/release/marketplace-contract.wasm --network testnet
   ```
3. Wire up frontend environment variables in `frontend/.env`:
   ```env
   VITE_NFT_CONTRACT_ID=YOUR_NFT_CONTRACT_ID
   VITE_MARKETPLACE_CONTRACT_ID=YOUR_MARKETPLACE_CONTRACT_ID
   VITE_NATIVE_TOKEN_CONTRACT_ID=YOUR_NATIVE_TOKEN_CONTRACT_ID
   ```

## Requirement mapping

| Requirement | Implementation |
|---|---|
| Inter-contract communications | `contracts/marketplace/src/lib.rs` uses `NftContractClient` to call NFT contract methods |
| Event streaming | `contracts/nft/src/lib.rs` emits `mint` and `transfer`; `contracts/marketplace/src/lib.rs` emits `listed`, `bought`, and `cancelled` |
| CI/CD | `.github/workflows/ci.yml` validates contracts and frontend on push/PR to main |
| Mobile responsive UI | `frontend/src/App.tsx` uses responsive Tailwind utilities for mobile layout |
| Error handling | `frontend/src/lib/wallet.ts` maps wallet errors to typed `WalletError`; `frontend/src/lib/contracts.ts` throws `ContractCallError` |
| Tests | `contracts/*/src/test.rs` and `frontend/src/components/*.test.tsx` |
| Production practices | endpoint configuration, release profile optimization, separate contract crates, and CI pipeline |

## Placeholders

- NFT contract ID: `YOUR_NFT_CONTRACT_ID`
- Marketplace contract ID: `YOUR_MARKETPLACE_CONTRACT_ID`
- Native token contract ID: `YOUR_NATIVE_TOKEN_CONTRACT_ID`
- Example transaction hash: `YOUR_TX_HASH`
- Live demo URL: `YOUR_LIVE_DEMO_URL`
- Demo video URL: `YOUR_DEMO_VIDEO_URL`
