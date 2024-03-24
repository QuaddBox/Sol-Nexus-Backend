use anchor_lang::prelude::*;

#[account]
pub struct UserProfile {
    pub authority : Pubkey,
    pub name : String,
    pub email : String,
    pub password : String,
    pub date : String,
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
    pub status : Status
} 

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub enum Status {
    Available,
    Closed,
}


#[account]
#[derive(Default)]
pub struct TokenState {
    pub bump : u8,
    pub amount : u64
}