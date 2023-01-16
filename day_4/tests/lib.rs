mod process_raw_line_tests {
    use day_4::process_raw_line;

    #[test]
    fn test_process_raw_line() {
        let lines = vec![
            ("2-4,6-8", vec![vec![2, 4], vec![6, 8]]),
            ("2-3,4-5", vec![vec![2, 3], vec![4, 5]]),
            ("5-7,7-9", vec![vec![5, 7], vec![7, 9]]),
            ("2-8,3-7", vec![vec![2, 8], vec![3, 7]]),
            ("6-6,4-6", vec![vec![6, 6], vec![4, 6]]),
            ("2-6,4-8", vec![vec![2, 6], vec![4, 8]])
        ];

        for (input, output) in lines {
            assert_eq!(process_raw_line(&input), output);
        }
    }
}
mod is_contained_tests {
    use day_4::is_contained;

    #[test]
    fn test_is_contained_true() {
        assert!(is_contained(&1, &10, &2, &9));
        assert!(is_contained(&-5, &5, &-4, &4));
        assert!(is_contained(&-10, &10, &-9, &9));
    }
    #[test]
    fn test_is_contained_false() {
        assert!(!is_contained(&1, &10, &11, &20));
        assert!(!is_contained(&-5, &5, &6, &10));
        assert!(!is_contained(&-10, &10, &-15, &-5));
    }
}
mod has_overlap_tests {
    use day_4::has_overlap;

    #[test]
    fn test_has_overlap_true() {
        assert!(has_overlap(&1, &10, &5, &15));
        assert!(has_overlap(&-5, &5, &-2, &2));
        assert!(has_overlap(&-10, &10, &-5, &5));
        assert!(has_overlap(&1, &10, &9, &20));
        assert!(has_overlap(&-5, &5, &-10, &-2));
    }
    #[test]
    fn test_has_overlap_false() {
        assert!(!has_overlap(&1, &10, &11, &20));
        assert!(!has_overlap(&-5, &5, &6, &10));
        assert!(!has_overlap(&-10, &10, &11, &20));
    }
}