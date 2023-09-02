use crate::expression::evaluate;
use std::env;

mod error;
mod expression;
mod number;

fn main() {
  let results = env::args().skip(1).map(|arg| evaluate::<f64>(arg.as_str()));
  for result in results {
    match result {
      Ok(value) => println!("{}", value),
      Err(error) => println!("SyntaxError({})", error),
    }
  }
}
