use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Event {
    pub bump: u8,
    pub creator: Pubkey,
    pub mint: Pubkey,
    pub merkle_tree: Pubkey,
    pub collection_nft: Pubkey,
    #[max_len(504)]
    pub creator_name: String,
    #[max_len(504)]
    pub creator_domain: String,
    #[max_len(1004)]
    pub event_name: String,
    #[max_len(2004)]
    pub event_description: String,
    #[max_len(504)]
    pub banner: String,
    pub date_time: u64,
    #[max_len(504)]
    pub location: String,
    #[max_len(504)]
    pub nfturi: String,
    pub ticket_price: u64,
    pub tickets_sold: u8,
    pub total_tickets: u8,
    pub pay_sol: u8,
}
