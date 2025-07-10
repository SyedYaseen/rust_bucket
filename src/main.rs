use rand::random_range;
use std::{cmp::Ordering, io};

fn main() {
    println!("Guess a number");
    let secret = random_range(1..10);

    loop {
        let mut guess: String = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed ot read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("Your guess is {} Secret was {}", guess, secret);

        match guess.cmp(&secret) {
            Ordering::Greater => println!("Guess is higher"),
            Ordering::Equal => {
                println!("Guess is equal");
                break;
            }
            Ordering::Less => println!("Guess is lower"),
        }
    }
}
