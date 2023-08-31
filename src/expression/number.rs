use nom::character::complete::i32;
use nom::IResult;
use crate::expression::Expression;

pub struct Number(i32);

impl Number {
    pub fn parser(input: &str) -> IResult<&str, Expression> {
        i32(input).map(|result| (result.0, Expression::Number(Self(result.1))))
    }
}

impl Number {
    pub fn evaluate(&self) -> i32 {
        self.0
    }
}