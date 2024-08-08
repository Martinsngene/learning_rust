extern crate rand;

use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    // Generate a secret number
    let secret_number = rand::thread_rng().gen_range(1..101);

    // println!("The secret number is: {}", secret_number);

    // Add a loop to allow multiple guesses
    loop {
        println!("Please input your guess!");

        // Create a new string instance
        let mut guess = String::new();

        // Store userinput by updating string variable
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line!");

        // Create another guess variable to shadow the first one
        // Convert the string from the userinput to an integer
        // When converting strings from std::io read_line() to integers, be sure to trim first.
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(err) => {
                println!("Enter a number: {}", err);
                continue;
            }
        };

        println!("You guessed: {}", guess);

        // Compare guess with secret_number
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Sorry too small, try again!"),
            Ordering::Greater => println!("Sorry too big, try again!"),
            Ordering::Equal => {
                println!("Congratulations, you guessed correctly!");
                break;
            }
        }
    }
}
