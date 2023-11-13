//  Library
use clap::Args;
use random::strings;

// ======
// STRING
// ======

/// Generate a random string
///
/// Generate a string of random characters containing alphanumerics and special characters
#[derive(Args)]
#[clap(verbatim_doc_comment)]
pub struct Strings {
    /// Length of the string to generate
    #[clap(short, long, default_value_t = 16)]
    length: u8,

    #[clap(long, default_value = "all")]
    charset: strings::Charset,

    /// Number of times to repeat command execution
    #[clap(short, long, short_aliases=['c'], aliases=["count"], default_value_t = 1)]
    repeat: u8,

    /// String to use to separate results
    #[clap(short, long, short_aliases=['d'], aliases=["delimiter"], default_value = "\n")]
    separator: String,
}

impl Strings {
    pub fn execute(&self) {
        let mut result: Vec<String> = Vec::new(); //  Vector to store results

        //  Generate random strings and store in results
        for _ in 00..self.repeat {
            let s: String = strings::generate_random(&self.charset, self.length as usize);
            result.push(s);
        }

        //  Show results
        println!("{}", result.join(&self.separator));
    }
}
