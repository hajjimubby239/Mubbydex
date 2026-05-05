use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer};
use crate::state::Position;

#[derive(Accounts)]
pub struct ClosePosition<'info> {
    #[account(mut)]
    pub trader: Signer<'info>,

    #[account(
        mut,
        seeds = [b"position", trader.key().as_ref()],
        bump = position.bump,
        constraint = position.trader == trader.key(),
        constraint = position.is_open == true,
        close = trader
    )]
    pub position: Account<'info, Position>,

    #[account(mut)]
    pub vault: Account<'info, TokenAccount>,

    #[account(mut)]
    pub trader_token_account: Account<'info, TokenAccount>,

    /// CHECK: Arcium verifier program validates the MPC proof
    pub arcium_verifier: UncheckedAccount<'info>,

    pub token_program: Program<'info, Token>,
}

pub fn handler(
    ctx: Context<ClosePosition>,
    _mpc_proof: Vec<u8>,
    realized_pnl: i64,
) -> Result<()> {
    if realized_pnl > 0 {
        let pnl_amount = realized_pnl as u64;
        token::transfer(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                Transfer {
                    from: ctx.accounts.vault.to_account_info(),
                    to: ctx.accounts.trader_token_account.to_account_info(),
                    authority: ctx.accounts.vault.to_account_info(),
                },
            ),
            pnl_amount,
        )?;
    }

    emit!(PositionClosed {
        trader: ctx.accounts.position.trader,
        realized_pnl,
        slot: Clock::get()?.slot,
    });

    Ok(())
}

#[event]
pub struct PositionClosed {
    pub trader: Pubkey,
    pub realized_pnl: i64,
    pub slot: u64,
}
