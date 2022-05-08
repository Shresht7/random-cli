use rand::{prelude::ThreadRng, Rng};

pub fn number(num1: i32, num2: Option<i32>, rng: &mut ThreadRng) {
    let result: i32 = match num2 {
        Some(num2) => get_random_number_between(num1, num2, rng),
        None => get_random_number(num1, rng),
    };
    println!("{}", result);
}

fn get_random_number_between(num1: i32, num2: i32, rng: &mut ThreadRng) -> i32 {
    let low = std::cmp::min(num1, num2);
    let max = std::cmp::max(num1, num2);
    return rng.gen_range(low..max);
}

fn get_random_number(num1: i32, rng: &mut ThreadRng) -> i32 {
    return rng.gen_range(0..num1);
}
