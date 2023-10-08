use anchor_lang::prelude::*;

use crate::constants::ID_LENGTH;

#[account]
pub struct TempLog {
    pub batch: Pubkey,
    pub timestamp: i64,
    pub temp: u16,
    pub id: String,
}

impl TempLog {
    pub const LEN: usize = 32 + 8 + 2 + ID_LENGTH;
}
