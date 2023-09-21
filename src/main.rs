use crate::expression::evaluate;
use std::env;
use std::fs::read_to_string;
use std::io::stdin;
use std::path::Path;

mod error;
mod expression;
mod number;

fn main() {
  let mut arguments = env::args().skip(1).peekable();
  let first_argument = arguments.peek();
  let Some(argument) = first_argument else {
    evaluate_from_stdin();
    return;
  };
  if matches!(argument.as_str(), "-f" | "--file") {
    for filename in arguments.skip(1) {
      evaluate_from_file(filename);
    }
    return;
  }
  evaluate_from_arguments(arguments);
}

fn evaluate_from_stdin() {
  let reader = stdin();
  let mut buffer = String::new();
  loop {
    let Ok(_) = reader.read_line(&mut buffer) else {
      return;
    };
    for expression in buffer.split_whitespace() {
      evaluate_and_print(expression);
    }
    buffer.clear();
  }
}

fn evaluate_from_file<P: AsRef<Path>>(filename: P) {
  let Ok(content) = read_to_string(&filename) else {
    println!("file could not been found: {}", filename.as_ref().display());
    return;
  };
  for expression in content.split_whitespace() {
    evaluate_and_print(expression);
  }
}

fn evaluate_from_arguments(arguments: impl Iterator<Item = String>) {
  for argument in arguments {
    evaluate_and_print(argument.as_str());
  }
}

fn evaluate_and_print(expression: &str) {
  let result = evaluate::<f64>(expression);
  match result {
    Ok(value) => println!("{}", value),
    Err(error) => eprintln!("{}", error),
  };
}
