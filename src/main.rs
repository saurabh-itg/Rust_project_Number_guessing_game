use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Welcome to the Number Guessing Game!");

    let secret_number = rand::thread_rng().gen_range(1..101);
    let mut attempts: u32 = 0;

    loop {
        println!("Please input your guess:");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");

        // Convert the input to a number
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input. Please enter a valid number.");
                continue;
            }
        };

        attempts += 1; // Increment the attempts by 1 after each attempt

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too low! Try again."),
            Ordering::Greater => println!("Too high! Try again."),
            Ordering::Equal => {
                println!("Congratulations! You guessed the number in {} attempts.", attempts);
                break;
            }
        }
    }

    println!("Game Over. Thanks for playing!");
}
