use std::io;
use chrono::{DateTime, Local};

fn main() {
    // TODO: 1. Prompt the user for their name
    println!("What's your name");
    // TODO: 2. Read the user's input
    let mut name = String::new();
    io::stdin().read_line(&mut name).expect("there is a problem");

    // TODO: 3. Print a personalized greeting
    println!("Hello {}", name.trim());
    // BONUS: Print the current date
    let localTime: DateTime<Local> = Local::now();
    println!("It is: {}", localTime);
    // Hint: You can use the chrono crate for this
}