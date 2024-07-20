use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Event {
    pub seed: u64,
    pub bump: u8,
    pub creator: Pubkey,
    pub mint: Pubkey,
    #[max_len(4 + 50)]
    pub creator_name: String,
    #[max_len(4 + 30)]
    pub creator_domain: String,
    #[max_len(4 + 100)]
    pub name: String,
    #[max_len(4 + 50)]
    pub banner: String,
    pub date_time: u64,
    #[max_len(4 + 50)]
    pub location: String,
    pub ticket_price: u64,
    pub tickets_sold: u8,
    pub total_tickets: u8,
}