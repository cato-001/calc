fn main() {
    let expression = Expression::Add((Expression::Number(73), Expression::Number(12312)).into());
    let result = expression.evaluate();
    println!("{}", result)
}

enum Expression {
    Number(i32),
    Add(Box<(Self, Self)>),
}

impl Expression {
    fn evaluate(&self) -> i32 {
        match self {
            Expression::Number(value) => *value,
            Expression::Add(values) => {
                let (value, other) = values.as_ref();
                value.evaluate() + other.evaluate()
            }
        }
    }
}
