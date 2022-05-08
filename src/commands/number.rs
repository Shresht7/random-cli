use rand::Rng;

/// Generates and prints a random number between zero and the given input
/// If two numbers are provided, it will generate a random number between those numbers
pub fn number(num1: i32, num2: Option<i32>) {
    let result: i32 = match num2 {
        Some(num2) => get_random_number_between(num1, num2),
        None => get_random_number(num1),
    };
    println!("{}", result);
}

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
