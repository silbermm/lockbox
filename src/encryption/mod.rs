extern crate sodiumoxide;
extern crate dirs;

use std::io;
use sodiumoxide::crypto::box_;
use sodiumoxide::crypto::box_::*;
use std::io::prelude::*;
use std::string::*;
use std::fs::File;
use std::io::{Error, ErrorKind};
    
static P_KEY_FILE: &str = "/.lockbox/public_key";
static S_KEY_FILE: &str = "/.lockbox/secret_key";
static NONCE_FILE: &str = "/.lockbox/nonce";

#[derive(Debug)]
pub struct EncryptedData {
    data: Vec<u8>
}

#[derive(Debug)]
pub struct CryptoBox {
    pkey: PublicKey,
    skey: SecretKey,
    nonce: Nonce
}

impl CryptoBox {
    pub fn encrypt(&self, data: &str) -> EncryptedData {
        // convert the string to bytes
        let to_encrypt = data.as_bytes();
        let data = box_::seal(to_encrypt, &self.nonce, &self.pkey, &self.skey);

        EncryptedData { data }
    }

    pub fn decrypt(&self, data: EncryptedData) -> Result<String, &str> {
        match box_::open(&data.data, &self.nonce, &self.pkey, &self.skey) {
            Ok(d) => match String::from_utf8(d) {
                Ok(res) => Ok(res),
                Err(_) => Err("Unable to convert from utf8")
            }
            Err(_) => return Err("Unable to decrypt data")
        }
    }
}

pub fn generate_keys() -> Result<CryptoBox, io::Error> {
    map_home_directory(|home| -> Result<CryptoBox, io::Error> {
        let (ourpk, oursk) = box_::gen_keypair();

        let sodiumoxide::crypto::box_::PublicKey(bytes) = ourpk;
        let mut file = File::create(home.clone() + P_KEY_FILE)?;
        file.write_all(&bytes)?;

        let sodiumoxide::crypto::box_::SecretKey(sbytes) = oursk;
        let mut pfile = File::create(home.clone() + S_KEY_FILE)?;
        pfile.write_all(&sbytes)?;

        let nonce = box_::gen_nonce();
        let sodiumoxide::crypto::box_::Nonce(nbytes) = nonce;
        let mut nonce_file = File::create(home + NONCE_FILE)?;
        nonce_file.write_all(&nbytes)?;

        Ok(CryptoBox {
            pkey: ourpk,
            skey: oursk,
            nonce
        })
    })
}

pub fn load_keys() -> Result<CryptoBox, io::Error> {
    map_home_directory(|home| -> Result<CryptoBox, io::Error> {

        let mut file = File::open(home.clone() + P_KEY_FILE)?;
        let mut buffer = Vec::<u8>::new();
        file.read_to_end(&mut buffer)?;

        let mut pubkey_bytes = [0u8; PUBLICKEYBYTES];
        for i in 0..PUBLICKEYBYTES {
            pubkey_bytes[i] = buffer[i];
        }
        let pkey = PublicKey(pubkey_bytes);

        let mut file = File::open(home.clone() + S_KEY_FILE)?;
        let mut buffer = Vec::<u8>::new();
        file.read_to_end(&mut buffer)?;

        let mut secretkey_bytes = [0u8; SECRETKEYBYTES];
        for i in 0..SECRETKEYBYTES {
            secretkey_bytes[i] = buffer[i];
        }
        let skey = SecretKey(secretkey_bytes);

        let mut file = File::open(home + NONCE_FILE)?;
        let mut buffer = Vec::<u8>::new();
        file.read_to_end(&mut buffer)?;

        let mut nonce_bytes = [0u8; NONCEBYTES];
        for i in 0..NONCEBYTES {
            nonce_bytes[i] = buffer[i];
        }
        let nonce = Nonce(nonce_bytes);


        Ok(CryptoBox {
            pkey,
            skey,
            nonce
        })
    })
}

fn map_home_directory<P>(f: P) -> Result<CryptoBox, io::Error> where P: Fn(String) -> Result<CryptoBox, io::Error> {
    match dirs::home_dir() {
        Some(home) => f(home.display().to_string()),
        None => Err(Error::new(ErrorKind::Other, "Unable to locate home directory"))
    }
}
