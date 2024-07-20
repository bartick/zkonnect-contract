use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{TokenInterface, Mint},
};

use crate::states::Event;

#[derive(Accounts)]
#[instruction(seed: u64)]
pub struct CreateEvent<'info> {
    #[account(mut)]
    pub creator: Signer<'info>,
    #[account(
        mint::token_program = token_program
    )]
    pub mint: InterfaceAccount<'info, Mint>,
    #[account(
        init, 
        payer = creator, 
        space = 8 + Event::INIT_SPACE,
        seeds = [b"zkonnect".as_ref(), creator.key().as_ref(), &seed.to_le_bytes()],
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
        seed: u64, 
        bumps: &CreateEventBumps,
        creator_name: String,
        creator_domain: String,
        name: String,
        banner: String,
        date_time: i64,
        location: String,
        ticket_price: u64,
        total_tickets: u8
    ) {
        self.event.set_inner(Event {
            seed,
            bump: bumps.event,
            creator_name,
            creator_domain,
            name,
            banner,
            date_time,
            location,
            ticket_price,
            mint: self.mint.key(),
            creator: self.creator.key(),
            tickets_sold: 0,
            total_tickets,
        });
    }
    
}