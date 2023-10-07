use anchor_lang::prelude::*;

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub enum LogType {
    Manufactured = 0,
    StoredByDistributor = 1,
    ReceivedByDoctor = 2,
}

#[account]
pub struct Log {
    batch: Pubkey,
    timestamp: i64,
    log_type: LogType,
}

impl Log {
    pub const LEN: usize = 32 + 8 + 1;
}

#[account]
pub struct TempLog {
    batch: Pubkey,
    timestamp: i64,
    temp: u8,
}

impl TempLog {
    pub const LEN: usize = 32 + 8 + 1;
}
