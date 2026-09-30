use anchor_lang::prelude::*;

#[constant]
pub const MINTER_SEED: &[u8] = b"minter";

#[constant]
pub const MAX_DECIMALS: u8 = 9;

#[constant]
pub const MAX_MINT_AMOUNT: u64 = 1_000_000_000_000;
