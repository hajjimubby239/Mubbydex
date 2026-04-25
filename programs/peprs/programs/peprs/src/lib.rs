use anchor_lang::prelude::*;

declare_id!("11111111111111111111111111111111");

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
        instructions::open_position::handler(
            ctx,
            size_commitment,
            direction,
            leverage_tier,
            arcium_job_id,
        )
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

pub mod instructions;
pub mod state;
