use crate::number::ParsableNumber;
use nom::character::complete::char;
use nom::IResult;

pub fn negative<Number>(input: &str) -> IResult<&str, Number>
where
  Number: ParsableNumber,
{
  let (input, _) = char('-')(input)?;
  let (input, number) = Number::parse(input)?;
  Ok((input, number))
}
