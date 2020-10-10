use std::sync::Arc;
use mycoc::nat::*;
use mycoc::parser::*;
use mycoc::nat::Nat::*;

fn main() {
  println!("{}", Succ (Arc::new (Zero)));
  println!("{}", leq (&Arc::new (Nat::Zero), &Arc::new (Nat::Succ (Arc::new (Nat::Zero)))));
  println!("{}", add (&Nat::Zero, Arc::new (Nat::Succ (Arc::new (Nat::Zero)))));
  println!("{}", nat_of_u64 (33));
  println!("{:?}", var ("3999t"));
  println!("{:?}", square ("square test"));
}
