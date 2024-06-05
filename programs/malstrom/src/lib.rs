use anchor_lang::prelude::*;

declare_id!("EmzwvnjmLCMmpbgVtQw7NYpyyk8P4d2UyJmiZKeFujKh");

#[program]
pub mod malstrom {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
