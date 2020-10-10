use self::Nat::{Zero, Succ};
use std::sync::Arc;
use std::fmt;

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

pub fn mul (n : &Nat, m : Arc<Nat>) -> Arc<Nat> {
  match n {
    Zero => Arc::new (Zero),
    Succ (k) => add (&(m.clone()), mul (k, m))
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

pub fn u64_of_nat (n : &Nat) -> u64 {
  match n {
    Zero => 0,
    Succ (k) => u64_of_nat (&k) + 1
  }
}

pub fn nat_of_u64 (x : u64) -> Arc<Nat> {
  if x == 0 {
    Arc::new (Zero)
  } else {
    Arc::new (Succ (nat_of_u64 (x - 1)))
  }
}

impl fmt::Display for Nat {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}", u64_of_nat (self))
  }
}
