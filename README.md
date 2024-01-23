# Guessing Game - Technical Specification

## Overview

The Guessing Game is a simple console-based application implemented in the Rust programming language. The game generates a random number, and the player attempts to guess the correct number within a specified range.

## Features

- Generates a random number within a specified range.
- Allows the player to input guesses.
- Provides feedback on each guess, indicating whether the guess is too small, too large, or correct.
- Continues the game until the correct number is guessed.
- Handles invalid input gracefully.

## Dependencies

- `rand` crate (version 0.8.5): Used for generating random numbers.

## Implementation Details

### Main Function

The main function serves as the entry point of the application. It initializes the game, generates a random number, and enters a loop to handle player input and check for the correctness of the guesses.

```rust
fn main() {
    // Initialization and game setup
    // ...

    loop {
        // Player input handling
        // ...

        // Guess evaluation and feedback
        // ...

        // Check if the game should continue or end
        // ...
    }
}
```

### Random Number Generation

The `rand::thread_rng().gen_range()` function is used to generate a random number within the specified range (1 to 100). The generated number serves as the target for the player to guess.

```rust
let secret_number = rand::thread_rng().gen_range(1..101);
```

### Player Input Handling

The game prompts the player to enter their guess. The input is read from the standard input, and input validation is performed to ensure the entered value is a valid unsigned integer.

```rust
let mut guess = String::new();

io::stdin()
    .read_line(&mut guess)
    .expect("Failed to read line");

let guess: u32 = match guess.trim().parse() {
    Ok(num) => num,
    Err(_) => {
        // Handle invalid input
        // ...
        continue;
    }
};
```

### Guess Evaluation and Feedback

The player's guess is compared to the secret number using a `match` statement to provide feedback on whether the guess is too small, too large, or correct.

```rust
match guess.cmp(&secret_number) {
    std::cmp::Ordering::Less => println!("Too small!"),
    std::cmp::Ordering::Greater => println!("Too large!"),
    std::cmp::Ordering::Equal => {
        println!("Congratulations! You guessed the correct number: {}", secret_number);
        break;
    }
}
```

### Game Continuation

The game continues in a loop until the player guesses the correct number. Once the correct guess is made, the loop is exited, and the game congratulates the player on their success.

## Build and Run

To build and run the Guessing Game, the following dependencies are required:

1. Rust compiler and Cargo.
2. Add the `rand` crate to the `Cargo.toml` file:

```toml
[dependencies]
rand = "0.8.5"
```

3. Run the following commands in the terminal:

```sh
$ cargo build
$ cargo run
```

## Conclusion

The Guessing Game provides a simple yet entertaining experience for players, challenging them to guess a randomly generated number within a specified range. The game showcases basic input handling, random number generation, and flow control in the Rust programming language.
