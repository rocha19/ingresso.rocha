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

#[allow(dead_code)]
#[derive(Default)]
pub struct EventRepositoryInMemory {
    events: RwLock<Vec<Event>>,
}

impl EventRepositoryInMemory {
    pub fn new() -> Self {
        Self {
            events: RwLock::new(vec![]),
        }
    }
}

#[async_trait]
impl EventRepository for EventRepositoryInMemory {
    async fn get_event(&self, _: String) -> Result<Event, String> {
        Ok(Event {
            event_id: "161d4eea-cc10-4c42-94d6-5a09fb3bd72e".to_string(),
            description: "Rockfest".to_string(),
            price: 100.00,
        })
    }
}
