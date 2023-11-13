//  Library
use clap::Args;
use rand::{self, Rng};

// ====
// TOSS
// ====

/// Generate a random boolean
///
/// Returns a random boolean result (default: 50-50 chance)
/// Probability to get true can be adjusted using the --weight flag (default: 0.5)
/// Using the --coin flag outputs Heads / Tails instead of true / false
#[derive(Args)]
#[clap(verbatim_doc_comment)]
pub struct Toss {
    /// Probabilistic weight to get truthy value
    #[clap(short, long, default_value_t = 0.5)]
    weight: f64,

    /// Results in Heads or Tails
    #[clap(long)]
    coin: bool,

    /// Number of times to repeat command execution
    #[clap(short, long, short_aliases=['c'], aliases=["count"], default_value_t = 1)]
    repeat: u8,

    /// String to use to separate results
    #[clap(short, long, short_aliases=['d'], aliases=["delimiter"], default_value = "\n")]
    separator: String,
}

impl Toss {
    pub fn execute(&self) {
        let mut result: Vec<String> = Vec::new(); //  Vector to store the results

        let mut rng = rand::thread_rng();

        //  Toss
        for _ in 00..self.repeat {
            let b: bool = rng.gen_bool(self.weight);

            if self.coin {
                //  If --coin, reformat results as Heads / Tails
                let coin_result = match b {
                    true => "Heads",
                    false => "Tails",
                };
                result.push(coin_result.to_string());
            } else {
                //  Return true / false as normal
                result.push(b.to_string());
            }
        }

        //  Show results
        println!("{}", result.join(&self.separator));
    }
}
