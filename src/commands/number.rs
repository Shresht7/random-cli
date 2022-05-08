use rand::{prelude::ThreadRng, Rng};

pub fn number(num1: i32, num2: Option<i32>, rng: &mut ThreadRng) {
    let result: i32 = match num2 {
        Some(num2) => {
            let low = std::cmp::min(num1, num2);
            let max = std::cmp::max(num1, num2);
            rng.gen_range(low..max)
        }
        None => rng.gen_range(0..num1.to_owned()),
    };
    println!("{}", result);
}
