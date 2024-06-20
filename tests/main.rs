#[cfg(test)]
mod tests {
    use ingresso_rocha::purchase_ticket::{Input, PurchaseTicket};

    #[tokio::test]
    async fn buy_event_ticket() {
        let input_purchase_ticket = Input {
            event_id: "".to_string(),
            email: "john.doe@email.com".to_string(),
        };

        let output_purchase_ticket = PurchaseTicket.execute(input_purchase_ticket).await;

        assert!(output_purchase_ticket.is_ok());
    }
}
