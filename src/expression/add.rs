use nom::branch::alt;
use nom::multi::separated_list1;
use nom::IResult;

use crate::expression::negative::Negative;
use crate::expression::number::Number;
use crate::expression::symbol::char_with_whitespace;
use crate::expression::Expression;

pub struct Add(Vec<Expression>);

impl Add {
  pub fn parser(input: &str) -> IResult<&str, Expression> {
    let (input, expressions) =
      separated_list1(char_with_whitespace('+'), Self::inner_expression)(input)?;
    Ok((input, Expression::Add(Self(expressions))))
  }

  fn inner_expression(input: &str) -> IResult<&str, Expression> {
    alt((Number::parser, Negative::parser))(input)
  }

  pub fn evaluate(&self) -> i32 {
    self.0.iter().map(Expression::evaluate).sum()
  }
}
