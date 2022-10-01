use std::io::{self, BufRead};

use crate::handling::handle_input;

mod handling;
mod structs;

fn main() -> io::Result<()> {
    let lines = io::stdin().lock().lines();
    let mut user_input = String::new();

    println!("Paste data with headers. End with an empty line:");
    for line in lines {
        let last_input = line.unwrap();

        // stop reading
        if last_input.is_empty() {
            break;
        }

        // add a new line once user_input starts storing user input
        if !user_input.is_empty() {
            user_input.push('\n');
        }

        // store user input
        user_input.push_str(&last_input);
    }

    println!("\nResult \n{}", handle_input(&user_input));

    // the lock is released after it goes out of scope
    Ok(())
}
