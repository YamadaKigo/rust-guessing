use std::io;

fn main() {
    println!("Guessing Numbers!");

    let mut guess = String::new();

    io::stdin().read_line(&mut guess)
    .expect("Cannot read line.");

    println!("Your guessed number is：{}", guess);
    
    //test
}
