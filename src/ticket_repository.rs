use async_trait::async_trait;
use rusqlite::{params, Connection, Result};
use tokio::sync::RwLock;

use crate::ticket::Ticket;

pub struct TicketRepositorySqlite {
    db_path: String,
}

impl TicketRepositorySqlite {
    pub async fn new() -> Self {
        Self {
            db_path: "database.sqlite".to_string(),
        }
    }

    pub async fn establish_connection(&self) -> Result<Connection> {
        Connection::open(&self.db_path)
    }
}

#[async_trait]
pub trait TicketRepository {
    async fn save_ticket(&self, ticket: Ticket);
    async fn get_ticket(&self, ticket_id: String) -> Result<Ticket, String>;
}

#[async_trait]
impl TicketRepository for TicketRepositorySqlite {
    async fn save_ticket(&self, ticket: Ticket) {
        let connection = self
            .establish_connection()
            .await
            .map_err(|e| e.to_string())
            .unwrap();

        let _ = connection
            .execute(
                "INSERT INTO ticket (ticket_id, event_id, email, price) VALUES (?, ?, ?, ?)",
                params![
                    ticket.ticket_id,
                    ticket.event_id,
                    ticket.email,
                    ticket.price
                ],
            )
            .map_err(|e| e.to_string());
    }

    async fn get_ticket(&self, ticket_id: String) -> Result<Ticket, String> {
        let connection = self
            .establish_connection()
            .await
            .map_err(|e| e.to_string())?;

        let mut stmt = connection
            .prepare("SELECT * FROM ticket WHERE ticket_id = ?")
            .map_err(|e| e.to_string())?;

        let output = stmt
            .query_row(params![ticket_id], |row| {
                Ok(Ticket {
                    ticket_id: row.get(0)?,
                    event_id: row.get(1)?,
                    email: row.get(2)?,
                    price: row.get(3)?,
                })
            })
            .map_err(|e| e.to_string())?;

        Ok(output)
    }
}

#[allow(dead_code)]
#[derive(Default)]
pub struct TicketRepositoryInMemory {
    tickets: RwLock<Vec<Ticket>>,
}

impl TicketRepositoryInMemory {
    pub fn new() -> Self {
        Self {
            tickets: RwLock::new(vec![]),
        }
    }
}

#[async_trait]
impl TicketRepository for TicketRepositoryInMemory {
    async fn save_ticket(&self, ticket: Ticket) {
        let mut tickets = self.tickets.write().await;
        tickets.push(ticket);
    }

    async fn get_ticket(&self, ticket_id: String) -> Result<Ticket, String> {
        let tickets = self.tickets.read().await;
        for ticket in tickets.iter() {
            if ticket.ticket_id == ticket_id {
                return Ok(ticket.clone());
            }
        }
        Err("Ticket not found".to_string())
    }
}
