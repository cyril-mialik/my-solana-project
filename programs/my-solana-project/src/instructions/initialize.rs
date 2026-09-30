use anchor_lang::prelude::*;

use crate::{constants::*, state::MinterState};

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(
        init,
        payer = payer,
        space = 8 + MinterState::INIT_SPACE,
        seeds = [MINTER_SEED],
        bump
    )]
    pub minter_state: Account<'info, MinterState>,
    pub system_program: Program<'info, System>,
}

pub fn handle_initialize(ctx: Context<Initialize>) -> Result<()> {
    let state = &mut ctx.accounts.minter_state;
    state.authority = ctx.accounts.payer.key();
    state.total_minted = 0;
    state.bump = ctx.bumps.minter_state;

    msg!("Minter initialized, authority = {}", state.authority);
    Ok(())
}
