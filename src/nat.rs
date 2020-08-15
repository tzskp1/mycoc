use self::Nat::*;
use std::sync::Arc;

#[derive(Debug, PartialEq)]
pub enum Nat {
    Zero,
    Succ (Arc<Nat>)
}

pub fn sub<'a> (n : &'a Nat, m : &'a Nat) -> &'a Nat {
    match (n, m) {
        (Succ (ref k), Succ (ref l)) => sub (k, l),
        _ => n
    }
}

pub fn add (n : &Nat, m : Arc<Nat>) -> Arc<Nat> {
    match n {
        Zero => m,
        Succ (ref k) => Arc::new (Succ (add (k, m)))
    }
}

pub fn leq (n : &Nat, m : &Nat) -> bool {
    match sub (n, m) {
        Zero => true,
        _ => false
    }
}
