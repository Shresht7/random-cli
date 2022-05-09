use clap::Args;
use rand::{self, Rng};

// ======
// STRING
// ======

const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                         abcdefghijklmnopqrstuvwxyz\
                         0123456789)(*&^%$#@!~";

/// Generate a random string
#[derive(Args)]
#[clap(verbatim_doc_comment)]
pub struct Strings {
    #[clap(short, long, default_value_t = 16)]
    length: u16,
}

impl Strings {
    pub fn execute(self: &Self) {
        let mut rng = rand::thread_rng();
        let result: String = (0..self.length)
            .map(|_| {
                let idx = rng.gen_range(0..CHARSET.len());
                CHARSET[idx] as char
            })
            .collect();
        println!("{}", result);
    }
}
