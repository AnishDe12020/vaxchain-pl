use anchor_lang::prelude::*;

#[account]
pub struct TempLog {
    pub batch: Pubkey,
    pub timestamp: i64,
    pub temp: u16,
    pub pubkey: Pubkey,
}

impl TempLog {
    pub const LEN: usize = 8 + 32 + 8 + 2 + 32;
}
