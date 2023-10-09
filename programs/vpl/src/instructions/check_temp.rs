use std::str::FromStr;

use anchor_lang::prelude::*;
use anchor_spl::token::{burn, Burn, Mint, Token, TokenAccount};

use crate::{
    constants::VAX_TOKEN_MINT,
    errors::VplError,
    state::{batch::Batch, logs::TempLog},
};

pub fn check_temp_ix(ctx: Context<CheckTemp>) -> Result<()> {
    let batch = &mut ctx.accounts.batch;
    let temp_log = &mut ctx.accounts.temp_log;
    let batch_pda = &mut ctx.accounts.batch_pda;
    let temp_log_pda = &mut ctx.accounts.temp_log_pda;
    let mint = &ctx.accounts.mint;
    let vault = &mut ctx.accounts.vault;
    let token_program = &mut ctx.accounts.token_program;

    require!(
        mint.key() == Pubkey::from_str(VAX_TOKEN_MINT).unwrap(),
        VplError::InvalidMint
    );

    let clock: Clock = Clock::get()?;

    let batch_key = batch.key();

    let (_batch_pda, batch_pda_bump) =
        Pubkey::find_program_address(&[b"batch", batch.key().as_ref()], ctx.program_id);

    let batch_pda_seeds = &[b"batch", batch_key.as_ref(), &[batch_pda_bump]];

    if batch_pda.latest_temp_log.is_none() {
        return Err(VplError::NoTempLog.into());
    }

    if batch_pda.latest_temp_log.unwrap() != temp_log.key() {
        return Err(VplError::InvalidTempLog.into());
    }

    if clock.unix_timestamp - temp_log_pda.timestamp >= 1 {
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
    } else {
        return Err(VplError::TempNotExpired.into());
    }

    Ok(())
}

#[derive(Accounts)]
pub struct CheckTemp<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
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
        seeds = [b"temp_log".as_ref(), batch.key().as_ref(), temp_log.key().as_ref()],
        bump,
    )]
    pub temp_log_pda: Account<'info, TempLog>,
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
