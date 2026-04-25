# Peprs — Getting Started: Scaffold & Boilerplate Guide

## 1. Initialize the Anchor project

```bash
anchor init peprs --javascript
cd peprs
```

## 2. Add dependencies

### Cargo.toml (Solana program)
```toml
[dependencies]
anchor-lang = "0.29.0"
anchor-spl = "0.29.0"
arcium-sdk = "0.1.0"       # Check https://github.com/arcium-hq for latest
```

### package.json (frontend)
```json
{
  "dependencies": {
    "@coral-xyz/anchor": "^0.29.0",
    "@solana/web3.js": "^1.91.0",
    "@solana/wallet-adapter-react": "^0.15.35",
    "@arcium-hq/arcium-client": "latest",
    "next": "^14.0.0",
    "react": "^18.0.0"
  }
}
```

## 3. Key files to create

### programs/peprs/src/lib.rs
```rust
use anchor_lang::prelude::*;

declare_id!("YOUR_PROGRAM_ID");

#[program]
pub mod peprs {
    use super::*;

    pub fn initialize_market(ctx: Context<InitMarket>, params: MarketParams) -> Result<()> {
        instructions::init_market::handler(ctx, params)
    }

    pub fn open_position(
        ctx: Context<OpenPosition>,
        size_commitment: [u8; 32],
        direction: u8,
        leverage_tier: u8,
        arcium_job_id: [u8; 32],
    ) -> Result<()> {
        instructions::open_position::handler(ctx, size_commitment, direction, leverage_tier, arcium_job_id)
    }

    pub fn close_position(
        ctx: Context<ClosePosition>,
        mpc_proof: Vec<u8>,
        realized_pnl: i64,
    ) -> Result<()> {
        instructions::close_position::handler(ctx, mpc_proof, realized_pnl)
    }

    pub fn liquidate(
        ctx: Context<Liquidate>,
        mpc_proof: Vec<u8>,
    ) -> Result<()> {
        instructions::liquidate::handler(ctx, mpc_proof)
    }
}
```

### programs/peprs/src/state/position.rs
```rust
use anchor_lang::prelude::*;

#[account]
pub struct Position {
    pub trader: Pubkey,            // 32
    pub market: Pubkey,            // 32
    pub size_commitment: [u8; 32], // 32 — Pedersen commitment
    pub direction: u8,             // 1
    pub leverage_tier: u8,         // 1
    pub arcium_job_id: [u8; 32],   // 32
    pub open_slot: u64,            // 8
    pub is_open: bool,             // 1
    pub bump: u8,                  // 1
}

impl Position {
    pub const LEN: usize = 8 + 32 + 32 + 32 + 1 + 1 + 32 + 8 + 1 + 1;
}
```

## 4. Arcium integration (TypeScript client)

```typescript
// app/src/hooks/useArcium.ts
import { ArciumClient } from '@arcium-hq/arcium-client';
import { useConnection, useWallet } from '@solana/wallet-adapter-react';

export function useArcium() {
  const { connection } = useConnection();
  const wallet = useWallet();

  const client = new ArciumClient({
    connection,
    cluster: 'devnet',
  });

  async function submitPrivateOrder(params: {
    size: number;
    price: number;
    direction: 'long' | 'short';
    leverage: number;
    market: string;
  }) {
    // 1. Submit to Arcium cluster (inputs stay secret)
    const jobId = await client.submitJob({
      circuit: 'order_match',
      inputs: {
        size: params.size,
        price: params.price,
        leverage: params.leverage,
      },
      publicInputs: {
        direction: params.direction === 'long' ? 0 : 1,
        market: params.market,
      },
    });

    // 2. Generate commitment to put on-chain
    const commitment = await client.generateCommitment(params.size, params.price);

    return { jobId, commitment };
  }

  async function closePrivatePosition(positionPubkey: string) {
    // Arcium computes PnL and returns proof + value
    const result = await client.resolveJob({
      jobId: positionPubkey,
      circuit: 'pnl_settle',
    });

    return {
      proof: result.proof,
      realizedPnl: result.output.pnl,
    };
  }

  return { submitPrivateOrder, closePrivatePosition };
}
```

## 5. Run tests

```bash
# Unit tests (Rust)
cargo test -p peprs

# Integration tests (Anchor)
anchor test --provider.cluster localnet

# Arcium circuit tests
arcium test circuits/liq_check.arcium
```

## 6. Deployment checklist

- [ ] Deploy Arcium circuits: `arcium deploy circuits/`
- [ ] Build program: `anchor build`
- [ ] Deploy program: `anchor deploy --provider.cluster devnet`
- [ ] Update program ID in `lib.rs` and `Anchor.toml`
- [ ] Fund program's collateral vault with test USDC
- [ ] Set environment vars: `NEXT_PUBLIC_PROGRAM_ID`, `NEXT_PUBLIC_ARCIUM_CLUSTER`
- [ ] Deploy frontend: `npm run build && vercel deploy`

## 7. Resources

- Arcium docs: https://docs.arcium.com
- Arcium GitHub: https://github.com/arcium-hq
- Anchor docs: https://www.anchor-lang.com
- Solana devnet faucet: https://faucet.solana.com
