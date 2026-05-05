use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer};
use crate::state::{Position, Market};

#[derive(Accounts)]
pub struct OpenPosition<'info> {
    #[account(mut)]
    pub trader: Signer<'info>,

    #[account(
        init,
        payer = trader,
        space = Position::LEN,
        seeds = [b"position", trader.key().as_ref()],
        bump
    )]
    pub position: Account<'info, Position>,

    #[account(mut)]
    pub vault: Account<'info, TokenAccount>,

    #[account(mut)]
    pub trader_token_account: Account<'info, TokenAccount>,

    pub market: Account<'info, Market>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
}

pub fn handler(
    ctx: Context<OpenPosition>,
    size_commitment: [u8; 32],
    direction: u8,
    leverage_tier: u8,
    arcium_job_id: [u8; 32],
) -> Result<()> {
    let pos = &mut ctx.accounts.position;
    pos.trader = ctx.accounts.trader.key();
    pos.market = ctx.accounts.market.key();
    pos.size_commitment = size_commitment;
    pos.direction = direction;
    pos.leverage_tier = leverage_tier;
    pos.arcium_job_id = arcium_job_id;
    pos.open_slot = Clock::get()?.slot;
    pos.is_open = true;
    pos.bump = ctx.bumps.position;

    token::transfer(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            Transfer {
                from: ctx.accounts.trader_token_account.to_account_info(),
                to: ctx.accounts.vault.to_account_info(),
                authority: ctx.accounts.trader.to_account_info(),
            },
        ),
        ctx.accounts.market.min_collateral,
    )?;

    emit!(PositionOpened {
        trader: pos.trader,
        market: pos.market,
        direction,
        slot: pos.open_slot,
    });

    Ok(())
}

#[event]
pub struct PositionOpened {
    pub trader: Pubkey,
    pub market: Pubkey,
    pub direction: u8,
    pub slot: u64,
}
