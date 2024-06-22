use uuid::Uuid;

#[derive(Clone, Debug)]
pub struct Ticket {
    pub ticket_id: String,
    pub event_id: String,
    pub email: String,
    pub price: f64,
}

impl Ticket {
    pub fn create(event_id: String, email: String, price: f64) -> Self {
        let uuid = Uuid::now_v7();
        let ticket_id = uuid.to_string();

        Self {
            ticket_id,
            event_id,
            email,
            price,
        }
    }
}
