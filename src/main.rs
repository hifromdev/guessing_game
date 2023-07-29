use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {

    println!("Guess the number!");

    // Assigns a random number between 1 to 100 inclusively to an immutable u32 variable 'secret_number'.
    let secret_number: u32 = rand::thread_rng().gen_range(1..=100);

    // This loop is indefinite until the user guesses the 'secret_number' through stdin.
    loop {

        println!("The secret number is: {secret_number}");
        println!("Please input your guess.");

        // Assigns user input to a new mutable String named 'guess'.
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line.");

        // Check if Result returns Ok or Err and acts accordingly.
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");

        // Compares 'secret_number' and 'guess' and acts accordingly.
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            },
        }
    }    
}