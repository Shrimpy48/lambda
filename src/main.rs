use std::{env, fs};

use lambda::untyped::*;

fn main() {
    let mut term = parser::parse(fs::read_to_string(env::args().nth(1).unwrap()).unwrap()).unwrap();
    let mut i: u32 = 0;
    println!("{}: {}", i, term);
    while let Some(t) = term.beta_reduce_lazy() {
        i += 1;
        println!("{}: {}", i, t);
        term = t;
    }
}
