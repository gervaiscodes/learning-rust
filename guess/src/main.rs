extern crate rand;

use rand::random;
use std::io;

fn get_guess() -> u8 {
    loop {
        println!("Input guess:");

        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Could not read input");

        match guess.trim().parse::<u8>() {
            Ok(v) => return v,
            Err(e) => println!("Could not parse input: {}", e)
        }
    }
}

fn handle_guess(guess: u8, correct: u8) -> bool {
    if guess < correct {
        println!("Too low");
        false
    } else if guess > correct {
        println!("Too high");
        false
    } else {
        println!("You git it!");
        true
    }
}

fn main() {
    println!("Welcome to the gussing game!");

    let correct: u8 = random::<u8>();
    println!("Correct value is {}:", correct);

    loop {
        let guess = get_guess();

        if handle_guess(guess, correct) {
            break
        }
    }
}
