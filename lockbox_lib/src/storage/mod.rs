extern crate dirs;
extern crate rusqlite;

use rusqlite::NO_PARAMS;
use rusqlite::{named_params, Connection, Error, Result};

static CREATE_TABLE_Q: &str = "create table if not exists passwords ( id integer primary key, account text not null, username text not null, password text not null, inserted_at text, updated_at text, unique (account, username))";

pub fn initialize() -> Result<Connection> {
    let home = get_home_directory()?;
    let conn = Connection::open(home + "/.lockbox/passwords.db")?;
    conn.execute(CREATE_TABLE_Q, NO_PARAMS)?;

    Ok(conn)
}

pub fn add(conn: Connection, account: Account) -> Result<(Connection, usize)> {
    let res = conn.execute(
        "INSERT INTO passwords (account, username, password, inserted_at, updated_at) values (?1, ?2, ?3, datetime('now'), datetime('now'))",
        &[&account.name, &account.username, &account.password]
    )?;

    Ok((conn, res))
}

pub fn remove(conn: Connection, account_name: String, username: String) -> Result<(Connection, usize)> {
    let result = conn.execute(
        "DELETE FROM passwords WHERE account = ?1 AND username = ?2",
        &[&account_name, &username]
    )?;
    Ok((conn, result))
}

pub fn find_accounts(conn: Connection) -> Result<(Connection, std::vec::Vec<String>)> {
    let res = {
        let mut stmt = conn.prepare("SELECT DISTINCT p.account from passwords p")?;
        let accounts = stmt.query_map(NO_PARAMS, |row| Ok(row.get(0)?))?;
        accounts.map(|a| a.unwrap()).collect()
    };
    Ok((conn, res))
}

pub fn find_by_account(conn: Connection, account_name: String) -> Result<(Connection, std::vec::Vec<Account>)> {
    let res = {
        let mut stmt = conn.prepare(
            "SELECT p.account, p.username, p.password from passwords p where p.account like :account",
        )?;
        let passwords = stmt.query_map_named(named_params! { ":account": account_name }, |row| {
            Ok(Account {
                name: row.get(0)?,
                username: row.get(1)?,
                password: row.get(2)?,
            })
        })?;
        passwords.map(|a| a.unwrap()).collect()
    };

    Ok((conn, res))
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
