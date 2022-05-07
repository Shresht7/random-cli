use clap::{Parser, Subcommand};
use rand::{self, prelude::ThreadRng, Rng};

//  ----------------------
//  Command Line Interface
//  ----------------------

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct CLI {
    #[clap(subcommand)]
    commands: Commands,
}

//  ------------
//  Sub-Commands
//  ------------

#[derive(Subcommand)]
enum Commands {
    Select { entries: Vec<String> },
}

fn select(entries: &Vec<String>, rng: &mut ThreadRng) {
    let selection = rng.gen_range(0..entries.len());
    println!("{}: {}", selection, entries[selection]);
}

//  ----
//  MAIN
//  ----

fn main() {
    //  Initialize random number generator
    let mut rng = rand::thread_rng();

    //  Parse Command Line Interface
    let cli = CLI::parse();

    //  Match Sub-Commands
    match &cli.commands {
        Commands::Select { entries } => {
            select(entries, &mut rng);
        }
    }
}
