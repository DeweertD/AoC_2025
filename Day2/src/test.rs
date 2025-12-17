use rstest::*;
use crate::*;

#[rstest]
#[case("R50\nR1\nL1", 2)]
#[case("R50\nR1", 1)]
#[case("L50\nR1\nL1", 2)]
#[case("L50\nL1", 1)]
#[case("R50\nR100", 2)]
#[case("R50\nL100", 2)]
#[case("L50\nL100", 2)]
#[case("L50\nR100", 2)]
#[case("L50\nR1000", 2)]
#[case("R50\nL1000", 2)]
fn part1_tests(#[case] input: String, #[case] expected: i32) {
    assert_eq!(expected, part1(read_values(&input)))
}


// #[rstest]
// #[case("R450\nR1\nL1", 6)]
// #[case("R49\nR2", 1)]
// #[case("R49\nR1", 1)]
// #[case("R49\nR2\nL1", 2)]
// #[case("R1000", 10)]
// #[case("R1000\nL50", 11)]
// fn part2_tests(#[case] input: String, #[case] expected: i32) {
//     assert_eq!(expected, part2(read_values(&input)))
// }