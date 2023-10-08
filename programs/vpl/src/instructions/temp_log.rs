use anchor_lang::prelude::*;

use crate::{
    errors::VplError,
    state::{
        batch::Batch,
        logs::TempLog,
        user::{Role, User},
    },
};

pub fn temp_log_ix(ctx: Context<TempLogAccounts>, temp: u16, id: String) -> Result<()> {
    let user = &mut ctx.accounts.user;
    let batch = &mut ctx.accounts.batch;
    let user_pda = &mut ctx.accounts.user_pda;
    let batch_pda = &mut ctx.accounts.batch_pda;
    let temp_log = &mut ctx.accounts.temp_log;

    let clock: Clock = Clock::get()?;

    require!(
        matches!(user_pda.role, Role::Distributor),
        VplError::UnauhtorizedRole
    );

    require!(
        batch_pda.distributor.unwrap() == user.key(),
        VplError::UnauhtorizedRole
    );

    temp_log.temp = temp;
    temp_log.timestamp = clock.unix_timestamp;
    temp_log.id = id;
    temp_log.batch = batch.key();

    Ok(())
}

#[derive(Accounts)]
#[instruction(temp: u16, id: String)]
pub struct TempLogAccounts<'info> {
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
    #[account(
        init,
        seeds = [b"temp_log".as_ref(), batch.key().as_ref(), ],
        payer = user,
        bump,
        space = TempLog::LEN
    )]
    pub temp_log: Account<'info, TempLog>,
    pub system_program: Program<'info, System>,
}
