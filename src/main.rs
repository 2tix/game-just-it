use std::io;
use rand::Rng;

fn main() {
    println!("Welcome to the Guessing Game!");

    let secret_number = rand::thread_rng().gen_range(1..101);

    loop {
        println!("Please enter your guess:");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid number!");
                continue;
            }
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            std::cmp::Ordering::Less => println!("Too small!"),
            std::cmp::Ordering::Greater => println!("Too large!"),
            std::cmp::Ordering::Equal => {
                println!("Congratulations! You guessed the correct number: {}", secret_number);
                break;
            }
        }
    }
}

