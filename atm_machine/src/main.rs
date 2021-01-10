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

fn deposit(balance: &mut u32) {
    println!("How much do you want to deposit? (£)");

    let mut user_input = String::new();

    stdin().read_line(&mut user_input).ok();

    match user_input.trim().parse::<u32>() {
        Ok(input) => {
            if input > 0 {
                *balance += input;
                println!("£{} has been deposited.", input);
            }            
        },
        Err(_) => println!("Please deposit a valid amount.")
    }
}

fn withdraw(balance: &mut u32) {
    println!("How much do you want to withdraw? (£)");

    let mut user_input = String::new();

    stdin().read_line(&mut user_input).ok();
    
    match user_input.trim().parse::<u32>() {
        Ok(input) => {
            if *balance >= input {
                *balance -= input;
                println!("£{} has been withdrawn.", input);
                return;
            }
            println!("Your balance is less than the amount that want to withdraw.");
        },
        Err(_) => println!("Please withdraw a valid amount.")
    }
}

fn menu() {
    let mut should_quit = false;
    let mut balance: u32 = 0;

    while !should_quit {
        println!("Options:");
        println!("1 - Balance");
        println!("2 - Deposit");
        println!("3 - Withdraw");
        println!("4 - Quit");

        // stored as vector of bytes
        let mut user_input = String::new();

        stdin().read_line(&mut user_input).ok();

        user_input = String::from(user_input.trim());

        match user_input.as_str() {
            "1" => println!("Your balance is: £{}.", balance),
            "2" => deposit(&mut balance),
            "3" => withdraw(&mut balance),
            "4" => should_quit = true,
            _ => println!("Option not recognised")
        }
    }
}

fn main() {
    println!("Please enter pin:");

    let mut user_input = String::new();

    match stdin().read_line(&mut user_input) {
        Ok(_) => {
            if is_valid_pin(user_input) {
                println!("Welcome!");
                menu();
            } else {
                println!("Invalid pin");
            }
        }
        Err(error) => println!("error: {}", error)
    }

    println!("Goodbye.");
}
