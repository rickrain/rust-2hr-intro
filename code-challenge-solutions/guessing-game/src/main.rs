use colored::*;
use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    print!("{esc}c", esc = 27 as char); // Clear the terminal

    // Generate a random number between 1..100
    let secret_number = rand::thread_rng().gen_range(1..101);
    println!("I have a secret number selected between 1 and 100.");
    println!("How quickly can you guess the number?");

    // A vector to keep track of guesses from the user
    let mut guesses: Vec<u32> = Vec::new();

    // The score is just the number of guesses the user makes
    let mut score = 0;

    loop {
        // Prompt the user to enter a guess
        println!("Enter your guess: ");
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line.");

        // User gets charged a guess after pressing ENTER
        score += 1;

        // Convert input to an integer
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!(
                    "{}",
                    format!("'{}' is not a number.", guess.replace("\n", "")).red()
                );
                continue;
            }
        };

        // If this is a new guess, add it to the list of guesses.
        if guesses.contains(&guess) {
            println!("You already guessed this number. That will cost you on your score!");
        } else {
            guesses.push(guess);
        }

        // Evaluate the user's guess
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("{}", format!("{} is too small. Try again!", guess).red()),
            Ordering::Greater => println!("{}", format!("{} is too big. Try again!", guess).red()),
            Ordering::Equal => {
                println!("{}", format!("You win with a score of {}!", score).green());
                break;
            }
        }
    }
}
