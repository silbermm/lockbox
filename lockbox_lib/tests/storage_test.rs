extern crate lockbox_lib;
extern crate rusqlite;

use lockbox_lib::{storage};

#[test]
fn test_account_add_and_remove() -> Result<(), rusqlite::Error> {
    let conn = storage::initialize()?;

    let account = storage::Account {
        name: String::from("test account only"),
        username: String::from("testing"),
        password: String::from("abcdefg=")
    };

    match storage::add(conn, account) {
        Ok((c, 1)) => {
            match storage::remove(c, String::from("test account only"), String::from("testing")) {
                Ok((_, 1)) => assert!(true),
                _ => assert!(false)
            }
        },
        _ => assert!(false)
    };
    Ok(())
}

#[test]
fn test_find_account() -> Result<(), rusqlite::Error> {
    let conn = storage::initialize()?;

    let account = storage::Account {
        name: String::from("test account only"),
        username: String::from("testing"),
        password: String::from("abcdefg=")
    };

    let (conn, _) = storage::add(conn, account)?;
    let (conn, passwords) = storage::find_by_account(conn, String::from("test_account only"))?;
    assert_eq!(passwords.len(), 1);
    let _ = storage::remove(conn, String::from("test account only"), String::from("testing"))?;
    Ok(())
}

