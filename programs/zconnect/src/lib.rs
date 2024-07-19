use anchor_lang::prelude::*;

declare_id!("EnELF8Dv1i1Lr6JQYt8UyRJqfR32PJoCeCBNdtijhWXv");

#[program]
pub mod zconnect {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
