use std::collections::HashSet;
use std::fmt;

use super::{fresh_var, untyped};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Kind {
    Type,
    Fn(Box<Kind>, Box<Kind>),
}

pub type KindEnvironment = Vec<(String, Kind)>;

#[derive(Debug, Clone, Eq)]
pub enum Type {
    Base,
    Variable(String),
    Fn(Box<Type>, Box<Type>),
    Abstraction(String, Kind, Box<Type>),
    Application(Box<Type>, Box<Type>),
}

impl Type {
    pub fn vars(&self) -> HashSet<String> {
        match self {
            Self::Base => HashSet::new(),
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
            Self::Fn(t, u) => {
                let mut vars = t.vars();
                vars.extend(u.vars());
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
            Self::Base => HashSet::new(),
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
            Self::Fn(t, u) => {
                let mut vars = t.free_vars();
                vars.extend(u.free_vars());
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
            Self::Base => Self::Base,
            Self::Variable(v) if v == from => Self::Variable(to.to_owned()),
            Self::Variable(_) => self.clone(),
            Self::Abstraction(v, k, t) => Self::Abstraction(
                if v == from {
                    to.to_owned()
                } else {
                    v.to_owned()
                },
                k.clone(),
                Box::new(t.rename(from, to)),
            ),
            Self::Fn(t, u) => Self::Fn(Box::new(t.rename(from, to)), Box::new(u.rename(from, to))),
            Self::Application(t, u) => {
                Self::Application(Box::new(t.rename(from, to)), Box::new(u.rename(from, to)))
            }
        }
    }

    /// Substitute the given type avoiding capture.
    pub fn substitute(&self, from: &str, to: &Type) -> Self {
        match self {
            Self::Base => Self::Base,
            Self::Variable(v) if v == from => to.to_owned(),
            Self::Variable(_) => self.clone(),
            Self::Abstraction(v, _, _) if v == from => self.clone(),
            Self::Abstraction(v, k, t) => {
                let mut vars = self.vars();
                vars.extend(to.vars());
                vars.insert(from.to_owned());
                let w = fresh_var(&vars);
                let new_body = Box::new(t.rename(v, &w).substitute(from, to));
                Self::Abstraction(w, k.clone(), new_body)
            }
            Self::Fn(t, u) => Self::Fn(
                Box::new(t.substitute(from, to)),
                Box::new(u.substitute(from, to)),
            ),
            Self::Application(t, u) => Self::Application(
                Box::new(t.substitute(from, to)),
                Box::new(u.substitute(from, to)),
            ),
        }
    }

    pub fn alpha_equivalent(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Base, Self::Base) => true,
            (Self::Variable(x), Self::Variable(y)) => x == y,
            (Self::Abstraction(x, k, t), Self::Abstraction(y, l, u)) => {
                if k != l {
                    return false;
                }
                let w = fresh_var(&(&self.vars() | &other.vars()));
                t.rename(x, &w).alpha_equivalent(&u.rename(y, &w))
            }
            (Self::Fn(t1, u1), Self::Fn(t2, u2)) => {
                t1.alpha_equivalent(t2) && u1.alpha_equivalent(u2)
            }
            (Self::Application(t1, u1), Self::Application(t2, u2)) => {
                t1.alpha_equivalent(t2) && u1.alpha_equivalent(u2)
            }
            _ => false,
        }
    }

    /// Beta-reduce the left-outermost redex if one exists ("normal order").
    /// This does not check the kind.
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
            Self::Fn(t, u) => {
                if let Some(t2) = t.beta_reduce_lazy() {
                    Some(Self::Fn(Box::new(t2), Box::new(u.as_ref().clone())))
                } else if let Some(u2) = u.beta_reduce_lazy() {
                    Some(Self::Fn(Box::new(t.as_ref().clone()), Box::new(u2)))
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
            Self::Fn(t, u) => match (t.parallel_reduct(), u.parallel_reduct()) {
                (None, None) => None,
                (t2, u2) => {
                    let t2 = t2.unwrap_or_else(|| t.as_ref().clone());
                    let u2 = u2.unwrap_or_else(|| u.as_ref().clone());

                    Some(Self::Fn(Box::new(t2), Box::new(u2)))
                }
            },
            _ => None,
        }
    }

    /// Eta-reduce the left-outermost simplification if one exists.
    /// This does not check the kind.
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
            Self::Fn(t, u) => {
                if let Some(t2) = t.eta_reduce_lazy() {
                    Some(Self::Fn(Box::new(t2), Box::new(u.as_ref().clone())))
                } else if let Some(u2) = u.eta_reduce_lazy() {
                    Some(Self::Fn(Box::new(t.as_ref().clone()), Box::new(u2)))
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

    /// The kind of this type in the given kind environment, if it is well-kinded.
    pub fn kind_in(&self, kind_env: &KindEnvironment) -> Option<Kind> {
        match self {
            Self::Base => Some(Kind::Type),
            Self::Fn(_, _) => Some(Kind::Type),
            Self::Variable(x) => kind_env
                .iter()
                .rev()
                .find(|(v, _)| v == x)
                .map(|(_, t)| t.clone()),
            Self::Abstraction(x, ty, t) => {
                let mut inner_env = kind_env.clone();
                inner_env.push((x.to_owned(), ty.clone()));
                t.kind_in(&inner_env)
                    .map(|t| Kind::Fn(Box::new(ty.clone()), Box::new(t)))
            }
            Self::Application(t, u) => {
                if let Some(Kind::Fn(a1, b)) = t.kind_in(kind_env) {
                    let a2 = u.kind_in(kind_env)?;
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

    /// The kind of this type in the empty kind environment, if it is well-kinded.
    pub fn kind_closed(&self) -> Option<Kind> {
        self.kind_in(&KindEnvironment::new())
    }
}

impl PartialEq for Type {
    fn eq(&self, other: &Self) -> bool {
        self.evaluate().alpha_equivalent(&other.evaluate())
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
                write_ty_func(t, f)?;
                write!(f, " -> {}", u)
            }
            Self::Variable(x) => x.fmt(f),
            Self::Abstraction(x, k, t) => write!(f, "λ{} :: {}. {}", x, k, t),
            Self::Application(t, u) => {
                write_ty_func(t, f)?;
                write!(f, " ")?;
                write_ty_term(u, f)
            }
        }
    }
}

fn write_ty_term(t: &Type, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match t {
        Type::Base | Type::Variable(_) => fmt::Display::fmt(t, f),
        _ => write!(f, "({})", t),
    }
}

fn write_ty_func(t: &Type, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match t {
        Type::Base | Type::Variable(_) | Type::Application(_, _) => fmt::Display::fmt(t, f),
        _ => write!(f, "({})", t),
    }
}

impl fmt::Display for Kind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Type => write!(f, "*"),
            Self::Fn(t, u) => {
                write_kind(t, f)?;
                write!(f, " -> {}", u)
            }
        }
    }
}

fn write_kind(k: &Kind, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match k {
        Kind::Type => fmt::Display::fmt(k, f),
        Kind::Fn(_, _) => write!(f, "({})", k),
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

#[cfg(test)]
mod tests {
    use super::*;
}
