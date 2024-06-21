-- SQLITE

-- Não há suporte para SCHEMA no SQLite, então removemos DROP SCHEMA e CREATE SCHEMA
-- Também não há suporte para a palavra reservada UUID no SQLite, então removemos o PRIMARY KEY
CREATE TABLE event (
  event_id TEXT PRIMARY KEY,
  description TEXT,
  price NUMERIC
);

CREATE TABLE ticket (
  ticket_id TEXT PRIMARY KEY,
  event_id TEXT,
  email TEXT,
  price NUMERIC,
  FOREIGN KEY(event_id) REFERENCES event(event_id)
);

INSERT INTO event (
  event_id, description, price
) VALUES ( '161d4eea-cc10-4c42-94d6-5a09fb3bd72e', 'Rockfest', 100.00 );

-- POSTGRES

-- DROP SCHEMA IF EXISTS rocha CASCADE;
-- CREATE SCHEMA rocha;

-- CREATE TABLE rocha.event (
--   event_id UUID PRIMARY KEY,
--   description TEXT,
--   price NUMERIC
-- );
--
-- CREATE TABLE rocha.ticket (
--   ticket_id UUID PRIMARY KEY,
--   event_id UUID REFERENCES rocha.event(event_id)
-- );
--
-- INSERT INTO rocha.event (
--   event_id, description, price
-- ) VALUES ( '161d4eea-cc10-4c42-94d6-5a09fb3bd72e', 'Rockfest', 100.00 );

