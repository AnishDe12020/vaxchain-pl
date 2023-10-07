use anchor_lang::prelude::*;
use anchor_spl::token::{transfer_checked, Mint, Token, TokenAccount, TransferChecked};

use crate::{
    constants,
    state::{
        batch::{Batch, BatchStatus},
        user::{Role, User},
    },
};

use crate::errors::VplError;

pub fn distributor_receive_ix(ctx: Context<DistributorReceive>) -> Result<()> {
    let batch_pda = &mut ctx.accounts.batch_pda;
    let user_pda = &mut ctx.accounts.user_pda;
    let token_program = &mut ctx.accounts.token_program;
    let distributor_token_account = &mut ctx.accounts.distributor_token_account;
    let manufacturer_token_account = &mut ctx.accounts.manufacturer_token_account;
    let vault = &mut ctx.accounts.vault;
    let distributor = &mut ctx.accounts.user;
    let mint = &ctx.accounts.mint;

    require!(
        matches!(user_pda.role, Role::Distributor),
        VplError::UnauhtorizedRole
    );

    require!(batch_pda.quantity > 0, VplError::BatchEmpty);

    transfer_checked(
        CpiContext::new(
            token_program.to_account_info(),
            TransferChecked {
                from: distributor_token_account.to_account_info(),
                mint: mint.to_account_info(),
                to: manufacturer_token_account.to_account_info(),
                authority: distributor.to_account_info(),
            },
        ),
        (batch_pda.quantity * batch_pda.cost_per_piece).into(),
        mint.decimals,
    )?;

    transfer_checked(
        CpiContext::new(
            token_program.to_account_info(),
            TransferChecked {
                from: distributor_token_account.to_account_info(),
                mint: mint.to_account_info(),
                to: vault.to_account_info(),
                authority: distributor.to_account_info(),
            },
        ),
        (batch_pda.quantity * constants::STAKE_PER_PIECE).into(),
        mint.decimals,
    )?;

    batch_pda.status = BatchStatus::StoredByDistributor;

    Ok(())
}

#[derive(Accounts)]
pub struct DistributorReceive<'info> {
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
    #[account(mut)]
    pub distributor_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub manufacturer_token_account: Account<'info, TokenAccount>,
    #[account(
        seeds = [b"vault".as_ref(), batch.key().as_ref(), mint.key().as_ref()],
        bump,
        token::mint = mint,
        token::authority = vault
    )]
    pub vault: Account<'info, TokenAccount>,
    pub mint: Account<'info, Mint>,
    pub token_program: Program<'info, Token>,
}
