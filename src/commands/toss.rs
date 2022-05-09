use clap::Args;
use rand::{self, Rng};

// ====
// TOSS
// ====

/// Toss a coin
#[derive(Args)]
#[clap(verbatim_doc_comment)]
pub struct Toss {
    #[clap(short, long, default_value_t = 0.5)]
    weight: f64,

    #[clap(short, long)]
    coin: bool,

    #[clap(short, long, default_value_t = 1)]
    repeat: u8,
}

impl Toss {
    pub fn execute(self: &Self) {
        let mut rng = rand::thread_rng();
        for _ in 00..self.repeat {
            let result: bool = rng.gen_bool(self.weight);
    
            if self.coin {
                let result = match result {
                    true => "H",
                    false => "T",
                };
                println!("{}", result);
            } else {
                println!("{}", result);
            }
        }
    }
}
