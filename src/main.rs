use crate::expression::Expression;
use std::env;

mod error;
mod expression;

fn main() {
  let results = env::args()
    .skip(1)
    .map(|arg| Expression::parse(&arg).map(|expression| expression.evaluate()));
  for result in results {
    match result {
      Ok(value) => println!("{}", value),
      Err(error) => println!("SyntaxError({})", error),
    }
  }
}
