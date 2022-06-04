//  Library
use crate::lib::{helpers, select};
use clap::Args;

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
    pub fn execute(&self) {
        //  Clone a mutable shadow of entries
        let mut entries = self.entries.clone();
        //  Read input redirected from standard input
        helpers::read_stdin_into(&mut entries);

        //  Select one or multiple entries based on the repeat flag
        let result = match self.repeat > 1 {
            true => select::select_multiple(&entries, self.repeat as u32)
                .iter()
                .map(|(v, _)| v.to_owned())
                .collect::<Vec<String>>()
                .join("\n"),

            false => select::select(&entries).0,
        };

        //  Show the result
        println!("{}", result);
    }
}
