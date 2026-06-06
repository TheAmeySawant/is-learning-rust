use rand::Rng;
use std::{cmp::Ordering};

fn main() {
    let secret_number: u32 = rand::thread_rng().gen_range(0..100);
    println!("Guess The Number!!");
    loop {
        println!("Enter you guess:");

        let mut guess: String = String::new();

        std::io::stdin()
            .read_line(&mut guess)
            .expect("Failed to take input");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("\nPlease enter NUMBER only!\n");
                continue;
            }
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too Less"),
            Ordering::Greater => println!("Too Big"),
            Ordering::Equal => {
                println!("You WIN !");
                break;
            }
        }
    }
}
