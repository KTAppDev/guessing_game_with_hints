use rand::Rng;
use std::time::Instant;
fn main() {
    println!("Computer will guess a number between 0 - 5000");
    let mut guesses: u16 = 0;
    let secret_number = rand::thread_rng().gen_range(0..=5000);
    let mut computer_guess = rand::thread_rng().gen_range(0..=5000);

    let start = Instant::now();

    let mut lowestguess: u16 = 0;
    let mut highestguess: u16 = 5000;

    while computer_guess != secret_number {
        if computer_guess > secret_number {
            highestguess = computer_guess;
            println!("The secret number is: {secret_number}");
            println!("Guess of {} is too high", computer_guess);
            println!(
                "Will guess new number between {} and {}",
                lowestguess, computer_guess
            );
            computer_guess = rand::thread_rng().gen_range(lowestguess..=computer_guess);
        } else {
            lowestguess = computer_guess;
            println!("The secret number is: {secret_number}");
            println!("Guess of {} is too low", computer_guess);
            println!(
                "Will guess new number between {} and {}",
                lowestguess, highestguess
            );
            computer_guess = rand::thread_rng().gen_range(lowestguess..=highestguess);
        }
        guesses += 1
    }
    let end = Instant::now();
    let duration = end - start;
    println!("You win! Only took {} guesses!", guesses);
    println!(
        "It took {} seconds to calculate the result.",
        duration.as_secs_f64()
    );
}
