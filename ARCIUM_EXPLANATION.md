# How Arcium Powers Peprs — Clear Explanation for Judges

## What is Arcium?

Arcium is a **confidential computing network** built for Solana. It runs a cluster of
independent nodes that can jointly execute computations over **secret-shared inputs** —
meaning no single node ever sees the full data, yet together they produce a correct result.
The result is verified on Solana via a compact proof.

Think of it like a locked safe that three people each hold one key to: you need all three
to open it, so no one person can act alone.

---

## The Core Privacy Primitive: MPC

Multi-Party Computation (MPC) lets N parties compute a function f(x) where:
- Each party holds a **share** of x (not the full value)
- They communicate to evaluate f without revealing their shares
- The final output is **reconstructed together** — not by any single node

Arcium operationalises this with a circuit DSL. Developers write the private logic as a
**circuit**, deploy it to the cluster, and get back an on-chain-verifiable result.

---

## Arcium in Peprs: Step by Step

### Step 1 — Order submission
When a trader places an order, the frontend:
1. Generates a Pedersen commitment to the sensitive fields (size, price, leverage)
2. Submits the commitment + public fields (direction, market) to the Peprs Solana program
3. Sends the secret inputs to the Arcium cluster via the Arcium SDK

The Solana transaction contains **no plaintext size or price**.

### Step 2 — MPC computation
The Arcium cluster runs three circuits in sequence:

```
order_match.arcium   → matches your order against the private orderbook
liq_check.arcium     → checks if your position is healthy (boolean output only)
pnl_settle.arcium    → computes realized PnL on close
```

Each circuit is evaluated across all three MPC nodes via secret-sharing protocols
(e.g. SPDZ or Shamir). No node learns your position size, entry price, or liq threshold.

### Step 3 — On-chain settlement
When computation is complete, Arcium outputs:
- A **proof** that the computation was run correctly
- The **PnL** value (the only output that needs to be public for settlement)

The Peprs program verifies the proof via CPI to the Arcium verifier program, then
settles the PnL from the collateral vault to the trader.

---

## What Stays Private vs What Is Revealed

```
PRIVATE (MPC-shielded, never on-chain):     PUBLIC (on-chain, visible):
├── Position size                           ├── Direction (long/short)
├── Entry price                             ├── Market (SOL-PERP, etc.)
├── Leverage (exact)                        ├── Realized PnL (on close)
├── Liquidation threshold                   └── Collateral floor
└── Pending orders
```

---

## Why This Changes Trader Behaviour

Without privacy:
- Whale opens 500k SOL long → bots copy immediately
- Trader is at liq price $120 → searchers push price to $119.99
- Large limit order visible → front-runners jump queue

With Arcium MPC:
- Position data is off-limits — copy-trading requires observing PnL over time
- Liquidation threshold is never on-chain — targeted hunting is impossible
- Orders match privately — no front-running vector in the orderbook

---

## Technical Integration Points

| Integration | Implementation |
|---|---|
| Arcium SDK | `arcium-sdk` Rust crate + JS client |
| Circuit deployment | `arcium deploy` CLI |
| On-chain verification | CPI to `arcium_verifier` program |
| Secret sharing | Handled by Arcium cluster internals |
| Commitment scheme | Pedersen commitments on order inputs |

