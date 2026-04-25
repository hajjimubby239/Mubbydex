use anchor_lang::prelude::*;

#[account]
pub struct Position {
    pub trader: Pubkey,            // 32
    pub market: Pubkey,            // 32
    pub size_commitment: [u8; 32], // 32 — Pedersen commitment, not plaintext
    pub direction: u8,             // 1  — 0 = long, 1 = short (public)
    pub leverage_tier: u8,         // 1  — bucketed tier, not exact leverage
    pub arcium_job_id: [u8; 32],   // 32 — reference to MPC computation job
    pub open_slot: u64,            // 8
    pub is_open: bool,             // 1
    pub bump: u8,                  // 1
}

impl Position {
    pub const LEN: usize = 8 + 32 + 32 + 32 + 1 + 1 + 32 + 8 + 1 + 1;
}

#[account]
pub struct Market {
    pub authority: Pubkey,         // 32
    pub base_mint: Pubkey,         // 32
    pub quote_mint: Pubkey,        // 32
    pub vault: Pubkey,             // 32
    pub min_collateral: u64,       // 8  — floor collateral stored on-chain
    pub max_leverage_tier: u8,     // 1
    pub is_active: bool,           // 1
    pub bump: u8,                  // 1
}

impl Market {
    pub const LEN: usize = 8 + 32 + 32 + 32 + 32 + 8 + 1 + 1 + 1;
}
