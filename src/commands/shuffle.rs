use std::io::Read;

use atty::Stream;
use clap::Args;
use rand::Rng;

//  =======
//  SHUFFLE
//  =======

/// Shuffle the given list
///
/// Inputs can also be piped from stdin
///
/// Examples:
/// random shuffle pizza burger pasta    -   Shuffles pizza, burger or pasta
/// gh repo list | random shuffle        -   Shuffles entries piped through stdin
#[derive(Args)]
#[clap(verbatim_doc_comment)]
pub struct Shuffle {
    /// List of entries to choose from
    entries: Vec<String>,
}

impl Shuffle {
    pub fn execute(self: &Self) {
        //  Initialize rng
        let mut rng = rand::thread_rng();

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
        
        //  Shuffle entries
        let mut shuffle: Vec<String> = Vec::new();
        while entries.len() > 0 {
            let idx = rng.gen_range(0..entries.len());
            shuffle.push(entries[idx].clone());
            entries.remove(idx);
        }

        //  Show results
        println!("{}", shuffle.join(" "));
    }
}
