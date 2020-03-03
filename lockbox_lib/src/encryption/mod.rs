extern crate dirs;
extern crate sodiumoxide;

use base64;
use sodiumoxide::crypto::box_;
use sodiumoxide::crypto::box_::*;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::io::{Error, ErrorKind};
use std::string::*;

#[path = "../constants/mod.rs"]
mod constants;
use constants::{P_KEY_FILE, S_KEY_FILE, NONCE_FILE};

#[derive(Debug)]
pub struct EncryptedData {
    data: Vec<u8>,
}

impl EncryptedData {
    pub fn to_string(&self) -> String {
        base64::encode(&self.data)
    }
}

#[derive(Debug)]
pub struct CryptoBox {
    pkey: PublicKey,
    skey: SecretKey,
    nonce: Nonce,
}

impl CryptoBox {
    pub fn encrypt(&self, data: &str) -> EncryptedData {
        let to_encrypt = data.as_bytes();
        let data = box_::seal(to_encrypt, &self.nonce, &self.pkey, &self.skey);

        EncryptedData { data }
    }

    pub fn decrypt(&self, data: EncryptedData) -> Result<String, &str> {
        match box_::open(&data.data, &self.nonce, &self.pkey, &self.skey) {
            Ok(d) => match String::from_utf8(d) {
                Ok(res) => Ok(res),
                Err(_) => Err("Unable to convert from utf8"),
            },
            Err(_) => return Err("Unable to decrypt data"),
        }
    }
}

// TODO: Store keys base64 encoded instead of binary
pub fn generate_keys() -> Result<CryptoBox, io::Error> {
    map_home_directory(|home| -> Result<CryptoBox, io::Error> {
        // do keys already exist?
        let pub_file_name = format!("{}{}", home, P_KEY_FILE);
        let priv_file_name = format!("{}{}", home, S_KEY_FILE);
        let nonce_file_name = format!("{}{}", home, NONCE_FILE);

        let _ = err_if_keys_exist(vec![
                                  pub_file_name.to_owned(),
                                  priv_file_name.to_owned(),
                                  nonce_file_name.to_owned(),
        ])?;

        let (ourpk, oursk) = box_::gen_keypair();
        let nonce = box_::gen_nonce();

        let mut pub_file = File::create(pub_file_name)?;
        let pub_file_contents = base64::encode(&ourpk.as_ref());
        pub_file.write_all(&pub_file_contents.as_bytes())?;

        let mut private_file = File::create(priv_file_name)?;
        let priv_file_contents = base64::encode(&oursk.as_ref());
        private_file.write_all(&priv_file_contents.as_bytes())?;

        let mut nonce_file = File::create(nonce_file_name)?;
        let nonce_file_contents = base64::encode(&nonce.as_ref());
        nonce_file.write_all(&nonce_file_contents.as_bytes())?;

        Ok(CryptoBox {
            pkey: ourpk,
            skey: oursk,
            nonce,
        })
    })
}

pub fn load_keys() -> Result<CryptoBox, io::Error> {
    map_home_directory(|home| -> Result<CryptoBox, io::Error> {
        let p_file = format!("{}{}", home, P_KEY_FILE);
        let mut file = File::open(p_file)?;
        let mut buffer = Vec::<u8>::new();
        file.read_to_end(&mut buffer)?; 

        let pkey = match base64::decode(&buffer) {
            Ok(res) => {
                let mut pubkey_bytes = [0u8; PUBLICKEYBYTES];
                for i in 0..PUBLICKEYBYTES {
                    pubkey_bytes[i] = res[i];
                }
                Ok(PublicKey(pubkey_bytes))
            }
            _ => Err(Error::new(ErrorKind::Other, "Unable to read in public key"))
        }?;

        let s_file = format!("{}{}", home, S_KEY_FILE);
        let mut file = File::open(s_file)?;
        let mut sbuffer = Vec::<u8>::new();
        file.read_to_end(&mut sbuffer)?;

        let skey = match base64::decode(&sbuffer) {
            Ok(res) => {
                let mut secretkey_bytes = [0u8; SECRETKEYBYTES];
                for i in 0..SECRETKEYBYTES {
                    secretkey_bytes[i] = res[i];
                }
                Ok(SecretKey(secretkey_bytes))
            }
            _ => Err(Error::new(ErrorKind::Other, "Unable to read in secret key"))
        }?;


        let n_file = format!("{}{}", home, NONCE_FILE);
        let mut file = File::open(n_file)?;
        let mut nbuffer = Vec::<u8>::new();
        file.read_to_end(&mut nbuffer)?;


        let nonce = match base64::decode(&nbuffer) {
            Ok(res) => {
                let mut nonce_bytes = [0u8; NONCEBYTES];
                for i in 0..NONCEBYTES {
                    nonce_bytes[i] = res[i];
                }
                Ok(Nonce(nonce_bytes))
            }
            Err(_) => Err(Error::new(ErrorKind::Other, "Unable to read in nonce"))
        }?;

        Ok(CryptoBox { pkey, skey, nonce })
    })
}

pub fn load_from_encoded(encoded: String) -> Result<EncryptedData, io::Error> {
    match base64::decode(&encoded) {
        Ok(data) => Ok(EncryptedData { data }),
        Err(_) => Err(Error::new(ErrorKind::Other, "Unable to decode passwords")),
    }
}

fn map_home_directory<P, Q>(f: P) -> Result<Q, io::Error>
where
P: Fn(&str) -> Result<Q, io::Error>,
{
    match dirs::home_dir() {
        Some(home) => f(&home.display().to_string()),
        None => Err(Error::new(
                ErrorKind::Other,
                "Unable to locate home directory",
                )),
    }
}

fn err_if_keys_exist(key_paths: std::vec::Vec<String>) -> Result<(), io::Error> {
    let r: Vec<bool> = key_paths
        .into_iter()
        .map(|p| std::path::Path::new(&p).exists())
        .collect();

    match r.into_iter().find(|&x| x) {
        Some(_) => Err(Error::new(ErrorKind::AlreadyExists, "Keys Exist")),
        None => Ok(()),
    }
}
