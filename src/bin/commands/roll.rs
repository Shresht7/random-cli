//  Library
use clap::Args;
use random::die;

//  ====
//  ROLL
//  ====

/// Roll die
///
/// Roll die accepts input in the {n}d{S}+{X} format; where {n} is the number of dice thrown
/// and {S} is the number of sides each die has and {X} is the number to add to the result.
/// (example: 3d8 - Three 8-sided die, 2d6+4 - Two 6-sided dice and add 4 to the result)
/// defaults to 1d20
///
/// Examples:
/// random roll             -   Rolls one twenty-sided die
/// random roll 1d6         -   Rolls one six-sided die
/// random roll 3d12        -   Rolls three twelve-sided dice
/// random roll 1d8+5       -   Rolls a eight-sided die and adds 5 to the result
/// random roll 1d8+2d6+3   -   Rolls an eight sided die, two six-sided dice and adds 3 to the result
#[derive(Args)]
#[clap(verbatim_doc_comment)]
pub struct Roll {
    /// The dice to roll
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

        // Extract the terms
        let terms = die.split("+").map(|x| x.trim()).collect::<Vec<&str>>();

        // Determine rolls, additions and the total
        let mut total: u8 = 0;
        for term in &terms {
            // If this is a dice term ...
            if term.to_lowercase().contains("d") {
                // Roll the dice
                let mut roll = die::roll(&term);

                if &terms[0] != term {
                    print!("\n");
                } // Print newline for everything except the first
                print!("Roll {}: {:?} ", term, roll); // Print the Roll

                // If the roll was with advantage ...
                if self.with_advantage {
                    // Determine the max roll
                    roll.sort();
                    let roll = roll.last().unwrap_or(&0);
                    print!("-- Advantage --> {} ", roll);
                    total += roll;
                // ... else if the roll was with disadvantage ...
                } else if self.with_disadvantage {
                    // Determine the min roll
                    roll.sort();
                    let roll = roll.first().unwrap_or(&0);
                    print!("-- Disadvantage --> {} ", roll);
                    total += roll;
                // ... otherwise, use the rolls as is
                } else {
                    total += &roll.iter().sum::<u8>();
                }
            // ... else if this is an addition
            } else {
                print!("+ {}", term);
                total += term.parse::<u8>().unwrap_or_default();
            }
        }

        // Print the total
        print!("\nTotal: {} ", total);

        // If the difficulty check option was passed
        if self.difficulty_check > 0 {
            print!("(Difficulty: {}) ", self.difficulty_check);
            // If the total beats the difficulty check, print success ...
            if total >= self.difficulty_check {
                println!("Success")
            // ...otherwise, print failure.
            } else {
                println!("Failure")
            }
        }
    }
}
