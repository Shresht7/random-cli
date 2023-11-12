//  Library
use clap::Args;
use random::die;

//  ====
//  ROLL
//  ====

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
    /// The dice to roll in {n}d{S} format
    die: Option<String>,

    /// Roll the die with advantage. Take the greatest value
    #[clap(short = 'a', long)]
    with_advantage: bool,

    /// Roll the die with disadvantage. Take the smallest value
    #[clap(short = 'd', long)]
    with_disadvantage: bool,

    /// Difficulty check. The rolls total must be higher than this value to succeed
    #[clap(short = 'c', long, default_value_t = 0)]
    difficulty_check: u8,
}

impl Roll {
    pub fn execute(&self) {
        //  Read user-input or take default die
        let die = match self.die.to_owned() {
            Some(x) => x,
            None => String::from("1d20"),
        };

        //  Roll the die and determine the total
        let mut result = die::roll(&die);
        let mut total: &u8 = &result.iter().sum::<u8>();

        //  Show results
        println!("Rolls: {:?} = {}", result, total);

        result.sort(); // Sort the results to easily determine advantage and disadvantage

        // If rolling with advantage...
        if self.with_advantage {
            total = result.last().unwrap();
            println!("With Advantage: {}", total);
        }

        // If rolling with disadvantage...
        if self.with_disadvantage {
            total = result.first().unwrap();
            println!("With Disadvantage: {}", total);
        }

        // If a difficulty check is present, print Success or Failure based on the results
        if self.difficulty_check > 0 {
            println!("Difficulty: {}", self.difficulty_check);
            if total >= &self.difficulty_check {
                println!("Success")
            } else {
                println!("Failure")
            }
        }
    }
}
