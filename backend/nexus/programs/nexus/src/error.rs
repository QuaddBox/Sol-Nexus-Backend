use anchor_lang::prelude::*;

#[error_code]
pub enum EventError {
    #msg("Ticket is not Valid")
    InvalidTicket
    #msg("Not allowed")
    NotAllowed
    #msg("Ticket has expired")
    Expired
    #[msg("Math operation overflow")]
    MathOverflow
}