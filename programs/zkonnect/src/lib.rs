#![allow(clippy::result_large_err)]

use anchor_lang::prelude::*;

pub mod states;
pub use states::*;

pub mod zkonnect_utils;
pub use zkonnect_utils::*;

declare_id!("4DgWhPv3V23cav3LwS5FaxXqxja8zbV58Hw7vhBXEtz4");

#[program]
pub mod zkonnect {
    use super::*;

    #[allow(clippy::too_many_arguments)]
    pub fn create_event(
        ctx: Context<CreateEvent>,
        seed: u64,
        creator_name: String,
        creator_domain: String,
        name: String,
        event_description: String,
        banner: String,
        date_time: u64,
        location: String,
        ticket_price: u64,
        total_tickets: u64,
    ) -> Result<()> {
        ctx.accounts.create_event(
            seed,
            &ctx.bumps,
            creator_name,
            creator_domain,
            name,
            event_description,
            banner,
            date_time,
            location,
            ticket_price,
            total_tickets,
        );
        Ok(())
    }

    pub fn pay_sol_for_ticket(ctx: Context<PaySolForTicket>) -> Result<()> {
        ctx.accounts.buy_ticket()?;
        Ok(())
    }

    pub fn pay_for_ticket(ctx: Context<PayForTicket>) -> Result<()> {
        ctx.accounts.buy_ticket()?;
        Ok(())
    }
}
