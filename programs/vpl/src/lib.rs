use anchor_lang::prelude::*;

declare_id!("6JuaxB1fEN9n6ApcvRxy6avr25H8qTzGRGpT43qrCvm4");

pub mod errors;
pub mod instructions;
pub mod state;

use crate::state::user::Role;

use instructions::create_batch::*;
use instructions::create_user::*;
use instructions::create_vaccine::*;

#[program]
pub mod vpl {

    use super::*;

    pub fn create_user(ctx: Context<CreateUser>, role: Role) -> Result<()> {
        create_user_ix(ctx, role)
    }

    pub fn create_batch(
        ctx: Context<CreateBatch>,
        expires_at: i64,
        temp_min: u16,
        temp_max: u16,
        cost_per_piece: u16,
    ) -> Result<()> {
        create_batch_ix(ctx, expires_at, temp_min, temp_max, cost_per_piece)
    }

    pub fn create_vaccine(ctx: Context<CreateVaccine>) -> Result<()> {
        create_vaccine_ix(ctx)
    }
}
