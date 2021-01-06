use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");
    
    // generate random number between 1 and 101 by pulling in a crate
    let secret_number = rand::thread_rng().gen_range(1..101);
    
    println!("The secret number is: {}", secret_number);

    println!("Please input your guess.");
    
    // create a place to store input, make it mutable
    // it is bound to a new empty instance of String
    let mut guess = String::new();

    // pass mutable reference of the string to read_line
    // so user input can be captured
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
    
    // "shadow" previous var, normally done when type conversion needs to occur
    let guess: u32 = guess.trim().parse().expect("Please type a number!");
        
    println!("You guessed: {}", guess);
    
    // compare input and secret number
    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You win!"),
    }
}
