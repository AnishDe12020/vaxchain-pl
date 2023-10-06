use crate::*;
use std::str::FromStr;

#[account]
pub struct Vaccine {
    pub authority: Pubkey,
    pub id: String,

    // both the dates in timestamp
    pub prod_date: i64,
    pub exp_date: i64,

    // user's pubkey who created this vaccine
    pub manufacturer: Pubkey,

    // patient's id who received this vaccine
    pub receiver: String,

    // vaccine's status
    pub status: VaccineStatus,

    pub bump: u8,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq, Eq)]
pub enum VaccineStatus {
    Created = 1,
    ShippedToDist = 2,
    ReceivedByDist = 3,
    ShippedToDr = 4,
    ReceivedByDr = 5,
    Used = 6,
}

impl Vaccine {
    pub const MAX_ID_LEN: usize = 20 * 4;
    pub const MAX_RECEIVER_LEN: usize = 15 * 4;
    pub const LEN: usize =
        8 + 32 + (4 + Self::MAX_ID_LEN) + 8 + 8 + 32 + (4 + Self::MAX_RECEIVER_LEN) + 1 + 1;
}

impl FromStr for VaccineStatus {
    type Err = Error;
    fn from_str(s: &str) -> Result<VaccineStatus> {
        match s {
            "Created" => Ok(VaccineStatus::Created),
            "ShippedToDist" => Ok(VaccineStatus::ShippedToDist),
            "ReceivedByDist" => Ok(VaccineStatus::ReceivedByDist),
            "ShippedToDr" => Ok(VaccineStatus::ShippedToDr),
            "ReceivedByDr" => Ok(VaccineStatus::ReceivedByDr),
            "Used" => Ok(VaccineStatus::Used),
            _ => Ok(VaccineStatus::Used),
        }
    }
}
