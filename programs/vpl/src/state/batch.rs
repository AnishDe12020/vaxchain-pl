use anchor_lang::prelude::*;

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub enum BatchStatus {
    Manufactured = 0,
    StoredByDistributor = 1,
    ReceivedByDoctor = 2,
}

#[account]
pub struct Batch {
    pub pubkey: Pubkey,
    pub manufacturer: Pubkey,
    pub distributor: Option<Pubkey>,
    pub doctor: Option<Pubkey>,
    pub manufactured_at: i64,
    pub expires_at: i64,
    pub quantity: u16,
    pub temp_min: u16,
    pub temp_max: u16,
    pub cost_per_piece: u16,
    pub status: BatchStatus,
    pub temp_defect: bool,
    pub start_date: i64,
    pub stop_date: i64,
    pub latest_temp_log: Option<Pubkey>,
}

impl Batch {
    pub const LEN: usize =
        8 + 32 + 32 + (1 + 32) + (1 + 32) + 8 + 8 + 2 + 2 + 2 + 2 + 1 + 1 + 8 + 8 + (1 + 32);
}
