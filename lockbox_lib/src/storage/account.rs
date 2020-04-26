extern crate rusqlite;

use rusqlite::NO_PARAMS;
use rusqlite::{named_params, Connection, Result};


#[derive(Debug)]
pub struct Account {
    pub name: String,
    pub username: String,
    pub password: String,
    pub updated_at: Option<String>,
}

impl Account {
    pub fn to_string(&self) -> String {
        let d = match &self.updated_at {
            Some(date) => date,
            None => ""
        };
        format!("{}:{}:{}:{}", &self.name, &self.username, &self.password, d)
    }

    pub fn update_password(self, new_password: String) -> Account {
        Account {
            password: new_password,
            ..self
        }
    }
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

pub fn find_all(conn: Connection) -> Result<(Connection, std::vec::Vec<String>)> {
    let res = {
        let mut stmt = conn.prepare("SELECT DISTINCT p.account from passwords p")?;
        let accounts = stmt.query_map(NO_PARAMS, |row| Ok(row.get(0)?))?;
        accounts.map(|a| a.unwrap()).collect()
    };
    Ok((conn, res))
}

pub fn all(conn: Connection) -> Result<(Connection, std::vec::Vec<Account>)> {
    let res = {
        let mut stmt = conn.prepare(
         "SELECT account, username, password, updated_at from passwords"
        )?;

        let accounts = stmt.query_map(NO_PARAMS, |row| {
            Ok(Account {
                name: row.get(0)?,
                username: row.get(1)?,
                password: row.get(2)?,
                updated_at: Some(row.get(3)?)
             })
        })?;
        accounts.map(|a| a.unwrap()).collect()
    };
    Ok((conn, res))
}

pub fn find(conn: Connection, account_name: String) -> Result<(Connection, std::vec::Vec<Account>)> {
    let res = {
        let mut stmt = conn.prepare(
            "SELECT p.account, p.username, p.password from passwords p where p.account like :account",
        )?;
        let passwords = stmt.query_map_named(named_params! { ":account": account_name }, |row| {
            Ok(Account {
                name: row.get(0)?,
                username: row.get(1)?,
                password: row.get(2)?,
                updated_at: None,
            })
        })?;
        passwords.map(|a| a.unwrap()).collect()
    };

    Ok((conn, res))
}
