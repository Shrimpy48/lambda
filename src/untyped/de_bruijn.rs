use std::collections::{HashSet, VecDeque};
use std::fmt;

#[cfg(test)]
use proptest::prelude::*;

/// A lambda calculus term using De Bruijn indexing.
/// Unlike the usual convention, this is zero-indexed
/// (so an index of 0 binds to the first enclosing abstraction).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Term {
    Variable(u64),
    Abstraction(Box<Term>),
    Application(Box<Term>, Box<Term>),
}

impl Term {
    // This could use a bitset and shift instead for big fast (but smol range)
    pub fn free_vars(&self) -> HashSet<u64> {
        match self {
            Self::Variable(v) => {
                let mut vars = HashSet::new();
                vars.insert(*v);
                vars
            }
            Self::Abstraction(t) => t
                .free_vars()
                .into_iter()
                .filter_map(|v| v.checked_sub(1))
                .collect(),
            Self::Application(t, u) => {
                let mut vars = t.free_vars();
                vars.extend(u.free_vars());
                vars
            }
        }
    }

    /// Substitute following the formal definition, where [to] is an unbounded list
    /// of terms to substitute - the ith term for the ith free variable.
    /// Since Rust is strict, the unbounded list is represented by a function from
    /// indices to values.
    pub fn substitute(&self, to: &dyn Fn(u64) -> Term) -> Self {
        match self {
            Self::Variable(v) => to(*v),
            Self::Abstraction(t) => {
                fn shift1(i: u64) -> Term {
                    Term::Variable(i + 1)
                }
                let new_to = |i| match i {
                    0 => Term::Variable(0),
                    _ => to(i - 1).substitute(&shift1),
                };
                Self::Abstraction(Box::new(t.substitute(&new_to)))
            }
            Self::Application(t, u) => {
                Self::Application(Box::new(t.substitute(&to)), Box::new(u.substitute(to)))
            }
        }
    }

    /// Beta-reduce the left-outermost redex if one exists ("normal order").
    pub fn beta_reduce_lazy(&self) -> Option<Self> {
        match self {
            Self::Application(t, u) => {
                if let Self::Abstraction(b) = t.as_ref() {
                    Some(b.substitute(&|i| {
                        if i == 0 {
                            u.as_ref().clone()
                        } else {
                            Term::Variable(i)
                        }
                    }))
                } else if let Some(t2) = t.beta_reduce_lazy() {
                    Some(Self::Application(
                        Box::new(t2),
                        Box::new(u.as_ref().clone()),
                    ))
                } else if let Some(u2) = u.beta_reduce_lazy() {
                    Some(Self::Application(
                        Box::new(t.as_ref().clone()),
                        Box::new(u2),
                    ))
                } else {
                    None
                }
            }
            Self::Abstraction(t) => t
                .beta_reduce_lazy()
                .map(|t2| Self::Abstraction(Box::new(t2))),
            _ => None,
        }
    }

    /// Beta-reduce all redexes simultaneously if any exist.
    pub fn parallel_reduct(&self) -> Option<Self> {
        match self {
            Self::Application(t, u) => {
                if let Self::Abstraction(b) = t.as_ref() {
                    let b2 = b.parallel_reduct().unwrap_or_else(|| b.as_ref().clone());
                    let u2 = u.parallel_reduct().unwrap_or_else(|| u.as_ref().clone());
                    Some(b2.substitute(&|i| {
                        if i == 0 {
                            u2.clone()
                        } else {
                            Term::Variable(i)
                        }
                    }))
                } else {
                    match (t.parallel_reduct(), u.parallel_reduct()) {
                        (None, None) => None,
                        (t2, u2) => {
                            let t2 = t2.unwrap_or_else(|| t.as_ref().clone());
                            let u2 = u2.unwrap_or_else(|| u.as_ref().clone());

                            Some(Self::Application(Box::new(t2), Box::new(u2)))
                        }
                    }
                }
            }
            Self::Abstraction(t) => t
                .parallel_reduct()
                .map(|t2| Self::Abstraction(Box::new(t2))),
            _ => None,
        }
    }

    /// Eta-reduce the left-outermost simplification if one exists.
    pub fn eta_reduce_lazy(&self) -> Option<Self> {
        match self {
            Self::Abstraction(t) => {
                if let Self::Application(f, u) = t.as_ref() {
                    if let Self::Variable(y) = u.as_ref() {
                        if *y == 1 && !f.free_vars().contains(&1) {
                            return Some(f.substitute(&|i| Term::Variable(i - 1)));
                        }
                    }
                }
                t.eta_reduce_lazy()
                    .map(|u2| Self::Abstraction(Box::new(u2)))
            }
            Self::Application(t, u) => {
                if let Some(t2) = t.eta_reduce_lazy() {
                    Some(Self::Application(
                        Box::new(t2),
                        Box::new(u.as_ref().clone()),
                    ))
                } else if let Some(u2) = u.eta_reduce_lazy() {
                    Some(Self::Application(
                        Box::new(t.as_ref().clone()),
                        Box::new(u2),
                    ))
                } else {
                    None
                }
            }
            _ => None,
        }
    }

    /// Fully beta- and eta-reduce the term. Since the lambda calculus is Turing-complete,
    /// this may not halt.
    pub fn evaluate(&self) -> Self {
        let mut res = self.clone();
        while let Some(new_res) = res.beta_reduce_lazy() {
            res = new_res;
        }
        while let Some(new_res) = res.eta_reduce_lazy() {
            res = new_res;
        }
        res
    }
}

#[cfg(test)]
struct TermGen {
    func: Box<dyn Fn(u64) -> Term>,
}

#[cfg(test)]
impl TermGen {
    fn new<F: Fn(u64) -> Term + 'static>(func: F) -> Self {
        Self {
            func: Box::new(func),
        }
    }
}

#[cfg(test)]
impl fmt::Debug for TermGen {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "<func>")
    }
}

/// Higher-order shenanigans to achieve a stateful recursive Strategy.
#[cfg(test)]
fn arb_term_with() -> impl Strategy<Value = TermGen> {
    let leaf = any::<prop::sample::Index>().prop_map(|i| {
        TermGen::new(move |n| {
            if n == 0 {
                Term::Abstraction(Box::new(Term::Variable(0)))
            } else {
                Term::Variable(i.index(n as usize) as u64)
            }
        })
    });
    leaf.prop_recursive(16, 256, 2, |inner| {
        prop_oneof![
            (inner.clone(), inner.clone()).prop_map(|(f1, f2)| TermGen::new(move |n| {
                Term::Application(Box::new((f1.func)(n)), Box::new((f2.func)(n)))
            })),
            inner
                .clone()
                .prop_map(|f| TermGen::new(move |n| Term::Abstraction(Box::new((f.func)(n + 1))))),
        ]
    })
}

#[cfg(test)]
impl Arbitrary for Term {
    type Parameters = ();
    type Strategy = BoxedStrategy<Self>;

    fn arbitrary_with(_args: Self::Parameters) -> Self::Strategy {
        arb_term_with().prop_map(|f| (f.func)(0)).boxed()
    }
}

impl fmt::Display for Term {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Variable(x) => (x + 1).fmt(f),
            // Self::Variable(x) => x.fmt(f),
            Self::Abstraction(t) => write!(f, "λ {}", t),
            Self::Application(t, u) => {
                write_func(t, f)?;
                write!(f, " ")?;
                write_term(u, f)
            }
        }
    }
}

fn write_term(t: &Term, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match t {
        Term::Variable(_) => fmt::Display::fmt(t, f),
        _ => write!(f, "({})", t),
    }
}

fn write_func(t: &Term, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match t {
        Term::Variable(_) | Term::Application(_, _) => fmt::Display::fmt(t, f),
        _ => write!(f, "({})", t),
    }
}

impl From<Term> for super::Term {
    fn from(val: Term) -> Self {
        // TODO: handle non-closed terms
        give_names(val, &VecDeque::new())
    }
}

fn give_names(val: Term, to: &VecDeque<String>) -> super::Term {
    match val {
        Term::Variable(v) => super::Term::Variable(to[v as usize].clone()),
        Term::Abstraction(t) => {
            let new_var = super::fresh_var(&to.iter().cloned().collect());
            let mut inner = to.clone();
            inner.push_front(new_var.clone());
            super::Term::Abstraction(new_var, Box::new(give_names(*t, &inner)))
        }
        Term::Application(t, u) => {
            super::Term::Application(Box::new(give_names(*t, to)), Box::new(give_names(*u, to)))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::untyped;

    #[test]
    fn skk_beta_reduction() {
        let s = untyped::Term::Abstraction(
            "x".into(),
            Box::new(untyped::Term::Abstraction(
                "y".into(),
                Box::new(untyped::Term::Abstraction(
                    "z".into(),
                    Box::new(untyped::Term::Application(
                        Box::new(untyped::Term::Application(
                            Box::new(untyped::Term::Variable("x".into())),
                            Box::new(untyped::Term::Variable("z".into())),
                        )),
                        Box::new(untyped::Term::Application(
                            Box::new(untyped::Term::Variable("y".into())),
                            Box::new(untyped::Term::Variable("z".into())),
                        )),
                    )),
                )),
            )),
        );
        let k = untyped::Term::Abstraction(
            "x".into(),
            Box::new(untyped::Term::Abstraction(
                "y".into(),
                Box::new(untyped::Term::Variable("x".into())),
            )),
        );
        let mut term_1 = untyped::Term::Application(
            Box::new(untyped::Term::Application(Box::new(s), Box::new(k.clone()))),
            Box::new(k),
        );
        let mut term_2: Term = term_1.clone().try_into().unwrap();
        loop {
            match (term_1.beta_reduce_lazy(), term_2.beta_reduce_lazy()) {
                (None, None) => break,
                (None, Some(_)) | (Some(_), None) => panic!("different number of beta-reductions"),
                (Some(t1), Some(t2)) => {
                    let t1_conv: Term = t1.clone().try_into().unwrap();
                    assert_eq!(t1_conv, t2);
                    (term_1, term_2) = (t1, t2);
                }
            }
        }
    }

    use proptest::prelude::*;

    proptest! {
        #[test]
        fn conversion_matches(f in any::<Term>()) {
            let f2 = untyped::Term::from(f.clone());
            let f3 = Term::try_from(f2).unwrap();
            assert_eq!(f, f3);
        }

        #[test]
        fn beta_reduce_matches(f in any::<Term>()) {
            let f2 = untyped::Term::from(f.clone());
            match (f2.beta_reduce_lazy(), f.beta_reduce_lazy()) {
                (None, None) => {},
                (None, Some(_)) | (Some(_), None) => panic!("different number of beta-reductions"),
                (Some(t1), Some(t2)) => {
                    let t1_conv: Term = t1.clone().try_into().unwrap();
                    assert_eq!(t1_conv, t2);
                }
            }
        }
    }
}
