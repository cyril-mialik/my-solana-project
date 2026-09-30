use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("Only the minter authority can mint tokens")]
    Unauthorized,
    #[msg("Amount must be greater than zero")]
    InvalidAmount,
    #[msg("Decimals must not exceed 9")]
    InvalidDecimals,
    #[msg("Mint would exceed the global cap")]
    MintCapExceeded,
    #[msg("Arithmetic overflow")]
    Overflow,
}
