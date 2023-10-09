use anchor_lang::prelude::*;

use crate::{state::{vaccine::Vaccine, user::{User, Role}}, errors::VplError};

pub fn use_vaccine_ix(ctx: Context<UseVaccine>) -> Result<()> {
    let user_pda = &mut ctx.accounts.user_pda;
    let vaccine_pda = &mut ctx.accounts.vaccine_pda;

    require!(
        !vaccine_pda.used,
        VplError::VaccineAlreadyUsed
    );

    require!(
        matches!(user_pda.role, Role::Doctor),
        VplError::UnauhtorizedRole
    );

    let clock = Clock::get()?;

    vaccine_pda.used = true;
    vaccine_pda.used_at = Some(clock.unix_timestamp);
    vaccine_pda.used_by = Some(user_pda.pubkey);

    Ok(())
}

#[derive(Accounts)]
pub struct UseVaccine<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(
        seeds = [b"user".as_ref(), user.key().as_ref()],
        bump,
    )]
    pub user_pda: Account<'info, User>,
    /// CHECK: identifier
    pub vaccine: AccountInfo<'info>,
    #[account(
        mut, 
        seeds = [b"vaccine".as_ref(), vaccine.key().as_ref()],
        bump,
    )]
    pub vaccine_pda: Account<'info, Vaccine>,
}
