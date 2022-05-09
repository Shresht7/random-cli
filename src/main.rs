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
}

//  --------
//  Commands
//  --------

#[derive(Subcommand)]
enum CMD {
    /// Generate a random number
    ///
    /// If only one number is specified, the command will generate a number between zero and num1.
    /// If two numbers are specified, the command will generate a number between those two numbers.
    /// If no parameters are specified, the command will default to generate a number between 0 and 1000
    ///
    /// Examples:
    /// random number           -   Generates a number between 0 and 1000
    /// random number 10        -   Generates a number between 0 and 10
    /// random number 10 15     -   Generates a number between 10 and 15
    Number {
        /// First Number
        num1: Option<i32>,
        /// Second Number
        num2: Option<i32>,
    },

    /// Roll die
    ///
    /// Roll die accepts input in the {n}d{S} format; where {n} is the number of dice thrown
    /// and {S} is the number of sides each die has. (example: 3d8 - Three 8-sided die)
    /// defaults to 1d20
    ///
    /// Examples:
    /// random die          -   Rolls one twenty-sided die
    /// random die 1d6      -   Rolls one six-sided die
    /// random die 3d12     -   Rolls three twelve-sided die
    Roll { die: String },
    Select {
        entries: Vec<String>,

        #[clap(short, long, default_value_t = 1)]
        repeat: u8,
    },
}

//  ----
//  MAIN
//  ----

fn main() {
    //  Parse Command Line Interface
    let cli = CLI::parse();

    //  Match Sub-Commands
    match &cli.commands {
        CMD::Number { num1, num2 } => {
            commands::number(num1.to_owned(), num2.to_owned());
        }
        CMD::Roll { die } => {
            commands::roll(die);
        }
        CMD::Select { entries, repeat } => {
            commands::select(entries, repeat.to_owned());
        }
    }
}
