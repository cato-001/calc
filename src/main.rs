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
