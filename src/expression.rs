use nom::branch::alt;
use nom::combinator::complete;
use nom::{Finish, IResult};

use crate::error::SyntaxError;
use crate::expression::add::AddAndSub;
use crate::expression::negative::Negative;
use crate::expression::number::Number;

mod add;
mod negative;
mod number;
mod symbol;

pub enum Expression {
  Number(Number),
  Negative(Negative),
  AddAndSub(AddAndSub),
}

impl Expression {
  pub fn parse(input: &str) -> Result<Self, SyntaxError> {
    Self::parser(input)
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

  fn parser(input: &str) -> IResult<&str, Self> {
    complete(alt((AddAndSub::parser, Negative::parser, Number::parser)))(input)
  }
}

impl Expression {
  pub fn evaluate(&self) -> i32 {
    match self {
      Self::Number(expression) => expression.evaluate(),
      Self::Negative(expression) => expression.evaluate(),
      Self::AddAndSub(expression) => expression.evaluate(),
    }
  }
}
