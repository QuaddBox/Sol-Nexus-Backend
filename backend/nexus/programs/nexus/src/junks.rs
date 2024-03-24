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