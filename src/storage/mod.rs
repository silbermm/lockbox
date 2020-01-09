extern crate rusqlite;
extern crate dirs;

use rusqlite::{Connection, Result, Error, named_params};
use rusqlite::NO_PARAMS;

static CREATE_TABLE_Q: &str = "create table if not exists passwords ( id integer primary key, account text not null, username text not null, password text not null, inserted_at text, updated_at text)";

pub fn initialize() -> Result<Connection> {
    let home = get_home_directory()?;
    let conn = Connection::open(home + "/.lockbox/passwords.db")?;
    conn.execute(CREATE_TABLE_Q, NO_PARAMS)?;

    Ok(conn)
}

pub fn add(conn: Connection, account: Account) -> Result<()> {
    conn.execute(
        "INSERT INTO passwords (account, username, password, inserted_at, updated_at) values (?1, ?2, ?3, datetime('now'), datetime('now'))",
        &[&account.name, &account.username, &account.password]
    )?;

    Ok(())
}

pub fn find_by_account(conn: Connection, account_name: String) -> Result<std::vec::Vec<Account>> {
   let mut stmt = conn.prepare("SELECT p.account, p.username, p.password from passwords p where p.account like :account")?;
   let passwords = stmt.query_map_named(named_params!{ ":account": account_name }, |row|
            Ok(
                Account {
                    name: row.get(0)?,
                    username: row.get(1)?,
                    password: row.get(2)?,
                }
            )
        )?;

   Ok(passwords.map(|a| a.unwrap()).collect())
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
    pub password: String
}
