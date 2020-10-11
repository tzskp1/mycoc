use mycoc::nat::Nat::*;
use mycoc::nat::*;
use mycoc::parser::*;
use std::sync::Arc;

fn main() {
    println!("{}", Succ(Arc::new(Zero)));
    println!(
        "{}",
        leq(
            &Arc::new(Nat::Zero),
            &Arc::new(Nat::Succ(Arc::new(Nat::Zero)))
        )
    );
    println!(
        "{}",
        add(&Nat::Zero, Arc::new(Nat::Succ(Arc::new(Nat::Zero))))
    );
    println!("{}", nat_of_u64(33));
    // println!("{:?}", var("3999t"));
    println!("{:?}", term("square test"));
    println!("{:?}", term("\\ 1 -> 2"));
    println!("{:?}", term("\\ 1 0 -> 3"));
    println!("{:?}", term("\\ square -> 3"));
    println!("{:?}", term("\\ square -> star"));
    println!("{:?}", term("0 1 2 3"));
    println!("{:?}", term("(0 1) 2 3"));
    println!("{:?}", term("0 1 (2 3)"));
    println!("{:?}", term("0 (1 (2 3))"));
    println!("{:?}", term("((0 1) 2) 3"));
}
