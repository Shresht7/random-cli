use rand::{Rng};

pub fn roll(die: &str) {
    //  Initialize the rng
    let mut rng = rand::thread_rng();

    //  Split string and retrieve die number and range
    let mut die_split = die.split("d");
    let number_of_die: u32 = match die_split.next() {
        Some(x) => x.parse::<u32>().expect("Failed to parse as u32"),
        None => 1,
    };

    let range_of_die: u32 = die_split
        .next()
        .expect("failed to retrieve range")
        .parse::<u32>()
        .expect("Failed to parse as u32");

    //  Calculate and return result
    let mut result = 0;
    let mut i = 0;
    while i < number_of_die {
        result = result + rng.gen_range(1..=range_of_die);
        i += 1;
    }
    println!("{}", result);
}
