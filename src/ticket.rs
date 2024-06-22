use std::{result::Result, string::String};
use uuid::Uuid;

use crate::email::Email;

#[derive(Clone, Debug)]
pub struct Ticket {
    pub ticket_id: String,
    pub event_id: String,
    pub email: String,
    pub price: f64,
}

#[derive(Debug)]
pub enum TicketCreationError {
    InvalidEmailFormat,
    InvalidPrice,
}

impl Ticket {
    pub fn create(
        event_id: String,
        email: String,
        price: f64,
    ) -> Result<Self, TicketCreationError> {
        let valid_email = Email::new(email);
        match (valid_email.is_ok(), price > 0.0) {
            (true, true) => Ok(Ticket {
                ticket_id: Uuid::now_v7().to_string(),
                event_id,
                email: valid_email.unwrap().get_value(),
                price,
            }),
            (_, false) => Err(TicketCreationError::InvalidPrice),
            (false, _) => Err(TicketCreationError::InvalidEmailFormat),
        }
    }
}
