use std::fmt;

// we add outer attribute to opt in to be able to debug
// since structs do not enable this by default
#[derive(Debug)]
struct Playable {
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

fn main() {
  let playableOne = Playable {
    id: String::from("a1"),
    title: String::from("Title 1"),
    bitrate: 132,
    order: 1
  };
  
  let playableTwo = Playable {
    id: String::from("a2"),
    title: String::from("Title 2"),
    bitrate: 132,
    order: 3
  };
  
  // There's no -> or . (as in C/C++)
  // Instead there's automatic referencing and dereferencing
  // &, &mut, or * automatically get added
  // it is the same as (&playableOne).is_next(&playableTwo)
  println!("{:?}", playableOne.is_next(&playableTwo));
  println!("{:?}", playableOne.id_title());
}
