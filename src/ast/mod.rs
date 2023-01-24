use std::collections::HashSet;

pub mod untyped;
pub mod simply_typed;

fn fresh_var(vs: &HashSet<String>) -> String {
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
