#[cfg(test)]
mod is_prime_tests {
    use prime_factors::is_prime;

    #[test]
    fn two_is_prime() {
        assert_eq!(is_prime(2), true);
    }

    #[test]
    fn four_is_not_prime() {
        assert_eq!(is_prime(4), false);
    }

    #[test]
    fn five_is_prime() {
        assert_eq!(is_prime(5), true);
    }

    #[test]
    fn six_is_not_prime() {
        assert_eq!(is_prime(6), false);
    }

    #[test]
    fn nineteen_is_prime() {
        assert_eq!(is_prime(19), true);
    }

    #[test]
    fn twenty_one_is_not_prime() {
        assert_eq!(is_prime(21), false);
    }
}


#[cfg(test)]
mod factors_tests {
    use prime_factors::factors;

    #[test]
    fn test_no_factors() {
        assert_eq!(factors(1), vec![]);
    }

    #[test]
    fn test_prime_number() {
        assert_eq!(factors(2), vec![2]);
    }

    #[test]
    fn test_square_of_a_prime() {
        assert_eq!(factors(9), vec![3, 3]);
    }

    #[test]
    fn test_cube_of_a_prime() {
        assert_eq!(factors(8), vec![2, 2, 2]);
    }

    #[test]
    fn test_product_of_primes_and_non_primes() {
        assert_eq!(factors(12), vec![2, 2, 3]);
    }

    #[test]
    fn test_product_of_primes() {
        assert_eq!(factors(901_255), vec![5, 17, 23, 461]);
    }

    #[test]
    fn test_factors_include_large_prime() {
        assert_eq!(factors(93_819_012_551), vec![11, 9539, 894_119]);
    }
}