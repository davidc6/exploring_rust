use std::{
    cell::{Ref, RefCell},
    collections::HashMap,
    fmt::Debug,
};

trait Logger {
    fn log(&self, value: &str);
}

enum ErrorType {
    Warn,
    Info,
    Err,
}

struct LoggerCustom {
    log_type: ErrorType,
}

impl Logger for LoggerCustom {
    fn log(&self, value: &str) {
        match self.log_type {
            ErrorType::Err => {
                println!("Error: {:?}", value);
            }
            ErrorType::Info => {
                println!("Info: {:?}", value);
            }
            ErrorType::Warn => {
                println!("Warn: {:?}", value);
            }
        }
    }
}

#[derive(Debug, PartialEq)]
struct Person<'person> {
    name: &'person str,
    connections: RefCell<Vec<&'person Person<'person>>>,
}

impl<'person> Person<'person> {
    fn new(name: &'person str) -> Self {
        Person {
            name,
            connections: RefCell::new(vec![]),
        }
    }
}

impl<'person> Logger for Person<'person> {
    fn log(&self, value: &str) {
        println!("Printing {:?}", value);
    }
}

struct PersonContext<'person> {
    person: Person<'person>,
    logger: Box<dyn Logger>,
}

impl<'person> PersonContext<'person> {
    fn new(person: Person<'person>, logger: Box<dyn Logger>) -> Self {
        PersonContext { person, logger }
    }
}

impl<'person> Person<'person> {
    fn add_connection(&self, connection: &'person Person<'person>) {
        let mut borrowed_person = self.connections.borrow_mut();
        borrowed_person.push(connection);
    }

    fn first_connection(&self) -> Option<&Person<'person>> {
        self.connections.borrow().first().copied()
    }

    fn all_connections(&mut self) -> Ref<Vec<&'person Person<'person>>> {
        self.connections.borrow()
    }
}

struct PersonIter<'person> {
    current: usize,
    size: usize,
    collection: RefCell<Vec<&'person Person<'person>>>,
}

impl<'person> Iterator for PersonIter<'person> {
    type Item = &'person Person<'person>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current == self.size {
            None
        } else {
            self.current += 1;
            Some(self.collection.borrow()[self.current - 1])
        }
    }
}

impl<'person> IntoIterator for &'person Person<'person> {
    type Item = &'person Person<'person>;
    type IntoIter = PersonIter<'person>;

    fn into_iter(self) -> Self::IntoIter {
        PersonIter {
            current: 0,
            size: self.connections.borrow().len(),
            collection: self.connections.clone(),
        }
    }
}

impl<'person> Iterator for Person<'person> {
    type Item = &'person Person<'person>;

    fn next(&mut self) -> Option<Self::Item> {
        self.connections.get_mut().pop()
    }
}

struct Connections<'a> {
    logger: Box<dyn Logger>,
    connections: HashMap<String, Person<'a>>,
}

impl<'a> Connections<'a> {
    fn new(logger: Box<dyn Logger>) -> Self {
        Self {
            logger,
            connections: HashMap::new(),
        }
    }
}

impl<'a> Connections<'a> {
    fn add(&mut self, name: &str, person: Person<'a>) {
        self.connections.insert(name.to_owned(), person);
    }

    fn get_mut(&mut self, name: &str) -> &'a mut Person {
        let person_with = self.connections.get_mut(name).unwrap();
        person_with
    }

    fn get(&self, name: &str) -> Option<&'a Person> {
        self.connections.get(name)
    }

    fn connect(&'a self, name: &str, with: &'a Person<'a>) -> Option<&'a Person<'a>> {
        if let Some(person) = self.connections.get(name) {
            person.add_connection(with);

            self.logger
                .log(&format!("{} connected with {}", name, with.name));

            Some(person)
        } else {
            None
        }
    }
}

fn main() {
    let list_of_names = ["Ann", "John", "Mary", "Michael", "Kathy"];

    let logger = LoggerCustom {
        log_type: ErrorType::Info,
    };
    let mut connections = Connections::new(Box::new(logger));

    for name in list_of_names {
        let person = Person::new(name);
        connections.add(name, person);
    }

    if let Some(person) = connections.get("John") {
        connections.connect("Ann", person);
    }

    if let Some(person) = connections.get("Mary") {
        connections.connect("Ann", person);
    }

    if let Some(person) = connections.get("Ann") {
        connections.connect("John", person);
    }

    // Example
    // Get Ann's connections and whether these are connected with Ann too
    if let Some(connections) = connections.get("Ann") {
        for connection in connections {
            println!("Ann is connected with: {:?}", connection.name);

            let borrowed_connections = connection.connections.borrow();
            let has_connected_with_ann = borrowed_connections.iter().any(|p| p.name == "Ann");

            let connection_text = if has_connected_with_ann {
                "is not connected with"
            } else {
                "is connected with"
            };

            println!("{} {} Ann", connection.name, connection_text);
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{Connections, LoggerCustom, Person};

    #[test]
    fn add_and_connect_person() {
        let logger = LoggerCustom {
            log_type: crate::ErrorType::Info,
        };
        let mut connections = Connections::new(Box::new(logger));

        let person_one = Person::new("Jim");
        let person_one_name = person_one.name;

        let person_two = Person::new("John");

        connections.add(person_one_name, person_one);
        connections.connect(person_one_name, &person_two);

        assert_eq!(
            connections
                .get(person_one_name)
                .unwrap()
                .connections
                .borrow()[0],
            &person_two
        );
    }
}
