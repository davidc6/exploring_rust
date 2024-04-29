use std::{
    borrow::BorrowMut,
    cell::{Ref, RefCell},
    collections::HashMap,
    fmt::{Debug, Display},
};

trait Logger {
    fn log(&self, value: String);
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
    fn log(&self, value: String) {
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

// impl<T: Debug> Logger<T> for LoggerCustom {
//     fn log(&self, value: &T) {
//         println!("{:?}", &value);
//     }
// }

// struct Logger(Arc<dyn Logger>);

// Logger(

#[derive(Debug)]
struct Person<'person> {
    name: String,
    connections: RefCell<Vec<&'person Person<'person>>>,
}

impl<'person> Person<'person> {
    fn new(name: String) -> Self {
        Person {
            name,
            connections: RefCell::new(vec![]),
        }
    }
}

impl<'person> Logger for Person<'person> {
    fn log(&self, value: String) {
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
    fn add_connection(&mut self, connection: &'person mut Person<'person>) {
        let name = connection.name.clone();
        self.connections.get_mut().push(connection);
        self.log(name);
    }

    fn first_connection(&mut self) -> &Person<'person> {
        // self
        self.connections.borrow()[0]
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

// impl<'person> Iterator for &Person<'person> {
//     type Item = &'person Person<'person>;

//     fn next(&mut self) -> Option<Self::Item> {
//         // self.connections.borrow().pop()
//     }
// }

// fn add_connection_and_log(person, connection, logger) {
//     let name = person.name;
//     let name_two = connection.name;

//     person.add_connection(connection);
//     logger.log("{:?} added {:?}", name, name_two);
// }

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

    // fn connect(&mut self, name: &str, connection: &'a mut Person<'a>) {
    //     // let name_a = person.name.clone();
    //     let name_b = connection.name.clone();

    //     let person_a = self.connections.get_mut(name).unwrap();

    //     person_a.add_connection(connection);

    //     let l = &self.logger;
    //     // .log(&format!("{:?} connected with {:?}", name_a, name_b));
    //     l.log(format!("{:?} {:?}", name, name_b));
    // }

    fn get_mut(&mut self, name: &str) -> &'a mut Person {
        let person_with = self.connections.get_mut(name).unwrap();
        person_with
    }

    fn get(&self, name: &str) -> Option<&'a Person> {
        self.connections.get(name)
    }

    fn connect(&'a self, name: &str, with: &'a Person<'a>) -> Option<&'a Person<'a>> {
        if let Some(person) = self.connections.get(name) {
            let mut borrowed_person = person.connections.borrow_mut();
            borrowed_person.push(with);

            Some(person)
        } else {
            None
            // create person
            // add to connections
            // add connection to person's connection list
        }
    }
}

fn main() {
    let person_a = Person::new("Ann".to_owned());

    // person_a.first_connection(); // TODO: out of bounds -> how to deal?
    // context
    // let l: LoggerCustomer = LoggerCustomer {};
    // let mut person_a_new = PersonContext::new(Person::new("Ann".to_owned()));

    let person_b = Person::new("John".to_owned());
    let person_c = Person::new("Mary".to_owned());
    let person_d = Person::new("Michal".to_owned());
    let person_e = Person::new("Kathy".to_owned());

    let logger = LoggerCustom {
        log_type: ErrorType::Info,
    };
    let mut connections = Connections::new(Box::new(logger));

    connections.add("Ann", person_a);
    connections.add("John", person_b);
    connections.add("Mary", person_c);
    connections.add("Michael", person_d);
    connections.add("Kathy", person_e);

    if let Some(person) = connections.get("John") {
        connections.connect("Ann", person);
    }

    if let Some(person) = connections.get("Mary") {
        connections.connect("Ann", person);
    }

    if let Some(person) = connections.get("Ann") {
        connections.connect("John", person);
    }

    if let Some(connections) = connections.get("Ann") {
        for connection in connections {
            println!("{:?}", connection.name);
        }
    }
}
