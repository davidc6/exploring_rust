use std::rc::Rc; // reference counting / track valid references
use std::cell::RefCell; // mutable memory location with dynamically checked borrow rules

#[derive(Debug, Clone, Copy)] // tells copiler to add implementation of each trait
struct CubeSat {
  id: u64
}

impl CubeSat {
  fn receive(&self, mailbox: &mut Mailbox) -> Option<Message> {
    mailbox.deliver(self)
  }
}

#[derive(Debug)]
struct Message {
  content: String,
  to: u64
}  

#[derive(Debug)]
struct GroundStation {
  freq: f64 // Mhz
}

#[derive(Copy, Clone)]
struct GroundBase;

impl GroundBase {
  fn connect(&self, sat_id: u64) -> CubeSat {
    CubeSat { id: sat_id }
  }

  fn send(&self, mailbox: &mut Mailbox, msg: Message) {
    mailbox.post(msg);
  }
}

struct Mailbox {
  messages: Vec<Message>
}

impl Mailbox {
  fn deliver(&mut self, recipient: &CubeSat) -> Option<Message> {
    for i in 0..self.messages.len() {
      if self.messages[i].to == recipient.id {
        let msg = self.messages.remove(i);
        return Some(msg);
      }
    }

    None
  }
  
  fn post(&mut self, msg: Message) {
    self.messages.push(msg);
  }
}

fn fetch_all_sat_ids() -> Vec<u64> {
  vec![1,2,3]
}

fn check_sat(sat: CubeSat) -> u64 {
  sat.id
}

fn main() {
  // testing Copy and Clone traits
  let sat_d = CubeSat { id: 100 };

  let sat_d_id = check_sat(sat_d.clone());
  println!("sat id: {:?}", sat_d_id.clone());

  let sat_d_id = check_sat(sat_d);
  println!("sat id: {:?}", sat_d_id);

  // ch 4.20
  // 1. Adding functionality can reduce runtime performance
  // 2. If Clone is expensive, Rc<T> can be a good alternative (allows to share ownership)
  let base: Rc<RefCell<GroundStation>> = Rc::new(RefCell::new(
    GroundStation {
      freq: 87.65
    }
  ));

  println!("base: {:?}", base);

  {
    let mut base_2 = base.borrow_mut();
    base_2.freq -= 12.34;
    println!("base 2: {:?}", base_2);
  }

  println!("base: {:?}", base);
  let mut base_3 = base.borrow_mut();
  base_3.freq += 43.21;

  println!("base: {:?}", base);
  println!("base_3: {:?}", base_3);

  // testing actual program implementation
  let mut mailbox = Mailbox { messages: vec![] };
  let ground_base = GroundBase{};
  
  let sat_ids = fetch_all_sat_ids();
  
  for (i, id) in sat_ids.iter().enumerate() {
    let sat = ground_base.connect(*id);
    let msg = Message { to: sat.id, content: format!("{} {}",String::from("hello"), i) };

    ground_base.send(&mut mailbox, msg);
  }

  // let sat_ids = fetch_all_sat_ids();
  
  for id in sat_ids {
    let sat = ground_base.connect(id);
    let msg = sat.receive(&mut mailbox);

    println!("{:?}: {}", sat, msg.unwrap().content);
  }
}
