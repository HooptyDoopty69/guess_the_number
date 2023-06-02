// #![allow(unused)]

use rand::{ thread_rng, Rng };
use std::io::stdin;
use core::cmp::Ordering;

fn main() {
    // Generate random number that the user has to guess
    let secret: i8 = thread_rng().gen_range(1..=100);

    // Prompt the user to input a random number until they guess correctly
    loop {
        println!("guess a number!\n");

        let usr_num: String = get_usr_number();

        // Checks if usr_num can be parsed as an i16 integer
        let usr_num: i8 = validate_usr_num(usr_num);

        match usr_num.cmp(&secret) {
            Ordering::Less => println!("Too low!"),
            Ordering::Equal => {
                break;
            }
            Ordering::Greater => println!("Too high!"),
        }
    }

    println!("Nice job! You won!")
}

fn get_usr_number() -> String {
    let mut input = String::new();
    stdin().read_line(&mut input).expect("Failed to read input");
    return input;
}

fn validate_usr_num(num: String) -> i8 {
    let number = num.trim().parse::<i8>().expect("Couldn't parse input. Try again.");
    return number;
}
