extern crate dirs;
extern crate rusqlite;

use rusqlite::NO_PARAMS;
use rusqlite::{Connection, Error, Result};

#[path = "../constants/mod.rs"]
mod constants;
use constants::{DATABASE_FILE};

pub mod account;

static CREATE_PASSWORDS_TABLE_Q: &str = "create table if not exists passwords ( id integer primary key, account text not null, username text not null, password text not null, inserted_at text, updated_at text, unique (account, username))";

pub fn initialize() -> Result<Connection> {
    let home = get_home_directory()?;
    let conn = Connection::open(home + DATABASE_FILE)?;
    conn.execute(CREATE_PASSWORDS_TABLE_Q, NO_PARAMS)?;

    Ok(conn)
}

fn get_home_directory() -> Result<String> {
    match dirs::home_dir() {
        Some(home) => Ok(home.display().to_string()),
        None => Err(Error::InvalidQuery),
    }
}

#[derive(Debug)]
pub struct Account {
    pub name: String,
    pub username: String,
    pub password: String,
}
