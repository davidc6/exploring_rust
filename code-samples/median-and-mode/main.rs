use std::collections::HashMap;

type ReturnType = (usize, usize);
// return variant 2
// type ReturnType<'a> = (usize, HashMap<&'a usize, usize>);

fn find(i: &mut [usize; 7]) -> ReturnType {
    i.sort();
    let median = i.len() / 2;
    
    let mut hm = HashMap::new();
    let mut most_frequent = 0;
    
    for val in i.iter() {
        if hm.contains_key(&val) {
            let new_val = hm.get(&val).unwrap() + 1;

            hm.insert(val, new_val);
        } else {
            hm.insert(val, 1);
        }

        if hm.get(&val) > hm.get(&most_frequent) {
            most_frequent = *val;
        }
    }
    
    // return variant 2
    // (median, hm)
    (median, most_frequent)
}

fn main() {
    let mut i: [usize; 7] = [4, 1, 6, 3, 4, 2, 0];
    let result = find(&mut i);
    
    println!("{:?}", result);
}
 