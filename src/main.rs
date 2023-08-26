use std::{env, fs};

use lambda::untyped::*;

fn main() {
    let term = parser::parse(fs::read_to_string(env::args().nth(1).unwrap()).unwrap()).unwrap();
    println!("{}", term);
    println!("{:#?}", compile::compile_toplevel(&term));
}
