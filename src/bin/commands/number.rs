//  Library
use clap::Args;
use random::numbers;

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
    num1: Option<i32>,

    /// Second Number
    num2: Option<i32>,

    /// The number of times to repeat the command
    #[clap(short, long, short_aliases=['c'], aliases=["count"], default_value_t = 1)]
    repeat: u8,

    /// String to use to separate results
    #[clap(short, long, short_aliases=['d'], aliases=["delimiter"], default_value = "\n")]
    separator: String,
}

impl Number {
    pub fn execute(&self) {
        let mut result: Vec<String> = Vec::new(); //  Vector to store results

        //  Generate the random numbers and store the results
        let num1 = self.num1.unwrap_or(1000);
        for _ in 0..self.repeat {
            let n: i32 = match self.num2 {
                Some(num2) => numbers::get_random_number_between(num1, num2),
                None => numbers::get_random_number(num1),
            };
            result.push(n.to_string());
        }

        //  Show the results
        println!("{}", result.join(&self.separator));
    }
}
