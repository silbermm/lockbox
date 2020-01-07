extern crate rand;
extern crate sodiumoxide;

use std::io::ErrorKind;
use structopt::StructOpt;

mod storage;
mod encryption;
mod password;

fn main() {
    let args = Cli::from_args();

    match storage::initialize() {
        Ok(_) => println!("Database initialized"),
        Err(_) => println!("Unable to initialize database")
    };

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

        println!("{}", password);

        let edata = cryptobox.encrypt(&password);

        match edata.save() {
            Ok(()) => println!("Data saved to file"),
            Err(r) => println!("Unable to save password to file - error = {}", r),
        }
    }

    if args.show {
        let cryptobox = encryption::load_keys().expect("Unable to load encryption keys");
        let edata = encryption::load_passwords().expect("Unable to load passwords");
        match cryptobox.decrypt(edata) {
            Ok(p) => println!("passwords = {}", p),
            Err(r) => println!("error = {}", r),
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
