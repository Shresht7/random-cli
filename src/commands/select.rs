use std::io::Read;

use rand::Rng;

pub fn select(entries: &Vec<String>, repeat: u8) {
    //  Clone a mutable shadow of entries
    let mut entries = entries.clone();

    //  Read input from stdin
    let mut lines = String::new();
    std::io::stdin().read_to_string(&mut lines).unwrap();
    for line in lines.lines() {
        entries.push(String::from(line));
    }

    for _ in 00..repeat {
        //  Select one entry at random
        let selection = rand::thread_rng().gen_range(0..entries.len());
        //  Show results
        println!("{}", entries[selection]);
    }
}
