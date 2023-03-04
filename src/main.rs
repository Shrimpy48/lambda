use std::{env, fs};

use lambda::untyped::*;

fn main() {
    let f = application(
        application(
            application(
                variable("_"),
                application(variable("__8p0e"), variable("la2_73__ri__4zrlid")),
            ),
            variable("b7xn88754v_"),
        ),
        abstraction(
            "o1_e__t",
            abstraction(
                "_tm920t_tuemz__4__stpf1__83__",
                abstraction(
                    "_7f_7_4acg213__3c0lx__",
                    variable("__6_7tlml74_z57urt20_0lz96h"),
                ),
            ),
        ),
    );
    println!("{f}");
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
