use nom::{Finish, IResult};
use nom::branch::alt;
use nom::combinator::complete;

use crate::error::SyntaxError;
use crate::expression::add::Add;
use crate::expression::negative::Negative;
use crate::expression::number::Number;

mod number;
mod negative;
mod add;
mod symbol;

pub enum Expression {
    Number(Number),
    Negative(Negative),
    Add(Add),
}

impl Expression {
    pub fn parse(input: &str) -> Result<Self, SyntaxError> {
        Self::parser(input)
            .finish()
            .map_err(Into::into)
            .and_then(|result| if result.0.is_empty() {
                Ok(result.1)
            } else {
                Err(SyntaxError::new(format!("ExpressionParsingIncomplete: {}", result.0)))
            })
    }

    fn parser(input: &str) -> IResult<&str, Self> {
        complete(alt((
            Add::parser,
            Negative::parser,
            Number::parser,
        )))(input)
    }
}

impl Expression {
    pub fn evaluate(&self) -> i32 {
        match self {
            Self::Number(expression) => expression.evaluate(),
            Self::Negative(expression) => expression.evaluate(),
            Self::Add(expression) => expression.evaluate(),
        }
    }
}
