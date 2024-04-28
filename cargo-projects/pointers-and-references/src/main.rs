use std::fmt::{Debug, Display};

trait Logger<T> {
    fn log(&self, value: &T);
}

struct LoggerCustomer {}

impl<T: Debug> Logger<T> for LoggerCustomer {
    fn log(&self, value: &T) {
        println!("{:?}", &value);
    }
}

#[derive(Debug)]
struct Person<'person> {
    name: String,
    connections: Vec<&'person mut Person<'person>>,
}

impl<'person> Person<'person> {
    fn new(name: String) -> Self {
        Person {
            name,
            connections: vec![],
        }
    }
}

impl<'person, T: Debug> Logger<T> for Person<'person> {
    fn log(&self, value: &T) {
        println!("Printing {:?}", value);
    }
}

struct PersonContext<'person> {
    person: Person<'person>,
}

impl<'person> PersonContext<'person> {
    fn new(person: Person<'person>) -> Self {
        PersonContext { person }
    }
}

impl<'person> Person<'person> {
    fn add_connection(&mut self, connection: &'person mut Person<'person>) {
        let mut name = connection.name.clone();
        self.connections.push(connection);
        // let immut = connection;
        self.log(&mut name);
    }

    fn first_connection(&mut self) -> &mut Person<'person> {
        // self
        self.connections[0]
    }

    fn all_connections(&mut self) -> &Vec<&'person mut Person<'person>> {
        &self.connections
    }
}

// struct PersonIter {
//     current: usize,
// }

impl<'person> Iterator for Person<'person> {
    type Item = &'person mut Person<'person>;

    fn next(&mut self) -> Option<Self::Item> {
        self.connections.pop()
    }
}

fn main() {
    let mut person_a = Person::new("Ann".to_owned());
    // person_a.first_connection(); // TODO
    // context
    // let l: LoggerCustomer = LoggerCustomer {};
    // let mut person_a_new = PersonContext::new(Person::new("Ann".to_owned()));

    let mut person_b = Person::new("John".to_owned());
    let mut person_c = Person::new("Mary".to_owned());

    // person_a.add_connection(&mut person_b);
    person_a.add_connection(&mut person_b);

    let person_a_first_connection = &mut person_a.first_connection();
    person_a_first_connection.add_connection(&mut person_c);

    let mut person_d = Person::new("Michal".to_owned());
    let mut person_f = Person::new("Kathy".to_owned());

    person_a.add_connection(&mut person_d);
    person_a.add_connection(&mut person_f);

    for connection in person_a {
        println!("Name: {:?}", connection.name);
    }
}
