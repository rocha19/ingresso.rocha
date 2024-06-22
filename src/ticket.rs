use regex::Regex;
use std::{result::Result, string::String};
use uuid::Uuid;
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
        let email_regex = Regex::new(r"^[\w\.-]+@[\w\.-]+\.\w+$").unwrap();

        match (email_regex.is_match(&email), price > 0.0) {
            (false, _) => Err(TicketCreationError::InvalidEmailFormat),
            (_, false) => Err(TicketCreationError::InvalidPrice),
            (true, true) => {
                let uuid = Uuid::now_v7();
                let ticket_id = uuid.to_string();

                Ok(Self {
                    ticket_id,
                    event_id,
                    email,
                    price,
                })
            }
        }
    }
}
