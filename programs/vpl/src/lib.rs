use anchor_lang::prelude::*;

declare_id!("6JuaxB1fEN9n6ApcvRxy6avr25H8qTzGRGpT43qrCvm4");

pub mod state;

#[program]
pub mod vpl {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
