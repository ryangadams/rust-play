use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    let secret_number = rand::thread_rng().gen_range(1..=100);

    let mut is_correct = false;
    let mut guess_count = 0;

    println!("Let's play a guessing game, pick a number:");
    while !is_correct {
        guess_count += 1;
        is_correct = check_guess(&secret_number, &guess_count, read_guess());
    }
}

fn check_guess(secret_number: &u32, guess_count: &i32, guess: u32) -> bool {
    println!("You guessed {guess}");
    match guess.cmp(&secret_number) {
        Ordering::Less => {
            println!("Too small");
            false
        }
        Ordering::Greater => {
            println!("Too big");
            false
        }
        Ordering::Equal => {
            println!("Just right, you win");
            println!("You got the right answer in {guess_count} guesses");
            true
        }
    }
}

fn read_guess() -> u32 {
    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
    guess.trim().parse().expect("You didn't enter a number")
}
