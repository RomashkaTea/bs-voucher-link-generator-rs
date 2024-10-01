use uuid::Uuid;
use std::fs::File;
use std::io::Write;

const URL: &str = "https://link.brawlstars.com/?action=voucher&code=";

fn generate_voucher_code() -> Uuid {
    Uuid::new_v4()
}

fn main() {
    let mut s = String::new();
    print!("Count: ");

    std::io::stdout().flush().expect("flush failed"); // it doesn't print before read_line without flush()
    std::io::stdin().read_line(&mut s).expect("Failed to read input");

    let value: i32 = s.trim().parse().expect("Your input should be a number!");
    let mut result = String::new();

    for _ in 0..value {
        let id = generate_voucher_code();
    
        result.push_str(&format!("{}{}\n", URL, id));
    }

    let mut file = File::create("links.txt").expect("Failed to create file links.txt!");
    file.write_all(result.as_bytes()).expect("Failed to write file links.txt!");
}
