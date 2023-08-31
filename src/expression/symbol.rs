use nom::character::complete::{char, space0};
use nom::IResult;

pub fn char_with_whitespace(character: char) -> impl Fn(&str) -> IResult<&str, ()> {
    move |input| {
        let (input, _) = space0(input)?;
        let (input, _) = char(character)(input)?;
        let (input, _) = space0(input)?;
        Ok((input, ()))
    }
}