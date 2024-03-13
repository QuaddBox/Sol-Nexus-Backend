use anchor_lang::prelude::*;
use anchor_lang::solana_program::{program_error::ProgramError};
use anchor_lang::solana_program::entrypoint::ProgramResult;
use anchor_spl::token::{self, TokenAccount, Transfer};

declare_id!("FcNNxr9x2FHRtAyNjzePDJeu6m4ShGkBy9XedjUo1aCX");

#[program]
pub mod nexus {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, _name : String, _email : String) -> Result<()> {
        Ok(())
    }

    pub fn new(_ctx: Context<Initialize>, num_events: u32) -> ProgramResult {
        let events = vec![Event {
            id: 0,
            name: "Example Event".to_string(),
            tickets_available: 100,
            ticket_price: 100, // Price in lamports
        }; num_events as usize];

        Ok(Self {
            events,
            tickets: Vec::new(),
        })
    }

    pub fn create_event(&mut self, ctx: Context<CreateEvent>, name: String, tickets_available: u32, ticket_price: u64, location : String) -> ProgramResult {
        let new_event = Event {
            id: self.events.len() as u32,
            name,
            location,
            tickets_available,
            ticket_price,
        };
        self.events.push(new_event);
        Ok(())
    }

    pub fn buy_ticket(&mut self, ctx: Context<BuyTicket>, event_id: u32) -> ProgramResult {
        let event = self.events.get_mut(event_id as usize).ok_or(ProgramError::InvalidArgument)?;

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
        self.tickets.push(ticket);

        event.tickets_available -= 1;
        Ok(())
    }

    pub fn transfer_ticket(&mut self, ctx: Context<TransferTicket>, ticket_id: u32, new_owner: Pubkey) -> ProgramResult {
        let ticket = self.tickets.get_mut(ticket_id as usize).ok_or(ProgramError::InvalidArgument)?;

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

    pub fn validate_ticket(&self, _ctx: Context<ValidateTicket>, ticket_id: u32) -> ProgramResult {
        let ticket = self.tickets.get(ticket_id as usize).ok_or(ProgramError::InvalidArgument)?;

        match ticket.status {
            TicketStatus::Valid => Ok(()),
            _ => Err(ProgramError::Custom(4)), // Ticket not valid
        }
    }

    pub fn is_ticket_from_organization(&self, ctx: Context<IsTicketFromOrganization>, ticket_id: u32) -> ProgramResult {
        let ticket = self.tickets.get(ticket_id as usize).ok_or(ProgramError::InvalidArgument)?;

       
        Ok(())
    }
}


#[derive(Accounts)]
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

#[derive(Accounts)]
pub struct Ticket {
    pub event_id: u32,
    pub owner: Pubkey,
    pub status: TicketStatus,
}

#[derive(Accounts)]
pub struct Initialize {}

#[derive(Accounts)]
pub struct CreateEvent<'info> {
    #[account(init, payer = user, space = 256)]
    pub event_account: Account<'info, Event>,
    #[account(mut)]
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
pub struct ValidateTicket<'info> {
    
}

#[derive(Accounts)]
pub struct IsTicketFromOrganization<'info> {
    pub authority: AccountInfo<'info>,
}

#[derive(Accounts, AnchorSerialize, AnchorDeserialize, Clone)]
pub struct Event {
    pub id: u32,
    pub name: String,
    // pub category : Vec<String>,
    pub location : String,
    pub tickets_available: u32,
    pub ticket_price: u64,
}

pub struct Initialize {

}
