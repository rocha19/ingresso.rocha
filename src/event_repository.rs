use async_trait::async_trait;
use tokio::sync::RwLock;

use crate::event::Event;
use rusqlite::{params, Connection, Result};

pub struct EventRepositorySqlite {
    db_path: String,
}

impl EventRepositorySqlite {
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
pub trait EventRepository {
    async fn get_event(&self, event_id: String) -> Result<Event, String>;
}

#[async_trait]
impl EventRepository for EventRepositorySqlite {
    async fn get_event(&self, event_id: String) -> Result<Event, String> {
        let connection = self
            .establish_connection()
            .await
            .map_err(|e| e.to_string())?;

        let mut stmt = connection
            .prepare("SELECT * FROM event WHERE event_id = ?")
            .map_err(|e| e.to_string())?;

        let output = stmt
            .query_row(params![event_id], |row| {
                Ok(Event {
                    event_id: row.get(0)?,
                    description: row.get(1)?,
                    price: row.get(2)?,
                })
            })
            .map_err(|e| e.to_string())?;

        Ok(output)
    }
}
