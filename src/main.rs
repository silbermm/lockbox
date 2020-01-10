extern crate rand;
extern crate sodiumoxide;

use std::io::{self, Write};
use std::io::ErrorKind;
use structopt::StructOpt;

mod storage;
mod encryption;
mod password;

fn main() {
    let args = Cli::from_args();

    if args.generate_keys {
        match encryption::generate_keys() {
            Ok(_) => println!("Keys generated and saved in your home directory"),
            Err(ref e) if e.kind() == ErrorKind::AlreadyExists => println!("Encryption keys already exist, regenerating will likely cause data loss. Use -f if you want to force regeneration."),
            Err(_) => println!("Unable to generate keys")
        }
    }

    if args.generate_password {
        let cryptobox = encryption::load_keys().expect("Unable to load encryption keys");

        let p: Vec<String> = password::generate(6);
        let password: String = p.into_iter().collect();

        println!("{} ", password);
        print!("Save this password? (Y/n) ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        let _ = io::stdin().read_line(&mut input);

        if input == "\n" || input == "Y\n"  || input == "y\n" {
            let edata = cryptobox.encrypt(&password);

            match storage::initialize() {
                Ok(conn) => {
                    println!("Database initialized");
                    let account = build_account(edata);
                    let _ = storage::add(conn, account).unwrap();
                },
                Err(_) => println!("Unable to initialize database")
            };
        } else {
            println!("don't save password");
        }
    }

    if args.show {
        let cryptobox = encryption::load_keys().expect("Unable to load encryption keys");
        match storage::initialize() {
            Ok(conn) => {
                let accounts = storage::find_by_account(conn, String::from("gmail.com")).unwrap();
                for account in accounts {
                    let encrypted_data = encryption::load_from_encoded(account.password).unwrap();
                    println!("account name = {}", account.name);
                    println!("username = {}", account.username);
                    println!("password = {}", cryptobox.decrypt(encrypted_data).unwrap())
                }
            },
            Err(_) => println!("Unable to connect to password database")
        }
    }
}

#[derive(StructOpt)]
#[structopt(rename_all = "kebab-case")]
struct Cli {
    #[structopt(short = "g", long, help = "Generates new encryption keys")]
    generate_keys: bool,

    #[structopt(short = "p", long = "password", help = "Generates a new password")]
    generate_password: bool,

    #[structopt(short = "s", long = "show", help = "Show Passwords")]
    show: bool,
}

fn build_account(e: encryption::EncryptedData) -> storage::Account {
    print!("What account is this for (i.e. google.com)? ");
    io::stdout().flush().unwrap();

    let mut account = String::new();
    let _ = io::stdin().read_line(&mut account);

    print!("What is username should we save with this? ");
    io::stdout().flush().unwrap();

    let mut username = String::new();
    let _ = io::stdin().read_line(&mut username);

    storage::Account {
        name: account.trim().to_string(),
        username: username.trim().to_string(),
        password: e.to_string()
    }
}
