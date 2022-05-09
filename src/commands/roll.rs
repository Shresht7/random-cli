use clap::Args;
use rand::Rng;

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
#[derive(Args)]
#[clap(verbatim_doc_comment)]
pub struct Roll {
    pub die: String,
}

impl Roll {
    pub fn execute(self: &Self) {
        //  Initialize the rng
        let mut rng = rand::thread_rng();

        //  Split string and retrieve die number and range
        let mut die_split = self.die.split("d");
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
}
