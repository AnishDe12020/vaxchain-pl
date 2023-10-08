use std::str::FromStr;

use anchor_lang::prelude::*;
use anchor_spl::token::{TokenAccount, Mint, Token};

use crate::{state::{
    user::{Role, User},
    batch::{Batch, BatchStatus},  
}, constants::VAX_TOKEN_MINT};

use crate::errors::VplError;

pub fn create_batch_ix(ctx: Context<CreateBatch>, expires_at: i64, temp_min: u16, temp_max: u16, cost_per_piece: u16, quantity: u16) -> Result<()> {
    let user_pda = &mut ctx.accounts.user_pda;
    let batch_pda = &mut ctx.accounts.batch_pda;  
    let mint = &ctx.accounts.mint;

    require!(mint.key() == Pubkey::from_str(VAX_TOKEN_MINT).unwrap(), VplError::InvalidMint);

    require!(matches!(user_pda.role, Role::Manufacturer), VplError::UnauhtorizedRole);

    let clock: Clock =  Clock::get()?;

    require!(clock.unix_timestamp < expires_at, VplError::InvalidExpirationDate);

    batch_pda.pubkey = *ctx.accounts.batch.key;
    batch_pda.manufacturer = *ctx.accounts.user.key;
    batch_pda.manufactured_at = clock.unix_timestamp;
    batch_pda.expires_at = expires_at;
    batch_pda.temp_min = temp_min;
    batch_pda.temp_max = temp_max;
    batch_pda.cost_per_piece = cost_per_piece;
    batch_pda.status = BatchStatus::Manufactured;
    batch_pda.quantity = quantity;
    batch_pda.start_date = 0;
    batch_pda.stop_date = 0;

    Ok(())
}

#[derive(Accounts)]
pub struct CreateBatch<'info> {
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
        init, 
        seeds = [b"batch".as_ref(), batch.key().as_ref()],
        payer = user,
        bump,
        space = Batch::LEN
    )]
    pub batch_pda: Account<'info, Batch>,
    #[account(
        init,
        payer = user,
        seeds = [b"vault".as_ref(), batch.key().as_ref(), mint.key().as_ref()],
        bump,
        token::mint = mint,
        token::authority = batch_pda
    )]
    pub vault: Account<'info, TokenAccount>,
    pub mint: Account<'info, Mint>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}
