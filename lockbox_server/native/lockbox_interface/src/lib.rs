extern crate lockbox_lib;

use rustler::{Encoder, Env, Error, Term};
use rustler::types::atom::{ok, error};
use lockbox_lib::{encryption, storage, constants};
use rusqlite::{Connection};

rustler::rustler_export_nifs! {
    "Elixir.Lockbox.Lib",
    [
        ("decrypt", 1, decrypt),
        ("public_key_path", 0, public_key_path),
        ("nonce_path", 0, nonce_path),
        ("get_passwords_for_public_key", 1, passwords_for_public_key),
    ],
    None
}


fn public_key_path<'a>(env: Env<'a>, _args: &[Term<'a>]) -> Result<Term<'a>, Error> {
    Ok((ok(), constants::P_KEY_FILE).encode(env))
}

fn nonce_path<'a>(env: Env<'a>, _args: &[Term<'a>]) -> Result<Term<'a>, Error> {
    Ok((ok(), constants::NONCE_FILE).encode(env))
}

fn passwords_for_public_key<'a>(env: Env<'a>, args: &[Term<'a>]) -> Result<Term<'a>, Error> {
    let public_key: String = args[0].decode()?;
    match storage::initialize() {
        Ok(conn) => {
            let current_passwords = storage::account::all(conn);
            let new_passwords = current_passwords.map(|x| {
                
            }).collect();
            Ok((ok(), new_passwords).encode(env))
        }
        Err(_) => Ok((error(), "Unable to get passwords from the database").encode(env))
    }
}

fn decrypt<'a>(env: Env<'a>, args: &[Term<'a>]) -> Result<Term<'a>, Error> {
    let cryptobox = encryption::load_keys().expect("Unable to load encryption keys");

    let data: String = args[0].decode()?;
    match encryption::load_from_encoded(data) {
        Ok(encrypted_data) => match cryptobox.decrypt(encrypted_data) {
            Ok(unencrypted) => Ok((ok(), unencrypted).encode(env)),
            Err(err) => Ok((error(), err.to_owned()).encode(env))
        },
        Err(_) => Ok((error(), "Unable to decode password").encode(env))
    }
}

fn map_accounts(accounts_from_db: Result<(Connection, std::vec::Vec<storage::account::Account>)>) -> std::vec::Vec<storage::account::Account> {
    match accounts_from_db {
        Ok(_conn, accounts) => {
            let new_passwords = current_passwords.map(|x| {

            }).collect();
        },
        Err(_) => std::vec::Vec<storage::account::Account>
    }
}
