extern crate lockbox_lib;

use rustler::{Encoder, Env, Error, Term};
use rustler::types::atom::{ok, error};
use lockbox_lib::{encryption, storage};

rustler::rustler_export_nifs! {
    "Elixir.Lockbox.Lib",
    [
        ("decrypt", 1, decrypt),
    ],
    None
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
