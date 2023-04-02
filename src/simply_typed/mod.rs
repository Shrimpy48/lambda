pub mod general;

use std::collections::HashSet;
use std::fmt;

use super::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Type {
    Base,
    Fn(Box<Type>, Box<Type>),
}

impl Type {
    pub fn order(&self) -> usize {
        match self {
            Self::Base => 0,
            Self::Fn(a, b) => (a.order() + 1).max(b.order()),
        }
    }
}

#[derive(Debug, Clone, Eq)]
pub enum Term {
    Variable(String),
    Abstraction(String, Type, Box<Term>),
    Application(Box<Term>, Box<Term>),
}

pub type TypeEnvironment = Vec<(String, Type)>;

impl Term {
    pub fn vars(&self) -> HashSet<String> {
        match self {
            Self::Variable(v) => {
                let mut vars = HashSet::new();
                vars.insert(v.to_owned());
                vars
            }
            Self::Abstraction(v, _, t) => {
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
            Self::Abstraction(v, _, t) => {
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
            Self::Abstraction(v, ty, t) => Self::Abstraction(
                if v == from {
                    to.to_owned()
                } else {
                    v.to_owned()
                },
                ty.clone(),
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
            (Self::Abstraction(x, ty1, t), Self::Abstraction(y, ty2, u)) => {
                let w = fresh_var(&(&self.vars() | &other.vars()));
                ty1 == ty2 && t.rename(x, &w).alpha_equivalent(&u.rename(y, &w))
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
            Self::Abstraction(v, _, _) if v == from => self.clone(),
            Self::Abstraction(v, ty, t) => {
                let mut vars = self.vars();
                vars.extend(to.vars());
                vars.insert(from.to_owned());
                let w = fresh_var(&vars);
                let new_body = Box::new(t.rename(v, &w).substitute(from, to));
                Self::Abstraction(w, ty.clone(), new_body)
            }
            Self::Application(t, u) => Self::Application(
                Box::new(t.substitute(from, to)),
                Box::new(u.substitute(from, to)),
            ),
        }
    }

    /// Beta-reduce the left-outermost redex if one exists ("normal order").
    /// This does not check the type.
    pub fn beta_reduce_lazy(&self) -> Option<Self> {
        match self {
            Self::Application(t, u) => {
                if let Self::Abstraction(x, _, b) = t.as_ref() {
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
            Self::Abstraction(x, ty, t) => t
                .beta_reduce_lazy()
                .map(|t2| Self::Abstraction(x.to_owned(), ty.clone(), Box::new(t2))),
            _ => None,
        }
    }

    /// Beta-reduce all redexes simultaneously if any exist.
    /// This does not check the type.
    pub fn parallel_reduct(&self) -> Option<Self> {
        match self {
            Self::Application(t, u) => {
                if let Self::Abstraction(x, _, b) = t.as_ref() {
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
            Self::Abstraction(x, ty, t) => t
                .parallel_reduct()
                .map(|t2| Self::Abstraction(x.to_owned(), ty.clone(), Box::new(t2))),
            _ => None,
        }
    }

    /// Eta-reduce the left-outermost simplification if one exists.
    /// This does not check the type.
    pub fn eta_reduce_lazy(&self) -> Option<Self> {
        match self {
            Self::Abstraction(x, ty, t) => {
                if let Self::Application(f, u) = t.as_ref() {
                    if let Self::Variable(y) = u.as_ref() {
                        if x == y && !f.free_vars().contains(x) {
                            return Some(f.as_ref().clone());
                        }
                    }
                }
                t.eta_reduce_lazy()
                    .map(|u2| Self::Abstraction(x.to_owned(), ty.clone(), Box::new(u2)))
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

    /// Fully beta- and eta-reduce the term. If the term is well-typed, this will halt.
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

    /// The type of this term in the given type environment, if it is well-typed.
    pub fn type_in(&self, type_env: &TypeEnvironment) -> Option<Type> {
        match self {
            Self::Variable(x) => type_env
                .iter()
                .rev()
                .find(|(v, _)| v == x)
                .map(|(_, t)| t.clone()),
            Self::Abstraction(x, ty, t) => {
                let mut inner_env = type_env.clone();
                inner_env.push((x.to_owned(), ty.clone()));
                t.type_in(&inner_env)
                    .map(|t| Type::Fn(Box::new(ty.clone()), Box::new(t)))
            }
            Self::Application(t, u) => {
                if let Some(Type::Fn(a1, b)) = t.type_in(type_env) {
                    let a2 = u.type_in(type_env)?;
                    if a1.as_ref() == &a2 {
                        Some(b.as_ref().clone())
                    } else {
                        None
                    }
                } else {
                    None
                }
            }
        }
    }

    /// The type of this term in the empty type environment, if it is well-typed.
    pub fn type_closed(&self) -> Option<Type> {
        self.type_in(&TypeEnvironment::new())
    }
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
            Self::Abstraction(x, ty, t) => write!(f, "λ{}: {}. {}", x, ty, t),
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

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Base => write!(f, "ι"),
            Self::Fn(t, u) => {
                write_ty(t, f)?;
                write!(f, " -> {}", u)
            }
        }
    }
}

fn write_ty(ty: &Type, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match ty {
        Type::Base => fmt::Display::fmt(ty, f),
        Type::Fn(_, _) => write!(f, "({})", ty),
    }
}

impl From<Term> for untyped::Term {
    fn from(val: Term) -> Self {
        match val {
            Term::Variable(x) => untyped::Term::Variable(x),
            Term::Abstraction(x, _, t) => untyped::Term::Abstraction(x, t.into()),
            Term::Application(t, u) => untyped::Term::Application(t.into(), u.into()),
        }
    }
}

impl From<Box<Term>> for Box<untyped::Term> {
    fn from(val: Box<Term>) -> Self {
        Box::new((*val).into())
    }
}

impl From<Term> for system_f::Term {
    fn from(val: Term) -> Self {
        match val {
            Term::Variable(x) => system_f::Term::Variable(x),
            Term::Abstraction(x, ty, t) => system_f::Term::Abstraction(x, ty.into(), t.into()),
            Term::Application(t, u) => system_f::Term::Application(t.into(), u.into()),
        }
    }
}

impl From<Type> for system_f::Type {
    fn from(val: Type) -> Self {
        match val {
            Type::Base => system_f::Type::Base,
            Type::Fn(t, u) => system_f::Type::Fn(t.into(), u.into()),
        }
    }
}

impl From<Box<Term>> for Box<system_f::Term> {
    fn from(val: Box<Term>) -> Self {
        Box::new((*val).into())
    }
}

impl From<Box<Type>> for Box<system_f::Type> {
    fn from(val: Box<Type>) -> Self {
        Box::new((*val).into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn i_type() {
        let i = Term::Abstraction("x".into(), Type::Base, Box::new(Term::Variable("x".into())));
        let expected = Type::Fn(Box::new(Type::Base), Box::new(Type::Base));
        assert_eq!(i.type_closed(), Some(expected));
    }

    #[test]
    fn k_type() {
        let k = Term::Abstraction(
            "x".into(),
            Type::Base,
            Box::new(Term::Abstraction(
                "y".into(),
                Type::Base,
                Box::new(Term::Variable("x".into())),
            )),
        );
        let expected = Type::Fn(
            Box::new(Type::Base),
            Box::new(Type::Fn(Box::new(Type::Base), Box::new(Type::Base))),
        );
        assert_eq!(k.type_closed(), Some(expected));
    }

    #[test]
    fn s_type() {
        let s = Term::Abstraction(
            "x".into(),
            Type::Fn(
                Box::new(Type::Base),
                Box::new(Type::Fn(Box::new(Type::Base), Box::new(Type::Base))),
            ),
            Box::new(Term::Abstraction(
                "y".into(),
                Type::Fn(Box::new(Type::Base), Box::new(Type::Base)),
                Box::new(Term::Abstraction(
                    "z".into(),
                    Type::Base,
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
        );
        let expected = Type::Fn(
            Box::new(Type::Fn(
                Box::new(Type::Base),
                Box::new(Type::Fn(Box::new(Type::Base), Box::new(Type::Base))),
            )),
            Box::new(Type::Fn(
                Box::new(Type::Fn(Box::new(Type::Base), Box::new(Type::Base))),
                Box::new(Type::Fn(Box::new(Type::Base), Box::new(Type::Base))),
            )),
        );
        assert_eq!(s.type_closed(), Some(expected));
    }
}
