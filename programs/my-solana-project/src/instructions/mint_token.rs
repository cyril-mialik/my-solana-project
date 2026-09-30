use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, MintTo, Token, TokenAccount};

use crate::{constants::*, error::ErrorCode, state::MinterState};

#[derive(Accounts)]
pub struct MintToken<'info> {
    #[account(
        mut,
        seeds = [MINTER_SEED],
        bump = minter_state.bump,
        has_one = authority,
    )]
    pub minter_state: Account<'info, MinterState>,
    pub authority: Signer<'info>,

    #[account(mut)]
    pub mint: Account<'info, Mint>,

    #[account(mut)]
    pub token_account: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
}

pub fn handle_mint_token(
    ctx: Context<MintToken>,
    amount: u64,
    decimals: u8,
) -> Result<()> {
    require!(amount > 0, ErrorCode::InvalidAmount);
    require!(decimals <= MAX_DECIMALS, ErrorCode::InvalidDecimals);

    let new_total = ctx.accounts.minter_state.total_minted
        .checked_add(amount)
        .ok_or(ErrorCode::Overflow)?;
    require!(new_total <= MAX_MINT_AMOUNT, ErrorCode::MintCapExceeded);

    let seeds = &[MINTER_SEED, &[ctx.accounts.minter_state.bump]];
    let signer_seeds = &[&seeds[..]];

    let cpi_accounts = MintTo {
        mint: ctx.accounts.mint.to_account_info(),
        to: ctx.accounts.token_account.to_account_info(),
        authority: ctx.accounts.minter_state.to_account_info(),
    };
    let cpi_ctx = CpiContext::new_with_signer(
        *ctx.accounts.token_program.key,
        cpi_accounts,
        signer_seeds,
    );
    token::mint_to(cpi_ctx, amount)?;

    ctx.accounts.minter_state.total_minted = new_total;
    msg!("Minted {} tokens, total = {}", amount, new_total);
    Ok(())
}
