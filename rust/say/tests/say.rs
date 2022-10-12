// Note: No tests created using 'and' with numbers.
// Apparently Most American English does not use the 'and' with numbers,
// where it is common in British English to use the 'and'.

mod tests {

    mod split_with_commas_tests {
        use say::group_by_thousands;

        #[test]
        fn one_million() {
            assert_eq!(group_by_thousands(1_234_567), vec!["1", "234", "567"])
        }
    }

    mod encode_tests {
        #[test]
        fn test_zero() { assert_eq!(say::encode(0), String::from("zero")); }

        #[test]
        fn test_one() {
            assert_eq!(say::encode(1), String::from("one"));
        }

        #[test]
        fn test_fourteen() {
            assert_eq!(say::encode(14), String::from("fourteen"));
        }

        #[test]
        fn test_twenty() {
            assert_eq!(say::encode(20), String::from("twenty"));
        }

        #[test]
        fn test_twenty_two() {
            assert_eq!(say::encode(22), String::from("twenty-two"));
        }

        #[test]
        fn test_one_hundred() {
            assert_eq!(say::encode(100), String::from("one hundred"));
        }

        // note, using American style with no and
        #[test]
        fn test_one_hundred_twenty() {
            assert_eq!(say::encode(120), String::from("one hundred twenty"));
        }

        #[test]
        fn test_one_hundred_twenty_three() {
            assert_eq!(say::encode(123), String::from("one hundred twenty-three"));
        }

        #[test]
        fn test_one_thousand() {
            assert_eq!(say::encode(1000), String::from("one thousand"));
        }

        #[test]
        fn test_one_thousand_two_hundred_thirty_four() {
            assert_eq!(
                say::encode(1234),
                String::from("one thousand two hundred thirty-four")
            );
        }

        // note, using American style with no and
        #[test]
        fn test_eight_hundred_and_ten_thousand() {
            assert_eq!(
                say::encode(810_000),
                String::from("eight hundred ten thousand")
            );
        }

        #[test]
        fn test_one_million() {
            assert_eq!(say::encode(1_000_000), String::from("one million"));
        }

        // note, using American style with no and
        #[test]
        fn test_one_million_two() {
            assert_eq!(say::encode(1_000_002), String::from("one million two"));
        }

        #[test]
        fn test_1002345() {
            assert_eq!(
                say::encode(1_002_345),
                String::from("one million two thousand three hundred forty-five")
            );
        }

        #[test]
        fn test_one_billion() {
            assert_eq!(say::encode(1_000_000_000), String::from("one billion"));
        }

        #[test]
        fn test_987654321123() {
            assert_eq!(
                say::encode(987_654_321_123),
                String::from(
                    "nine hundred eighty-seven billion \
             six hundred fifty-four million \
             three hundred twenty-one thousand \
             one hundred twenty-three"
                )
            );
        }

        /*
          These tests are only if you implemented full parsing for u64 type.
        */
        #[test]
        fn test_max_i64() {
            assert_eq!(
                say::encode(9_223_372_036_854_775_807),
                String::from(
                    "nine quintillion two hundred twenty-three \
             quadrillion three hundred seventy-two trillion \
             thirty-six billion eight hundred fifty-four million \
             seven hundred seventy-five thousand eight hundred seven"
                )
            );
        }

        #[test]
        fn test_max_u64() {
            assert_eq!(
                say::encode(18_446_744_073_709_551_615),
                String::from(
                    "eighteen quintillion four hundred forty-six \
             quadrillion seven hundred forty-four trillion \
             seventy-three billion seven hundred nine million \
             five hundred fifty-one thousand six hundred fifteen"
                )
            );
        }
    }
}