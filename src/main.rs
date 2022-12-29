mod parse_string;
mod rust_borrowings;
use std::io;
use std::cmp::Ordering;
use rand::Rng;
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
        
        println!("The length of the string is {length}", length=rust_borrowings::calculate_length(&guess));

        let guess: u32 = match parse_string::parse_string(&guess) {
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
