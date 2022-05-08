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
    Number { num1: i32, num2: Option<i32> },
    Select { entries: Vec<String> },
}

fn number(num1: i32, num2: Option<i32>, rng: &mut ThreadRng) {
    let result: i32 = match num2 {
        Some(num2) => {
            let low = std::cmp::min(num1, num2);
            let max = std::cmp::max(num1, num2);
            rng.gen_range(low..max)
        }
        None => rng.gen_range(0..num1.to_owned()),
    };
    println!("{}", result);
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
        Commands::Number { num1, num2 } => {
            number(num1.to_owned(), num2.to_owned(), &mut rng);
        }
        Commands::Select { entries } => {
            select(entries, &mut rng);
        }
    }
}
