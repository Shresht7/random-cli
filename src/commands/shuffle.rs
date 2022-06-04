//  Library
use clap::Args;
use rand::Rng;

use crate::lib::helpers;

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
        //  Read standard input
        helpers::read_stdin_into(&mut entries);

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
