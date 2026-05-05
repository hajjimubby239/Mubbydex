use anchor_lang::prelude::*;

#[account]
pub struct Position {
    pub trader: Pubkey,
    pub market: Pubkey,
    pub size_commitment: [u8; 32],
    pub direction: u8,
    pub leverage_tier: u8,
    pub arcium_job_id: [u8; 32],
    pub open_slot: u64,
    pub is_open: bool,
    pub bump: u8,
}

impl Position {
    pub const LEN: usize = 8 + 32 + 32 + 32 + 1 + 1 + 32 + 8 + 1 + 1;
}

#[account]
pub struct Market {
    pub authority: Pubkey,
    pub base_mint: Pubkey,
    pub quote_mint: Pubkey,
    pub vault: Pubkey,
    pub min_collateral: u64,
    pub max_leverage_tier: u8,
    pub is_active: bool,
    pub bump: u8,
}

impl Market {
    pub const LEN: usize = 8 + 32 + 32 + 32 + 32 + 8 + 1 + 1 + 1;
}
