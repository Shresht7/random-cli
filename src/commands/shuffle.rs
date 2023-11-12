//  Library
use clap::Args;
use random::{helpers, shuffle};

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

    /// String to use to separate results
    #[clap(short, long, default_value = " ")]
    separator: String,
}

impl Shuffle {
    pub fn execute(&self) {
        //  Clone a mutable shadow of entries
        let mut entries = self.entries.clone();
        //  Read input redirected to standard input
        helpers::read_stdin_into(&mut entries);

        //  Shuffle entries
        let shuffle = shuffle::shuffle(&entries);

        //  Show results
        println!("{}", shuffle.join(&self.separator));
    }
}
