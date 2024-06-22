#[cfg(test)]
mod ticket {
    use ingresso_rocha::ticket::Ticket;

    #[test]
    fn create_ticket() {
        let ticket = Ticket::create(
            "161d4eea-cc10-4c42-94d6-5a09fb3bd72e".to_string(),
            "john.doe@email.com".to_string(),
            100.00,
        )
        .unwrap();

        assert_eq!(ticket.event_id, "161d4eea-cc10-4c42-94d6-5a09fb3bd72e");
        assert_eq!(ticket.email, "john.doe@email.com");
        assert_eq!(ticket.price, 100.00);
    }

    #[test]
    fn create_ticket_with_invalid_email() {
        let ticket = Ticket::create(
            "161d4eea-cc10-4c42-94d6-5a09fb3bd72e".to_string(),
            "john.doe".to_string(),
            100.00,
        );
        assert!(ticket.is_err());
    }
}
