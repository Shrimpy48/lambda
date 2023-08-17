use std::{env, fs};

use lambda::calc_of_cons::infer::*;

fn main() {
    let term = parser::parse(fs::read_to_string(env::args().nth(1).unwrap()).unwrap()).unwrap();
    let ty = term.synthesise_type_closed().unwrap();
    println!("{}", Term::Annotation(Box::new(term), Box::new(ty)));
}
