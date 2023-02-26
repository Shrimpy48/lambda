use std::{env, fs};

use lambda::lambda_omega::*;

fn main() {
    let pair = Type::Abstraction(
        "x".into(),
        Kind::Type,
        Box::new(Type::Abstraction(
            "y".into(),
            Kind::Type,
            Box::new(Type::Fn(
                Box::new(Type::Fn(
                    Box::new(Type::Variable("x".into())),
                    Box::new(Type::Fn(
                        Box::new(Type::Variable("y".into())),
                        Box::new(Type::Base),
                    )),
                )),
                Box::new(Type::Base),
            )),
        )),
    );
    let make_pair = Term::Abstraction(
        "x".into(),
        Type::Variable("a".into()),
        Box::new(Term::Abstraction(
            "y".into(),
            Type::Variable("b".into()),
            Box::new(Term::Abstraction(
                "f".into(),
                Type::Fn(
                    Box::new(Type::Variable("a".into())),
                    Box::new(Type::Fn(
                        Box::new(Type::Variable("b".into())),
                        Box::new(Type::Variable("c".into())),
                    )),
                ),
                Box::new(Term::Application(
                    Box::new(Term::Application(
                        Box::new(Term::Variable("f".into())),
                        Box::new(Term::Variable("x".into())),
                    )),
                    Box::new(Term::Variable("y".into())),
                )),
            )),
        )),
    );
    println!("{}", pair);
    println!("{}", make_pair);
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
