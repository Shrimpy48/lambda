use std::collections::{HashMap, HashSet};
use std::fmt;

#[cfg(test)]
use proptest::prelude::*;

use super::fresh_var;

pub mod continuation;
pub mod de_bruijn;
pub mod parser;

#[derive(Debug, Clone, Eq)]
pub enum Term {
    Variable(String),
    Abstraction(String, Box<Term>),
    Application(Box<Term>, Box<Term>),
}

impl Term {
    pub fn vars(&self) -> HashSet<String> {
        match self {
            Self::Variable(v) => {
                let mut vars = HashSet::new();
                vars.insert(v.to_owned());
                vars
            }
            Self::Abstraction(v, t) => {
                let mut vars = t.vars();
                vars.insert(v.to_owned());
                vars
            }
            Self::Application(t, u) => {
                let mut vars = t.vars();
                vars.extend(u.vars());
                vars
            }
        }
    }

    pub fn free_vars(&self) -> HashSet<String> {
        match self {
            Self::Variable(v) => {
                let mut vars = HashSet::new();
                vars.insert(v.to_owned());
                vars
            }
            Self::Abstraction(v, t) => {
                let mut vars = t.free_vars();
                vars.remove(v);
                vars
            }
            Self::Application(t, u) => {
                let mut vars = t.free_vars();
                vars.extend(u.free_vars());
                vars
            }
        }
    }

    /// A simple renaming operation ignoring shadowing.
    fn rename(&self, from: &str, to: &str) -> Self {
        match self {
            Self::Variable(v) if v == from => Self::Variable(to.to_owned()),
            Self::Variable(_) => self.clone(),
            Self::Abstraction(v, t) => Self::Abstraction(
                if v == from {
                    to.to_owned()
                } else {
                    v.to_owned()
                },
                Box::new(t.rename(from, to)),
            ),
            Self::Application(t, u) => {
                Self::Application(Box::new(t.rename(from, to)), Box::new(u.rename(from, to)))
            }
        }
    }

    pub fn alpha_equivalent(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Variable(x), Self::Variable(y)) => x == y,
            (Self::Abstraction(x, t), Self::Abstraction(y, u)) => {
                let w = fresh_var(&(&self.vars() | &other.vars()));
                t.rename(x, &w).alpha_equivalent(&u.rename(y, &w))
            }
            (Self::Application(t1, u1), Self::Application(t2, u2)) => {
                t1.alpha_equivalent(t2) && u1.alpha_equivalent(u2)
            }
            _ => false,
        }
    }

    /// Substitute the given term avoiding capture.
    pub fn substitute(&self, from: &str, to: &Term) -> Self {
        match self {
            Self::Variable(v) if v == from => to.to_owned(),
            Self::Variable(_) => self.clone(),
            Self::Abstraction(v, _) if v == from => self.clone(),
            Self::Abstraction(v, t) => {
                let mut vars = self.vars();
                vars.extend(to.vars());
                vars.insert(from.to_owned());
                let w = fresh_var(&vars);
                let new_body = Box::new(t.rename(v, &w).substitute(from, to));
                Self::Abstraction(w, new_body)
            }
            Self::Application(t, u) => Self::Application(
                Box::new(t.substitute(from, to)),
                Box::new(u.substitute(from, to)),
            ),
        }
    }

    /// Beta-reduce the left-outermost redex if one exists ("normal order").
    pub fn beta_reduce_lazy(&self) -> Option<Self> {
        match self {
            Self::Application(t, u) => {
                if let Self::Abstraction(x, b) = t.as_ref() {
                    Some(b.substitute(x, u))
                } else if let Some(t2) = t.beta_reduce_lazy() {
                    Some(Self::Application(
                        Box::new(t2),
                        Box::new(u.as_ref().clone()),
                    ))
                } else {
                    u.beta_reduce_lazy()
                        .map(|u2| Self::Application(Box::new(t.as_ref().clone()), Box::new(u2)))
                }
            }
            Self::Abstraction(x, t) => t
                .beta_reduce_lazy()
                .map(|t2| Self::Abstraction(x.to_owned(), Box::new(t2))),
            _ => None,
        }
    }

    /// Beta-reduce all redexes simultaneously if any exist.
    pub fn parallel_reduct(&self) -> Option<Self> {
        match self {
            Self::Application(t, u) => {
                if let Self::Abstraction(x, b) = t.as_ref() {
                    let b2 = b.parallel_reduct().unwrap_or_else(|| b.as_ref().clone());
                    let u2 = u.parallel_reduct().unwrap_or_else(|| u.as_ref().clone());
                    Some(b2.substitute(x, &u2))
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
            Self::Abstraction(x, t) => t
                .parallel_reduct()
                .map(|t2| Self::Abstraction(x.to_owned(), Box::new(t2))),
            _ => None,
        }
    }

    /// Eta-reduce the left-outermost simplification if one exists.
    pub fn eta_reduce_lazy(&self) -> Option<Self> {
        match self {
            Self::Abstraction(x, t) => {
                if let Self::Application(f, u) = t.as_ref() {
                    if let Self::Variable(y) = u.as_ref() {
                        if x == y && !f.free_vars().contains(x) {
                            return Some(f.as_ref().clone());
                        }
                    }
                }
                t.eta_reduce_lazy()
                    .map(|u2| Self::Abstraction(x.to_owned(), Box::new(u2)))
            }
            Self::Application(t, u) => {
                if let Some(t2) = t.eta_reduce_lazy() {
                    Some(Self::Application(
                        Box::new(t2),
                        Box::new(u.as_ref().clone()),
                    ))
                } else {
                    u.eta_reduce_lazy()
                        .map(|u2| Self::Application(Box::new(t.as_ref().clone()), Box::new(u2)))
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
impl Arbitrary for Term {
    type Parameters = ();
    type Strategy = BoxedStrategy<Self>;

    fn arbitrary_with(_args: Self::Parameters) -> Self::Strategy {
        arb_term().boxed()
    }
}

#[cfg(test)]
fn arb_term() -> impl Strategy<Value = Term> {
    let leaf = "[a-zA-Zα-κμ-ωΑ-ΚΜ-ΟΡ-Ω_][a-zA-Zα-κμ-ωΑ-ΚΜ-ΟΡ-Ω0-9_]*'*".prop_map(Term::Variable);
    leaf.prop_recursive(16, 256, 2, |inner| {
        prop_oneof![
            (inner.clone(), inner.clone())
                .prop_map(|(f, x)| Term::Application(Box::new(f), Box::new(x))),
            (
                "[a-zA-Zα-κμ-ωΑ-ΚΜ-ΟΡ-Ω_][a-zA-Zα-κμ-ωΑ-ΚΜ-ΟΡ-Ω0-9_]*'*",
                inner
            )
                .prop_map(|(x, b)| Term::Abstraction(x, Box::new(b))),
        ]
    })
}

impl PartialEq for Term {
    fn eq(&self, other: &Self) -> bool {
        self.alpha_equivalent(other)
    }
}

impl fmt::Display for Term {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Variable(x) => x.fmt(f),
            Self::Abstraction(x, t) => write!(f, "λ{}. {}", x, t),
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

/// Convert a term to De Bruijn indexing.
fn to_de_bruijn(term: &Term, binds: &HashMap<String, u64>) -> Option<de_bruijn::Term> {
    match term {
        Term::Variable(x) => binds.get(x).map(|i| de_bruijn::Term::Variable(*i)),
        Term::Abstraction(x, t) => {
            let mut new_binds: HashMap<_, _> =
                binds.iter().map(|(v, i)| (v.clone(), i + 1)).collect();
            new_binds.insert(x.to_owned(), 0);
            Some(de_bruijn::Term::Abstraction(Box::new(to_de_bruijn(
                t, &new_binds,
            )?)))
        }
        Term::Application(t, u) => Some(de_bruijn::Term::Application(
            Box::new(to_de_bruijn(t, binds)?),
            Box::new(to_de_bruijn(u, binds)?),
        )),
    }
}

/// Convert a closed term to De Bruijn indexing.
impl TryFrom<Term> for de_bruijn::Term {
    type Error = ();

    fn try_from(value: Term) -> Result<Self, Self::Error> {
        to_de_bruijn(&value, &HashMap::new()).ok_or(())
    }
}

// convenience functions
pub fn variable(v: impl Into<String>) -> Term {
    Term::Variable(v.into())
}
pub fn abstraction(v: impl Into<String>, t: impl Into<Term>) -> Term {
    Term::Abstraction(v.into(), Box::new(t.into()))
}
pub fn application(t: impl Into<Term>, u: impl Into<Term>) -> Term {
    Term::Application(Box::new(t.into()), Box::new(u.into()))
}
