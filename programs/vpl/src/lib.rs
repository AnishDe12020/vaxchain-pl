use anchor_lang::prelude::*;

declare_id!("6JuaxB1fEN9n6ApcvRxy6avr25H8qTzGRGpT43qrCvm4");

pub mod constants;
pub mod errors;
pub mod instructions;
pub mod state;
pub mod utils;

use crate::state::user::Role;

use instructions::create_batch::*;
use instructions::create_user::*;
use instructions::create_vaccine::*;
use instructions::distributor_receive::*;
use instructions::doctor_receive::*;
use instructions::temp_log::*;

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
        quantity: u16,
    ) -> Result<()> {
        create_batch_ix(
            ctx,
            expires_at,
            temp_min,
            temp_max,
            cost_per_piece,
            quantity,
        )
    }

    pub fn create_vaccine(ctx: Context<CreateVaccine>) -> Result<()> {
        create_vaccine_ix(ctx)
    }

    pub fn distributor_receive(ctx: Context<DistributorReceive>) -> Result<()> {
        distributor_receive_ix(ctx)
    }

    pub fn doctor_receive(ctx: Context<DoctorReceive>) -> Result<()> {
        doctor_receive_ix(ctx)
    }

    pub fn temp_log(ctx: Context<TempLogAccounts>, temp: u16, id: String) -> Result<()> {
        temp_log_ix(ctx, temp, id)
    }
}
