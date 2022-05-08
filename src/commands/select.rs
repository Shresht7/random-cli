use rand::{Rng};

pub fn select(entries: &Vec<String>) {
    let selection = rand::thread_rng().gen_range(0..entries.len());
    println!("{}: {}", selection, entries[selection]);
}
