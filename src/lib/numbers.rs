//  Library
use rand::Rng;

/// Get random number between zero and the provided parameter
pub fn get_random_number(num1: i32) -> i32 {
    return rand::thread_rng().gen_range(0..num1);
}

///  Get random number between the two provided parameters
pub fn get_random_number_between(num1: i32, num2: i32) -> i32 {
    let low = std::cmp::min(num1, num2);
    let max = std::cmp::max(num1, num2);
    return rand::thread_rng().gen_range(low..max);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_random_number() {
        let result = get_random_number(10);
        if result < 0 || result > 10 {
            panic!("Result not in range");
        }
    }

    #[test]
    fn test_get_random_number_between() {
        let min = 5;
        let max = 12;
        let result = get_random_number_between(min, max);
        if result < min || result > max {
            panic!("Result not in range");
        }
    }
}
