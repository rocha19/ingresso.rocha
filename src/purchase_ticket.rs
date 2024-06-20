use uuid::Uuid;

#[derive(Default)]
pub struct PurchaseTicket;

impl PurchaseTicket {
    pub async fn new() -> Self {
        Self
    }
    pub async fn execute(&self, input: Input) -> Result<Output, String> {
        let uuid = Uuid::now_v7();
        // let ts = Timestamp::from_unix(uuid::NoContext, 1645557742, 0);
        // let specific_uuid_v7 = Uuid::new_v7(ts);
        let ticket_id = uuid.to_string();
        Ok(Output {
            ticket_id: Ok(ticket_id),
        })
    }
}

pub struct Input {
    pub event_id: String,
    pub email: String,
}

pub struct Output {
    pub ticket_id: Result<String, String>,
}
