use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{transfer_checked, TokenAccount, TransferChecked},
    token_interface::{Mint, TokenInterface},
};

use crate::{Event, MyError};

#[derive(Accounts)]
pub struct PayForTicket<'info> {
    #[account(mut)]
    pub from: Signer<'info>,
    #[account(mut)]
    pub to: SystemAccount<'info>,
    #[account(
        mint::token_program = token_program
    )]
    pub mint: InterfaceAccount<'info, Mint>,
    #[account(
        init_if_needed,
        payer = from,
        associated_token::mint = mint,
        associated_token::authority = from,
    )]
    pub from_ata: Box<Account<'info, TokenAccount>>,
    #[account(
        mut,
        associated_token::mint = mint,
        associated_token::authority = to,
    )]
    pub to_ata: Box<Account<'info, TokenAccount>>,
    #[account(
        mut,
        has_one = mint,
        seeds=[b"zkonnect", to.key().as_ref(), event.event_name.as_bytes().as_ref()],
        bump = event.bump,
    )]
    pub event: Account<'info, Event>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}

impl<'info> PayForTicket<'info> {
    pub fn buy_ticket(&mut self) -> Result<()> {
        require!(self.event.pay_sol > 0, MyError::PayOnlySol);

        require!(
            self.event.tickets_sold < self.event.total_tickets,
            MyError::TicketSoldOut
        );

        transfer_checked(
            self.into_deposit_context(),
            self.event.ticket_price,
            self.mint.decimals,
        )?;

        self.event.tickets_sold += 1;

        Ok(())
    }

    fn into_deposit_context(&self) -> CpiContext<'_, '_, '_, 'info, TransferChecked<'info>> {
        let cpi_accounts = TransferChecked {
            from: self.from_ata.to_account_info(),
            mint: self.mint.to_account_info(),
            to: self.to_ata.to_account_info(),
            authority: self.from.to_account_info(),
        };
        CpiContext::new(self.token_program.to_account_info(), cpi_accounts)
    }
}
