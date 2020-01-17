extern crate rand;
extern crate sodiumoxide;

use std::io::{self, Write};
use std::io::ErrorKind;
use structopt::StructOpt;

mod storage;
mod encryption;
mod password;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::from_args();

    if args.generate_keys {
        match encryption::generate_keys() {
            Ok(_) => println!("Keys generated and saved in your home directory"),
            Err(ref e) if e.kind() == ErrorKind::AlreadyExists => println!("Encryption keys already exist, regenerating will likely cause data loss. Use -f if you want to force regeneration."),
            Err(_) => println!("Unable to generate keys")
        }
    }

    if args.generate_password {
        let cryptobox = encryption::load_keys()?;

        let p: Vec<String> = password::generate(6);
        let password: String = p.into_iter().collect();

        println!("{} ", password);
        print!("Save this password? (Y/n) ");
        io::stdout().flush()?;

        let mut input = String::new();
        let _ = io::stdin().read_line(&mut input);

        if input == "\n" || input == "Y\n"  || input == "y\n" {
            let edata = cryptobox.encrypt(&password);

            let conn = storage::initialize()?;
            println!("Database initialized");
            let account = build_account(edata);
            let _ = storage::add(conn, account)?;

        } else {
            println!("don't save password");
        }
    }

    if args.accounts {
        println!();
        let conn = storage::initialize()?; 
        let accounts = storage::find_accounts(conn).unwrap();
        for account in accounts {
            println!("{}", account);
        }
        println!();
    } else if args.account.is_some() {
        println!();
        let cryptobox = encryption::load_keys().expect("Unable to load encryption keys");
        let conn = storage::initialize()?;
        let accounts = storage::find_by_account(conn, String::from(args.account.unwrap()))?;
        for account in accounts {
            let encrypted_data = encryption::load_from_encoded(account.password).unwrap();
            println!("account name = {}", account.name);
            println!("username = {}", account.username);
            println!("password = {}", cryptobox.decrypt(encrypted_data).unwrap());
            println!()
        }
    }
    Ok(())
}

#[derive(StructOpt)]
#[structopt(rename_all = "kebab-case")]
pub struct Cli {
    #[structopt(short = "g", long, help = "Generates new encryption keys")]
    generate_keys: bool,

    #[structopt(short = "p", long = "generate-password", help = "Generates a new password")]
    generate_password: bool,


    #[structopt(short = "a", long = "show-accounts", help = "Show saved accounts")]
    accounts: bool,

    #[structopt(short = "s", long = "show-password", help = "Show password for a specific account")]
    account: Option<String>,


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
