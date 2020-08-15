use self::Nat::*;
use std::sync::Arc;

#[derive(Debug, PartialEq, Clone)]
pub enum Nat {
    Zero,
    Succ (Arc<Nat>)
}

pub fn sub_arc (n : Arc<Nat>, m : Arc<Nat>) -> Arc<Nat> {
    match (&*n, &*m) {
        (Succ (k), Succ (l)) => sub_arc (k.clone(), l.clone()),
        _ => n
    }
}

pub fn sub<'a> (n : &'a Nat, m : &'a Nat) -> &'a Nat {
    match (n, m) {
        (Succ (k), Succ (l)) => sub (k, l),
        _ => n
    }
}

pub fn add (n : &Nat, m : Arc<Nat>) -> Arc<Nat> {
    match n {
        Zero => m,
        Succ (k) => Arc::new (Succ (add (k, m)))
    }
}

pub fn leq (n : &Nat, m : &Nat) -> bool {
    match sub (n, m) {
        Zero => true,
        _ => false
    }
}

pub fn eq (n : &Nat, m : &Nat) -> bool {
    match (n, m) {
        (Zero, Zero) => true,
        (Succ (k), Succ (l)) => eq (k, l),
        _ => false
    }
}
