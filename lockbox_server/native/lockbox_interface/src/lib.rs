extern crate lockbox_lib;

use rustler::{Encoder, Env, Error, Term};
use rustler::types::atom::{ok, error};
use lockbox_lib::{encryption, storage, constants};
use std::collections::HashMap;

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
    let passwords = match storage::initialize() {
        Ok(conn) => {
            match storage::account::all(conn) {
                Ok((_, current_passwords)) => {
                    current_passwords.into_iter().map(|x| {
                        x.to_string()
                    }).collect()
                },
                Err(_) => vec!()
            }
        },
        Err(_) => vec!()
    };
    Ok((ok(), passwords).encode(env))
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
