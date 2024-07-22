use anchor_lang::error_code;

#[error_code]
pub enum MyError {
    #[msg("Pay Sol not enabled")]
    PaySolNotEnabled,
    #[msg("Ticket sold out")]
    TicketSoldOut,
    #[msg("Only SOL is accepted")]
    PayOnlySol,
}