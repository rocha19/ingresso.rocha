extern crate rusqlite;

use rusqlite::{Connection, Result};
use std::fs::File;
use std::io::prelude::*;

fn main() -> Result<()> {
    let mut file = File::open("create.sql").expect("Não foi possível abrir o arquivo create.sql");
    let mut sql_script = String::new();
    file.read_to_string(&mut sql_script)
        .expect("Não foi possível ler o arquivo create.sql");

    let conn = Connection::open("database.sqlite")?;

    conn.execute_batch(&sql_script)?;

    println!("Banco de dados SQLite criado com sucesso!");

    Ok(())
}
