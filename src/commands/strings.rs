use clap::Args;
use rand::{self, Rng};

// ======
// STRING
// ======

const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                         abcdefghijklmnopqrstuvwxyz\
                         0123456789)(*&^%$#@!~";

/// Generate a random string
///
/// Generate a string of random characters containing alphanumerics and special characters
#[derive(Args)]
#[clap(verbatim_doc_comment)]
pub struct Strings {
    /// Length of the string to generate
    #[clap(short, long, default_value_t = 16)]
    length: u16,

    /// Number of times to repeat command execution
    #[clap(short, long, default_value_t = 1)]
    repeat: u8,
}

impl Strings {
    pub fn execute(self: &Self) {
        let mut result: Vec<String> = Vec::new();

        let mut rng = rand::thread_rng();

        for _ in 00..self.repeat {
            let s: String = (0..self.length)
                .map(|_| {
                    let idx = rng.gen_range(0..CHARSET.len());
                    CHARSET[idx] as char
                })
                .collect();

            result.push(s);
        }

        println!("{}", result.join("\n"));
    }
}
