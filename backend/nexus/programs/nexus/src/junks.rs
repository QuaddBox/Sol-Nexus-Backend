pub fn buy_ticket(ctx: Context<BuyTicket>, event_id: u32, events: &mut Vec<Event>, tickets: &mut Vec<Ticket>) -> ProgramResult {
    let event = &mut ctx.accounts.event_account;

    if event.tickets_available == 0 {
        return Err(ProgramError::Custom(1)); // No tickets available
    }

    let ticket_price = event.ticket_price;
    token::transfer(ctx.accounts.into(), ctx.accounts.from.clone(), ticket_price)?;

    let ticket = Ticket {
        event_id,
        owner: *ctx.accounts.from.key,
        status: TicketStatus::Valid,
    };
    tickets.push(ticket);

    event.tickets_available -= 1;
    Ok(())
}

/ pub fn new(ctx: Context<Initialize>, cat: Vec<String>, num_events: u32) -> ProgramResult {
    //     let events = vec![
    //         Event {
    //             id: 0,
    //             name: "Example Event".to_string(),
    //             category: cat,
    //             location: "Nigeria".to_string(),
    //             tickets_available: 100,
    //             ticket_price: 100, // Price in lamports
    //         };
    //         num_events as usize
    //     ];
    //     Ok(())
    // }


pub fn transfer_ticket(ctx: Context<TransferTicket>, ticket_id: u32, new_owner: Pubkey, tickets: &mut Vec<Ticket>) -> ProgramResult {
    let ticket = tickets.get_mut(ticket_id as usize).ok_or(ProgramError::InvalidArgument)?;

    if ticket.status != TicketStatus::Valid {
        return Err(ProgramError::Custom(2)); // Ticket not valid
    }

    if ticket.owner != *ctx.accounts.authority.key {
        return Err(ProgramError::Custom(3)); // Not ticket owner
    }

    ticket.owner = new_owner;
    ticket.status = TicketStatus::Transferred(new_owner);
    Ok(())
}

pub fn validate_ticket(ctx: Context<ValidateTicket>, ticket_id: u32, tickets: &Vec<Ticket>) -> ProgramResult {
    let ticket = tickets.get(ticket_id as usize).ok_or(ProgramError::InvalidArgument)?;

    match ticket.status {
        TicketStatus::Valid => Ok(()),
        _ => Err(ProgramError::Custom(4)), // Ticket not valid
    }
}

pub fn is_ticket_from_organization(ctx: Context<IsTicketFromOrganization>, ticket_id: u32, tickets: &Vec<Ticket>, organization_pubkey: Pubkey) -> ProgramResult {
    let ticket = tickets.get(ticket_id as usize).ok_or(ProgramError::InvalidArgument)?;

    if ticket.owner == organization_pubkey {
        Ok(())
    } else {
        Err(ProgramError::Custom(5)) // Ticket not from organization
    }
}




#[derive(Accounts)]
pub struct CreateEvent<'info> {
    #[account(init, payer = user, space = 256)]
    pub event_account: Account<'info, Event>,
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct BuyTicket<'info> {
    #[account(mut)]
    pub from: Account<'info, TokenAccount>,
    #[account(mut)]
    pub to: Account<'info, TokenAccount>,
    pub authority: Signer<'info>,
    pub token_program: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct TransferTicket<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
}

#[derive(Accounts)]
pub struct ValidateTicket<'info> {}

#[derive(Accounts)]
pub struct IsTicketFromOrganization<'info> {
    pub authority: AccountInfo<'info>,
}

#[derive(Accounts, AnchorSerialize, AnchorDeserialize, Clone)]
pub struct Event {
    pub id: u32,
    pub name: String,
    pub category: Vec<String>,
    pub location: String,
    pub tickets_available: u32,
    pub ticket_price: u64,
}

//     pub fn new(_ctx: Context<Initialize>, cat : Vec<String>, num_events: u32) -> ProgramResult {
//         let events = vec![Event {
//             id: 0,
//             name: "Example Event".to_string(),
//             category : cat,
//             location : "Nigeria",
//             tickets_available: 100,
//             ticket_price: 100, // Price in lamports
//         }; num_events as usize];
//         Ok(())

//         // Ok(Self {
//         //     events,
//         //     tickets: Vec::new(),
//         // })
//     }

//     // pub fn create_event(&mut self, ctx: Context<CreateEvent>, name: String, category : Vec<String>, tickets_available: u32, ticket_price: u64, location : String) -> ProgramResult {
//     //     let new_event = Event {
//     //         id: self.events.len() as u32,
//     //         name,
//     //         category,
//     //         location,
//     //         tickets_available,
//     //         ticket_price,
//     //     };
//     //     self.events.push(new_event);
//     //     Ok(())
//     // }

//     pub fn create_event(ctx: Context<CreateEvent>, name: String, tickets_available: u32, ticket_price: u64, location: String, events: &mut Vec<Event>) -> ProgramResult {
//         let new_event = Event {
//             id: events.len() as u32,
//             name,
//             category,
//             location,
//             tickets_available,
//             ticket_price,
//         };
//         events.push(new_event);
//         Ok(())
//     }

//     // pub fn buy_ticket(ctx: Context<BuyTicket>, event_id: u32, events: &mut Vec<Event>, tickets: &mut Vec<Ticket>) -> ProgramResult {
//     //     let event = events.get_mut(event_id as usize).ok_or(ProgramError::InvalidArgument)?;

//     //     if event.tickets_available == 0 {
//     //         return Err(ProgramError::Custom(1)); // No tickets available
//     //     }

//     //     let ticket_price = event.ticket_price;
//     //     token::transfer(ctx.accounts.into(), ctx.accounts.from.clone(), ticket_price)?;

//     //     let ticket = Ticket {
//     //         event_id,
//     //         owner: *ctx.accounts.from.key,
//     //         status: TicketStatus::Valid,
//     //     };
//     //     tickets.push(ticket);

//     //     event.tickets_available -= 1;
//     //     Ok(())
//     // }

//     pub fn buy_ticket(ctx: Context<BuyTicket>, event_id: u32,  events: &mut Vec<Event>, tickets: &mut Vec<Ticket>) -> ProgramResult {
//         let event = events.get_mut(event_id as usize).ok_or(ProgramError::InvalidArgument)?;

//         if event.tickets_available == 0 {
//             return Err(ProgramError::Custom(1)); // No tickets available
//         }

//         let ticket_price = event.ticket_price;
//         token::transfer(ctx.accounts.into(), ctx.accounts.from.clone(), ticket_price)?;

//         let ticket = Ticket {
//             event_id,
//             owner: *ctx.accounts.from.key,
//             status: TicketStatus::Valid,
//         };
//         self.tickets.push(ticket);

//         event.tickets_available -= 1;
//         Ok(())
//     }

//     // pub fn transfer_ticket(ctx: Context<TransferTicket>, ticket_id: u32, new_owner: Pubkey, tickets: &mut Vec<Ticket>) -> ProgramResult {
//     //     let ticket = tickets.get_mut(ticket_id as usize).ok_or(ProgramError::InvalidArgument)?;

//     //     if ticket.status != TicketStatus::Valid {
//     //         return Err(ProgramError::Custom(2)); // Ticket not valid
//     //     }

//     //     if ticket.owner != *ctx.accounts.authority.key {
//     //         return Err(ProgramError::Custom(3)); // Not ticket owner
//     //     }

//     //     ticket.owner = new_owner;
//     //     ticket.status = TicketStatus::Transferred(new_owner);
//     //     Ok(())
//     // }


//     pub fn transfer_ticket(ctx: Context<TransferTicket>, ticket_id: u32, new_owner: Pubkey, tickets: &mut Vec<Ticket>) -> ProgramResult {
//         let ticket = tickets.get_mut(ticket_id as usize).ok_or(ProgramError::InvalidArgument)?;

//         if ticket.status != TicketStatus::Valid {
//             return Err(ProgramError::Custom(2)); // Ticket not valid
//         }

//         if ticket.owner != *ctx.accounts.authority.key {
//             return Err(ProgramError::Custom(3)); // Not ticket owner
//         }

//         ticket.owner = new_owner;
//         ticket.status = TicketStatus::Transferred(new_owner);
//         Ok(())
//     }

//     // pub fn validate_ticket(&self, _ctx: Context<ValidateTicket>, ticket_id: u32) -> ProgramResult {
//     //     let ticket = self.tickets.get(ticket_id as usize).ok_or(ProgramError::InvalidArgument)?;

//     //     match ticket.status {
//     //         TicketStatus::Valid => Ok(()),
//     //         _ => Err(ProgramError::Custom(4)), // Ticket not valid
//     //     }
//     // }

//     pub fn validate_ticket(_ctx: Context<ValidateTicket>, ticket_id: u32, tickets: &Vec<Ticket>) -> ProgramResult {
//         let ticket = tickets.get(ticket_id as usize).ok_or(ProgramError::InvalidArgument)?;

//         match ticket.status {
//             TicketStatus::Valid => Ok(()),
//             _ => Err(ProgramError::Custom(4)), // Ticket not valid
//         }
//         Ok(())
//     }


//     // pub fn is_ticket_from_organization(&self, ctx: Context<IsTicketFromOrganization>, ticket_id: u32) -> ProgramResult {
//     //     let ticket = self.tickets.get(ticket_id as usize).ok_or(ProgramError::InvalidArgument)?;

//     //     Ok(())
//     // }

//     pub fn is_ticket_from_organization(_ctx: Context<IsTicketFromOrganization>, ticket_id: u32, tickets: &Vec<Ticket>, organization_pubkey: Pubkey) -> ProgramResult {
//         let ticket = tickets.get(ticket_id as usize).ok_or(ProgramError::InvalidArgument)?;

//         if ticket.owner == organization_pubkey {
//             Ok(())
//         } else {
//             Err(ProgramError::Custom(5)) // Ticket not from organization
//         }
//     }
// }


// #[derive(Accounts)]
// pub struct Ticketing {
//     pub events: Vec<Event>,
//     pub tickets: Vec<Ticket>,
// }

// #[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
// pub enum TicketStatus {
//     Valid,
//     Invalid,
//     Transferred(Pubkey),
// }

// #[derive(Accounts)]
// pub struct Ticket {
//     pub event_id: u32,
//     pub owner: Pubkey,
//     pub status: TicketStatus,
// }

// #[derive(Accounts)]

// pub struct Initialize {
//     pub name : String,
//     pub email : String,
//     pub password : String,
// }

// #[derive(Accounts)]
// pub struct CreateEvent<'info> {
//     #[account(init, payer = user, space = 256)]
//     pub event_account: Account<'info, Event>,
//     #[account(mut)]
//     pub user: Signer<'info>,
//     pub system_program: Program<'info, System>,
// }

// #[derive(Accounts)]
// pub struct BuyTicket<'info> {
//     #[account(mut)]
//     pub from: Account<'info, TokenAccount>,
//     #[account(mut)]
//     pub to: Account<'info, TokenAccount>,
//     pub authority: Signer<'info>,
//     pub token_program: AccountInfo<'info>,
// }

// #[derive(Accounts)]
// pub struct TransferTicket<'info> {
//     #[account(mut)]
//     pub authority: Signer<'info>,
// }

// #[derive(Accounts)]
// pub struct ValidateTicket<'info> {
    
// }

// #[derive(Accounts)]
// pub struct IsTicketFromOrganization<'info> {
//     pub authority: AccountInfo<'info>,
// }

// #[derive(Accounts, AnchorSerialize, AnchorDeserialize, Clone)]
// pub struct Event {
//     pub id: u32,
//     pub name: String,
//     pub category : Vec<String>,
//     pub location : String,
//     pub tickets_available: u32,
//     pub ticket_price: u64,
// }



use anchor_lang::prelude::*;
use anchor_lang::solana_program::{program_error::ProgramError};
use anchor_lang::solana_program::entrypoint::ProgramResult;
use anchor_spl::token::{self, TokenAccount, Transfer};

pub mod models;
pub mod error;
pub mod constants;

use crate::{constants::*, error::*, models::*};

// This is your program's public key and it will update
// automatically when you build the project.
declare_id!("9Jq4s9ApC5rvWUn5Y8wat1j9zwbNdV2vLcSCD2qyucAD");

#[program]
pub mod nexus {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, _name : String, _email : String, _password : String, _date : String) -> Result<()> {
        let profile  = &mut ctx.accounts.user_profile; 
        profile.authority = ctx.accounts.autority.key();
        profile.name = _name.to_string();
        profile.email = _email.to_string();
        profile.password = _password.to_string();
        profile.date = _date.to_string();
        profile.total_ticket = 0;
        profile.events_created = 0;        
        Ok(())
    }

    pub fn transfer_lamports(ctx: Context<TransferLamports>, amount: u64) -> Result<()> {
        let from_account = &ctx.accounts.from;
        let to_account = &ctx.accounts.to;

        // Create the transfer instruction
        let transfer_instruction =
            system_instruction::transfer(from_account.key, to_account.key, amount);

        // Invoke the transfer instruction
        anchor_lang::solana_program::program::invoke_signed(
            &transfer_instruction,
            &[
                from_account.to_account_info(),
                to_account.clone(),
                ctx.accounts.system_program.to_account_info(),
            ],
            &[],
        )?;

        Ok(())
    }

    pub fn update_event_count(ctx : Context<UpdateEventCount>, _count : u8) -> Result<()> {
        let profile = &mut ctx.accounts.user_profile;

        profile.events_created = _count;
        Ok(())
    }


    pub fn create_event(ctx: Context<CreateEvent>, _id : u8, _name: String, _tickets_available: u32, _ticket_price: u64, _location: String, _events: &mut Vec<Event>, _category : &mut Vec<String>, _starting_date : String, _ending_date : String, _event_type : String) -> ProgramResult {
        let event =  &mut ctx.accounts.event_account;
        let profile =  &mut ctx.accounts.user_profile;
        event.id = profile.events_created;
        profile.events_created = profile.events_created.checked_add(1).unwrap();
        event.authority = ctx.accounts.authority.key();
        event.creator = profile.name.clone();
        event.name = _name.to_string();
        event.category = _category;
        event.location = _location.to_string();
        event.tickets_available = _tickets_available;
        event.ticket_price = _ticket_price; // In lamports
        event.purchasers = Vec::with_capacity(_tickets_available.try_into().unwrap());
        event.starting_date = _starting_date;
        event.ending_date = _ending_date;
        event.status = Status::Available; // Available for sale

        // if _event_type == "free" {

        // }

        Ok(())
    }

    pub fn start_event(ctx :Context<StartEvent>, _event_id : u8) -> Result<()> {
        let event = &mut ctx.accounts.event_account;
        require!(
            event.status == Status::InProgress,
            EventError::NotAllowed
        );
        event.status = Status::InProgress;
        Ok(())
   
    }

    pub fn end_event(ctx : Context<EndEvent>, _event_id : u8) -> Result<()> {
        let event = &mut ctx.accounts.event_account;
        require!(
            event.status == Status::Closed,
            EventError::Expired
        );
        event.status = Status::Closed;
        Ok(())
    }
    
}

#[derive(Accounts)]
#[instruction()]
pub struct InitializeUser<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        init,
        seeds = [USER_IDENTIFIER, authority.key().as_ref()],
        bump,
        payer = authority,
        space = 8 + std::mem::size_of::<UserProfile>(),
    )]
    pub user_profile: Box<Account<'info, UserProfile>>,

    pub system_program: Program<'info, System>,
}


#[derive(Accounts)]
#[instruction()]
pub struct CreateEvent<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        mut,
        seeds = [USER_IDENTIFIER, authority.key().as_ref()],
        bump,
        has_one = authority,
    )]
    pub user_profile: Box<Account<'info, UserProfile>>,

    #[account(
        init,
        seeds = [EVENT_IDENTIFIER, &[user_profile.events_created as u8].as_ref()],
        bump,
        payer = authority,
        space = 8 + std::mem::size_of::<Event>(),
    )]
    pub event_account: Box<Account<'info, Event>>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction()]
pub struct UpdateEventCount<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        mut,
        seeds = [USER_IDENTIFIER, authority.key().as_ref(),],
        bump,
        has_one = authority,
    )]
    pub user_profile: Box<Account<'info, UserProfile>>,

    pub system_program: Program<'info, System>,
}


#[derive(Accounts)]
#[instruction(event_id: u8)]
pub struct StartEvent<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        mut,
        seeds = [USER_IDENTIFIER, authority.key().as_ref(),],
        bump,
    )]
    pub user_profile: Box<Account<'info, UserProfile>>,

    #[account(
        mut,
        seeds = [EVENT_IDENTIFIER, &[event_id].as_ref()],
        bump,
    )]
    pub event_account: Box<Account<'info, Event>>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(event_id: u8)]
pub struct EndEvent<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        mut,
        seeds = [USER_IDENTIFIER, authority.key().as_ref(),],
        bump,
        has_one = authority,
    )]
    pub user_profile: Box<Account<'info, UserProfile>>,

    #[account(
        mut,
        seeds = [EVENT_IDENTIFIER, &[event_id].as_ref()],
        bump,
        has_one = authority,
    )]
    pub event_account: Box<Account<'info, Event>>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(event_id : u8)]
pub struct PurchaseTicketTournament<'info> {
    #[account(mut)]
    pub from: Signer<'info>,

    #[account(
        mut,
        seeds = [USER_IDENTIFIER, authority.key().as_ref()],
        bump,
    )]
    pub user_profile: Box<Account<'info, UserProfile>>,

    #[account(
        mut,
        seeds = [EVENT_IDENTIFIER, &[event_id].as_ref()],
        bump,
    )]
    pub event_account: Box<Account<'info, Event>>,

    // #[account(mut)]
    // pub from: Account<'info, TokenAccount>,
    #[account(mut)]
    pub to: Account<'info, TokenAccount>,
    pub authority: Signer<'info>,
    // pub token_program: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct TransferLamports<'info> {
    #[account(mut)]
    pub from: Signer<'info>,
    #[account(mut)]
    pub to: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}

