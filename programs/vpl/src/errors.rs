use anchor_lang::prelude::*;

#[error_code]
pub enum VplError {
    #[msg("Role doesn't have permission to perform this action")]
    UnauhtorizedRole,
    #[msg("Batch is not manufactured")]
    BatchNotManufactured,
    #[msg("Invalid expiration date")]
    InvalidExpirationDate,
    #[msg("Invalid mint")]
    InvalidMint,
    #[msg("Empty batch")]
    BatchEmpty,
}
