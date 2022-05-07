use rand::{self, Rng};

fn main() {
    //  Collect the parsed arguments in a vector
    let args: Vec<String> = std::env::args().collect::<Vec<String>>()[1..].to_vec();

    println!("{:#?}", args);

    let mut rng = rand::thread_rng();
    let selection = rng.gen_range(0..args.len());

    println!("{}: {}", selection, args[selection]);
}
