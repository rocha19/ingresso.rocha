use std::sync::Arc;

use tokio::sync::RwLock;

use crate::{
    event_repository::EventRepository,
    ticket::{Ticket, TicketCreationError},
    ticket_repository::TicketRepository,
};

pub struct PurchaseTicket {
    event_repository: Arc<RwLock<dyn EventRepository>>,
    ticket_repository: Arc<RwLock<dyn TicketRepository>>,
}

impl PurchaseTicket {
    pub async fn new(
        event_repository: Arc<RwLock<dyn EventRepository>>,
        ticket_repository: Arc<RwLock<dyn TicketRepository>>,
    ) -> Self {
        Self {
            event_repository,
            ticket_repository,
        }
    }

    pub async fn execute(&self, input: Input) -> Result<Output, String> {
        let event_repository = self.event_repository.read().await;
        let event_data = event_repository
            .get_event(input.event_id.clone())
            .await
            .unwrap();

        let ticket_repository = self.ticket_repository.write().await;
        match Ticket::create(input.event_id, input.email, event_data.price) {
            Ok(ticket) => {
                let _ = ticket_repository.save_ticket(ticket.clone()).await;
                Ok(Output {
                    ticket_id: Ok(ticket.ticket_id),
                })
            }
            Err(TicketCreationError::InvalidEmailFormat) => Err("Invalid email format".to_string()),
            Err(TicketCreationError::InvalidPrice) => Err("Invalid price".to_string()),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Input {
    pub event_id: String,
    pub email: String,
}

#[derive(Debug)]
pub struct Output {
    pub ticket_id: Result<String, String>,
}
