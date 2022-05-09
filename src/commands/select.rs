use std::io::Read;

use atty::Stream;
use clap::Args;
use rand::Rng;

//  ======
//  SELECT
//  ======

/// Select one entry from the given inputs
///
/// Inputs can also be piped from stdin
///
/// Examples:
/// random select pizza burger pasta    -   Selects either pizza, burger or pasta
/// gh repo list | random select        -   Select one entry piped through stdin
#[derive(Args)]
#[clap(verbatim_doc_comment)]
pub struct Select {
    /// List of entries to choose from
    entries: Vec<String>,

    /// The number of times to repeat the execution
    #[clap(short, long, default_value_t = 1)]
    repeat: u8,
}

impl Select {
    pub fn execute(self: &Self) {
        //  Clone a mutable shadow of entries
        let mut entries = self.entries.clone();

        //  Read input from stdin
        let mut lines = String::new();
        if !atty::is(Stream::Stdin) {
            //  Read stdin only if redirected using a pipe
            std::io::stdin().read_to_string(&mut lines).unwrap();
        }
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
