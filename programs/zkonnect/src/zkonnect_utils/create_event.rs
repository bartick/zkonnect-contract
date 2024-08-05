use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{Mint, TokenInterface},
};

use crate::states::Event;

#[derive(Accounts)]
#[instruction(event_name: String)]
pub struct CreateEvent<'info> {
    #[account(mut)]
    pub creator: Signer<'info>,
    #[account(
        mint::token_program = token_program
    )]
    pub mint: InterfaceAccount<'info, Mint>,
    #[account(mut)]
    /// CHECK: This is not dangerous because we don't read or write from this account
    pub merkle_tree: UncheckedAccount<'info>,
    #[account(mut)]
    pub collection_nft: InterfaceAccount<'info,Mint>,
    #[account(
        init, 
        payer = creator, 
        space = 8 + Event::INIT_SPACE,
        seeds = [b"zkonnect".as_ref(), creator.key().as_ref(), event_name.as_bytes().as_ref()],
        bump
    )]
    pub event: Account<'info, Event>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}

impl<'info> CreateEvent<'info> {
    pub fn create_event(
        &mut self,
        bumps: &CreateEventBumps,
        event_name: String,
        creator_name: String,
        creator_domain: String,
        event_description: String,
        banner: String,
        date_time: u64,
        location: String,
        nfturi: String,
        ticket_price: u64,
        total_tickets: u8,
        pay_sol: u8,
    ) {
        self.event.set_inner(Event {
            bump: bumps.event,
            event_name,
            creator_name,
            creator_domain,
            event_description,
            banner,
            date_time,
            location,
            nfturi,
            ticket_price,
            mint: self.mint.key(),
            creator: self.creator.key(),
            merkle_tree: self.merkle_tree.key(),
            collection_nft: self.collection_nft.key(),
            tickets_sold: 0,
            total_tickets,
            pay_sol,
        });
    }
}
