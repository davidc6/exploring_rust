// use std::fmt::Debug;

#[derive(Debug)]
struct CubeSat {
  id: u64,
  messages: Vec<String>
}

struct Message {
  content: String,
  to: u64
}

impl CubeSat {
  fn receive(&self, mailbox: &mut Mailbox) -> Option<Message> {
    mailbox.deliver(&self)
  }
}

struct GroundBase;

impl GroundBase {
  fn connect(&self, sat_id: u64) -> CubeSat {
    CubeSat { id: sat_id, messages: vec![] }
  }

  fn send(&self, to: &mut CubeSat, msg: String) {
    to.messages.push(msg);
  }
}

struct Mailbox {
  // to: u64,
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
}

fn fetch_all_sat_ids() -> Vec<u64> {
  vec![1,2,3]
}

fn main() {
  let mut mail = Mailbox { messages: vec![] };
  let ground_base = GroundBase{};
  
  let sat_ids = fetch_all_sat_ids();
  
  for id in sat_ids {
    let mut sat = ground_base.connect(id);
    let msg = String::from("hello");

    ground_base.send(&mut sat, msg);

    println!("CubeSat ID: {}, message: {:?}", sat.id, sat.messages);
  }
  
  let sat_ids = fetch_all_sat_ids();
  
  for id in sat_ids {
    let sat = ground_base.connect(id);
    
    let msg = sat.receive(&mut mail);
  }
  
  // let mut sat_a = CubeSat { id: 1, messages: Vec::new() };
  // let message_a = String::from("hello"); // stored on the heap
  
  // let mut sat_b = CubeSat { id: 2, messages: Vec::new() };
  // let message_b = String::from("hello 2");
  
  // ground_base.send_msg(&mut sat_a, message_a);
  // ground_base.send_msg(&mut sat_b, message_b);

  // // println!("{:?}", sat_a);
  // println!("{}", ground_base.id);
  // println!("CubeSat ID: {}, message: {:?}", sat_a.id, sat_a.messages);
  // println!("{}", sat_a.receive().unwrap());
  // println!("CubeSat ID: {}, message: {:?}", sat_b.id, sat_b.messages);
}
