use anchor_lang::prelude::*;

pub mod states;
pub use states::*;

pub mod zkonnect_utils;
pub use zkonnect_utils::*;

declare_id!("DJKkDHARbjNUe2WucgNkwY2dJEFtkMh9urf8dB62cSuP");

#[program]
pub mod zkonnect {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }

    pub fn create_event(
        ctx: Context<CreateEvent>, 
        seed: u64,
        creator_name: String,
        creator_domain: String,
        name: String,
        banner: String,
        date_time: i64,
        location: String,
        ticket_price: u64
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
            ticket_price
        );
        Ok(())
    }

    // pub fn pay_for_ticket(ctx: Context<PayForTicket>, ticket_price: u64) -> Result<()> {
    //     let event = &mut ctx.accounts.event;
    //     event.ticket_price = ticket_price;
    //     Ok(())
    // }
}

#[derive(Accounts)]
pub struct Initialize {}
