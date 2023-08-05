use std::cmp::Ordering;

use rand::Rng;

fn main() {
    println!("Guess");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("The secret number is: {secret_number}");

    println!("Input a number");

    loop {
        let mut guess = String::new();

        match std::io::stdin().read_line(&mut guess) {
            Ok(n) => {
                println!("{n} bytes read");
            }
            Err(e) => panic!("Can't read from stdin, err={e}"),
        };

        let guess = guess.trim().parse::<u32>().expect("Please type a number!");

        match guess.cmp(&secret_number) {
            Ordering::Equal => {
                println!("You guessed right!");
                break;
            }
            _ => print!("You guessed wrong!"),
        };
    }
}
