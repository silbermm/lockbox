extern crate rusqlite;
extern crate dirs;

use rusqlite::{Connection, Result, Error};
use rusqlite::NO_PARAMS;
use std::collections::HashMap;


static CREATE_TABLE_Q: &str = "create table if not exists passwords ( id integer primary key, account text not null, username text not null, password text not null)";

pub fn initialize() -> Result<()> {
    let home = get_home_directory()?;
    let conn = Connection::open(home + "/.lockbox/passwords.db")?;
    conn.execute(CREATE_TABLE_Q, NO_PARAMS)?;

    Ok(())
}

fn get_home_directory() -> Result<String> {
    match dirs::home_dir() {
        Some(home) => Ok(home.display().to_string()),
        None => Err(Error::InvalidQuery),
    }
}
