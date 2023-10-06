use crate::*;

#[account]
pub struct User {
    pub authority: Pubkey,
    pub username: String,
    pub role: String,
    pub is_admin: bool,
    pub bump: u8,
}

impl User {
    pub const MAX_USERNAME_LEN: usize = 20 * 4;
    pub const MAX_ROLE_LEN: usize = 15 * 4;
    pub const LEN: usize = 8 + 32 + (4 + Self::MAX_USERNAME_LEN) + (4 + Self::MAX_ROLE_LEN) + 1 + 1;
}
