use std::collections::HashSet;

pub mod calc_of_cons;
pub mod cube;
pub mod hindley_milner;
pub mod lambda_omega;
pub mod lambda_p;
pub mod lang;
pub mod simply_typed;
pub mod system_f;
pub mod system_f_sub;
pub mod untyped;

pub fn fresh_var(vs: &HashSet<String>) -> String {
    for i in 0u64.. {
        let var = format!("x{i}");
        if !vs.contains(&var) {
            return var;
        }
    }
    // In the absurd case that all integers are used,
    // this will panic in debug mode and loop infinitely in release mode
    unreachable!()
}
