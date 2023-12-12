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
    winning_numbers: Vec<u32>,
    my_numbers: Vec<u32>,
}

impl Card {
    fn find_winning_numbers(&self) -> Vec<&u32> {
        self.my_numbers
            .iter()
            .filter(|&number| self.winning_numbers.contains(number))
            .collect()
    }

    fn calculate_points(&self) -> u32 {
        let my_winners = self.find_winning_numbers().len();
        (my_winners > 0)
            .then(|| 2u32.pow(my_winners as u32 - 1))
            .unwrap_or(0)
        // match my_winners {
        //     0 => 0,
        //     _ => 2u32.pow(my_winners as u32 - 1),
        // }
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
    let (input, (_, winning_numbers, _, my_numbers)) = tuple((
        tuple((
            multispace0,
            tag("Card"),
            multispace1,
            digit1,
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
            winning_numbers,
            my_numbers,
        },
    ))
}

fn parse_cards(input: &str) -> IResult<&str, Vec<Card>> {
    many1(card)(input)
}

pub fn process(input: &str) -> miette::Result<String, AocError> {
    let (_, cards) = parse_cards(input).expect("this should parse");
    dbg!(&cards.len());
    Ok(cards
        .iter()
        .map(|card| card.calculate_points())
        .sum::<u32>()
        .to_string())
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
        "13"
    )]
    #[case(
        // card 1 winners: 71 88 83  5 15 54 89 55 69 79 - 10 winners so 2^9 = 512
        // card 2 winners: 66 67 95 78 71 98 65 - 7 winners so 2^6 = 64
        // card 3 winners: 71 40 25 7 - 4 winners so 2^3 = 8
        "Card   1: 71 88 83  5 15 54 89 55 69 79 | 83 39 58 32 99 54 91 19 44  5 57 29 88  9 95 15 79 71 90 69 43 66 55 12 89
Card   2: 33 11 66 48 67 95 78 71 98 65 | 66  2  1 59 77 95 61 71  8 38 18 62 10 65 53 17 75 92 64 50 67 21 51 78 98
Card   3: 28 58 71 40 25 13  7 19 61 72 | 47 89 96  3 84 77 81 76 93 20 34  7 25 91 71 22 36  9 40 98 60 67 35 54 49
Card   4: 58 26 74 94 42 29  9 90 76 54 | 74 90 41 32 19 80 27 97  9  2 57 45 29 42 76 37 83 58 25 46 94 86 63 24 12
Card   5: 70 65 54 14 32 29 39 98  9 51 | 61 14 43  9 45 94 40 69 62 29 54 50 15 92 30  4 49 31 55 98 28  1 70 39 20
Card   6:  2 88 60 55 45 19 59 95 79  4 | 61 65 19 88 34 74 93  4 70 28 60 18 75 55  2 49  3 67 86 57 91 59 95 79 45
Card   7: 16 87 54 73 30 43 95 74 50  9 |  9 79 13 71 11 87 82 78 50 12  1 18 24 54 73 42 85  3  7 31 40 30 43 83 92
Card   8: 48 72 63 92 40 70 53 84 37 31 | 72 40 37 60 82 70  7 88 65  6 92 83 56 48 46 33 31 53 14 34 26 63 61 84 91",
        "1864"
    )]
    fn tests(#[case] input: &str, #[case] expected: String) -> miette::Result<()> {
        assert_eq!(expected, process(input)?);
        Ok(())
    }
}
