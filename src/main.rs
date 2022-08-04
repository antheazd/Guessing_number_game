use std::{io, cmp::Ordering};
use rand::Rng;
fn main() {
    println!("Guess the number between 0 and 100!!");
    println!("Input your guess");
    let mut guess = String::new();
    
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read input");
    
    println!("You guessed {guess}");

    let secret_number = rand::thread_rng().gen_range(1..101).to_string();
    
    
    match guess.cmp(&secret_number){
        Ordering::Less => println!("Your number is too small"),
        Ordering::Greater => println!("Your number is too big"),
        Ordering::Equal => println!("Your number was correct"),

    }

    println!("The correct guess is: {secret_number}");

}
