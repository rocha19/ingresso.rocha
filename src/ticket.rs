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

impl Ticket {
    pub fn create(event_id: String, email: String, price: f64) -> Result<Self, String> {
        let uuid = Uuid::now_v7();
        let ticket_id = uuid.to_string();

        let email_regex = Regex::new(r"^[\w\.-]+@[\w\.-]+\.\w+$").unwrap();

        if !email_regex.is_match(&email) {
            return Err("Invalid email format".to_string());
        }

        Ok(Self {
            ticket_id,
            event_id,
            email,
            price,
        })
    }
}
