#![allow(clippy::result_large_err)]

use anchor_lang::prelude::*;
use std::ops::DerefMut;

pub mod states;
pub use states::*;

pub mod zkonnect_utils;
pub use zkonnect_utils::*;

declare_id!("D57msu1skRML54zj1pfZ2fzewCx9UPveeT29hys94jrk");

#[program]
pub mod zkonnect {
    use super::*;

    #[allow(clippy::too_many_arguments)]
    pub fn create_event(
        ctx: Context<CreateEvent>,
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
    ) -> Result<()> {
        ctx.accounts.create_event(
            &ctx.bumps,
            event_name,
            creator_name,
            creator_domain,
            event_description,
            banner,
            date_time,
            location,
            nfturi,
            ticket_price,
            total_tickets,
            pay_sol,
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

    pub fn close_account(ctx: Context<CloseAccount>) -> Result<()> {
        let account = ctx.accounts.account.to_account_info();

        let dest_starting_lamports = ctx.accounts.receiver.lamports();

        **ctx.accounts.receiver.lamports.borrow_mut() = dest_starting_lamports
            .checked_add(account.lamports())
            .unwrap();
        **account.lamports.borrow_mut() = 0;

        let mut data = account.try_borrow_mut_data()?;
        for byte in data.deref_mut().iter_mut() {
            *byte = 0;
        }

        // let dst: &mut [u8] = &mut data;
        // let mut cursor = std::io::Cursor::new(dst);
        // cursor
        //     .write_all(&anchor_lang::__private::CLOSED_ACCOUNT_DISCRIMINATOR)
        //     .unwrap();

        Ok(())
    }
}
