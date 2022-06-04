//  Library
use rand::Rng;

// ====
// ROLL
// ====

/// Rolls the dice and returns the results as a Vector
pub fn roll(die: &str) -> Vec<u32> {
    let (number_of_die, range_of_die) = get_die_count_and_range(&die);

    //  Calculate and return result
    let mut result = Vec::new();
    for _ in 0..number_of_die {
        let n = rand::thread_rng().gen_range(1..=range_of_die);
        result.push(n);
    }
    return result;
}

// ----------------
// HELPER FUNCTIONS
// ----------------

/// Determine the number of dice and their type (e.g. 3d8 -> (3, 8) - Three 8-sided dice)
fn get_die_count_and_range(die: &str) -> (u32, u32) {
    //  Split string and retrieve die count ...
    let mut die_split = die.split("d");
    let number_of_die: u32 = match die_split.next() {
        Some(x) => x.parse::<u32>().expect("Failed to parse as u32"),
        None => 1,
    };

    //  ... and range
    let range_of_die: u32 = die_split
        .next()
        .expect("failed to retrieve range")
        .parse::<u32>()
        .expect("Failed to parse as u32");

    return (number_of_die, range_of_die);
}

// -----
// TESTS
// -----

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_die_count_and_range() {
        assert_eq!(get_die_count_and_range("1d20"), (1, 20));
        assert_eq!(get_die_count_and_range("3d8"), (3, 8));
        assert_eq!(get_die_count_and_range("4d6"), (4, 6));
    }
}
