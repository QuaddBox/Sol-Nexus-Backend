use anchor_lang::prelude::*;
// use anchor_lang::solana_program::{program_error::ProgramError};
use anchor_lang::solana_program::entrypoint::ProgramResult;
use anchor_spl::token::{self, Token, TokenAccount, Transfer};

use solana_program::system_instruction;

pub mod models;
pub mod error;
pub mod constants;

use crate::{constants::*, error::*, models::*};

// This is your program's public key and it will update
// automatically when you build the project.
declare_id!("CiYL3S3NbUhBXzdW5gYnr7bgMnpsT5yiB5TKv92Gc9pw");


#[program]

mod solnexus {
    
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, _name : String, _test : String, _avatar : String, _email : String, _password : String, _date : String) -> Result<()> {
        let profile  = &mut ctx.accounts.user_profile; 
        profile.authority = ctx.accounts.authority.key();
        profile.name = _name.to_string();
        profile.avatar = _avatar.to_string();
        profile.email = _email.to_string();
        // profile.password = _password.to_string();
        profile.date = _date.to_string();
        profile.total_ticket = 0;
        profile.events_created = 0; 
        msg!(" User Initialised! ");
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

    pub fn update_event_count(ctx : Context<UpdateEventCount>, _count : u32) -> Result<()> {
        let profile = &mut ctx.accounts.user_profile;

        profile.events_created = _count;
        Ok(())
    }


    pub fn create_event<'a>(ctx: Context<CreateEvent>, _id : u8, _name: String, _tickets_available: u32, _ticket_price: u64, _location: String, _category : Vec<String>, _starting_date : String, _ending_date : String, _event_type : String) -> Result<()> {
        let event =  &mut ctx.accounts.event_account;
        let profile =  &mut ctx.accounts.user_profile;
        event.id = profile.events_created;
        profile.events_created = profile.events_created.checked_add(1).unwrap();
        event.authority = ctx.accounts.authority.key();
        event.creator = profile.name.clone();
        event.name = _name.to_string();
        event.category = _category.to_vec();
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

    pub fn validate_ticket(ctx: Context<ValidateTicket>, ticket_id: u32) -> Result<()> {
        // let ticket = ctx.accounts.tickets.get(ticket_id as usize).ok_or(EventError::InvalidTicket)?;
        let event = &mut ctx.accounts.event_account;

        if event.status == Status::Closed {
            return err!(EventError::EventClosed)
        };
        
        let ticket = &mut ctx.accounts.ticket_account;

        match ticket.status {
            TicketStatus::Valid => Ok(()),
            _ => err!(EventError::InvalidTicket),
        }


    }

    pub fn buy_ticket(ctx: Context<PurchaseTicketTournament>, _ticket_hash : String) -> Result<()> {
        // let events = ctx.accounts.events.get_mut(event_id as usize).ok_or(EventError::InvalidEvent)?;

        let event = &mut ctx.accounts.event_account;
        let profile = &mut ctx.accounts.user_profile;


        if event.tickets_available == 0 {
            return err!(EventError::NoTicketsAvailable);
        }

        let ticket_price = event.ticket_price;
        // token::transfer(ctx.accounts.into(), ctx.accounts.from.clone(), ticket_price)?;

        let ticket = &mut ctx.accounts.ticket_account;

        ticket.event_id = event.id;
        ticket.ticket_hash = _ticket_hash.to_string();
        ticket.owner = ctx.accounts.authority.key();
        ticket.status = TicketStatus::Valid;

        profile.total_ticket = profile.total_ticket.checked_add(1).unwrap();

        event.tickets_available -= 1;
        Ok(())
    }

    pub fn transfer_ticket(ctx: Context<TransferTicket>, new_owner: Pubkey) -> Result<()> {
        // let ticket = ctx.accounts.tickets.get_mut(ticket_id as usize).ok_or(EventError::InvalidTicket)?;

        let ticket = &mut ctx.accounts.ticket_account;

        if ticket.owner != ctx.accounts.authority.key() {
            return err!(EventError::NotTicketOwner);
        }

        ticket.owner = new_owner;
        ticket.status = TicketStatus::Transferred(new_owner);
        Ok(())
    }
    
}

#[derive(Accounts)]
#[instruction()]
pub struct Initialize<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        init,
        seeds = [USER_IDENTIFIER, authority.key().as_ref()],
        bump,
        payer = authority,
        space = 8 + 2312 + 2312
        // std::mem::size_of::<UserProfile>(),
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
#[instruction(ticket_hash: String, event_id : u8)]
pub struct ValidateTicket<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

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

    #[account{
        mut,
        seeds = [TICKET_IDENTIFIER, authority.key().as_ref()],
        bump,
    }]
    pub ticket_account : Box<Account<'info, Ticket>>,

    pub system_program: Program<'info, System>,

    // #[account(mut)]
    // pub tickets: Vec<Ticket>,
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

    #[account{
        mut,
        seeds = [TICKET_IDENTIFIER, authority.key().as_ref()],
        bump,
    }]
    pub ticket_account : Box<Account<'info, Ticket>>,

    // #[account(mut)]
    // pub from: Account<'info, TokenAccount>,
    #[account(mut)]
    pub to: Account<'info, TokenAccount>,
    pub authority: Signer<'info>,
    // pub token_program: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
    // pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
#[instruction(ticket_hash: String, new_owner: Pubkey)]
pub struct TransferTicket<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
     #[account{
        mut,
        seeds = [TICKET_IDENTIFIER, authority.key().as_ref()],
        bump,
    }]
    pub ticket_account : Box<Account<'info, Ticket>>,

    pub system_program: Program<'info, System>,

}

// // #[derive(Accounts)]
// // #[instruction(event_id: u32)]
// // pub struct BuyTicket<'info> {
// //     #[account(mut)]
// //     pub from: Signer<'info>,
// //     #[account(mut)]
// //     pub events: Vec<Event>,
// //     pub system_program: Program<'info, System>,
// //     pub token_program: Program<'info, Token>,
// // }

#[derive(Accounts)]
pub struct TransferLamports<'info> {
    #[account(mut)]
    pub from: Signer<'info>,
    #[account(mut)]
    pub to: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}




