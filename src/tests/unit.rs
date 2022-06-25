#[allow(dead_code)]

fn add(num_one: u8, num_two: u8) -> u8 {
    num_one + num_two
}

#[cfg(test)]
mod tests {
    // allow access to outer fn
    use super::*;

    #[test]
    fn sum() {
        assert_eq!(2, add(1, 1));
        assert_ne!(2, add(2, 2));
    }

    #[test]
    fn not_sum() {
        assert_ne!(2, add(2, 2));
    }
}
