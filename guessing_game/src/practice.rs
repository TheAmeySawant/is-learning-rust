use std::{cmp::Ordering, io};

use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number: u32 = rand::thread_rng().gen_range(1..100);

    loop {
        println!("Please input your guess: ");

        let mut guess: String = String::new();

        //read_line method of stdin handle instance takes input from the user and appends it to the string in argument.
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess
            .trim()
            .parse(){
                Ok(num) => num,
                Err(_) => {
                    print!("\nPlease Enter a number!\n");
                    continue;
                },
            };

        // match a.cmp(&b){
        //     Ordering::Less => println!(), // a < b
        //     Ordering::Greater => println!(), // a > b
        //     Ordering::Equal => println!() // a == b
        // }

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
