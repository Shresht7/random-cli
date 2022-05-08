use clap::{Parser, Subcommand};

mod commands;

//  ----------------------
//  Command Line Interface
//  ----------------------

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct CLI {
    #[clap(subcommand)]
    commands: CMD,

    #[clap(short, long, default_value_t = 1)]
    repeat: u8,
}

//  ------------
//  Sub-Commands
//  ------------

#[derive(Subcommand)]
enum CMD {
    Number {
        num1: Option<i32>,
        num2: Option<i32>,
    },
    Roll {
        die: String,
    },
    Select {
        entries: Vec<String>,
    },
}

//  ----
//  MAIN
//  ----

fn main() {
    //  Parse Command Line Interface
    let cli = CLI::parse();

    for _ in 0..cli.repeat {
        //  Match Sub-Commands
        match &cli.commands {
            CMD::Number { num1, num2 } => {
                commands::number(num1.to_owned(), num2.to_owned());
            }
            CMD::Roll { die } => {
                commands::roll(die);
            }
            CMD::Select { entries } => {
                commands::select(entries);
            }
        }
    }
}
