use nom::IResult;

fn main() {
    let expression = Expression::Add((Expression::Number(73), Expression::Number(12312)).into());
    let result = expression.evaluate();
    println!("{}", result)
}

enum Expression {
    Number(i32),
    Negative(Box<Self>),
    Add(Box<(Self, Self)>),
}

impl Expression {
    fn parse(&self, input: &str) -> Result<Self, SyntaxError> {

    }

    fn inner_parse(&self, input: &str) -> IResult<&str, Self> {

    }

    fn evaluate(&self) -> i32 {
        match self {
            Self::Number(value) => *value,
            Self::Negative(value) => -value.evaluate(),
            Self::Add(expressions) => {
                let (expression, other) = expressions.as_ref();
                expression.evaluate() + other.evaluate()
            }
        }
    }
}

struct SyntaxError {
    value: String
}
