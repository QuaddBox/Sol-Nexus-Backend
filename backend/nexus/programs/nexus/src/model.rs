use anchor_lang::prelude::*;

#[account]
pub struct UserProfile {
    pub authority : Pubkey,
    pub name : String,
    pub email : String,
    pub password : String,
    pub date : String,
    pub total_ticket : u8,
    pub events_created : u8,
}

#[account]
pub struct Event {
    pub authority : Pubkey,
    pub creator : String,
    pub id: u32,
    pub name: String,
    pub category: Vec<String>,
    pub location: String,
    pub tickets_available: u32,
    pub ticket_price: u64,
    pub purchasers : Vec<Pubkey>,
    pub starting_date : String,
    pub ending_date ; String,
    pub status : Status
} 

#[accounts]
pub struct Ticketing {
    pub events: Vec<Event>,
    pub tickets: Vec<Ticket>,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub enum TicketStatus {
    Valid,
    Invalid,
    Transferred(Pubkey),
}

#[accounts]
pub struct Ticket {
    pub event_id: u32,
    pub owner: Pubkey,
    pub status: TicketStatus,
}


#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub enum Status {
    Available,
    InProgress,
    Closed,
}


#[account]
#[derive(Default)]
pub struct TokenState {
    pub bump : u8,
    pub amount : u64
}