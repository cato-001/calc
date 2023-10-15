use derive_more::{Display, Error};

#[derive(Debug, Display, Error, PartialEq, Eq)]
#[display("SyntaxError({value})")]
pub struct SyntaxError {
  value: String,
}

impl SyntaxError {
  pub fn new(value: String) -> Self {
    Self { value }
  }
}

impl From<nom::error::Error<&str>> for SyntaxError {
  fn from(error: nom::error::Error<&str>) -> Self {
    Self::new(format!("{}", error))
  }
}
