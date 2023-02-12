use std::{env, fs};

use lambda::system_f::*;

fn main() {
    for term in [
        self_interpret::id_(),
        self_interpret::true_(),
        self_interpret::false_(),
        self_interpret::not_(),
        self_interpret::interpreter(),
    ] {
        let ty = term.type_closed().expect("term should be well-typed");
        let term2 = Term::Application(
            Box::new(Term::TypeApplication(
                Box::new(self_interpret::interpreter()),
                ty,
            )),
            Box::new(self_interpret::shallow_encode(term.clone())),
        );
        assert_eq!(term2.evaluate(), term);
        println!("{}", term);
    }
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
