use std::collections::{HashMap, hash_map::Entry::{Vacant, Occupied}};
use std::sync::{Arc, Mutex};

fn count(s: String, threads: usize) -> char {
    let char_freq = Arc::new(Mutex::new(HashMap::<String, usize>::new()));
    let mut most_freq_char = ' ';

    let chars_per_thread = if s.len() == threads { threads } else { s.len() / threads };
    let last_split_len =
        match s.len() % threads {
            0 => chars_per_thread,
            _ => s.len() % threads + chars_per_thread
        };

    let iterations = if s.len() == threads { 1 } else { chars_per_thread };

    for n in 0..iterations {
        let str_cloned = s.clone();
        let char_freq_cloned = char_freq.clone();

        let t = std::thread::spawn(move || {
            let start = if n == 0 { 0 } else { n * threads };
            let end = if n == 0 { chars_per_thread } else { start + threads };

            let str_section = &str_cloned[start..end];
            let mut chars_map = char_freq_cloned.lock().unwrap();

            for char in str_section.chars() {
                if char == ' ' {
                    continue;
                }

                match chars_map.entry(char.to_string()) {
                    Occupied(mut e) => {
                        e.insert(e.get() + 1);
                    },
                    Vacant(e) => {
                        e.insert(1);
                    }
                }
            }
            
        });

        t.join();
    }

    if last_split_len != chars_per_thread {
        let last = &s[s.len() - (last_split_len - chars_per_thread)..s.len()];
        let mut chars_map = char_freq.lock().unwrap();

        for char in last.chars() {
            match chars_map.entry(char.to_string()) {
                Occupied(mut e) => {
                    e.insert(e.get() + 1);
                },
                Vacant(e) => {
                    e.insert(1);
                }
            }
        }
    }

    let chars = char_freq.lock().unwrap();

    for (char, _) in chars.iter() {
        if most_freq_char == ' ' {
            most_freq_char = char.chars().next().unwrap();
            continue;
        }

        if chars.get(char) > chars.get(&most_freq_char.to_string()) {
            most_freq_char = char.chars().next().unwrap();
        }
    }

    most_freq_char
}

fn main() {
    let s = String::from("hello, this is a message for the future");
    let str_section = count(s, 6);
    println!("{}", str_section);
}
