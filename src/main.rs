extern crate rand;

use rand::Rng;
use std::cmp::Ordering;
use std::collections::HashSet;
use std::io;

use std::fs;

fn guess_the_number() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("you win!");
                break;
            }
        }
    }
}

fn main() {
    let mut state_names = HashSet::new();
    let states_filename = "states.txt";
    let state_contents =
        fs::read_to_string(states_filename).expect("Something went wrong reading the file");
    for line in state_contents.lines() {
        state_names.insert(line.trim().to_lowercase());
    }

    let world_capitals_filename = "world_capitals.txt";
    let contents =
        fs::read_to_string(world_capitals_filename).expect("Something went wrong reading the file");

    for world_capital_line in contents.lines() {
        if (world_capital_line.len() < 5) {
            continue;
        }
        let trimmed: String = world_capital_line
            .to_lowercase()
            .split_ascii_whitespace()
            .collect();
        let first_three = &trimmed[..2];
        let last_three = &trimmed[4..];
        let together = format!("{}{}", first_three, last_three);

        if (state_names.contains(&together[..])) {
            println!("{}", together);
        }
    }
}
