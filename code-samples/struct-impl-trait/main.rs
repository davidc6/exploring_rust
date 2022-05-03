use std::fmt;

// we add outer attribute to opt in to be able to debug
// since structs do not enable this by default
#[derive(Debug)]
pub struct Playable {
  id: String,
  title: String,
  bitrate: u8,
  order: u8
}

// Traits - very similar to interfaces, defines a functionality a type can have
// This functionality can be shared between types
trait IdTitle {
  fn id_title(&self) -> String;
}

impl Playable {
  fn is_low_bitrate(&self) -> bool {
    if &self.bitrate < &160 {
      return true
    }
    return false
  }

  fn is_next(&self, other_item: &Playable) -> bool {
    let diff = other_item.order - self.order;

    if diff == 1 {
      return true;
    }
    return false;
  }
}

// Playable struct implements IdTitle struct
impl IdTitle for Playable {
  fn id_title(&self) -> String {
    return format!("{} {}", self.id, self.title);
  }
}

// this impl block could also be implemented for playable struct
// impl fmt::Display for Playable {
//   fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//     write!(f, "({}, {}, {})", self.id, self.title, self.bitrate)
//   }
// }

// modules
mod item_formatter {
  pub mod displaying {
    pub fn show(id: &str, title: &str) -> String {
      return format!("id: {}, title: {}", &id, &title)
    }
  }

  pub mod editing {
    pub fn change_name(playable: &mut crate::Playable) {
      // mutable reference allows to mutate title
      playable.title = String::from("Hello World");
    }
  }
}

fn main() {
  let mut playable_one = Playable {
    id: String::from("a1"),
    title: String::from("Title 1"),
    bitrate: 132,
    order: 1
  };

  let playable_two = Playable {
    id: String::from("a2"),
    title: String::from("Title 2"),
    bitrate: 132,
    order: 3
  };

  // There's no -> or . (as in C/C++)
  // Instead there's automatic referencing and dereferencing
  // &, &mut, or * automatically get added
  // it is the same as (&playableOne).is_next(&playableTwo)
  println!("{:?}", playable_one.is_next(&playable_two));
  println!("{:?}", playable_one.id_title());
  println!("{:?}", playable_one.is_low_bitrate());
  println!("{:?}", item_formatter::displaying::show(&playable_one.id, &playable_one.title));
  // pass mutable reference (address that can be followed)
  item_formatter::editing::change_name(&mut playable_one);
  println!("{:?}", playable_one.id_title());
}
