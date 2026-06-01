use std::io::stdin;
use std::cmp::Ordering;
use rand::random_range;

fn main() {
	
    println!("Welcome to my guessing game!");
    
        /*

        let random_number = rand::thread_rng() 
             .gen_range(1..=100);
        
        I don't think that rand::thread_rng() works in the latest version of the rand crate, I checked docs.rs and I think I found the solution based on what I wrote below
        */

        let random_number = random_range(1..=100);

    loop {
        println!("Enter a whole number.");
            
        let mut guess = String::new();
        stdin()
            .read_line(&mut guess)
            .expect("Failed to read your input.");
        
        println!("The random number is: {random_number}");

        let guess: u32 = match guess.trim()
            .parse() {
                Ok(num) => num,
                Err(_) => continue,
            };

        println!("Your guess: {guess}");
        
        match guess.cmp(&random_number) {
            Ordering::Less => println!("Too little!"),
            Ordering::Greater => println!("Too large!"),
            Ordering::Equal => {
                println!("You guessed the random number!");
                break;
            }
        }
    }
}
