// Generate the nth Fibonacci number.

use std::io::{self, Write};


fn main() {
    loop {
        print!("Input which fibonacci number to calculate (n): ");
            io::stdout().flush().expect("Flush failed!");
    
            let mut n = String::new();
        
            io::stdin()
                .read_line(&mut n)
                .expect("Failed to read line");
    
    
            let n: u32 = match n
                .trim()
                .parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("Please enter a valid number!");
                        continue;
                    }
                };
            
            let mut num1: u64 = 0;
            let mut num2: u64 = 1;

            if n == 0 {
                println!("Please enter a valid number (> 0)!");
                continue;
            } else if n == 1 {
                println!("The {n}th Fibonacci number is {num1}");
            } else if n == 2 {
                println!("The {n}th Fibonacci number is {num2}")
            } else if n > 3 {
                for _i in 0..n-2 {
                    let next_num = num1 + num2;
                    num1 = num2;
                    num2 = next_num;
                }
                println!("The {n}th Fibonacci number is {num2}")
            }
    }

}
