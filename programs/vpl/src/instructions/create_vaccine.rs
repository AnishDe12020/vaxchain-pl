use anchor_lang::prelude::*;

use crate::state::{
    batch::{Batch, BatchStatus},
    user::{Role, User},
    vaccine::Vaccine,
};

use crate::errors::VplError;

pub fn create_vaccine_ix(ctx: Context<CreateVaccine>) -> Result<()> {
    let user_pda = &mut ctx.accounts.user_pda;
    let batch_pda = &mut ctx.accounts.batch_pda;
    let vaccine_pda = &mut ctx.accounts.vaccine_pda;

    require!(
        matches!(user_pda.role, Role::Manufacturer),
        VplError::UnauhtorizedRole
    );

    require!(
        matches!(batch_pda.status, BatchStatus::Manufactured),
        VplError::BatchNotManufactured
    );

    vaccine_pda.pubkey = *ctx.accounts.vaccine.key;
    vaccine_pda.batch = *ctx.accounts.batch.key;
    vaccine_pda.used = false;
    vaccine_pda.used_at = None;
    vaccine_pda.used_by = None;

    Ok(())
}

#[derive(Accounts)]
pub struct CreateVaccine<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(
        seeds = [b"user".as_ref(), user.key().as_ref()],
        bump,
    )]
    pub user_pda: Account<'info, User>,
    /// CHECK: identifier
    pub batch: AccountInfo<'info>,
    #[account(
        seeds = [b"batch".as_ref(), batch.key().as_ref()],
        bump,
    )]
    pub batch_pda: Account<'info, Batch>,
    /// CHECK: identifier
    pub vaccine: AccountInfo<'info>,
    #[account(
        init,
        seeds = [b"vaccine".as_ref(), vaccine.key().as_ref()],
        payer = user,
        bump,
        space = Vaccine::LEN
    )]
    pub vaccine_pda: Account<'info, Vaccine>,
    pub system_program: Program<'info, System>,
}
