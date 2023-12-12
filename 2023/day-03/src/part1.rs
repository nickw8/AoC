use crate::custom_error::AocError;

use nom::{
    character::complete::line_ending, character::complete::not_line_ending, multi::separated_list1,
    IResult,
};

#[derive(Debug)]
struct PartNumber {
    value: u32,
    positions: Vec<usize>,
    line: usize,
}
#[derive(Debug)]
struct SchematicLine {
    chars: Vec<char>,
    part_numbers: Vec<PartNumber>,
}

impl PartNumber {
    fn check_if_engine_part(&self, schematics: &[SchematicLine]) -> bool {
        let positions = &self.positions;
        let line = self.line;

        let directions = [
            (-1, -1),
            (-1, 0),
            (-1, 1),
            (0, -1),
            /* (0, 0), */
            (0, 1),
            (1, -1),
            (1, 0),
            (1, 1),
        ];

        let check_surround_for_symbols =
            |surrounding_line: usize, surrounding_position: usize| -> bool {
                let c = schematics[surrounding_line].chars[surrounding_position];
                !c.is_digit(10) && c != '.'
            };

        positions.iter().enumerate().any(|(_index, &position)| {
            directions.iter().any(|&(dx, dy)| {
                let surrounding_line = (line as isize) + dx;
                let surrounding_position = (position as isize) + dy;

                if surrounding_line < 0
                    || surrounding_position < 0
                    || surrounding_line as usize >= schematics.len()
                    || surrounding_position as usize >= schematics[line as usize].chars.len()
                {
                    false
                } else {
                    check_surround_for_symbols(
                        surrounding_line as usize,
                        surrounding_position as usize,
                    )
                }
            })
        })
    }
}

fn parse_schematic_line(line_number: usize, input: &str) -> IResult<&str, SchematicLine> {
    let (input, line_chars) = not_line_ending(input)?;
    let mut part_numbers = Vec::new();
    let mut current_number = String::new();
    let mut current_positions = Vec::new();

    line_chars.chars().enumerate().for_each(|(position, c)| {
        if c.is_digit(10) {
            current_number.push(c);
            current_positions.push(position);
        } else if !current_number.is_empty() {
            let value = current_number.parse::<u32>().unwrap();
            part_numbers.push(PartNumber {
                value,
                positions: current_positions.clone(),
                line: line_number,
            });
            current_number.clear();
            current_positions.clear();
        }
    });

    // Handle the last number in the line
    if !current_number.is_empty() {
        let value = current_number.parse::<u32>().unwrap();
        part_numbers.push(PartNumber {
            value,
            positions: current_positions,
            line: line_number,
        });
    }
    // dbg!(&line_chars, &part_numbers);
    Ok((
        input,
        SchematicLine {
            chars: line_chars.chars().collect(),
            part_numbers,
        },
    ))
}

fn parse_schematics(input: &str) -> IResult<&str, Vec<SchematicLine>> {
    let (input, lines) = separated_list1(line_ending, not_line_ending)(input)?;

    let mut schematics = Vec::new();

    for (line_number, line) in lines.iter().enumerate() {
        let (_, schematic_line) = parse_schematic_line(line_number, line)?;
        schematics.push(schematic_line);
    }

    Ok((input, schematics))
}

pub fn process(input: &str) -> miette::Result<String, AocError> {
    let (_, schematics) = parse_schematics(input).expect("this should parse");

    let sum: u32 = schematics
        .iter()
        .flat_map(|schematic| &schematic.part_numbers)
        .filter(|part_number| part_number.check_if_engine_part(&schematics))
        .map(|part_number| part_number.value)
        .sum();

    Ok(sum.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(
        "12.......*..
+.........34
.......-12..
..78........
..*....60...
78..........
.......23...
....90*12...
............
2.2......12.
.*.........*
1.1.......56",
        "413"
    )]
    #[case(
        "12.......*..
+.........34
.......-12..
..78........
..*....60...
78.........9
.5.....23..$
8...90*12...
............
2.2......12.
.*.........*
1.1..503+.56",
        "925"
    )]
    #[case(
        ".......5......
..7*..*.....4*
...*13*......9
.......15.....
..............
..............
..............
..............
..............
..............
21............
...*9.........",
        "62"
    )]
    fn reddit_tests(#[case] input: &str, #[case] expected: String) {
        assert_eq!(expected, process(input).unwrap())
    }

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";
        assert_eq!("4361", process(input)?);
        Ok(())
    }
}
