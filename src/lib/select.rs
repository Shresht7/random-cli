//  Library
use rand::RngExt;

//  ======
//  SELECT
//  ======

/// Select a single entry from a vector
pub fn select<T: Clone>(vector: &Vec<T>) -> (T, usize) {
    //  Select one entry at random
    let index = rand::rng().random_range(00..vector.len());
    return (vector[index].clone(), index);
}

/// Select multiple entries from a vector
pub fn select_multiple<T: Clone>(vector: &Vec<T>, count: u32) -> Vec<(T, usize)> {
    let mut result: Vec<(T, usize)> = Vec::new();
    for _ in 00..count {
        let selection = select::<T>(vector);
        result.push(selection);
    }
    return result;
}
