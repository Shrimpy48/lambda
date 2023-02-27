use std::collections::HashSet;
use std::fmt;

use super::{fresh_var, untyped};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Kind {
    Type,
    TermFn(Type, Box<Kind>),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Type {
    Base,
    Fn(String, Box<Type>, Box<Type>),
    TermAbstraction(String, Box<Type>, Box<Type>),
    TermApplication(Box<Type>, Box<Term>),
}

pub type KindEnvironment = Vec<(String, Kind)>;

impl Type {
    pub fn term_vars(&self) -> HashSet<String> {
        match self {
            Self::Base => HashSet::new(),
            Self::TermAbstraction(v, _, t) => {
                let mut vars = t.term_vars();
                vars.insert(v.to_owned());
                vars
            }
            Self::Fn(v, t, u) => {
                let mut vars = t.term_vars();
                vars.extend(u.term_vars());
                vars.insert(v.to_owned());
                vars
            }
            Self::TermApplication(t, u) => {
                let mut vars = t.term_vars();
                vars.extend(u.vars());
                vars
            }
        }
    }

    pub fn free_term_vars(&self) -> HashSet<String> {
        match self {
            Self::Base => HashSet::new(),
            Self::TermAbstraction(v, _, t) => {
                let mut vars = t.free_term_vars();
                vars.remove(v);
                vars
            }
            Self::Fn(v, t, u) => {
                let mut vars = t.free_term_vars();
                vars.extend(u.free_term_vars());
                vars.remove(v);
                vars
            }
            Self::TermApplication(t, u) => {
                let mut vars = t.free_term_vars();
                vars.extend(u.free_vars());
                vars
            }
        }
    }

    /// A simple renaming operation ignoring shadowing.
    fn term_rename(&self, from: &str, to: &str) -> Self {
        match self {
            Self::Base => Self::Base,
            Self::TermAbstraction(v, k, t) => Self::TermAbstraction(
                if v == from {
                    to.to_owned()
                } else {
                    v.to_owned()
                },
                k.clone(),
                Box::new(t.term_rename(from, to)),
            ),
            Self::Fn(v, t, u) => Self::Fn(
                if v == from {
                    to.to_owned()
                } else {
                    v.to_owned()
                },
                Box::new(t.term_rename(from, to)),
                Box::new(u.term_rename(from, to)),
            ),
            Self::TermApplication(t, u) => Self::TermApplication(
                Box::new(t.term_rename(from, to)),
                Box::new(u.rename(from, to)),
            ),
        }
    }

    /// Substitute the given term avoiding capture.
    pub fn term_substitute(&self, from: &str, to: &Term) -> Self {
        match self {
            Self::Base => Self::Base,
            Self::TermAbstraction(v, _, _) if v == from => self.clone(),
            Self::TermAbstraction(v, k, t) => {
                let mut vars = self.term_vars();
                vars.extend(to.vars());
                vars.insert(from.to_owned());
                let w = fresh_var(&vars);
                let new_body = Box::new(t.term_rename(v, &w).term_substitute(from, to));
                Self::TermAbstraction(w, k.clone(), new_body)
            }
            Self::Fn(v, t, u) if v == from => Self::Fn(
                v.to_owned(),
                Box::new(t.term_substitute(from, to)),
                Box::new(u.as_ref().clone()),
            ),
            Self::Fn(v, t, u) => {
                let mut vars = self.term_vars();
                vars.extend(to.vars());
                vars.insert(from.to_owned());
                let w = fresh_var(&vars);
                let new_rhs = Box::new(u.term_rename(v, &w).term_substitute(from, to));
                Self::Fn(v.to_owned(), Box::new(t.term_substitute(from, to)), new_rhs)
            }
            Self::TermApplication(t, u) => Self::TermApplication(
                Box::new(t.term_substitute(from, to)),
                Box::new(u.substitute(from, to)),
            ),
        }
    }

    pub fn alpha_equivalent(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Base, Self::Base) => true,
            (Self::TermAbstraction(x, k, t), Self::TermAbstraction(y, l, u)) => {
                if k != l {
                    return false;
                }
                let w = fresh_var(&(&self.term_vars() | &other.term_vars()));
                t.term_rename(x, &w).alpha_equivalent(&u.term_rename(y, &w))
            }
            (Self::Fn(x, t1, u1), Self::Fn(y, t2, u2)) => {
                let w = fresh_var(&(&self.term_vars() | &other.term_vars()));
                t1.alpha_equivalent(t2)
                    && u1
                        .term_rename(x, &w)
                        .alpha_equivalent(&u2.term_rename(y, &w))
            }
            (Self::TermApplication(t1, u1), Self::TermApplication(t2, u2)) => {
                t1.alpha_equivalent(t2) && u1.alpha_equivalent(u2)
            }
            _ => false,
        }
    }

    /// Beta-reduce the left-outermost redex if one exists ("normal order").
    /// This does not check the kind.
    pub fn beta_reduce_lazy(&self) -> Option<Self> {
        match self {
            Self::TermApplication(t, u) => {
                if let Self::TermAbstraction(x, _, b) = t.as_ref() {
                    Some(b.term_substitute(x, u))
                } else if let Some(t2) = t.beta_reduce_lazy() {
                    Some(Self::TermApplication(
                        Box::new(t2),
                        Box::new(u.as_ref().clone()),
                    ))
                } else if let Some(u2) = u.beta_reduce_lazy() {
                    Some(Self::TermApplication(
                        Box::new(t.as_ref().clone()),
                        Box::new(u2),
                    ))
                } else {
                    None
                }
            }
            Self::TermAbstraction(x, ty, t) => t
                .beta_reduce_lazy()
                .map(|t2| Self::TermAbstraction(x.to_owned(), ty.clone(), Box::new(t2))),
            Self::Fn(v, t, u) => {
                if let Some(t2) = t.beta_reduce_lazy() {
                    Some(Self::Fn(
                        v.to_owned(),
                        Box::new(t2),
                        Box::new(u.as_ref().clone()),
                    ))
                } else if let Some(u2) = u.beta_reduce_lazy() {
                    Some(Self::Fn(
                        v.to_owned(),
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

    /// Beta-reduce all redexes simultaneously if any exist.
    /// This does not check the kind.
    pub fn parallel_reduct(&self) -> Option<Self> {
        match self {
            Self::TermApplication(t, u) => {
                if let Self::TermAbstraction(x, _, b) = t.as_ref() {
                    let b2 = b.parallel_reduct().unwrap_or_else(|| b.as_ref().clone());
                    let u2 = u.parallel_reduct().unwrap_or_else(|| u.as_ref().clone());
                    Some(b2.term_substitute(x, &u2))
                } else {
                    match (t.parallel_reduct(), u.parallel_reduct()) {
                        (None, None) => None,
                        (t2, u2) => {
                            let t2 = t2.unwrap_or_else(|| t.as_ref().clone());
                            let u2 = u2.unwrap_or_else(|| u.as_ref().clone());

                            Some(Self::TermApplication(Box::new(t2), Box::new(u2)))
                        }
                    }
                }
            }
            Self::TermAbstraction(x, ty, t) => t
                .parallel_reduct()
                .map(|t2| Self::TermAbstraction(x.to_owned(), ty.clone(), Box::new(t2))),
            Self::Fn(v, t, u) => match (t.parallel_reduct(), u.parallel_reduct()) {
                (None, None) => None,
                (t2, u2) => {
                    let t2 = t2.unwrap_or_else(|| t.as_ref().clone());
                    let u2 = u2.unwrap_or_else(|| u.as_ref().clone());

                    Some(Self::Fn(v.to_owned(), Box::new(t2), Box::new(u2)))
                }
            },
            _ => None,
        }
    }

    /// Eta-reduce the left-outermost simplification if one exists.
    /// This does not check the kind.
    pub fn eta_reduce_lazy(&self) -> Option<Self> {
        match self {
            Self::TermAbstraction(x, ty, t) => {
                if let Self::TermApplication(f, u) = t.as_ref() {
                    if let Term::Variable(y) = u.as_ref() {
                        if x == y && !f.free_term_vars().contains(x) {
                            return Some(f.as_ref().clone());
                        }
                    }
                }
                t.eta_reduce_lazy()
                    .map(|u2| Self::TermAbstraction(x.to_owned(), ty.clone(), Box::new(u2)))
            }
            Self::TermApplication(t, u) => {
                if let Some(t2) = t.eta_reduce_lazy() {
                    Some(Self::TermApplication(
                        Box::new(t2),
                        Box::new(u.as_ref().clone()),
                    ))
                } else if let Some(u2) = u.eta_reduce_lazy() {
                    Some(Self::TermApplication(
                        Box::new(t.as_ref().clone()),
                        Box::new(u2),
                    ))
                } else {
                    None
                }
            }
            Self::Fn(v, t, u) => {
                if let Some(t2) = t.eta_reduce_lazy() {
                    Some(Self::Fn(
                        v.to_owned(),
                        Box::new(t2),
                        Box::new(u.as_ref().clone()),
                    ))
                } else if let Some(u2) = u.eta_reduce_lazy() {
                    Some(Self::Fn(
                        v.to_owned(),
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

    /// Fully beta- and eta-reduce the type. If the type is well-kinded (or whatever the proper
    /// terminology is), this will halt.
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

    /// The kind of this type in the given type environment, if it is well-kinded.
    pub fn kind_in(&self, type_env: &TypeEnvironment) -> Option<Kind> {
        match self {
            Self::Base => Some(Kind::Type),
            Self::Fn(_, _, _) => Some(Kind::Type),
            Self::TermAbstraction(x, ty, t) => {
                let mut inner_env = type_env.clone();
                inner_env.push((x.to_owned(), ty.as_ref().clone()));
                t.kind_in(&inner_env)
                    .map(|t| Kind::TermFn(ty.as_ref().clone(), Box::new(t)))
            }
            Self::TermApplication(t, u) => {
                if let Some(Kind::TermFn(a1, b)) = t.kind_in(type_env) {
                    let a2 = u.type_in(type_env)?;
                    if a1 == a2 {
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

    /// The kind of this type in the empty type environment, if it is well-kinded.
    pub fn kind_closed(&self) -> Option<Kind> {
        self.kind_in(&TypeEnvironment::new())
    }
}

#[derive(Debug, Clone, Eq)]
pub enum Term {
    Variable(String),
    Abstraction(String, Box<Type>, Box<Term>),
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
                } else if let Some(u2) = u.beta_reduce_lazy() {
                    Some(Self::Application(
                        Box::new(t.as_ref().clone()),
                        Box::new(u2),
                    ))
                } else {
                    None
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
                inner_env.push((x.to_owned(), ty.as_ref().clone()));
                t.type_in(&inner_env)
                    .map(|t| Type::Fn(x.to_owned(), Box::new(ty.as_ref().clone()), Box::new(t)))
            }
            Self::Application(t, u) => {
                if let Some(Type::Fn(v, a1, b)) = t.type_in(type_env) {
                    let a2 = u.type_in(type_env)?;
                    if a1.as_ref() == &a2 {
                        Some(b.term_substitute(&v, u))
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
            Self::Fn(v, t, u) => write!(f, "Π{}: {}. {}", v, t, u),
            Self::TermAbstraction(x, ty, t) => write!(f, "Λ{}: {}. {}", x, ty, t),
            Self::TermApplication(t, u) => {
                write_ty(t, f)?;
                write!(f, " ")?;
                write_term(u, f)
            }
        }
    }
}

fn write_ty(ty: &Type, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match ty {
        Type::Base | Type::TermApplication(_, _) => fmt::Display::fmt(ty, f),
        _ => write!(f, "({})", ty),
    }
}

impl Into<untyped::Term> for Term {
    fn into(self) -> untyped::Term {
        match self {
            Self::Variable(x) => untyped::Term::Variable(x),
            Self::Abstraction(x, _, t) => untyped::Term::Abstraction(x, t.into()),
            Self::Application(t, u) => untyped::Term::Application(t.into(), u.into()),
        }
    }
}

impl Into<Box<untyped::Term>> for Box<Term> {
    fn into(self) -> Box<untyped::Term> {
        Box::new((*self).into())
    }
}
