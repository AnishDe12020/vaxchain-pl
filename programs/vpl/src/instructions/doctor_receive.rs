use std::str::FromStr;

use anchor_lang::prelude::*;
use anchor_spl::token::{transfer_checked, Mint, Token, TokenAccount, TransferChecked};

use crate::{
    constants::*,
    state::{
        batch::{Batch, BatchStatus},
        user::{Role, User},
    },
    utils::*,
};

use crate::errors::VplError;

pub fn doctor_receive_ix(ctx: Context<DoctorReceive>) -> Result<()> {
    let batch_pda = &mut ctx.accounts.batch_pda;
    let user_pda = &mut ctx.accounts.user_pda;
    let token_program = &mut ctx.accounts.token_program;
    let distributor_token_account = &mut ctx.accounts.distributor_token_account;
    let doctor_token_account = &mut ctx.accounts.doctor_token_account;
    let vault = &mut ctx.accounts.vault;
    let doctor = &mut ctx.accounts.user;
    let mint = &ctx.accounts.mint;
    let batch = &ctx.accounts.batch;

    require!(mint.key() == Pubkey::from_str(VAX_TOKEN_MINT).unwrap(), VplError::InvalidMint);

    require!(
        matches!(user_pda.role, Role::Doctor),
        VplError::UnauhtorizedRole
    );

    require!(batch_pda.quantity > 0, VplError::BatchEmpty);

    let clock = Clock::get()?;


    let days = calculate_days(batch_pda.start_date, clock.unix_timestamp);



    transfer_checked(
        CpiContext::new(
            token_program.to_account_info(),
            TransferChecked {
                from: doctor_token_account.to_account_info(),
                mint: mint.to_account_info(),
                to: distributor_token_account.to_account_info(),
                authority: doctor.to_account_info(),
            },
        ),
       ((batch_pda.quantity * batch_pda.cost_per_piece) as u16 + (calculate_refrigeration_cost(days as u16, batch_pda.temp_max)) * batch_pda.quantity) as u64 * 10_u64.pow(mint.decimals.into()),
        mint.decimals,
    )?;

    let batch_key = batch.key();

    let (_batch_pda, batch_pda_bump) = Pubkey::find_program_address(
        &[b"batch", batch.key().as_ref()],
        ctx.program_id,
    );

    let batch_pda_seeds = &[
        b"batch",
        batch_key.as_ref(),
        &[batch_pda_bump],
    ];

    transfer_checked(
        CpiContext::new(
            token_program.to_account_info(),
            TransferChecked {
                from: vault.to_account_info(),
                mint: mint.to_account_info(),
                to: distributor_token_account.to_account_info(),
                authority: batch_pda.to_account_info(),
            },
        )
        .with_signer(&[batch_pda_seeds]),
        (batch_pda.quantity * STAKE_PER_PIECE) as u64 * 10_u64.pow(mint.decimals.into()),
        mint.decimals,
    )?;

    batch_pda.status = BatchStatus::ReceivedByDoctor;
    batch_pda.stop_date = clock.unix_timestamp;
    batch_pda.doctor = Some(doctor.key());

    Ok(())
}

#[derive(Accounts)]
pub struct DoctorReceive<'info> {
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
    #[account(mut)]
    pub distributor_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub doctor_token_account: Account<'info, TokenAccount>,
    #[account(
        mut,
        seeds = [b"vault".as_ref(), batch.key().as_ref(), mint.key().as_ref()],
        bump,
        token::mint = mint,
        token::authority = batch_pda
    )]
    pub vault: Account<'info, TokenAccount>,
    pub mint: Account<'info, Mint>,
    pub token_program: Program<'info, Token>,
}
