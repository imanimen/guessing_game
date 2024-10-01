use std::{cmp::Ordering, io};

use rand::Rng;
// use std::io::stdin  // another way to call the method of the standard library

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    // println!("The secret number is: {secret_number}");
    loop {
            
        println!("Please input your guess.");

        let mut guess = String::new();

        // dividing lines for more redability
        io::stdin()
            .read_line(&mut guess)
            .expect("failed to read line");

        let guess: u32 = guess.trim().parse().expect("Please type a number!");

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => println!("You win!") 
        }
    }
}