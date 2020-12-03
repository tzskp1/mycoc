use super::nat::Nat::{Succ, Zero};
use super::term::PseudoTerm;
use super::term::PseudoTerm::{App, Lambda, Pi, Square, Star, Var};
use super::term::{shift, subst};
use std::sync::Arc;

// pub fn beta1(p: &PseudoTerm) -> Arc<PseudoTerm> {
//     match p {
//         App(x, n) => {
//             match &**x {
//                 Lambda(t, m) => {
//                     return subst(m, n.clone(), &Zero);
//                 }
//                 _ => {
//                     return Arc::new(Star);
//                 }
//             }
//         }
//     }
// }

// pub fn full_beta(t: &PseudoTerm) -> Arc<PseudoTerm> {
//     return Star;
// }
