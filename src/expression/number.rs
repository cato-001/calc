use nom::character::complete::i32;
use nom::IResult;

pub struct Value(i32);

impl Value {
  pub fn parser<Number>(input: &str) -> IResult<&str, Number> {
    i32(input).map(|result| (result.0, Expression::Number(Self(result.1))))
  }
}

impl Value {
  pub fn evaluate(&self) -> i32 {
    self.0
  }
}
