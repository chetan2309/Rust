use std::io;
use std::cmp::Ordering;
use rand::Rng;

use crate::parse_string::ParseString;
mod parse_string;
mod tests;

fn main() {
    println!("Guess the number!");

    let secret_num = rand::thread_rng().gen_range(1..=100);
    loop {
        println!("Please input your guess");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read a line");

        let guess: u32 = match u32::parse_string(&guess) {
            Ok(num) => num,
            Err(e) => {
                println!("Error: {}", e);
                0
            }
        };
        
        println!("You guessed: {guess}");

        match guess.cmp(&secret_num) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win");
                break;
            }
        }    
    }
}
