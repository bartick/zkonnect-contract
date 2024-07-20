#![allow(clippy::result_large_err)]

use anchor_lang::prelude::*;

pub mod states;
pub use states::*;

pub mod zkonnect_utils;
pub use zkonnect_utils::*;

declare_id!("GnD1KfuNYzxg7y9zxuS9u9cFFzPxQRPfauBKuWogbc5P");

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
        banner: String,
        date_time: u64,
        location: String,
        ticket_price: u64,
        total_tickets: u8
    ) -> Result<()> {
        ctx.accounts.create_event(
            seed,
            &ctx.bumps,
            creator_name,
            creator_domain,
            name,
            banner,
            date_time,
            location,
            ticket_price,
            total_tickets
        );
        Ok(())
    }

    pub fn pay_sol_for_ticket(ctx: Context<PaySolForTicket>) -> Result<()> {
        ctx.accounts.buy_ticket()?;
        Ok(())
    }

    pub fn pay_for_ticket(ctx: Context<PayForTicket>, decimal: u32) -> Result<()> {
        ctx.accounts.buy_ticket(decimal)?;
        Ok(())
    }
}