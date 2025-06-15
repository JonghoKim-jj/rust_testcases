#[cfg(test)]
extern crate rstest;

fn main() {
    // do nothing
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    // Tests not using any crates.
    #[test]
    fn test_plus_operation_without_any_dependencies1() {
        assert_eq!(1 + 2, 3)
    }

    #[test]
    fn test_plus_operation_without_any_dependencies2() {
        assert_eq!(2 + 3, 5)
    }

    #[test]
    fn test_plus_operation_without_any_dependencies3() {
        assert_eq!(3 + 4, 7)
    }

    #[test]
    #[should_panic]
    fn test_plus_operation_without_any_dependencies4() {
        assert_eq!(1 + 2, 4)
    }

    #[test]
    #[should_panic]
    fn test_plus_operation_without_any_dependencies5() {
        assert_eq!(2 + 3, 6)
    }

    // Tests using *rstest* crate.
    #[rstest]
    #[case(1, 2, 3)]
    #[case::any_description_for_your_test_case(2, 3, 5)]
    #[case::no_panic(3, 4, 7)]
    #[should_panic]
    #[case::panic(1, 2, 4)]
    #[should_panic]
    #[case(2, 3, 6)]
    fn test_plus_operation_with_rstest(#[case] a: u32, #[case] b: u32, #[case] c: u32) {
        assert_eq!(a + b, c)
    }
}
