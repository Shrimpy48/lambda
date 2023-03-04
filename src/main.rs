use std::{env, fs};

use lambda::calc_of_cons::*;

fn main() {
    let s = Term::Abstraction(
        "a".into(),
        Box::new(Term::Sort(Sort::Type)),
        Box::new(Term::Abstraction(
            "b".into(),
            Box::new(Term::Sort(Sort::Type)),
            Box::new(Term::Abstraction(
                "c".into(),
                Box::new(Term::Sort(Sort::Type)),
                Box::new(Term::Abstraction(
                    "x".into(),
                    Box::new(Term::Product(
                        "x".into(),
                        Box::new(Term::Variable("a".into())),
                        Box::new(Term::Product(
                            "x".into(),
                            Box::new(Term::Variable("b".into())),
                            Box::new(Term::Variable("c".into())),
                        )),
                    )),
                    Box::new(Term::Abstraction(
                        "y".into(),
                        Box::new(Term::Product(
                            "x".into(),
                            Box::new(Term::Variable("a".into())),
                            Box::new(Term::Variable("b".into())),
                        )),
                        Box::new(Term::Abstraction(
                            "z".into(),
                            Box::new(Term::Variable("a".into())),
                            Box::new(Term::Application(
                                Box::new(Term::Application(
                                    Box::new(Term::Variable("x".into())),
                                    Box::new(Term::Variable("z".into())),
                                )),
                                Box::new(Term::Application(
                                    Box::new(Term::Variable("y".into())),
                                    Box::new(Term::Variable("z".into())),
                                )),
                            )),
                        )),
                    )),
                )),
            )),
        )),
    );
    let string = format!("{s}");
    println!("{string}");
    let s2 = parser::parse(string).unwrap();
    assert_eq!(s2, s);
}

// fn main() {
//     let mut term = parser::parse(fs::read_to_string(env::args().nth(1).unwrap()).unwrap()).unwrap();
//     let mut i: u32 = 0;
//     println!("{}: {}", i, term);
//     while let Some(t) = term.beta_reduce_lazy() {
//         i += 1;
//         println!("{}: {}", i, t);
//         term = t;
//     }
// }
