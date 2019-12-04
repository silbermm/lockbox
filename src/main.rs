extern crate rand;
extern crate sodiumoxide;

use structopt::StructOpt;
use rand::{Rng};
use console::style;
use sodiumoxide::crypto::box_;
use sodiumoxide::crypto::box_::*;
use std::io;
use std::io::prelude::*;
use std::fs::File;

fn main() {
    let args = Cli::from_args();    
    let content = std::fs::read_to_string(&args.password_list)
        .expect("could not read file");

    match build_rsa_keypair() {
        Ok(()) => println!("file write success"),
        Err(e) => println!("file write err {:?}", e)
    }

    let _p : Vec<String> = (0..5).map(|_| {
        let num : String = random_number();
        let word = find_line(&content, &num);
        print!("{}", style(&word).white());
        word
    }).collect();
}

fn random_number() -> String {
    const CHARSET: &[u8] = b"123456";

    const PASSWORD_LEN: usize = 5;
    let mut rng = rand::thread_rng();

    (0..PASSWORD_LEN)
        .map(|_| {
            let idx = rng.gen_range(0, CHARSET.len());
            CHARSET[idx] as char
        })
    .collect()
}

fn find_line(content: &str, number: &str) -> String {
    let mut iter = content.lines();
    let result = iter.find(|&x| x.starts_with(number));
    match result {
        Some(line) => {
            let v : Vec<&str> = line.split("\t").collect();
            match v.last() {
                Some(n) => { n.to_string() }
                None => { "".to_string() }
            }
        }
        None => {
            "".to_string()
        }
    }
}

fn build_rsa_keypair() -> Result<(), io::Error> {
    let (ourpk, oursk) = box_::gen_keypair();
    let nonce = box_::gen_nonce();
    let sodiumoxide::crypto::box_::PublicKey(bytes) = ourpk;

    let plaintext = b"some data";
    let ciphertext = box_::seal(plaintext, &nonce, &ourpk, &oursk);

    let mut file = File::create("public_key")?;
    file.write_all(&bytes)?;

    let mut file = File::open("public_key")?;
    let mut buffer = Vec::<u8>::new();
    file.read_to_end(&mut buffer)?;

    let mut pubkey_bytes = [0u8; PUBLICKEYBYTES];
    for i in 0..PUBLICKEYBYTES {
        pubkey_bytes[i] = buffer[i];
    }

    let newpk = PublicKey(pubkey_bytes);

    let their_plaintext = box_::open(&ciphertext, &nonce, &newpk, &oursk).unwrap();
    assert!(plaintext == &their_plaintext[..]);

    Ok(())
}

#[derive(StructOpt)]
struct Cli {
    /// The path to the file to read
    #[structopt(parse(from_os_str), short = "p", long = "password_list")]
    password_list: std::path::PathBuf,
}
