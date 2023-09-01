use crate::expression::add_and_sub::AddAndSub;
use nom::branch::alt;
use nom::character::complete::one_of;
use nom::multi::many1;
use nom::IResult;

use crate::expression::negative::Negative;
use crate::expression::number::Number;
use crate::expression::Expression;

pub struct MulAndDiv {
  start: Box<Expression>,
  parts: Vec<(Operator, Expression)>,
}

impl MulAndDiv {
  pub fn parser(input: &str) -> IResult<&str, Expression> {
    let (input, start) = inner_expression(input)?;
    let (input, parts) = many1(inner_expression_with_operator)(input)?;
    Ok((
      input,
      Expression::MulAndDiv(Self {
        start: start.into(),
        parts,
      }),
    ))
  }

  pub fn evaluate(&self) -> i32 {
    self
      .parts
      .iter()
      .fold(self.start.evaluate(), |combined, current| {
        let (operator, expression) = current;
        let result = expression.evaluate();
        match operator {
          Operator::Mul => combined * result,
          Operator::Div => combined / result,
        }
      })
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

fn inner_expression_with_operator(input: &str) -> IResult<&str, (Operator, Expression)> {
  let (input, operator) = Operator::parser(input)?;
  let (input, expression) = inner_expression(input)?;
  Ok((input, (operator, expression)))
}

fn inner_expression(input: &str) -> IResult<&str, Expression> {
  alt((Number::parser, Negative::parser))(input)
}
