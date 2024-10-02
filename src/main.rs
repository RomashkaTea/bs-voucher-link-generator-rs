use uuid::Uuid;
use std::fs::File;
use std::io::Write;
use std::io::stdin;
use std::io::stdout;

const URL: &str = "https://link.brawlstars.com/?action=voucher&code=";

fn generate_voucher_code() -> Uuid {
    Uuid::new_v4()
}

fn main() {
    let mut s = String::new();
    print!("Count: ");

    stdout().flush().unwrap(); // it doesn't print before read_line without flush()
    stdin().read_line(&mut s).unwrap();

    let value: i32 = s.trim().parse().expect("Your input should be a number!");
    let mut result = String::new();

    for _ in 0..value {
        let id = generate_voucher_code();
    
        result.push_str(&format!("{}{}\n", URL, id));
    }

    let mut file = File::create("links.txt").unwrap();
    file.write_all(result.as_bytes()).unwrap();
}
