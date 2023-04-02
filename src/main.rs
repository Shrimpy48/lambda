use std::{env, fs};

use lambda::hindley_milner::*;
use lambda::untyped;

// fn main() {
//     let fact = Term::Application(
//         Box::new(Term::Application(
//             Box::new(Term::Primitive(Prim::Func(PrimFunc::Rec))),
//             Box::new(Term::Product(
//                 "n".into(),
//                 Box::new(Term::Primitive(Prim::Lit(Lit::IntType))),
//                 Box::new(Term::Primitive(Prim::Lit(Lit::IntType))),
//             )),
//         )),
//         Box::new(Term::Abstraction(
//             "f".into(),
//             Box::new(Term::Product(
//                 "n".into(),
//                 Box::new(Term::Primitive(Prim::Lit(Lit::IntType))),
//                 Box::new(Term::Primitive(Prim::Lit(Lit::IntType))),
//             )),
//             Box::new(Term::Abstraction(
//                 "n".into(),
//                 Box::new(Term::Primitive(Prim::Lit(Lit::IntType))),
//                 Box::new(Term::Application(
//                     Box::new(Term::Application(
//                         Box::new(Term::Application(
//                             Box::new(Term::Application(
//                                 Box::new(Term::Primitive(Prim::Func(PrimFunc::If))),
//                                 Box::new(Term::Primitive(Prim::Lit(Lit::IntType))),
//                             )),
//                             Box::new(Term::Application(
//                                 Box::new(Term::Application(
//                                     Box::new(Term::Primitive(Prim::Func(PrimFunc::IntEq))),
//                                     Box::new(Term::Variable("n".into())),
//                                 )),
//                                 Box::new(Term::Primitive(Prim::Lit(Lit::IntLit(0)))),
//                             )),
//                         )),
//                         Box::new(Term::Primitive(Prim::Lit(Lit::IntLit(1)))),
//                     )),
//                     Box::new(Term::Application(
//                         Box::new(Term::Application(
//                             Box::new(Term::Primitive(Prim::Func(PrimFunc::Mul))),
//                             Box::new(Term::Variable("n".into())),
//                         )),
//                         Box::new(Term::Application(
//                             Box::new(Term::Variable("f".into())),
//                             Box::new(Term::Application(
//                                 Box::new(Term::Application(
//                                     Box::new(Term::Primitive(Prim::Func(PrimFunc::Sub))),
//                                     Box::new(Term::Variable("n".into())),
//                                 )),
//                                 Box::new(Term::Primitive(Prim::Lit(Lit::IntLit(1)))),
//                             )),
//                         )),
//                     )),
//                 )),
//             )),
//         )),
//     );
//     let mut term = Term::Application(
//         Box::new(fact),
//         Box::new(Term::Primitive(Prim::Lit(Lit::IntLit(20)))),
//     );
//     let mut i: u32 = 0;
//     println!("{}: {}", i, term);
//     while let Some(t) = term.beta_reduce_lazy() {
//         i += 1;
//         println!("{}: {}", i, t);
//         term = t;
//     }
// }

// fn main() {
//     let pow = Term::Abstraction(
//         "n".into(),
//         Box::new(Term::Primitive(Prim::Lit(Lit::IntType))),
//         Box::new(Term::Abstraction(
//             "k".into(),
//             Box::new(Term::Primitive(Prim::Lit(Lit::IntType))),
//             Box::new(Term::Application(
//                 Box::new(Term::Application(
//                     Box::new(Term::Application(
//                         Box::new(Term::Application(
//                             Box::new(Term::Primitive(Prim::Func(PrimFunc::For))),
//                             Box::new(Term::Primitive(Prim::Lit(Lit::IntType))),
//                         )),
//                         Box::new(Term::Variable("k".into())),
//                     )),
//                     Box::new(Term::Application(
//                         Box::new(Term::Primitive(Prim::Func(PrimFunc::Mul))),
//                         Box::new(Term::Variable("n".into())),
//                     )),
//                 )),
//                 Box::new(Term::Primitive(Prim::Lit(Lit::IntLit(1)))),
//             )),
//         )),
//     );
//     let mut term = Term::Application(
//         Box::new(Term::Application(
//             Box::new(pow),
//             Box::new(Term::Primitive(Prim::Lit(Lit::IntLit(5)))),
//         )),
//         Box::new(Term::Primitive(Prim::Lit(Lit::IntLit(3)))),
//     );
//     let mut i: u32 = 0;
//     println!("{}: {}", i, term);
//     while let Some(t) = term.beta_reduce_lazy() {
//         i += 1;
//         println!("{}: {}", i, t);
//         term = t;
//     }
// }

fn main() {
    let term =
        untyped::parser::parse(fs::read_to_string(env::args().nth(1).unwrap()).unwrap()).unwrap();
    let mut term: Term = term.into();
    println!("{} : {}", term, term.type_closed().unwrap());
    // let mut i: u32 = 0;
    // println!("{}: {}", i, term);
    // while let Some(t) = term.beta_reduce_lazy() {
    //     i += 1;
    //     println!("{}: {}", i, t);
    //     term = t;
    // }
}
