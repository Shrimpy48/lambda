use std::{env, fs};

use lambda::calc_of_cons::*;

fn main() {
        let xx = Term::Abstraction(
            "x".into(),
            Box::new(Term::Product(
                "a".into(),
                Box::new(Term::Sort(Sort::Type)),
                Box::new(Term::Product(
                    "x".into(),
                    Box::new(Term::Variable("a".into())),
                    Box::new(Term::Variable("a".into())),
                )),
            )),
            Box::new(Term::Application(
                Box::new(Term::Application(
                    Box::new(Term::Variable("x".into())),
                    Box::new(Term::Product(
                        "a".into(),
                        Box::new(Term::Sort(Sort::Type)),
                        Box::new(Term::Product(
                            "x".into(),
                            Box::new(Term::Variable("a".into())),
                            Box::new(Term::Variable("a".into())),
                        )),
                    )),
                )),
                Box::new(Term::Variable("x".into())),
            )),
        );
        let expected = Term::Product(
            "x".into(),
            Box::new(Term::Product(
                "a".into(),
                Box::new(Term::Sort(Sort::Type)),
                Box::new(Term::Product(
                    "x".into(),
                    Box::new(Term::Variable("a".into())),
                    Box::new(Term::Variable("a".into())),
                )),
            )),
            Box::new(Term::Product(
                "a".into(),
                Box::new(Term::Sort(Sort::Type)),
                Box::new(Term::Product(
                    "x".into(),
                    Box::new(Term::Variable("a".into())),
                    Box::new(Term::Variable("a".into())),
                )),
            )),
        );
        println!("{}", xx);
        println!("{}", expected);
        assert_eq!(xx.type_closed(), Some(expected));
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
