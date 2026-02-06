use rstest::*;

#[rstest]
#[case(10, 10)]
#[case(5, 10)]
#[case(20, 100)]
pub fn test_basic(#[case] i: usize, #[case] max_allowed: usize) {
    assert!(i <= max_allowed);
}
