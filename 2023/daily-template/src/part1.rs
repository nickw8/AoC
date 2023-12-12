use crate::custom_error::AocError;

pub fn process(_input: &str) -> miette::Result<String, AocError> {
    todo!("day 01 - part 1");
}

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case("", "")]

    fn tests(#[case] input: &str, #[case] expected: String) -> miette::Result<()> {
        assert_eq!(expected, process(input)?);
        Ok(())
    }
}
