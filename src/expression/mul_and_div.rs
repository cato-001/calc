use crate::expression::negative::negative;
use crate::expression::parenthesis::parenthesis;
use nom::branch::alt;
use nom::character::complete::one_of;
use nom::IResult;

use crate::number::ParsableNumber;

pub fn mul_and_div<Number>(input: &str) -> IResult<&str, Number>
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
      Operator::Mul => result * number,
      Operator::Div => result / number,
    }
  }
}

enum Operator {
  Mul,
  Div,
}

impl Operator {
  fn parser(input: &str) -> IResult<&str, Operator> {
    let (input, operator) = one_of("x*/")(input)?;
    let operator = match operator {
      'x' | '*' => Self::Mul,
      '/' => Self::Div,
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
  alt((parenthesis, negative, Number::parse))(input)
}
