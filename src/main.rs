use std::{io, cmp::Ordering};
use rand::Rng;

fn main() {
    println!("Welcome to the Guessing Game");

    let secret_number: u32 = rand::thread_rng().gen_range(1..101);
    loop {
        println!("The secret number is: {}", secret_number);

    println!("Please input your guess number");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
    let guess: u32 = match guess.trim().parse() {
        Ok(num) => num,
        Err(_) => continue

    };

    let face = '\u{1F600}';
    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too Small"),
        Ordering::Greater => println!("Too Big"),
        Ordering::Equal => {
            println!("You Win! {}", face);
            break;
        }

    }
    }
    


}
