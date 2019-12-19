extern crate sodiumoxide;
extern crate dirs;

use std::io;
use sodiumoxide::crypto::box_;
use sodiumoxide::crypto::box_::*;
use std::io::prelude::*;
use std::fs::File;
use std::io::{Error, ErrorKind};

pub fn generate_keys() -> Result<(PublicKey, SecretKey), io::Error> {
    map_home_directory(|home| -> Result<(PublicKey, SecretKey), io::Error> {
        let (ourpk, oursk) = box_::gen_keypair();

        // write public key to file
        let sodiumoxide::crypto::box_::PublicKey(bytes) = ourpk;
        let mut file = File::create(home.clone() + "/.lockbox/public_key")?;
        file.write_all(&bytes)?;

        // write secret key to file
        let sodiumoxide::crypto::box_::SecretKey(sbytes) = oursk;
        let mut pfile = File::create(home + "/.lockbox/private_key")?;
        pfile.write_all(&sbytes)?;

        Ok((ourpk, oursk))
    })
}

pub fn load_keys() -> Result<(PublicKey, SecretKey), io::Error> {
    map_home_directory(|home| -> Result<(PublicKey,SecretKey), io::Error> {
        let dir = home + "/.lockbox/";
        let mut file = File::open(dir.clone() + "public_key")?;
        let mut buffer = Vec::<u8>::new();
        file.read_to_end(&mut buffer)?;

        let mut pubkey_bytes = [0u8; PUBLICKEYBYTES];
        for i in 0..PUBLICKEYBYTES {
            pubkey_bytes[i] = buffer[i];
        }
        let newpk = PublicKey(pubkey_bytes);

        let mut file = File::open(dir + "secret_key")?;
        let mut buffer = Vec::<u8>::new();
        file.read_to_end(&mut buffer)?;

        let mut pubkey_bytes = [0u8; PUBLICKEYBYTES];
        for i in 0..PUBLICKEYBYTES {
            pubkey_bytes[i] = buffer[i];
        }
        let newsk = SecretKey(pubkey_bytes);

        Ok((newpk, newsk))
    })
}

fn map_home_directory<P>(f: P) -> Result<(PublicKey, SecretKey), io::Error> where P: Fn(String) -> Result<(PublicKey, SecretKey), io::Error> {
    match dirs::home_dir() {
        Some(home) => f(home.display().to_string()),
        None => Err(Error::new(ErrorKind::Other, "Unable to locate home directory"))
    }
}
