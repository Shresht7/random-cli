use rand::{prelude::ThreadRng, Rng};

pub fn select(entries: &Vec<String>, rng: &mut ThreadRng) {
    let selection = rng.gen_range(0..entries.len());
    println!("{}: {}", selection, entries[selection]);
}
