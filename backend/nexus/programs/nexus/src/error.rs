use anchor_lang::prelude::*;

#[error_code]
pub enum EventError {
    #[msg("Ticket is not Valid")]
    InvalidTicket,
    #[msg("Not allowed")]
    NotAllowed,
    #[msg("Event has ended! ")]
    EventClosed,
    #[msg("You are not the owner of this ticket")]
    NotTicketOwner,
    #[msg("Ticket has expired")]
    Expired,
    #[msg("Math operation overflow")]
    MathOverflow,
    #[msg("No ticket available")]
    NoTicketsAvailable
}