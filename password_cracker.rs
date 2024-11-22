use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::time::Instant;
use sha2::{Sha256, Digest};
use std::env;

fn main() {
    // Get the password hash from the user input
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <password_hash>", args[0]);
        return;
    }
    let password_hash = &args[1];

    let wordlist_path = "wordlist.txt";

    if let Ok(lines) = read_lines(wordlist_path) {
        let start = Instant::now();
        for line in lines {
            if let Ok(word) = line {
                let mut hasher = Sha256::new();
                hasher.update(word.as_bytes());
                let result = hasher.finalize();
                let hash = format!("{:x}", result);

                if hash == *password_hash {
                    println!("Password found: {}", word);
                    let duration = start.elapsed();
                    println!("Time taken: {:?}", duration);
                    return;
                }
            }
        }
        println!("Password not found in wordlist.");
    } else {
        println!("Could not read wordlist file.");
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
