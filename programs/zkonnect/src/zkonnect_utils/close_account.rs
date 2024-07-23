use anchor_lang::prelude::*;

use crate::states::Event;

#[derive(Accounts)]
pub struct CloseAccount<'info> {
    #[account(
        mut, 
        close = receiver
    )]
    pub account: Account<'info, Event>,
    #[account(mut)]
    pub receiver: SystemAccount<'info>,
}
