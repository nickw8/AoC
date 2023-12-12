use crate::custom_error::AocError;
use nom::{
    bytes::complete::tag,
    character::complete::{digit1, multispace0, multispace1},
    combinator::map_res,
    multi::{many1, separated_list1},
    sequence::tuple,
    IResult,
};

#[derive(Debug)]
struct Card {
    card_number: u32,
    winning_numbers: Vec<u32>,
    my_numbers: Vec<u32>,
    cards_won: u32,
}

impl Card {
    #[allow(dead_code)]

    fn find_winning_numbers(&self) -> Vec<&u32> {
        self.my_numbers
            .iter()
            .filter(|&number| self.winning_numbers.contains(number))
            .collect()
    }

    fn calculate_points(&self)  {
        let self.cards_won = self.find_winning_numbers().len().unwrap_or(0);

    }
}

fn parse_card_number(input: &str) -> IResult<&str, u32> {
    // dbg!(input);
    map_res(digit1, str::parse::<u32>)(input)
}

fn parse_card_numbers(input: &str) -> IResult<&str, Vec<u32>> {
    // dbg!(&input);
    separated_list1(multispace1, parse_card_number)(input)
}

fn card(input: &str) -> IResult<&str, Card> {
    // // dbg!(&input);
    // let (input, _) = tuple((
    //     multispace0,
    //     tag("Card"),
    //     multispace1,
    //     digit1,
    //     tag(":"),
    //     multispace1,
    // ))(input.trim())?;
    // // dbg!(&input);
    // let (input, winning_numbers) = parse_card_numbers(input)?;
    // // dbg!(&input);
    // let (input, _) = tuple((multispace1, tag("|"), multispace1))(input)?;
    // // dbg!(&input);
    // let (input, my_numbers) = parse_card_numbers(input)?;
    // // dbg!(&input);

    // dbg!(&input);
    let (input, ((_, _, _, card_number, _, _), winning_numbers, _, my_numbers)) = tuple((
        tuple((
            multispace0,
            tag("Card"),
            multispace1,
            map_res(digit1, str::parse::<u32>),
            tag(":"),
            multispace1,
        )),
        parse_card_numbers,
        tuple((multispace1, tag("|"), multispace1)),
        parse_card_numbers,
    ))(input)?;

    Ok((
        input,
        Card {
            card_number,
            winning_numbers,
            my_numbers,
            cards_won: 0,
        },
    ))
}

fn parse_cards(input: &str) -> IResult<&str, Vec<Card>> {
    many1(card)(input)
}

pub fn process(input: &str) -> miette::Result<String, AocError> {
    let (_, cards) = parse_cards(input).expect("this should parse");
    dbg!(&cards.len());
    Ok(cards.len().to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(
        "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11",
        "30"
    )]
    fn tests(#[case] input: &str, #[case] expected: String) -> miette::Result<()> {
        assert_eq!(expected, process(input)?);
        Ok(())
    }
}
