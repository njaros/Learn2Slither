
pub fn pow(a: usize, b: usize) -> usize {
    match b {
        0 => 1,
        _ => a * pow(a, b - 1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pow() {
        assert_eq!(pow(0, 0), 1);
        assert_eq!(pow(0, 1), 0);
        assert_eq!(pow(0, 2), 0);
        assert_eq!(pow(1, 0), 1);
        assert_eq!(pow(1, 1), 1);
        assert_eq!(pow(1, 110), 1);
        assert_eq!(pow(2, 2), 4);
        assert_eq!(pow(3, 3), 27);
        assert_eq!(pow(1000, 1), 1000);
        assert_eq!(pow(1000, 2), 1000000);
        assert_eq!(pow(1000, 0), 1);
    }
}
