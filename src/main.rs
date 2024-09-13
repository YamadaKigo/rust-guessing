use std::{cmp::Ordering, io};
use rand::Rng; // trait
 
fn main() {
    println!("Guessing Numbers!");
    println!("Generating pseudo random numbers.");
    let secret_number: u32 = rand::thread_rng().gen_range(1.. 101);
    //println!("The random number is:{}", secret_number);
    
    loop {
        println!("Please guess a number!");
        let mut guess: String = String::new();
    
        io::stdin().read_line(&mut guess)
        .expect("Cannot read line.");
    
        let guess: u32= match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        
        println!("Your guessed number is:{}", guess);
        
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You Win!");
                break;
            },
        };
    }
}