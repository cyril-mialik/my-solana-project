pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("JDaFRNRx4bZZWdGTXvQjexsPhYqyHDBNpFuYRQuMDhp9");

#[program]
pub mod my_solana_project {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        crate::instructions::initialize::handle_initialize(ctx)
    }

    pub fn mint_token(
        ctx: Context<MintToken>,
        amount: u64,
        decimals: u8,
    ) -> Result<()> {
        crate::instructions::mint_token::handle_mint_token(ctx, amount, decimals)
    }
}
