#[derive(Debug)]
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

fn main() {
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
