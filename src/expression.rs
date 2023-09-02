use nom::branch::alt;
use nom::combinator::complete;
use nom::{Finish, IResult};

use crate::error::SyntaxError;
use crate::expression::add_and_sub::AddAndSub;
use crate::expression::mul_and_div::MulAndDiv;
use crate::expression::negative::Negative;
use crate::expression::number::Value;

mod add_and_sub;
mod mul_and_div;
mod negative;
mod number;
mod symbol;

pub fn evaluate<Number>(input: &str) -> Result<Number, SyntaxError> {
  parser(input)
    .finish()
    .map_err(Into::into)
    .and_then(|result| {
      if result.0.is_empty() {
        Ok(result.1)
      } else {
        Err(SyntaxError::new(format!(
          "ExpressionParsingIncomplete: {}",
          result.0
        )))
      }
    })
}

fn parser<Number>(input: &str) -> IResult<&str, Number> {
  complete(alt((
    MulAndDiv::div_and_mul,
    AddAndSub::parser,
    Negative::parser,
    Value::parser,
  )))(input)
}
