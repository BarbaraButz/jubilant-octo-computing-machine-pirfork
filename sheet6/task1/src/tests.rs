    #[test]
    fn test_clamp() {
        use super::clamp;
        assert_eq!(clamp(2, 1, 3), Some(2));
        assert_eq!(clamp("4711", "z", "a"), None);
        assert_eq!(clamp(0.5, 1.0, 39.33), Some(1.0));
        assert_eq!(clamp(vec![1, 4], vec![1, -10], vec![1, 0]), Some(vec![1, 0]));
        assert_eq!(clamp('s', 's', 's'), Some('s'));

        use std::f64::NAN;
        assert_eq!(clamp(NAN, NAN, NAN), None);
        assert_eq!(clamp(NAN, 1.0, 2.0), None);
        assert_eq!(clamp(0.0, NAN, 2.0), None);
        assert_eq!(clamp(1.0, 1.0, NAN), None);
    }

    #[test]
    fn test_sum_and_prod() {
        use super::sum_and_product;
        assert_eq!(sum_and_product(1, 2), (3, 2));
    }

    #[test]
    fn test_into_opt() {
        use super::IntoOptExt;
        assert_eq!(false.into_option(3), None);
        assert_eq!(true.into_option("Susi"), Some("Susi"));
    }
