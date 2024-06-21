use rusqlite::{params, Connection, Result};
use uuid::Uuid;

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

    pub async fn establish_connection(&self) -> Result<Connection> {
        Connection::open(&self.db_path)
    }

    pub async fn execute(&self, input: Input) -> Result<Output, String> {
        let uuid = Uuid::now_v7();
        let ticket_id = uuid.to_string();

        let connection = self.establish_connection().await.unwrap();

        let mut stmt = connection
            .prepare("SELECT * FROM event WHERE event_id = ?")
            .map_err(|e| e.to_string())?;

        let mut event_data: f64 = 0.0;
        stmt.query_row(params![input.event_id], |row| {
            event_data = row.get(2).unwrap();
            Ok(())
        })
        .map_err(|e| e.to_string())?;

        let _ = connection.execute(
            "INSERT INTO ticket (ticket_id, event_id, email, price) VALUES (?, ?, ?, ?)",
            params![ticket_id, input.event_id, input.email, event_data], // occupied_spaces initialized as 0
        );

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
