use super::nat::Nat::{Succ, Zero};
use super::term::PseudoTerm::{App, Lambda, Pi, Square, Star, Var};
use super::term::{shift, subst, PseudoTerm};
use std::sync::Arc;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Context {
    Cons {
        head: PseudoTerm,
        rest: Arc<Context>,
    },
    Nil,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Judgement {
    context: Context,
    term: PseudoTerm,
    r#type: PseudoTerm,
}

pub enum ProofTree {
    Ax(Judgement),
    Var(Arc<ProofTree>, Judgement),
    Weak(Arc<ProofTree>, Arc<ProofTree>, Judgement),
    Pi(Arc<ProofTree>, Arc<ProofTree>, Judgement),
    Lambda(Arc<ProofTree>, Arc<ProofTree>, Judgement),
    App(Arc<ProofTree>, Arc<ProofTree>, Judgement),
    Conv(Arc<ProofTree>, Arc<ProofTree>, Judgement),
}

pub fn consequence(t: &ProofTree) -> &Judgement {
    match t {
        ProofTree::Ax(j) => j,
        ProofTree::Var(_, j) => j,
        ProofTree::Weak(_, _, j) => j,
        ProofTree::Pi(_, _, j) => j,
        ProofTree::Lambda(_, _, j) => j,
        ProofTree::App(_, _, j) => j,
        ProofTree::Conv(_, _, j) => j,
    }
}

pub fn is_valid(t: &ProofTree) -> bool {
    match t {
        ProofTree::Ax(j) => j.context == Context::Nil && j.term == Star && j.r#type == Square,
        ProofTree::Var(p, c) => match &c.context {
            Context::Nil => false,
            Context::Cons { head: h, rest: r } => {
                let pc = consequence(p);
                is_valid(p)
                    && c.term == Var(Arc::new(Zero))
                    && c.r#type == *shift(&pc.term, &Succ(Arc::new(Zero)), &Zero)
                    && pc.context == **r
                    && *h == pc.term
                    && (pc.r#type == Star || pc.r#type == Square)
            }
        },
        ProofTree::Weak(p1, p2, c) => match &c.context {
            Context::Nil => false,
            Context::Cons { head: h, rest: r } => {
                let pc1 = consequence(p1);
                let pc2 = consequence(p2);
                is_valid(p1)
                    && is_valid(p2)
                    && pc1.context == **r
                    && pc2.context == **r
                    && (pc1.r#type == Star || pc1.r#type == Square)
                    && *h == pc1.term
                    && c.term == *shift(&pc2.term, &Succ(Arc::new(Zero)), &Zero)
                    && c.r#type == *shift(&pc2.r#type, &Succ(Arc::new(Zero)), &Zero)
            }
        },
        ProofTree::Pi(p1, p2, c) => {
            let pc1 = consequence(p1);
            let pc2 = consequence(p2);
            match &pc2.context {
                Context::Nil => false,
                Context::Cons { head: h, rest: r } => {
                    is_valid(p1)
                        && is_valid(p2)
                        && c.context == **r
                        && pc1.context == **r
                        && *h == pc1.term
                        && (pc1.r#type == Star || pc1.r#type == Square)
                        && pc2.r#type == c.r#type
                        && (pc2.r#type == Star || pc2.r#type == Square)
                        && c.term == Pi(Arc::new(h.clone()), Arc::new(pc2.term.clone()))
                }
            }
        }
        ProofTree::Lambda(p1, p2, c) => {
            let pc1 = consequence(p1);
            let pc2 = consequence(p2);
            match &pc2.context {
                Context::Nil => false,
                Context::Cons { head: h, rest: r } => {
                    is_valid(p1)
                        && is_valid(p2)
                        && c.context == **r
                        && pc1.context == **r
                        && pc1.term == Pi(Arc::new(h.clone()), Arc::new(pc2.r#type.clone()))
                        && (pc1.r#type == Star || pc1.r#type == Square)
                        && c.r#type == Pi(Arc::new(h.clone()), Arc::new(pc2.r#type.clone()))
                        && c.term == Lambda(Arc::new(h.clone()), Arc::new(pc2.term.clone()))
                }
            }
        }
        ProofTree::App(p1, p2, c) => {
            let pc1 = consequence(p1);
            let pc2 = consequence(p2);
            match pc1.r#type {
                Pi(ref t, ref u) => {
                    c.context == pc1.context
                        && c.context == pc2.context
                        && is_valid(p1)
                        && is_valid(p2)
                        && c.r#type == *subst(&*u, Arc::new(pc2.term.clone()), &Zero)
                        && pc2.r#type == **t
                        && c.term == App(Arc::new(pc1.term.clone()), Arc::new(pc2.term.clone()))
                }
                _ => false,
            }
        }
        ProofTree::Conv(p1, p2, c) => {
            let pc1 = consequence(p1);
            let pc2 = consequence(p2);
            is_valid(p1)
                && is_valid(p2)
                && (pc1.r#type == Star || pc1.r#type == Square)
                && c.context == pc1.context
                && c.context == pc2.context
                && pc2.term == c.term
                && c.r#type == pc1.term
            // todo: conversion test
        }
    }
}

// pub fn is_valid (j: &Judgement) -> bool {
//     match (&j.term, &j.r#type) {
//         (Star, Square) => {
//             return j.context.is_empty();
//         }
//         (Var(d), _) => {
//             **d == super::nat::Nat::Zero
//         }
//         _ => {
//             dbg!(&j.context);
//             return true;
//         }
//     }
// }

// #[test]
// fn validate() {
//     assert_eq!(is_valid(&Judgement { context: vec![], term: Star, r#type: Square }), true);
//     dbg!(is_valid(&Judgement { context: vec![], term: Star, r#type: Star }));
// }
