## Backend codes goes here

https://beta.solpg.io/65f2da27cffcf4b13384cfb0


# Function Documentation

## `initialize`

Initializes a user profile.

### Arguments

- `ctx` - The context for the transaction.
- `_name` - The name of the user.
- `_test` - The test information.
- `_avatar` - The avatar of the user.
- `_email` - The email address of the user.
- `_password` - The password of the user.
- `_date` - The date information.

### Errors

Returns an error if the profile initialization fails.

## `transfer_lamports`

Transfers lamports from one account to another.

### Arguments

- `ctx` - The context for the transaction.
- `amount` - The amount of lamports to transfer.

### Errors

Returns an error if the transfer fails.

## `update_event_count`

Updates the event count in the user profile.

### Arguments

- `ctx` - The context for the transaction.
- `_count` - The count to update to.

### Errors

Returns an error if the update fails.

## `create_event`

Creates a new event.

### Arguments

- `ctx` - The context for the transaction.
- `_id` - The ID of the event.
- `_name` - The name of the event.
- `_tickets_available` - The number of tickets available.
- `_ticket_price` - The price of a ticket.
- `_location` - The location of the event.
- `_category` - The category of the event.
- `_starting_date` - The starting date of the event.
- `_ending_date` - The ending date of the event.
- `_event_type` - The type of the event.

### Errors

Returns an error if the creation fails.

## `start_event`

Starts an event.

### Arguments

- `ctx` - The context for the transaction.
- `_event_id` - The ID of the event to start.

### Errors

Returns an error if the start fails.

## `end_event`

Ends an event.

### Arguments

- `ctx` - The context for the transaction.
- `_event_id` - The ID of the event to end.

### Errors

Returns an error if the end fails.

