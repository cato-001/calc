use nom::branch::alt;
use nom::character::complete::one_of;
use nom::IResult;

use crate::expression::mul_and_div::mul_and_div;
use crate::expression::negative::negative;
use crate::expression::parenthesis;
use crate::number::ParsableNumber;

pub fn add_and_sub<Number>(input: &str) -> IResult<&str, Number>
where
  Number: ParsableNumber,
{
  let (mut input, mut result) = inner_expression(input)?;
  loop {
    let (next_input, (operator, number)) = match inner_expression_with_operator(input) {
      Ok(result) => result,
      Err(error) if matches!(error, nom::Err::Failure(_)) => return Err(error),
      Err(_) => return Ok((input, result)),
    };
    input = next_input;
    result = match operator {
      Operator::Add => result + number,
      Operator::Sub => result - number,
    }
  }
}

enum Operator {
  Add,
  Sub,
}

impl Operator {
  fn parser(input: &str) -> IResult<&str, Operator> {
    let (input, operator) = one_of("+-")(input)?;
    let operator = match operator {
      '+' => Self::Add,
      '-' => Self::Sub,
      _ => unreachable!(),
    };
    Ok((input, operator))
  }
}

fn inner_expression_with_operator<Number>(input: &str) -> IResult<&str, (Operator, Number)>
where
  Number: ParsableNumber,
{
  let (input, operator) = Operator::parser(input)?;
  let (input, expression) = inner_expression(input)?;
  Ok((input, (operator, expression)))
}

fn inner_expression<Number>(input: &str) -> IResult<&str, Number>
where
  Number: ParsableNumber,
{
  alt((mul_and_div, parenthesis, Number::parse, negative))(input)
}
