#[cfg(test)]
mod tests {
    use std::sync::Arc;
    use tokio::sync::RwLock;

    use ingresso_rocha::{
        event_repository::{EventRepository, EventRepositorySqlite},
        get_ticket::GetTicket,
        purchase_ticket::{Input, PurchaseTicket},
        ticket_repository::{TicketRepository, TicketRepositorySqlite},
    };

    #[tokio::test]
    async fn buy_event_ticket() {
        /*
        let parking_lot_repository_in_memory = ParkingLotRepositoryInMemory::new();
        let repository: Arc<RwLock<dyn ParkingLotRepository>> =
            Arc::new(RwLock::new(parking_lot_repository_in_memory));
        let enter_parking_lot = EnterParkingLot::new(repository.clone());
        let get_parking_lot = GetParkingLot::new(repository);
        */
        let event_repository_sqlite = EventRepositorySqlite::new().await;
        let ticket_repository_sqlite = TicketRepositorySqlite::new().await;
        let event_repository: Arc<RwLock<dyn EventRepository>> =
            Arc::new(RwLock::new(event_repository_sqlite));
        let ticket_repository: Arc<RwLock<dyn TicketRepository>> =
            Arc::new(RwLock::new(ticket_repository_sqlite));
        let purchase_ticket =
            PurchaseTicket::new(event_repository, ticket_repository.clone()).await;

        let input_purchase_ticket = Input {
            event_id: "161d4eea-cc10-4c42-94d6-5a09fb3bd72e".to_string(),
            email: "john.doe@email.com".to_string(),
        };

        let output_purchase_ticket = purchase_ticket
            .execute(input_purchase_ticket.clone())
            .await
            .unwrap();

        let get_ticket = GetTicket::new(ticket_repository).await;
        let output_get_ticket = get_ticket
            .execute(
                output_purchase_ticket
                    .ticket_id
                    .clone()
                    .expect("Failed to get ticket id"),
            )
            .await
            .unwrap();

        assert_eq!(
            output_get_ticket.ticket_id,
            output_purchase_ticket
                .ticket_id
                .expect("Failed to get ticket id")
        );
        assert_eq!(output_get_ticket.event_id, input_purchase_ticket.event_id);
        assert_eq!(output_get_ticket.email, input_purchase_ticket.email);
        assert_eq!(output_get_ticket.price, 100.00);
    }
}
