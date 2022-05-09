use clap::Args;
use rand::Rng;

// ======
// NUMBER
// ======

/// Generate a random number
///
/// If only one number is specified, the command will generate a number between zero and that number.
/// If two numbers are specified, the command will generate a number between those two numbers.
/// If no parameters are specified, the command will generate a number between 0 and 1000
///
/// Examples:
/// random number           -   Generates a number between 0 and 1000
/// random number 10        -   Generates a number between 0 and 10
/// random number 10 15     -   Generates a number between 10 and 15
#[derive(Args)]
#[clap(verbatim_doc_comment)]
pub struct Number {
    /// First Number
    pub num1: Option<i32>,
    /// Second Number
    pub num2: Option<i32>,
}

impl Number {
    pub fn execute(self: &Self) {
        let num1 = self.num1.unwrap_or(1000);
        let result: i32 = match self.num2 {
            Some(num2) => get_random_number_between(num1, num2),
            None => get_random_number(num1),
        };
        println!("{}", result);
    }
}

//  ----------------
//  HELPER FUNCTIONS
//  ----------------

/// Get random number between zero and the provided parameter
fn get_random_number(num1: i32) -> i32 {
    return rand::thread_rng().gen_range(0..num1);
}

///  Get random number between the two provided parameters
fn get_random_number_between(num1: i32, num2: i32) -> i32 {
    let low = std::cmp::min(num1, num2);
    let max = std::cmp::max(num1, num2);
    return rand::thread_rng().gen_range(low..max);
}
