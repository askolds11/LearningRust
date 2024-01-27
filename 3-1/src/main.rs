// Convert temperatures between Fahrenheit and Celsius.
use std::io::{self, Write};

fn main() {
    loop {
        print!("Input temperature in fahrenheit: ");
        io::stdout().flush().expect("Flush failed!");

        let mut fahrenheit = String::new();
    
        io::stdin()
            .read_line(&mut fahrenheit)
            .expect("Failed to read line");

        let fahrenheit: f64 = match fahrenheit
            .trim()
            .parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("Please enter a valid number!");
                    continue;
                }
            };

        let celsius = (fahrenheit - 32.0) * 5.0/9.0;
        println!("{fahrenheit}°F in celsius is {celsius:.2}°C");

    }
}
