use std::collections::HashMap;

// type definition
type Table = HashMap<String, Vec<String>>;

fn show(table: Table) {
    for (area, skills) in table {
        println!("Dev type: {}, skills required: {:?}", area, skills);
    }
}

// show2 receives a shared reference to the HashMap
fn show2(table: &Table) {
    for (area, skills) in table {
        println!("Dev type: {}, skills required: {:?}", area, skills);
    }
}

// show3 receives mutable reference and sorts the table in-place
fn show3(table: &mut Table) {
    for (_area, skills) in table {
        skills.sort();
    }
}

fn main() {
    let mut t = Table::new();
    
    t.insert("Front end".to_string(), vec!["HTML".to_string(), "CSS".to_string()]);
    t.insert("Back end".to_string(), vec!["Rust".to_string(), "C".to_string()]);

    // by giving ownership of t to the show function which prints it,
    // we actually destoy it
    // show(t);
    
    // by passing reference to show, we do not allow it to take ownership of it
    // it just borrows it for a bit
    // show2(&t);
    
    // passing mutable reference to sort alphabetically
    show3(&mut t);
    
    println!("{:?}", t);
}