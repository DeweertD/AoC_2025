/*
--- Day 2: Gift Shop ---

You get inside and take the elevator to its only other stop: the gift shop. "Thank you for visiting the North Pole!" gleefully exclaims a nearby sign. You aren't sure who is even allowed to visit the North Pole, but you know you can access the lobby through here, and from there you can access the rest of the North Pole base.

As you make your way through the surprisingly extensive selection, one of the clerks recognizes you and asks for your help.

As it turns out, one of the younger Elves was playing on a gift shop computer and managed to add a whole bunch of invalid product IDs to their gift shop database! Surely, it would be no trouble for you to identify the invalid product IDs for them, right?

They've even checked most of the product ID ranges already; they only have a few product ID ranges (your puzzle input) that you'll need to check. For example:

11-22,95-115,998-1012,1188511880-1188511890,222220-222224,
1698522-1698528,446443-446449,38593856-38593862,565653-565659,
824824821-824824827,2121212118-2121212124

(The ID ranges are wrapped here for legibility; in your input, they appear on a single long line.)

The ranges are separated by commas (,); each range gives its first ID and last ID separated by a dash (-).

Since the young Elf was just doing silly patterns, you can find the invalid IDs by looking for any ID which is made only of some sequence of digits repeated twice. So, 55 (5 twice), 6464 (64 twice), and 123123 (123 twice) would all be invalid IDs.

None of the numbers have leading zeroes; 0101 isn't an ID at all. (101 is a valid ID that you would ignore.)

Your job is to find all of the invalid IDs that appear in the given ranges. In the above example:

    11-22 has two invalid IDs, 11 and 22.
    95-115 has one invalid ID, 99.
    998-1012 has one invalid ID, 1010.
    1188511880-1188511890 has one invalid ID, 1188511885.
    222220-222224 has one invalid ID, 222222.
    1698522-1698528 contains no invalid IDs.
    446443-446449 has one invalid ID, 446446.
    38593856-38593862 has one invalid ID, 38593859.
    The rest of the ranges contain no invalid IDs.

Adding up all the invalid IDs in this example produces 1227775554.

What do you get if you add up all of the invalid IDs?
*/
use std::iter::Map;
use std::str;

fn main() {
    let file = std::fs::read_to_string("Day2/input.txt").expect("file not found or read");
    println!("Day 2 Part 1: {}", part1(read_values(&file)));
    // println!("Day 2 Part 2: {}", part2(read_values(&file)));
}

fn part1(values: Map<str::Split<'_, char>, fn(&str) -> (u64, u64)>) -> u64 {
    let mut invalid_id_accumulator = 0;
    values.for_each(|(lower, upper)| {
        let mut lower_half = 0;
        let mut upper_half = 0;

        for i in 0..=1 {
            let lower_zeros = (count_digits(lower) / 2) + i;
            let lower_divisor = 10u64.pow(lower_zeros) + 1;
            lower_half = (lower as f64 / lower_divisor as f64).ceil() as u64;
            if lower_half <= lower_divisor {
                break;
            };
        }

        for i in 0..1 {
            let upper_zeros = (count_digits(upper) / 2) + i;
            let upper_divisor = 10u64.pow(upper_zeros) + 1;
            upper_half = (upper as f64 / upper_divisor as f64).floor() as u64;
            if upper_half <= upper_divisor {
                break;
            };
        }

        for i in lower_half..=upper_half {
            let doubling_factor = 10u64.pow(count_digits(i)) + 1;
            let invalid_id = i * doubling_factor;
            if invalid_id < lower {
                continue;
            }
            if invalid_id > upper {
                break;
            }
            invalid_id_accumulator += invalid_id;
        }
    });
    invalid_id_accumulator
}

fn _part2(_values: Map<str::Split<'_, char>, fn(&str) -> (u64, u64)>) -> i32 {
    todo!()
}

fn read_values(file: &String) -> Map<str::Split<'_, char>, fn(&str) -> (u64, u64)> {
    file.split(',').map(|range| {
        let str_tuple = range.split_once('-').unwrap();
        (
            str_tuple.0.parse::<u64>().unwrap(),
            str_tuple.1.parse::<u64>().unwrap(),
        )
    })
}

fn count_digits(num: u64) -> u32 {
    1 + num.abs_diff(0).checked_ilog10().unwrap_or_default()
}

#[cfg(test)]
mod test;
