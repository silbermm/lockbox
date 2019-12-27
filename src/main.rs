extern crate rand;
extern crate sodiumoxide;

use structopt::StructOpt;
use rand::{Rng};

mod encryption;

fn main() {
    let args = Cli::from_args();    

    if args.generate_keys {
        println!("Generating new encryption keys");
        match encryption::generate_keys() {
            Ok(_) => println!("Keys generated and saved in your home directory"),
            Err(_) => println!("Unable to generate keys")
        }
    } else {
        let cryptobox = encryption::load_keys().expect("Unable to load encryption keys");

        let content = include_str!("diceware.wordlist.asc");

        let p : Vec<String> = (0..5).map(|_| {
            let num : String = random_number();
            find_line(&content, &num)
        }).collect();
        let password: String = p.into_iter().collect();  
        println!("{}", password);

        let edata = cryptobox.encrypt(&password);

        match cryptobox.decrypt(edata) {
            Ok(p) => println!("password = {}", p),
            Err(r) => println!("error = {}", r)
        }
    }
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

#[derive(StructOpt)]
#[structopt(rename_all = "kebab-case")] 
struct Cli {
    #[structopt(short = "g", long, help = "Generates new encryption keys")]
    generate_keys: bool
}
