fn main() {
    let expression = Expression::Number(73);
    let result = expression.evaluate();
    println!("{}", result)
}

enum Expression {
    Number(i32),
}

impl Expression {
    fn evaluate(&self) -> i32 {
        match self {
            Expression::Number(value) => *value,
        }
    }
}
