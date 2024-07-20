use anchor_lang::prelude::*;
use solana_program::system_instruction;

use crate::Event;

#[derive(Accounts)]
pub struct PaySolForTicket<'info> {
    #[account(mut)]
    pub from: Signer<'info>,
    #[account(mut)]
    pub to: SystemAccount<'info>,
    #[account(
        mut,
        seeds=[b"zconnect", to.key().as_ref(), event.seed.to_le_bytes().as_ref()],
        bump = event.bump,
    )]
    pub event: Account<'info, Event>,
    pub system_program: Program<'info, System>,
}

impl <'info> PaySolForTicket<'info> {
    pub fn buy_ticket(&mut self) -> Result<()> {
        // Create the transfer instruction
        let transfer_instruction = system_instruction::transfer(
            self.from.key, 
            self.to.key, 
            self.event.ticket_price
        );

        anchor_lang::solana_program::program::invoke(
            &transfer_instruction,
            &[
                self.from.to_account_info(),
                self.to.to_account_info(),
                self.system_program.to_account_info(),
            ],
        )?;

        self.event.tickets_sold += 1;

        Ok(())
    }
    
}