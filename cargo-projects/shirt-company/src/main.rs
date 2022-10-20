use std::fmt::{Debug, Display, Formatter, Result};

#[derive(Debug, Copy, Clone)]
enum ShirtColour {
    Black,
    White
}

// implementing Display trait enables to "convert" enums to Strings
impl Display for ShirtColour {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            ShirtColour::Black => write!(f, "Black"),
            ShirtColour::White => write!(f, "White"),
        }
    }
}

struct Inventory {
    shirts: Vec<ShirtColour>,
}

impl Inventory {
    /// returns if Some variant or 
    /// calls the closure which captures an immutable reference to self Inventory 
    /// (which function cannot capture the env) if None variant
    /// the result of
    fn giveaway(&self, user_preference: Option<ShirtColour>) -> ShirtColour {
        user_preference.unwrap_or_else(|| self.most_stocked())
    }

    fn most_stocked(&self) -> ShirtColour {
        let mut num_black = 0;
        let mut num_white = 0;

        for colour in &self.shirts {
            match colour {
                ShirtColour::Black => num_black += 1,
                ShirtColour::White => num_white += 1,
            }
        }

        if num_black > num_white {
            ShirtColour::Black
        } else {
            ShirtColour::White
        }
    }
}

fn main() {
    let store = Inventory {
        shirts: vec![ShirtColour::Black, ShirtColour::White, ShirtColour::Black],
    };

    let user_pref1 = Some(ShirtColour::Black);
    let giveaway1 = store.giveaway(user_pref1);
    println!(
        "User with pref {:?} gets {:?}",
        user_pref1,
        giveaway1
    );

    let user_pref2: Option<ShirtColour> = None;
    let giveaway2 = store.giveaway(user_pref2);
    println!(
        "User with pref {:?} gets {:?}",
        user_pref2,
        giveaway2
    );

    let colour: String = ShirtColour::Black.to_string();
    println!("{}", colour);
}
