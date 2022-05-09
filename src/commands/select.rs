use std::io::Read;

use clap::Args;
use rand::Rng;

/// Select one entry from the given inputs
#[derive(Args)]
pub struct Select {
    entries: Vec<String>,

    #[clap(short, long, default_value_t = 1)]
    repeat: u8,
}

impl Select {
    pub fn execute(self: &Self) {
        //  Clone a mutable shadow of entries
        let mut entries = self.entries.clone();

        //  Read input from stdin
        let mut lines = String::new();
        std::io::stdin().read_to_string(&mut lines).unwrap();
        for line in lines.lines() {
            entries.push(String::from(line));
        }

        for _ in 00..self.repeat {
            //  Select one entry at random
            let selection = rand::thread_rng().gen_range(0..entries.len());
            //  Show results
            println!("{}", entries[selection]);
        }
    }
}
