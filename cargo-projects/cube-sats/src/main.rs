// use std::fmt::Debug;

#[derive(Debug)]
struct CubeSat {
  id: u64,
  messages: Vec<String>
}

#[derive(Debug)]
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
  }
  
  let sat_ids = fetch_all_sat_ids();
  
  for id in sat_ids {
    let sat = ground_base.connect(id);
    
    let msg = sat.receive(&mut mail);
    println!("{:?}: {:?}", sat, msg);
  }
}
