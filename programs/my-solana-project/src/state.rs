use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct MinterState {
    pub authority: Pubkey,
    pub total_minted: u64,
    pub bump: u8,
}
