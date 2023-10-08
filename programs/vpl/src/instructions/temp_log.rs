use std::str::FromStr;

use anchor_lang::prelude::*;
use anchor_spl::token::{burn, Burn, Mint, Token, TokenAccount};

use crate::{
    constants::VAX_TOKEN_MINT,
    errors::VplError,
    state::{
        batch::Batch,
        logs::TempLog,
        user::{Role, User},
    },
};

pub fn temp_log_ix(ctx: Context<TempLogAccounts>, temp: u16) -> Result<()> {
    let user = &mut ctx.accounts.user;
    let batch = &mut ctx.accounts.batch;
    let last_temp_log = &mut ctx.accounts.last_temp_log;
    let temp_log = &mut ctx.accounts.temp_log;
    let user_pda = &mut ctx.accounts.user_pda;
    let batch_pda = &mut ctx.accounts.batch_pda;
    let temp_log_pda = &mut ctx.accounts.temp_log_pda;
    let last_temp_log_pda = &mut ctx.accounts.last_temp_log_pda;
    let mint = &ctx.accounts.mint;
    let vault = &mut ctx.accounts.vault;
    let token_program = &mut ctx.accounts.token_program;

    let clock: Clock = Clock::get()?;

    let batch_key = batch.key();

    let (_batch_pda, batch_pda_bump) =
        Pubkey::find_program_address(&[b"batch", batch.key().as_ref()], ctx.program_id);

    let batch_pda_seeds = &[b"batch", batch_key.as_ref(), &[batch_pda_bump]];

    match last_temp_log {
        Some(_) => {
            if last_temp_log_pda.is_none()
                || batch_pda.latest_temp_log.is_none()
                || last_temp_log.clone().unwrap().key() != batch_pda.latest_temp_log.unwrap()
            {
                return Err(VplError::InvalidTempLog.into());
            }

            let last_temp_data = last_temp_log_pda.clone().unwrap();

            if clock.unix_timestamp - last_temp_data.timestamp > 300 {
                burn(
                    CpiContext::new(
                        token_program.to_account_info(),
                        Burn {
                            from: vault.to_account_info(),
                            mint: mint.to_account_info(),
                            authority: batch_pda.to_account_info(),
                        },
                    )
                    .with_signer(&[batch_pda_seeds]),
                    vault.amount,
                )?;

                batch_pda.temp_defect = true;
            }
        }
        None => {
            if batch_pda.latest_temp_log.is_some() {
                return Err(VplError::TempLogNotPassedIn.into());
            }
        }
    }

    require!(
        mint.key() == Pubkey::from_str(VAX_TOKEN_MINT).unwrap(),
        VplError::InvalidMint
    );

    require!(
        matches!(user_pda.role, Role::Distributor),
        VplError::UnauhtorizedRole
    );

    require!(
        batch_pda.distributor.unwrap() == user.key(),
        VplError::UnauhtorizedRole
    );

    if temp < batch_pda.temp_min || temp > batch_pda.temp_max {
        burn(
            CpiContext::new(
                token_program.to_account_info(),
                Burn {
                    from: vault.to_account_info(),
                    mint: mint.to_account_info(),
                    authority: batch_pda.to_account_info(),
                },
            )
            .with_signer(&[batch_pda_seeds]),
            vault.amount,
        )?;

        batch_pda.temp_defect = true;
    }

    temp_log_pda.temp = temp;
    temp_log_pda.timestamp = clock.unix_timestamp;
    temp_log_pda.pubkey = temp_log.key();
    temp_log_pda.batch = batch.key();

    batch_pda.latest_temp_log = Some(temp_log.key());

    Ok(())
}

#[derive(Accounts)]
#[instruction(temp: u16)]
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
        mut,
        seeds = [b"batch".as_ref(), batch.key().as_ref()],
        bump,
    )]
    pub batch_pda: Account<'info, Batch>,
    /// CHECK: identifier
    pub temp_log: AccountInfo<'info>,
    #[account(
        init,
        seeds = [b"temp_log".as_ref(), batch.key().as_ref(), temp_log.key().as_ref()],
        payer = user,
        bump,
        space = TempLog::LEN
    )]
    pub temp_log_pda: Account<'info, TempLog>,
    /// CHECK: identifier
    pub last_temp_log: Option<AccountInfo<'info>>,
    #[account(
        mut,
        seeds = [b"temp_log".as_ref(), batch.key().as_ref(), last_temp_log.clone().unwrap().key().as_ref()],
        bump,
    )]
    pub last_temp_log_pda: Option<Account<'info, TempLog>>,
    #[account(
        mut,
        seeds = [b"vault".as_ref(), batch.key().as_ref(), mint.key().as_ref()],
        bump,
        token::mint = mint,
        token::authority = batch_pda
    )]
    pub vault: Account<'info, TokenAccount>,
    #[account(mut)]
    pub mint: Account<'info, Mint>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
}
