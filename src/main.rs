use std::{env, fs};

use lambda::untyped::*;

fn main() {
    let mut i: u32 = 0;
    let mut term = parser::parse(fs::read_to_string(env::args().nth(1).unwrap()).unwrap()).unwrap();
    // let mut term = de_bruijn::Term::try_from(term.clone()).unwrap();
    println!("{}: {}", i, term);
    while let Some(t) = term.beta_reduce_lazy() {
        i += 1;
        println!("{}: {}", i, t);
        term = t;
    }
}
