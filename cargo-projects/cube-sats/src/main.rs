// use std::fmt::Debug;

#[derive(Debug)]
struct CubeSat {
  id: u64,
  messages: Vec<String>
}

impl CubeSat {
  fn receive(&mut self) -> Option<String> {
    self.messages.pop()
  }
}

struct GroundBase {
  id: u16
}

impl GroundBase {
  fn send_msg(&self, sat: &mut CubeSat, message: String) {
    sat.messages.push(message);
  }
}

impl GroundBase {
  fn connect(&self, sat_id: u64) -> CubeSat {
    CubeSat { id: sat_id, messages: vec![] }
  }
}

fn main() {
  let ground_base = GroundBase { id: 1 };

  let mut sat_a = CubeSat { id: 1, messages: Vec::new() };
  let message_a = String::from("hello"); // stored on the heap
  
  let mut sat_b = CubeSat { id: 2, messages: Vec::new() };
  let message_b = String::from("hello 2");
  
  ground_base.send_msg(&mut sat_a, message_a);
  ground_base.send_msg(&mut sat_b, message_b);

  // println!("{:?}", sat_a);
  println!("{}", ground_base.id);
  println!("CubeSat ID: {}, message: {:?}", sat_a.id, sat_a.messages);
  println!("{}", sat_a.receive().unwrap());
  println!("CubeSat ID: {}, message: {:?}", sat_b.id, sat_b.messages);
}
