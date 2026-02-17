// Assignment 3: Guessing Game

fn check_guess(guess: i32, secret: i32) -> i32 {
    if guess == secret {
        0
    } else if guess > secret {
        1
    } else {
        -1
    }
}

fn main() {
    // 1) Secret number (hard-coded)
    let secret: i32 = 42;

    // We'll simulate "user input" with a fixed sequence of guesses.
    // (Still only arrays + loops + variables.)
    let guesses: [i32; 6] = [10, 60, 41, 50, 42, 100];

    let mut attempts: i32 = 0;
    let mut i: usize = 0;

    loop {
        // Safety in case the array ends (prevents infinite loop)
        if i >= guesses.len() {
            println!("Out of guesses!");
            break;
        }

        let guess: i32 = guesses[i];
        attempts = attempts + 1;

        let result = check_guess(guess, secret);

        if result == 0 {
            println!("Guess {}: {} -> Correct!", attempts, guess);
            break;
        } else if result == 1 {
            println!("Guess {}: {} -> Too high", attempts, guess);
        } else {
            println!("Guess {}: {} -> Too low", attempts, guess);
        }

        i = i + 1;
    }

    println!("It took {} guess(es).", attempts);
}
