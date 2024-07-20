use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Event {
    pub seed: u64,
    pub bump: u8,
    pub creator: Pubkey,
    pub mint: Pubkey,
    #[max_len(504)]
    pub creator_name: String,
    #[max_len(504)]
    pub creator_domain: String,
    #[max_len(1004)]
    pub name: String,
    #[max_len(2004)]
    pub event_description: String,
    #[max_len(504)]
    pub banner: String,
    pub date_time: u64,
    #[max_len(504)]
    pub location: String,
    pub ticket_price: u64,
    pub tickets_sold: u64,
    pub total_tickets: u64,
}