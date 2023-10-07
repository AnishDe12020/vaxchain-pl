use anchor_lang::prelude::*;

declare_id!("6JuaxB1fEN9n6ApcvRxy6avr25H8qTzGRGpT43qrCvm4");

pub mod instructions;
pub mod state;

use crate::state::user::Role;

use instructions::create_user::*;

#[program]
pub mod vpl {

    use super::*;

    pub fn create_user(ctx: Context<CreateUser>, role: Role) -> Result<()> {
        create_user_ix(ctx, role)
    }
}
