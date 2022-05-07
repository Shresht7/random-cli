use clap::Parser;
use rand::{self, Rng};

//  ----------------------
//  Command Line Interface
//  ----------------------

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct CLI {
    #[clap(long)]
    name: String
}


//  ----
//  MAIN
//  ----

fn main() {
    let cli =  CLI::parse();
    println!("{}", cli.name);

    //  Collect the parsed arguments in a vector
    let args: Vec<String> = std::env::args().collect::<Vec<String>>()[1..].to_vec();

    println!("{:#?}", args);

    let mut rng = rand::thread_rng();
    let selection = rng.gen_range(0..args.len());

    println!("{}: {}", selection, args[selection]);
}
