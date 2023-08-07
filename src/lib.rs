use std::io;
use std::cmp::Ordering;
use rand::Rng;

pub fn guessing_game() {
    println!("Guess a number between 1 and 100: ");
    let secret_number = rand::thread_rng().gen_range(1..=100);
    
    loop {
        let mut guess = String::new();

        io::stdin().read_line(&mut guess).expect("Failed to take input");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a number");
                continue;
            }
        };

        println!("Your guess: {guess}");
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("You got it right");
                break;
            }
        }
    }

    println!("The secret number is: {secret_number}");
}
