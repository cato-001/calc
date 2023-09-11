use crate::expression::expression;
use crate::number::ParsableNumber;
use nom::branch::alt;
use nom::character::complete::char;
use nom::sequence::delimited;
use nom::IResult;

pub fn parenthesis<Number>(input: &str) -> IResult<&str, Number>
where
  Number: ParsableNumber,
{
  alt((
    delimited(char('('), expression, char(')')),
    delimited(char('['), expression, char(']')),
  ))(input)
}
