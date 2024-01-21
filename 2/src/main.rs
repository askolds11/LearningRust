use std::{io::{self, Write}, cmp::Ordering};
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);
    // println!("The secret number is: {secret_number}");

    loop {
        print!("Please input your guess: ");
        io::stdout().flush().expect("Flush failed!");
    
        let mut guess = String::new();
    
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
    
    
        // the read_line contains the new line character from input
        // and as such causes a blank line in the ouput
        // guess = guess.trim().to_string();
        // println!("You guessed {guess}");
        
    
        let guess: u8 = match guess
            .trim()
            .parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("Please enter a valid number!");
                    continue;
                }
            };
    
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println! ("You win!");
                break;
            }
        }               
    }
}
