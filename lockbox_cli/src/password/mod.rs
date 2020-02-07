use rand::Rng;

pub fn generate(num_of_words: usize) -> Vec<String> {
    let content = include_str!("diceware.wordlist.asc");

    let num: usize = num_of_words - 1;

    (0..num)
        .map(|_| {
            let num: String = random_number();
            find_line(&content, &num)
        })
        .collect()
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
            let v: Vec<&str> = line.split("\t").collect();
            match v.last() {
                Some(n) => n.to_string(),
                None => "".to_string(),
            }
        }
        None => "".to_string(),
    }
}
