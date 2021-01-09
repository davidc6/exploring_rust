use std::io::stdin;

fn is_valid_pin(user_input: String) -> bool {
    const SECRET_PIN: u16 = 1234;

    let trimmed = user_input.trim();

    match trimmed.parse::<u16>() {
        Ok(i) => {
            if i == SECRET_PIN {
                return true
            } else {
                return false;
            }
        }
        Err(_) => false
    }
}

fn menu() {
  println!("Options:");
  println!("1 - Balance");
  println!("2 - Deposit");
  println!("3 - Withdraw");
  println!("4 - Quit");
}

fn main() {
    println!("Please enter pin:");

    let mut user_input = String::new();

    match stdin().read_line(&mut user_input) {
        Ok(_) => {            
            if is_valid_pin(user_input) {
                println!("Valid pin");
                menu();
            } else {
                println!("Invalid pin");
            }
        }
        Err(error) => println!("error: {}", error)
    }
    
    println!("Goodbye.");
}
