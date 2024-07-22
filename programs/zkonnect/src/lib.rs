#![allow(clippy::result_large_err)]

use anchor_lang::prelude::*;

pub mod states;
pub use states::*;

pub mod zkonnect_utils;
pub use zkonnect_utils::*;

declare_id!("2Wqc5L2npVrhRunpZMEzzby5K9cAEBpSUv2juWb5UPPC");

#[program]
pub mod zkonnect {
    use super::*;

    #[allow(clippy::too_many_arguments)]
    pub fn create_event(
        ctx: Context<CreateEvent>,
        name: String,
        creator_name: String,
        creator_domain: String,
        event_description: String,
        banner: String,
        date_time: u64,
        location: String,
        ticket_price: u64,
        total_tickets: u8,
        pay_sol: u8,
    ) -> Result<()> {
        ctx.accounts.create_event(
            &ctx.bumps,
            name,
            creator_name,
            creator_domain,
            event_description,
            banner,
            date_time,
            location,
            ticket_price,
            total_tickets,
            pay_sol
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
