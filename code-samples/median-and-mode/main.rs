use std::collections::HashMap;

type ReturnType<'a> = (usize, HashMap<&'a usize, i8>);

fn find(i: &mut [usize; 7]) -> ReturnType {
    i.sort();
    let median = i.len() / 2;
    
    let mut hm = HashMap::new();
    
    for val in i.iter() {
        if hm.contains_key(&val) {
            hm.insert(val, hm.get(&val).unwrap() + 1);
            continue;
        }
        hm.insert(val, 1);
    }
    
    (median, hm)
}

fn main() {
    let mut i: [usize; 7] = [4, 1, 6, 3, 4, 2, 0];
    let result = find(&mut i);
    
    println!("{:?}", result);
}
 