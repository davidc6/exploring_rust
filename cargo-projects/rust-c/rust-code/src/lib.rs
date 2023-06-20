mod wrapper;

#[derive(Clone, Debug)]
pub struct Person {
    first_name: String,
    last_name: String,
}

pub enum Error {
    Failed,
}

impl Person {
    pub fn new(first_name: &str, last_name: &str) -> Result<Person, Error> {
        Ok(Person {
            first_name: String::from(first_name),
            last_name: String::from(last_name),
        })
    }

    pub fn cap_first_name(&'static mut self) {
        let mut chars = self.first_name.chars();

        let caped = match chars.next() {
            None => String::new(),
            Some(c) => c.to_uppercase().collect::<String>() + chars.as_str(),
        };

        self.first_name = caped;
    }

    pub fn update_last_name(&'static mut self, last_name: String) {
        self.last_name = last_name;
    }
}
