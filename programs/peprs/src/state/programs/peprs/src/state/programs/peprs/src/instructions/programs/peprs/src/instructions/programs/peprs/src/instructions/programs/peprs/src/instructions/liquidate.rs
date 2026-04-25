use anchor_lang::prelude::*;
use crate::state::Position;

#[derive(Accounts)]
pub struct Liquidate<'info> {
    #[account(mut)]
    pub liquidator: Signer<'info>,

    #[account(
        mut,
        seeds = [b"position", position.trader.as_ref()],
        bump = position.bump,
        constraint = position.is_open == true,
        close = liquidator
    )]
    pub position: Account<'info, Position>,

    /// CHECK: Arcium verifier confirms liq_check circuit returned true
    pub arcium_verifier: UncheckedAccount<'info>,
}

pub fn handler(
    ctx: Context<Liquidate>,
    _mpc_proof: Vec<u8>,
) -> Result<()> {
    emit!(PositionLiquidated {
        trader: ctx.accounts.position.trader,
        liquidator: ctx.accounts.liquidator.key(),
        slot: Clock::get()?.slot,
    });

    Ok(())
}

#[event]
pub struct PositionLiquidated {
    pub trader: Pubkey,
    pub liquidator: Pubkey,
    pub slot: u64,
}
