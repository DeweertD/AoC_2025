use rstest::*;
use crate::*;

#[rstest]
#[case("11-22", 33)]
#[case("95-115", 99)]
#[case("998-1012", 1010)]
#[case("1188511880-1188511890", 1188511885)]
#[case("222220-222224", 222222)]
#[case("1698522-1698528", 0)]
#[case("446443-446449", 446446)]
#[case("38593856-38593862", 38593859)]
fn part1_tests(#[case] input: String, #[case] expected: u64) {
    assert_eq!(expected, part1(read_values(dbg!(&input))))
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