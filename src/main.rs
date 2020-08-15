use std::sync::Arc;
use mycoc::nat::*;
use mycoc::nat::Nat::*;
use self::PseudoTerm::*;

enum PseudoTerm {
    Box,
    Star,
    Var (Arc<Nat>),
    Lambda (Arc<PseudoTerm>, Arc<PseudoTerm>),
    Pi (Arc<PseudoTerm>, Arc<PseudoTerm>),
    App (Arc<PseudoTerm>, Arc<PseudoTerm>)
}

// fn shift(m : &PseudoTerm, n : &Nat, c : &Nat) -> Arc<PseudoTerm> {
//     match m {
//         Star => Arc::new (Star),
//         Box => Arc::new (Box),
//         Var (ref p) =>
//             if leq (c, p) {
//                 Arc::new (Var (add (n, p)))
//             } else {
//                 Arc::new (*m)
//             },
//         _ => Arc::new (Star)
//     }
// }

fn main() {
    println!("{:?}", Succ (Arc::new (Zero)));
    println!("{:?}", leq (&Arc::new (Nat::Zero), &Arc::new (Nat::Succ (Arc::new (Nat::Zero)))));
    println!("{:?}", add (&Nat::Zero, Arc::new (Nat::Succ (Arc::new (Nat::Zero)))));
}
