use std::i32;

fn main() {
    let input = include_str!("./input1.txt");
    let output = part1(input);
    dbg!(output);
}

fn part1(input: &str) -> String {
    // split by newline into vec of lines
    input
        .split('\n')
        .map(|line| find_number(line).parse::<i32>().unwrap())
        .sum::<i32>()
        .to_string()

    // &*results

    // get the right number chars out of each line
    // sum the numbers
    // for item in input_split {
    //     find_number(item)
    // }
    // todo!()
}

fn find_number(input: &str) -> String {
    // go to the first digit closest to the first char forward and the first digit closest to
    // the last char backwards and concantenate them together

    let mut number = input.chars().filter(|c| c.is_ascii_digit()).peekable();
    // dbg!(&number);
    if number.peek().is_none() {
        return "0".to_string();
    }
    let first = number.next().unwrap();
    // dbg!(first);
    let last = if number.peek().is_some() {
        number.next_back().unwrap()
    } else {
        first
    };
    // dbg!(last);

    // This returns a string but then part1 has to convert back to a number to sum
    format!("{}{}", first, last)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_single_line() {
        let test_line_2_nums = find_number("1abc2");
        assert_eq!(test_line_2_nums, "12");

        let test_line_multi_nums = find_number("a1b2c3d4e5f");
        assert_eq!(test_line_multi_nums, "15".to_string());

        let test_line_1_num = find_number("treb7uchet");
        assert_eq!(test_line_1_num, "77".to_string());

        let test_line_1_num2 = find_number("z5");
        assert_eq!(test_line_1_num2, "55".to_string());
    }

    #[test]
    fn it_works() {
        let test_input = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";
        // let test_split: Vec<&str> = test_input.split('\n').collect();
        // assert_eq!(
        //     test_split,
        //     &["1abc2", "pqr3stu8vwx", "a1b2c3d4e5f", "treb7uchet"]
        // );
        let result_part1 = part1(test_input);
        assert_eq!(result_part1, "142".to_string());
    }
}
