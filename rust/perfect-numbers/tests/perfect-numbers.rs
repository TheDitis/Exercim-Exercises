
macro_rules! tests {
    ($property_test_func:ident {
        $( $(#[$attr:meta])* $test_name:ident( $( $param:expr ),* ); )+
    }) => {
        $(
            $(#[$attr])*
            #[test]
            fn $test_name() {
                $property_test_func($( $param ),* )
            }
        )+
    }
}

mod tests {
    mod aliquot_sum_tests {
        use perfect_numbers::aliquot_sum;

        #[test]
        fn six_is_six() {
            assert_eq!(aliquot_sum(6), 6)
        }

        #[test]
        fn twentyeight_is_twentyeight() {
            assert_eq!(aliquot_sum(28), 28)
        }

        #[test]
        fn twelve_is_sixteen() {
            assert_eq!(aliquot_sum(12), 16)
        }

        #[test]
        fn twentyfour_is_thirtysix() {
            assert_eq!(aliquot_sum(24), 36)
        }

        #[test]
        fn eight_is_seven() {
            assert_eq!(aliquot_sum(8), 7)
        }
    }

    mod classify_tests {
        use perfect_numbers::{classify, Classification};

        fn test_classification(num: u64, result: Classification) {
            assert_eq!(classify(num), Some(result));
        }

        #[test]
        fn basic() {
            assert_eq!(classify(0), None);
        }

        tests! {
            test_classification {
                test_1(1, Classification::Deficient);
                test_2(2, Classification::Deficient);
                test_4(4, Classification::Deficient);
                test_6(6, Classification::Perfect);
                test_12(12, Classification::Abundant);
                test_28(28, Classification::Perfect);
                test_30(30, Classification::Abundant);
                test_32(32, Classification::Deficient);
                test_33550335(33_550_335, Classification::Abundant);
                test_33550336(33_550_336, Classification::Perfect);
                test_33550337(33_550_337, Classification::Deficient);
            }
        }
    }
}