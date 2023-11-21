# Computer's Number Guessing Game in Rust

I decided to code up a simple number-guessing scenario in Rust to to learn. 

## How It Works

1. **Setup:** The computer randomly picks a secret number between 0 and 5000.
2. **Guessing Game:** The computer takes a shot at guessing the secret number and adjusts its strategy based on feedback.
3. **Repeat:** The game loops until the computer cracks the code.

## What's Inside

- **Randomness with `rand`**: Used the `rand` crate for a bit of unpredictability.
- **Keeping Track**: The computer counts its guesses and notes how long the game takes.
- Keeps track of the highest and lowest guess and uses that to get closser to the target.

## Running the Game

1. Clone this repository.
2. Ensure Rust is installed.
3. Run `cargo run`

Enjoy the game! 💻
