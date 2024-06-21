#[cfg(test)]
mod tests {
    use ingresso_rocha::{
        get_ticket::GetTicket,
        purchase_ticket::{Input, PurchaseTicket},
    };

    #[tokio::test]
    async fn buy_event_ticket() {
        let purchase_ticket = PurchaseTicket::new("database.sqlite").await;

        let input_purchase_ticket = Input {
            event_id: "161d4eea-cc10-4c42-94d6-5a09fb3bd72e".to_string(),
            email: "john.doe@email.com".to_string(),
        };

        let output_purchase_ticket = purchase_ticket
            .execute(input_purchase_ticket.clone())
            .await
            .unwrap();

        let get_ticket = GetTicket::new("database.sqlite").await;
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
