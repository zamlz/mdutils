use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();

    for line in reader.lines() {
        match line {
            Ok(text) => println!("{}", text),
            Err(e) => {
                eprintln!("Error reading line: {}", e);
                break;
            }
        }
    }
}
