extern crate nom;
use crate::nat::{add, mul, nat_of_u64, Nat};
use crate::term::PseudoTerm;
use crate::term::PseudoTerm::{App, Lambda, Pi, Square, Star, Var};
use nom::{
    branch::alt,
    bytes::complete::{tag, take_while},
    character::complete::{space0, space1},
    combinator::map_res,
    IResult,
};
use std::sync::Arc;

fn is_digit(c: char) -> bool {
    c.is_digit(10)
}

fn from_digits(input: &str) -> Result<PseudoTerm, String> {
    let mut ret = Arc::new(Nat::Zero);
    if input.trim().is_empty() {
        return Err("empty".to_string());
    }
    for ch in input.chars() {
        let ch: u64 = ch.to_digit(10).ok_or("")?.into();
        ret = add(&nat_of_u64(ch), mul(&ret, nat_of_u64(10)));
    }
    return Ok(Var(ret));
}

pub fn var(input: &str) -> IResult<&str, PseudoTerm> {
    map_res(take_while(is_digit), from_digits)(input)
}

pub fn star(input: &str) -> IResult<&str, PseudoTerm> {
    let (input, _) = tag("star")(input)?;
    return Ok((input, Star));
}

pub fn square(input: &str) -> IResult<&str, PseudoTerm> {
    let (input, _) = tag("square")(input)?;
    return Ok((input, Square));
}

fn rotate_app(t: &PseudoTerm) -> PseudoTerm {
    match t {
        App(l, r) => match &**r {
            App(rl, rr) => App(
                Arc::new(rotate_app(&App(l.clone(), rl.clone()))),
                rr.clone(),
            ),
            _ => t.clone(),
        },
        _ => t.clone(),
    }
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
        return Ok((input, Lambda(Arc::new(ty), Arc::new(body))));
    }
    fn forall(input: &str) -> IResult<&str, PseudoTerm> {
        let (input, _) = tag("forall")(input)?;
        let (input, _) = space0(input)?;
        let (input, ty) = term(input)?;
        let (input, _) = space0(input)?;
        let (input, _) = tag(",")(input)?;
        let (input, _) = space0(input)?;
        let (input, body) = term(input)?;
        return Ok((input, Pi(Arc::new(ty), Arc::new(body))));
    }
    fn with_paren(input: &str) -> IResult<&str, PseudoTerm> {
        let (input, _) = tag("(")(input)?;
        let (input, _) = space0(input)?;
        let (input, t) = term(input)?;
        let (input, _) = space0(input)?;
        let (input, _) = tag(")")(input)?;
        return Ok((input, t));
    }
    fn right_app<'a>(left: &PseudoTerm, input: &'a str) -> IResult<&'a str, PseudoTerm> {
        let (input, _) = space1(input)?;
        match term(input) {
            Ok((input, right)) => Ok((
                input,
                rotate_app(&App(Arc::new(left.clone()), Arc::new(right))),
            )),
            Err(_) => {
                let (input, right) = with_paren(input)?;
                return Ok((input, App(Arc::new(left.clone()), Arc::new(right))));
            }
        }
    }
    fn with_paren2(input: &str) -> IResult<&str, PseudoTerm> {
        let (input, _) = tag("(")(input)?;
        let (input, _) = space0(input)?;
        let (input, t) = alt((forall, fun, var, star, square))(input)?;
        let (input, _) = space0(input)?;
        let (input, _) = tag(")")(input)?;
        return Ok((input, t));
    }
    let (input, left) = alt((with_paren2, forall, fun, var, star, square))(input)?;
    match right_app(&left, input) {
        Ok((input, t)) => Ok((input, t)),
        _ => Ok((input, left)),
    }
}
