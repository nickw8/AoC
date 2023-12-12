use rand::Rng;
use std::collections::HashSet;

use crate::custom_error::AocError;
use nom::{
    character::complete::line_ending, character::complete::not_line_ending, multi::separated_list1,
    IResult,
};

#[derive(Debug)]
struct Gear {
    index: usize,
    line: usize,
}

#[derive(Debug)]
struct PartNumber {
    uid: u32,
    value: u32,
    index: Vec<usize>,
    line: usize,
}
#[derive(Debug)]
struct SchematicLine {
    chars: Vec<char>,
    part_numbers: Vec<PartNumber>,
    gears: Vec<Gear>,
}

impl Gear {
    fn find_gear_ratio(&self, schematics: &[SchematicLine]) -> Option<u32> {
        // dbg!(self);
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

        let is_part_number = |index: usize, line: usize| {
            schematics[line]
                .part_numbers
                .iter()
                .find(|part_number| part_number.index.contains(&index))
                .map(|part_number| part_number.uid)
        };

        let surrounding_numbers: HashSet<u32> = directions
            .iter()
            .filter_map(|&(dx, dy)| {
                let surrounding_line_number = (self.line as isize) + dx;
                let surrounding_index = (self.index as isize) + dy;

                (surrounding_line_number >= 0
                    && surrounding_index >= 0
                    && surrounding_line_number < schematics.len() as isize
                    && surrounding_index
                        < schematics[surrounding_line_number as usize].chars.len() as isize)
                    .then(|| {
                        is_part_number(surrounding_index as usize, surrounding_line_number as usize)
                    })
                    .flatten()
            })
            .collect();

        if surrounding_numbers.len() != 2 {
            return None;
        }

        // let part_numbers: Vec<Option<u32>> = surrounding_numbers
        //     .into_iter()
        //     .map(|uid| {
        //         schematics
        //             .iter()
        //             .flat_map(|schematic| &schematic.part_numbers)
        //             .find(|part_number| part_number.uid == uid)
        //             .map(|part_number| part_number.value)
        //     })
        //     .collect();

        // match (part_numbers[0], part_numbers[1]) {
        //     (Some(a), Some(b)) => Some(a * b),
        //     _ => None,
        // }
        surrounding_numbers
            .into_iter()
            .map(|uid| {
                schematics
                    .iter()
                    .flat_map(|schematic| &schematic.part_numbers)
                    .find(|part_number| part_number.uid == uid)
                    .map(|part_number| part_number.value)
            })
            .fold(Some(1), |acc, x| match (acc, x) {
                (Some(a), Some(b)) => Some(a * b),
                _ => None,
            })
    }
}

impl PartNumber {
    #[allow(dead_code)]
    fn check_if_engine_part(&self, schematics: &[SchematicLine]) -> bool {
        let positions = &self.index;
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
                match (
                    surrounding_line >= schematics.len(),
                    surrounding_position >= schematics[surrounding_line].chars.len(),
                ) {
                    (true, _) | (_, true) => false,
                    _ => {
                        let c = schematics[surrounding_line].chars[surrounding_position];
                        !c.is_digit(10) && c != '.'
                    }
                }
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

fn process_current_number(
    current_number: &mut String,
    current_positions: &mut Vec<usize>,
    part_numbers: &mut Vec<PartNumber>,
    line_number: usize,
) {
    if !current_number.is_empty() {
        if let Ok(value) = current_number.parse::<u32>() {
            let uid: u32 = rand::thread_rng().gen();
            part_numbers.push(PartNumber {
                uid,
                value,
                index: current_positions.clone(),
                line: line_number,
            });
        }
        current_number.clear();
        current_positions.clear();
    }
}

fn parse_schematic_line(line_number: usize, input: &str) -> IResult<&str, SchematicLine> {
    let (input, line_chars) = not_line_ending(input)?;
    let mut part_numbers = Vec::new();
    let mut gears = Vec::new();
    let mut current_number = String::new();
    let mut current_positions = Vec::new();

    for (position, c) in line_chars.chars().enumerate() {
        match c {
            c if c.is_digit(10) => {
                current_number.push(c);
                current_positions.push(position);
            }
            '*' => {
                gears.push(Gear {
                    index: position,
                    line: line_number,
                });
                process_current_number(
                    &mut current_number,
                    &mut current_positions,
                    &mut part_numbers,
                    line_number,
                );
            }
            _ => {
                process_current_number(
                    &mut current_number,
                    &mut current_positions,
                    &mut part_numbers,
                    line_number,
                );
            }
        }
    }

    // Handles the case where the line ends with a number
    process_current_number(
        &mut current_number,
        &mut current_positions,
        &mut part_numbers,
        line_number,
    );

    Ok((
        input,
        SchematicLine {
            chars: line_chars.chars().collect(),
            part_numbers,
            gears,
        },
    ))
}

fn parse_schematics(input: &str) -> IResult<&str, Vec<SchematicLine>> {
    let (input, lines) = separated_list1(line_ending, not_line_ending)(input)?;

    // let mut schematics = Vec::new();

    // for (line_number, line) in lines.iter().enumerate() {
    //     let (_, schematic_line) = parse_schematic_line(line_number, line)?;
    //     schematics.push(schematic_line);
    // }

    // Ok((input, schematics))
    let schematics: Result<Vec<SchematicLine>, _> = lines
        .iter()
        .enumerate()
        .map(|(line_number, line)| {
            let (_, schematic_line) = parse_schematic_line(line_number, line)?;
            Ok(schematic_line)
        })
        .collect();

    Ok((input, schematics?))
}

pub fn process(input: &str) -> miette::Result<String, AocError> {
    let (_, schematics) = parse_schematics(input).expect("this should parse");

    // let sum: u32 = schematics
    //     .iter()
    //     .flat_map(|schematic| &schematic.gears)
    //     .map(|gear| gear.find_gear_ratio(&schematics).unwrap_or(0))
    //     .sum();
    let sum: u32 = schematics
        .iter()
        .flat_map(|schematic| &schematic.gears)
        .filter_map(|gear| gear.find_gear_ratio(&schematics))
        .sum();

    Ok(sum.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(
        "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..",
        "467835"
    )]
    #[case(
        ".2.
.*.
585",
        "1170"
    )]
    #[case(  // 78*78+12*56
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
        "6756"
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
        "478"
    )]
    fn tests(#[case] input: &str, #[case] expected: String) -> miette::Result<()> {
        assert_eq!(expected, process(input)?);
        Ok(())
    }
}
