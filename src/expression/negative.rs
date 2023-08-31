use nom::character::complete::char;
use nom::IResult;
use crate::expression::Expression;

pub struct Negative(Box<Expression>);

impl Negative {
    pub fn parser(input: &str) -> IResult<&str, Expression> {
        let (input, _) = char('-')(input)?;
        let (input, expression) = Expression::parser(input)?;
        Ok((input, Expression::Negative(Self(expression.into()))))
    }

    pub fn evaluate(&self) -> i32 {
        -self.0.evaluate()
    }
}