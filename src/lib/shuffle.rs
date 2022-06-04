//  Library
use rand::Rng;

//  =======
//  SHUFFLE
//  =======

/// Shuffles the given vector
pub fn shuffle<T: Clone>(vector: &Vec<T>) -> Vec<T> {
    let mut vector = vector.clone(); //  Mutable clone
    let mut shuffle: Vec<T> = Vec::new(); //  Shuffled Vector

    let mut rng = rand::thread_rng(); //  Initialize RNG

    while vector.len() > 0 {
        let index = rng.gen_range(0..vector.len());
        shuffle.push(vector[index].clone());
        vector.remove(index);
    }

    return shuffle;
}

//  -----
//  TESTS
//  -----

#[cfg(test)]
mod tests {
    use super::*;

    const FELLOWSHIP: [&str; 9] = [
        "Aragorn", "Gimli", "Legolas", "Gandalf", "Frodo", "Samwise", "Merry", "Pippin", "Boromir",
    ];

    #[test]
    fn test_shuffle_length() {
        assert_eq!(FELLOWSHIP.len(), shuffle(&FELLOWSHIP.to_vec()).len())
    }

    #[test]
    fn test_shuffle_contains() {
        let fellowship = shuffle(&FELLOWSHIP.to_vec());
        assert!(fellowship.contains(&"Gandalf"));
    }

    #[test]
    fn test_shuffle() {
        let fellowship = FELLOWSHIP.to_vec();
        let one = shuffle(&fellowship);
        let two = shuffle(&fellowship);
        assert_ne!(one, fellowship);
        assert_ne!(two, fellowship);
        assert_ne!(one, two);
    }
}
