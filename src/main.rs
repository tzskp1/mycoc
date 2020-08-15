use std::sync::Arc;
use mycoc::nat::*;
use mycoc::nat::Nat::*;
use self::PseudoTerm::*;

#[derive(Debug, PartialEq, Clone)]
enum PseudoTerm {
    Square,
    Star,
    Var (Arc<Nat>),
    Lambda (Arc<PseudoTerm>, Arc<PseudoTerm>),
    Pi (Arc<PseudoTerm>, Arc<PseudoTerm>),
    App (Arc<PseudoTerm>, Arc<PseudoTerm>)
}

fn shift (m : &PseudoTerm, n : &Nat, c : &Nat) -> Arc<PseudoTerm> {
    match m {
        Star => Arc::new (Star),
        Square => Arc::new (Square),
        Var (p) =>
            if leq (c, &p) {
                Arc::new (Var (add (n, p.clone())))
            } else {
                Arc::new (Var (p.clone()))
            },
        App (p, q) =>
            Arc::new (App (shift (&p, n, c), shift (&q, n, c))),
        Pi (p, q) =>
            Arc::new (Pi (shift (&p, n, c), shift (&q, n, &add (c, Arc::new (Succ (Arc::new (Zero))))))),
        Lambda (p, q) =>
            Arc::new (Lambda (shift (&p, n, c), shift (&q, n, &add (c, Arc::new (Succ (Arc::new (Zero))))))),
    }
}

fn subst (m : &PseudoTerm, n : Arc<PseudoTerm>, v : &Nat) -> Arc<PseudoTerm> {
    match m {
        Var (p) =>
            if eq (&p, v) {
                n
            } else if leq (&p, v) {
                Arc::new (m.clone())
            } else {
                Arc::new (Var (sub_arc (p.clone(), Arc::new (Succ (Arc::new (Zero))))))
            },
        Star => Arc::new (Star),
        Square => Arc::new (Square),
        Pi (p, q) =>
            Arc::new (Pi (subst (p, shift (&n, &(Succ (Arc::new (Zero))), &Zero), v),
                          subst (q, shift (&n, &(Succ (Arc::new (Zero))), &Zero),
                                 &add (v, Arc::new (Succ (Arc::new (Zero))))))),
        Lambda (p, q) =>
            Arc::new (Lambda (subst (p, shift (&n, &(Succ (Arc::new (Zero))), &Zero), v),
                              subst (q, shift (&n, &(Succ (Arc::new (Zero))), &Zero),
                                     &add (v, Arc::new (Succ (Arc::new (Zero))))))),
        App (p, q) =>
            Arc::new (App (subst (p, n.clone(), v), subst (q, n, v))),
    }
}

fn main() {
    println!("{:?}", Succ (Arc::new (Zero)));
    println!("{:?}", leq (&Arc::new (Nat::Zero), &Arc::new (Nat::Succ (Arc::new (Nat::Zero)))));
    println!("{:?}", add (&Nat::Zero, Arc::new (Nat::Succ (Arc::new (Nat::Zero)))));
}
