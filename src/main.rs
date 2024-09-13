use std::{cmp::Ordering, io};
use rand::Rng;

fn main() {
    println!("Guessing Numbers!");

    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("Generating pseudo-random number.");

    loop {
        println!("Please guess a number between 1 and 100:");

        let mut input = String::new();
        
        if io::stdin().read_line(&mut input).is_err() {
            eprintln!("Failed to read input.");
            continue;
        }

        let guess: u32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                eprintln!("Invalid input. Please enter a valid number.");
                continue;
            }
        };

        println!("Your guess: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
