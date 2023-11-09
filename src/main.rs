//  Library
use clap::Parser;

mod commands;
use commands::Commands;

//  ----------------------
//  Command Line Interface
//  ----------------------

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct CLI {
    #[clap(subcommand)]
    commands: Commands,
}

//  ====
//  MAIN
//  ====

fn main() {
    //  Parse Command Line Interface
    let cli = CLI::parse();

    //  Match and Execute Commands
    match &cli.commands {
        Commands::Number(command) => command.execute(),
        Commands::Roll(command) => command.execute(),
        Commands::Select(command) => command.execute(),
        Commands::Shuffle(command) => command.execute(),
        Commands::String(command) => command.execute(),
        Commands::Toss(command) => command.execute(),
    }
}
