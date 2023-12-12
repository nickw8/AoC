use aho_corasick::{AhoCorasick, PatternID};
// use regex::Regex;
use std::i32;

fn main() {
    let input = include_str!("./input1.txt");
    let output = part2(input);
    dbg!(output);
}

fn part2(input: &str) -> String {
    // split by newline into vec of lines
    input
        .split('\n')
        .map(|line| find_number(line).parse::<i32>().unwrap())
        .sum::<i32>()
        .to_string()
}

// have to find first number(digit or spelled out) from the front and first digit from the back
fn find_number(input: &str) -> String {
    let patterns = &[
        "1", "2", "3", "4", "5", "6", "7", "8", "9", "one", "two", "three", "four", "five", "six",
        "seven", "eight", "nine",
    ];
    let ac = AhoCorasick::new(patterns).unwrap();
    let results: Vec<PatternID> = ac
        .find_overlapping_iter(input)
        .map(|mat| mat.pattern())
        .collect::<Vec<_>>();
    // dbg!(&results);

    if results.is_empty() {
        return "0".to_string();
    }
    // dbg!(&results.get(0).unwrap().as_usize());

    let first_pattern_index = results.first().unwrap().as_usize();
    let first = convert_words_to_digits(patterns.get(first_pattern_index).unwrap());
    // dbg!(&first);

    let last_pattern_index = results.last().unwrap().as_usize();
    let last = convert_words_to_digits(patterns.get(last_pattern_index).unwrap());
    // dbg!(&last);

    // This returns a string but then part1 has to convert back to a number to sum
    format!("{}{}", first, last)
}

fn convert_words_to_digits(_input: &str) -> &str {
    match _input {
        "zero" => "0",
        "one" => "1",
        "two" => "2",
        "three" => "3",
        "four" => "4",
        "five" => "5",
        "six" => "6",
        "seven" => "7",
        "eight" => "8",
        "nine" => "9",
        i if i.parse::<i32>().is_ok() => i,
        _ => "0",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_single_line() {
        let test_line_multi_nums = find_number("a1b2c3d4e5f");
        assert_eq!(test_line_multi_nums, "15".to_string());

        let test_line_1_num2 = find_number("z5");
        assert_eq!(test_line_1_num2, "55".to_string());

        let test_line_1_word = find_number("one");
        assert_eq!(test_line_1_word, "11".to_string());
    }

    #[test]
    fn check_overlap1() {
        let test_line_2_nums = find_number("sevenine");
        assert_eq!(test_line_2_nums, "79");
    }

    #[test]
    fn check_overlap2() {
        let test_line_1_num = find_number("eighthree");
        assert_eq!(test_line_1_num, "83".to_string());
    }

    #[test]
    fn check_for_word_numbers() {
        let test_input = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";
        let result_part2 = part2(test_input);
        // 29+83+13+24+42+14+76=281
        assert_eq!(result_part2, "281".to_string());
    }
}
