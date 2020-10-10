extern crate nom;
use std::sync::Arc;
use nom::{
  IResult,
  bytes::complete::{tag, take_while},
  character::complete::{space0, space1},
  branch::alt,
  combinator::map_res
};
use crate::term::PseudoTerm;
use crate::nat::{Nat, mul, add, nat_of_u64};

fn is_digit(c: char) -> bool {
  c.is_digit(10)
}

fn from_digits(input: &str) -> Result<PseudoTerm, String> {
  let mut ret = Arc::new (Nat::Zero);
  if input.trim().is_empty() {
    return Err("empty".to_string())
  }
  for ch in input.chars() {
    let ch : u64 = ch.to_digit(10).ok_or("")?.into();
    ret = add (&nat_of_u64 (ch), mul (&ret, nat_of_u64 (10)));
  }
  return Ok (PseudoTerm::Var (ret))
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
  fn fun(input: &str) -> IResult<&str, PseudoTerm> {
    let (input, _) = tag("\\")(input)?;
    let (input, _) = space0(input)?;
    let (input, ty) = term(input)?;
    let (input, _) = space0(input)?;
    let (input, _) = tag("->")(input)?;
    let (input, _) = space0(input)?;
    let (input, body) = term(input)?;
    return Ok ((input, PseudoTerm::Lambda (Arc::new (ty), Arc::new (body))))
  }
  fn forall(input: &str) -> IResult<&str, PseudoTerm> {
    let (input, _) = tag("forall")(input)?;
    let (input, _) = space0(input)?;
    let (input, ty) = term(input)?;
    let (input, _) = space0(input)?;
    let (input, _) = tag(",")(input)?;
    let (input, _) = space0(input)?;
    let (input, body) = term(input)?;
    return Ok ((input, PseudoTerm::Lambda (Arc::new (ty), Arc::new (body))))
  }
  fn right_app(input: &str) -> IResult<&str, PseudoTerm> {
    let (input, _) = space1(input)?;
    return term(input)
  }
  let (input, left) = alt ((forall, fun, var, star, square))(input)?;
  match right_app (input) {
    Ok((input, right)) => Ok ((input, PseudoTerm::App (Arc::new (left), Arc::new (right)))),
    _ => Ok ((input, left))
  }
}
