use anchor_lang::prelude::*;

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub enum Role {
    Manufacturer = 0,
    Distributor = 1,
    Doctor = 2,
}

#[account]
pub struct User {
    pub pubkey: Pubkey,
    pub created_at: i64,
    pub updated_at: i64,
    pub role: Role,
}

impl User {
    pub const LEN: usize = 8 + 32 + 8 + 8 + 1;
}
