use std::sync::Arc;

use tokio::sync::RwLock;

use crate::ticket_repository::TicketRepository;

pub struct GetTicket {
    ticket_repository: Arc<RwLock<dyn TicketRepository>>,
}

impl GetTicket {
    pub async fn new(ticket_repository: Arc<RwLock<dyn TicketRepository>>) -> Self {
        Self { ticket_repository }
    }

    pub async fn execute(&self, ticket_id: String) -> Result<Output, String> {
        let repository = self.ticket_repository.read().await;
        let ticket = repository.get_ticket(ticket_id).await.unwrap();

        Ok(Output {
            ticket_id: ticket.ticket_id,
            event_id: ticket.event_id,
            email: ticket.email,
            price: ticket.price,
        })
    }
}

#[derive(Debug)]
pub struct Output {
    pub ticket_id: String,
    pub event_id: String,
    pub email: String,
    pub price: f64,
}

/*
use async_trait::async_trait;
use chrono::NaiveDateTime;
use rusqlite::{params, Connection, Result};

use crate::domain::{
    entity::parking_lot::ParkingLot, repository::parking_lot_repository::ParkingLotRepository,
};

pub struct ParkingLotRepositorySqlite {
    db_path: String,
}

impl ParkingLotRepositorySqlite {
    pub fn new(db_path: &str) -> Self {
        Self {
            db_path: db_path.to_string(),
        }
    }

    pub fn establish_connection(&self) -> Result<Connection> {
        Connection::open(&self.db_path)
    }

    pub fn add_parking_lot(&self, /*code: &str, capacity: i32, open_hour: i32, close_hour: i32 */) {
        let code = "shopping_mall";
        let capacity = 5;
        let open_hour = 8;
        let close_hour = 22;

        let connection = self.establish_connection().unwrap();
        let _ = connection.execute(
            "INSERT INTO parking_lots (code, capacity, open_hour, close_hour, occupied_spaces) VALUES (?, ?, ?, ?, ?)",
            params![code, capacity, open_hour, close_hour, 0], // occupied_spaces initialized as 0
        );
    }
}

#[async_trait]
impl ParkingLotRepository for ParkingLotRepositorySqlite {
    async fn get_parking_lot(&self, code: String) -> ParkingLot {
        let connection = self
            .establish_connection()
            .expect("Failed to establish connection");
        let mut stmt = connection
            .prepare("SELECT * FROM parking_lots WHERE code = ?")
            .expect("Failed to prepare statement");
        let parking_lot_row = stmt
            .query_row(params![code.clone()], |row| {
                let code: String = row.get(0)?;
                let capacity: i32 = row.get(1)?;
                let open_hour: i32 = row.get(2)?;
                let close_hour: i32 = row.get(3)?;
                let occupied_spaces: i32 = row.get(4)?;
                Ok((code, capacity, open_hour, close_hour, occupied_spaces))
            })
            .expect("Failed to query row");

        ParkingLot::new(
            &parking_lot_row.0,
            parking_lot_row.1,
            parking_lot_row.2,
            parking_lot_row.3,
            parking_lot_row.4,
        )
    }

    async fn save_parked_car(&mut self, code: String, plate: String, date: NaiveDateTime) {
        let connection = self
            .establish_connection()
            .expect("Failed to establish connection");
        let mut stmt = connection
            .prepare("INSERT INTO parked_cars (code, plate, date) VALUES (?, ?, ?)")
            .expect("Failed to prepare statement");
        stmt.execute(params![code.clone(), plate, date.and_utc().timestamp()]) // Cloning code to pass it as String
            .expect("Failed to execute statement");

        connection
            .execute(
                "UPDATE parking_lots SET occupied_spaces = occupied_spaces + 1 WHERE code = ?",
                params![code],
            )
            .expect("Failed to update parking lot occupied spaces");
    }
}
*/
