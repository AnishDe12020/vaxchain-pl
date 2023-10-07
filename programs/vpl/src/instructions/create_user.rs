use anchor_lang::prelude::*;

use crate::state::user::{User, Role};

pub fn create_user_ix(ctx: Context<CreateUser>, role: Role) -> Result<()> {
    let user_pda = &mut ctx.accounts.user_pda;  

    let clock: Clock =  Clock::get()?;

    user_pda.pubkey = *ctx.accounts.user.key;
    user_pda.created_at = clock.unix_timestamp;
    user_pda.updated_at = clock.unix_timestamp;
    user_pda.role = role;

    Ok(())
}

#[derive(Accounts)]
pub struct CreateUser<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(
        init, 
        seeds = [b"user".as_ref(), user.key().as_ref()],
        payer = user,
        bump,
        space = User::LEN
    )]
    pub user_pda: Account<'info, User>,
    pub system_program: Program<'info, System>,
}