extern crate nom;
use std::sync::Arc;
use nom::{
  IResult,
  bytes::complete::{tag, take_while},
  combinator::map_res,
  sequence::tuple
};
use crate::term::PseudoTerm;
use crate::nat::{Nat, mul, add, nat_of_u64};

fn is_digit(c: char) -> bool {
  c.is_digit(10)
}

fn from_digits(input: &str) -> Result<PseudoTerm, String> {
  let mut ret = Arc::new (Nat::Zero);
  for ch in input.chars() {
    let ch : u64 = ch.to_digit(10).ok_or("")?.into();
    ret = add (&nat_of_u64 (ch), mul (&ret, nat_of_u64 (10)));
  }
  return Ok (PseudoTerm::Var (ret));
}

pub fn var(input: &str) -> IResult<&str, PseudoTerm> {
  map_res(take_while(is_digit), from_digits)(input)
}

pub fn star(input: &str) -> IResult<&str, PseudoTerm> {
  let (input, _) = tag("star")(input)?;
  return Ok ((input, PseudoTerm::Star))
}

pub fn square(input: &str) -> IResult<&str, PseudoTerm> {
  let (input, _) = tag("square")(input)?;
  return Ok ((input, PseudoTerm::Square))
}

pub fn term(input: &str) -> IResult<&str, PseudoTerm> {
  return Ok ((input, PseudoTerm::Square))
}

// fn fun(input: &str) -> IResult<&str, Color> {
//   let (input, _) = tag("#")(input)?;
//   let (input, (red, green, blue)) = tuple((hex_primary, hex_primary, hex_primary))(input)?;

//   Ok((input, Color { red, green, blue }))
// }


// tag("fun")
