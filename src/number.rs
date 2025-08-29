use nom::character::complete::{i128, i16, i32, i64, i8, u128, u16, u32, u64, u8};
use nom::IResult;
use std::ops::{Add, Div, Mul, Sub};
use nom::number::complete::{double, float};

pub trait ParsableNumber
where
  Self: Sized + Add<Output = Self> + Sub<Output = Self> + Mul<Output = Self> + Div<Output = Self>,
{
  fn parse(input: &str) -> IResult<&str, Self>;
}

macro_rules! parsable_number {
  ($number:ty, $parse:expr) => {
    impl ParsableNumber for $number {
      fn parse(input: &str) -> IResult<&str, Self> {
        $parse(input)
      }
    }
  };
}

parsable_number!(i8, i8);
parsable_number!(i16, i16);
parsable_number!(i32, i32);
parsable_number!(i64, i64);
parsable_number!(i128, i128);
parsable_number!(u8, u8);
parsable_number!(u16, u16);
parsable_number!(u32, u32);
parsable_number!(u64, u64);
parsable_number!(u128, u128);
parsable_number!(f32, float);
parsable_number!(f64, double);
