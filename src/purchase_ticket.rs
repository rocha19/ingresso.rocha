use uuid::Uuid;

use crate::{event_repository::EventRepositorySqlite, ticket::Ticket, ticket_repository};

#[derive(Default, Debug)]
pub struct PurchaseTicket {
    db_path: String,
}

impl PurchaseTicket {
    pub async fn new(db_path: &str) -> Self {
        Self {
            db_path: db_path.to_string(),
        }
    }

    pub async fn execute(&self, input: Input) -> Result<Output, String> {
        let uuid = Uuid::now_v7();
        let ticket_id = uuid.to_string();

        let event_repository = EventRepositorySqlite::new(&self.db_path).await;
        let event_data = event_repository
            .get_event(input.event_id.clone())
            .await
            .unwrap();

        let ticket_repository = ticket_repository::TicketRepositorySqlite::new(&self.db_path).await;
        let ticket = Ticket {
            ticket_id: ticket_id.clone(),
            event_id: input.event_id,
            email: input.email,
            price: event_data.price,
        };
        let _ = ticket_repository.save_ticket(ticket).await;

        Ok(Output {
            ticket_id: Ok(ticket_id),
        })
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
