use nom::branch::alt;
use nom::combinator::complete;
use nom::{Finish, IResult};

use crate::error::SyntaxError;
use crate::expression::add_and_sub::add_and_sub;
use crate::expression::mul_and_div::mul_and_div;
use crate::expression::negative::negative;
use crate::number::ParsableNumber;

mod add_and_sub;
mod mul_and_div;
mod negative;
mod number;
mod symbol;

pub fn evaluate<Number>(input: &str) -> Result<Number, SyntaxError>
where
  Number: ParsableNumber,
{
  expression(input)
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

fn expression<Number>(input: &str) -> IResult<&str, Number>
where
  Number: ParsableNumber,
{
  complete(alt((add_and_sub, mul_and_div, negative, Number::parse)))(input)
}
