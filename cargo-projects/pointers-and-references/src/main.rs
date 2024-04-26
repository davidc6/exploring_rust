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

impl<'person> Person<'person> {
    fn add_connection(&mut self, connection: &'person mut Person<'person>) {
        self.connections.push(connection);
    }

    fn first_connection(&mut self) -> &mut Person<'person> {
        self.connections[0]
    }

    fn all_connections(&mut self) -> &Vec<&'person mut Person<'person>> {
        &self.connections
    }
}

struct PersonIter {
    current: usize,
}

impl<'person> Iterator for Person<'person> {
    type Item = &'person mut Person<'person>;

    fn next(&mut self) -> Option<Self::Item> {
        self.connections.pop()
    }
}

fn main() {
    let mut person_a = Person::new("Ann".to_owned());
    let mut person_b = Person::new("John".to_owned());
    let mut person_c = Person::new("Mary".to_owned());

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
