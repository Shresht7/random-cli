//  Library
use rand::distributions::Alphanumeric;
use rand::Rng;

pub const LOWERCASE_ALPHABETS: &[u8] = b"abcdefghijklmnopqrstuvwxyz";
pub const UPPERCASE_ALPHABETS: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ";
pub const ALPHABETS: &[u8] = b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
pub const NUMBERS: &[u8] = b"1234567890";
pub const ALPHANUMERIC: &[u8] = b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ1234567890";
pub const SPECIAL: &[u8] = b")(*&^%$#@!~";
pub const ALL: &[u8] = b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ1234567890)(*&^%$#@!~";

pub enum Charset {
    LowercaseAlphabets,
    UppercaseAlphabets,
    Alphabets,
    Numbers,
    Alphanumeric,
    Special,
    All,
    Custom(&'static [u8]),
}

impl Charset {
    fn get(&self) -> &'static [u8] {
        match self {
            Charset::LowercaseAlphabets => LOWERCASE_ALPHABETS,
            Charset::UppercaseAlphabets => UPPERCASE_ALPHABETS,
            Charset::Alphabets => ALPHABETS,
            Charset::Numbers => NUMBERS,
            Charset::Alphanumeric => ALPHANUMERIC,
            Charset::Special => SPECIAL,
            Charset::All => ALL,
            Charset::Custom(charset) => charset,
        }
    }
}

/// Generate a random string of given length
pub fn generate_random(charset: &Charset, length: usize) -> String {
    let mut rng = rand::thread_rng();
    let charset = charset.get();
    (00..length)
        .map(|_| {
            let index = rng.gen_range(00..charset.len());
            charset[index] as char
        })
        .collect()
}

/// Generate a random alphanumeric string of given length
pub fn generate_alphanumeric(length: usize) -> String {
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(length)
        .map(char::from)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generate_lowercase_alphabets() {
        let res = generate_random(&Charset::LowercaseAlphabets, 10);
        let lower_res = res.to_ascii_lowercase();
        assert!(
            res.chars().all(|x| x.is_lowercase()),
            "expected: {}\nactual: {}",
            lower_res,
            res
        );
    }

    #[test]
    fn generate_uppercase_alphabets() {
        let res = generate_random(&Charset::UppercaseAlphabets, 10);
        let upper_res = res.to_ascii_uppercase();
        assert!(
            res.chars().all(|x| x.is_uppercase()),
            "expected: {}\nactual: {}",
            upper_res,
            res
        );
    }

    #[test]
    fn generate_alphabets() {
        let res = generate_random(&Charset::Alphabets, 10);
        assert!(res.chars().all(|x| x.is_alphabetic()), "{}", res);
    }

    #[test]
    fn generate_numbers() {
        let res = generate_random(&Charset::Numbers, 10);
        assert!(res.chars().all(|x| x.is_numeric()), "{}", res);
    }

    #[test]
    fn generate_alphanumeric() {
        let res = generate_random(&Charset::Alphanumeric, 10);
        assert!(res.chars().all(|x| x.is_alphanumeric()), "{}", res);
    }

    #[test]
    fn generate_special() {
        let res = generate_random(&Charset::Special, 10);
        assert!(res.chars().all(|x| x.is_ascii_punctuation()), "{}", res);
    }

    #[test]
    fn generate_using_custom_charset() {
        let length = 5;
        let res = generate_random(&Charset::Custom(b"111"), length);
        assert_eq!(res, "1".repeat(length));
    }
}
