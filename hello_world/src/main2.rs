extern crate rand;
use rand::Rng;
use std::cmp::Ordering;
use std::io;

pub fn main() {
    // Secrey Number Generator
    println!("Guess the number game !");
    let secret_number: i32 = rand::thread_rng().gen_range(1..101);
    println!("The secret number is {}", secret_number);

    // User input
    println!("Please input your guess");
    let mut guess: String = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("Faield To Read line");
    let guess: i32 = guess.trim().parse().expect("Please Type A Number");
    println!("You have guessed {}", guess);

    // Match statement for guesses
    match guess.cmp(&secret_number) {
        Ordering::Equal => println!("You WIN !!!!"),
        Ordering::Greater => println!("A bit too high !"),
        Ordering::Less => println!("A little under !"),
    }
}
