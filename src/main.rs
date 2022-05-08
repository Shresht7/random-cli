use clap::{Parser, Subcommand};
use rand;

mod commands;

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
    Number { num1: i32, num2: Option<i32> },
    Select { entries: Vec<String> },
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
        Commands::Number { num1, num2 } => {
            commands::number(num1.to_owned(), num2.to_owned(), &mut rng);
        }
        Commands::Select { entries } => {
            commands::select(entries, &mut rng);
        }
    }
}
