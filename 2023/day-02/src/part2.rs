use crate::custom_error::AocError;
use nom::{
    bytes::complete::tag,
    character::complete::{self, alpha1, digit1, line_ending},
    multi::separated_list1,
    sequence::{preceded, separated_pair},
    IResult,
};

#[derive(Debug)]
struct Cube<'a> {
    color: &'a str,
    amount: u32,
}

#[derive(Debug)]
struct Game<'a> {
    // id: &'a str,
    rounds: Vec<Vec<Cube<'a>>>,
}

// The power of a set of cubes is equal to the numbers of red, green, and blue cubes multiplied together.
// Minimum set is the fewest number of cubes of each color that could have been in the bag to make the game possible
impl<'a> Game<'a> {
    fn power_of_min_cube_set(&self) -> Option<u32> {
        let colors = ["red", "blue", "green"];
        let product_of_max: u32 = colors
            .iter()
            .map(|&color| {
                self.rounds
                    .iter()
                    .flatten() // Flatten the Vec<Vec<Cube>> to an iterator over Cube
                    .filter(|cube| cube.color == color) // Filter out the cubes of the specific color
                    .max_by_key(|cube| cube.amount) // Find the cube with the maximum amount
                    .map(|cube| cube.amount) // Extract the amount
                    .unwrap_or(0) // If there are no cubes of this color, use 0
            })
            .product(); // Get the product of the maximum amounts

        Some(product_of_max)
    }
}

// 4 red
fn cube(input: &str) -> IResult<&str, Cube> {
    let (input, (amount, color)) = separated_pair(complete::u32, tag(" "), alpha1)(input)?;
    Ok((input, Cube { color, amount }))
}
// 3 blue, 4 red
fn round(input: &str) -> IResult<&str, Vec<Cube>> {
    let (input, cubes) = separated_list1(tag(", "), cube)(input)?;
    Ok((input, cubes))
}
// Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
fn game(input: &str) -> IResult<&str, Game> {
    let (input, _) = preceded(tag("Game "), digit1)(input)?;
    let (input, rounds) = preceded(tag(": "), separated_list1(tag("; "), round))(input)?;
    Ok((input, Game { rounds }))
}
fn parse_games(input: &str) -> IResult<&str, Vec<Game>> {
    let (input, games) = separated_list1(line_ending, game)(input)?;
    Ok((input, games))
}

pub fn process(input: &str) -> miette::Result<String, AocError> {
    // let map = BTreeMap::from([("red", 12), ("green", 13), ("blue", 14)]);
    let games = parse_games(input).expect("should parse");

    Ok(games
        .1
        .iter()
        .filter_map(|game| game.power_of_min_cube_set())
        .sum::<u32>()
        .to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        // todo!("haven't built test yet");
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        assert_eq!("2286", process(input)?);
        Ok(())
    }
}
