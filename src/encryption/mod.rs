extern crate sodiumoxide;
extern crate dirs;

use std::io;
use std::io::prelude::*;
use std::fs::File;
use sodiumoxide::crypto::box_;
use std::io::{Error, ErrorKind};

pub fn generate_keys() -> Result<(), io::Error> {
    // get home directory
    map_home_directory(|home| -> Result<(), io::Error> {
        let (ourpk, oursk) = box_::gen_keypair();

        // write public key to file
        let sodiumoxide::crypto::box_::PublicKey(bytes) = ourpk;
        let mut file = File::create(home.clone() + "/.lockbox/public_key")?;
        file.write_all(&bytes)?;

        // write secret key to file
        let sodiumoxide::crypto::box_::SecretKey(sbytes) = oursk;
        let mut pfile = File::create(home + "/.lockbox/private_key")?;
        pfile.write_all(&sbytes)?;

        Ok(())
    })
}

fn map_home_directory<P>(f: P) -> Result<(), io::Error> where P: Fn(String) -> Result<(), io::Error> {
    match dirs::home_dir() {
        Some(home) => f(home.display().to_string()),
        None => Err(Error::new(ErrorKind::Other, "Unable to locate home directory"))
    }
}
