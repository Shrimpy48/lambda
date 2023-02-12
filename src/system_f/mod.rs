pub mod self_interpret;

use std::collections::HashSet;
use std::fmt;

use super::{fresh_var, untyped};

#[derive(Debug, Clone, Eq)]
pub enum Type {
    Base,
    Variable(String),
    ForAll(String, Box<Type>),
    Fn(Box<Type>, Box<Type>),
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
            Self::ForAll(v, t) => {
                let mut vars = t.vars();
                vars.insert(v.to_owned());
                vars
            }
            Self::Fn(t, u) => {
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
            Self::ForAll(v, t) => {
                let mut vars = t.free_vars();
                vars.remove(v);
                vars
            }
            Self::Fn(t, u) => {
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
            Self::ForAll(v, t) => Self::ForAll(
                if v == from {
                    to.to_owned()
                } else {
                    v.to_owned()
                },
                Box::new(t.rename(from, to)),
            ),
            Self::Fn(t, u) => Self::Fn(Box::new(t.rename(from, to)), Box::new(u.rename(from, to))),
        }
    }

    /// Substitute the given type avoiding capture.
    pub fn substitute(&self, from: &str, to: &Type) -> Self {
        match self {
            Self::Base => Self::Base,
            Self::Variable(v) if v == from => to.to_owned(),
            Self::Variable(_) => self.clone(),
            Self::ForAll(v, _) if v == from => self.clone(),
            Self::ForAll(v, t) => {
                let mut vars = self.vars();
                vars.extend(to.vars());
                vars.insert(from.to_owned());
                let w = fresh_var(&vars);
                let new_body = Box::new(t.rename(v, &w).substitute(from, to));
                Self::ForAll(w, new_body)
            }
            Self::Fn(t, u) => Self::Fn(
                Box::new(t.substitute(from, to)),
                Box::new(u.substitute(from, to)),
            ),
        }
    }

    pub fn alpha_equivalent(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Base, Self::Base) => true,
            (Self::Variable(x), Self::Variable(y)) => x == y,
            (Self::ForAll(x, t), Self::ForAll(y, u)) => {
                let w = fresh_var(&(&self.vars() | &other.vars()));
                t.rename(x, &w).alpha_equivalent(&u.rename(y, &w))
            }
            (Self::Fn(t1, u1), Self::Fn(t2, u2)) => {
                t1.alpha_equivalent(t2) && u1.alpha_equivalent(u2)
            }
            _ => false,
        }
    }
}

impl PartialEq for Type {
    fn eq(&self, other: &Self) -> bool {
        self.alpha_equivalent(other)
    }
}

#[derive(Debug, Clone, Eq)]
pub enum Term {
    Variable(String),
    Abstraction(String, Type, Box<Term>),
    Application(Box<Term>, Box<Term>),
    TypeAbstraction(String, Box<Term>),
    TypeApplication(Box<Term>, Type),
}

#[derive(Debug, Clone)]
pub struct TypeEnvironment {
    term_variables: Vec<(String, Type)>,
    type_variables: HashSet<String>,
}

impl TypeEnvironment {
    fn new() -> TypeEnvironment {
        Self {
            term_variables: Vec::new(),
            type_variables: HashSet::new(),
        }
    }
}

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
            Self::TypeAbstraction(_, t) => t.vars(),
            Self::TypeApplication(t, _) => t.vars(),
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
            Self::TypeAbstraction(_, t) => t.free_vars(),
            Self::TypeApplication(t, _) => t.free_vars(),
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
            Self::TypeAbstraction(a, t) => {
                Self::TypeAbstraction(a.to_owned(), Box::new(t.rename(from, to)))
            }
            Self::TypeApplication(t, a) => {
                Self::TypeApplication(Box::new(t.rename(from, to)), a.to_owned())
            }
        }
    }

    pub fn type_vars(&self) -> HashSet<String> {
        match self {
            Self::Variable(_) => HashSet::new(),
            Self::Abstraction(_, ty, t) => {
                let mut vars = ty.vars();
                vars.extend(t.type_vars());
                vars
            }
            Self::Application(t, u) => {
                let mut vars = t.type_vars();
                vars.extend(u.type_vars());
                vars
            }
            Self::TypeAbstraction(v, t) => {
                let mut vars = t.type_vars();
                vars.insert(v.to_owned());
                vars
            }
            Self::TypeApplication(t, u) => {
                let mut vars = t.type_vars();
                vars.extend(u.vars());
                vars
            }
        }
    }

    pub fn free_type_vars(&self) -> HashSet<String> {
        match self {
            Self::Variable(_) => HashSet::new(),
            Self::Abstraction(_, ty, t) => {
                let mut vars = ty.free_vars();
                vars.extend(t.free_type_vars());
                vars
            }
            Self::Application(t, u) => {
                let mut vars = t.free_type_vars();
                vars.extend(u.free_type_vars());
                vars
            }
            Self::TypeAbstraction(v, t) => {
                let mut vars = t.free_type_vars();
                vars.remove(v);
                vars
            }
            Self::TypeApplication(t, u) => {
                let mut vars = t.free_type_vars();
                vars.extend(u.free_vars());
                vars
            }
        }
    }

    /// A simple renaming operation ignoring shadowing.
    fn type_rename(&self, from: &str, to: &str) -> Self {
        match self {
            Self::Variable(_) => self.clone(),
            Self::TypeAbstraction(v, t) => Self::TypeAbstraction(
                if v == from {
                    to.to_owned()
                } else {
                    v.to_owned()
                },
                Box::new(t.type_rename(from, to)),
            ),
            Self::Application(t, u) => Self::Application(
                Box::new(t.type_rename(from, to)),
                Box::new(u.type_rename(from, to)),
            ),
            Self::TypeApplication(t, u) => {
                Self::TypeApplication(Box::new(t.type_rename(from, to)), u.rename(from, to))
            }
            Self::Abstraction(a, ty, t) => Self::Abstraction(
                a.to_owned(),
                ty.rename(from, to),
                Box::new(t.type_rename(from, to)),
            ),
        }
    }

    /// Substitute the given type avoiding capture.
    pub fn type_substitute(&self, from: &str, to: &Type) -> Self {
        match self {
            Self::Variable(_) => self.clone(),
            Self::Abstraction(v, ty, t) => Self::Abstraction(
                v.to_owned(),
                ty.substitute(from, to),
                Box::new(t.type_substitute(from, to)),
            ),
            Self::Application(t, u) => Self::Application(
                Box::new(t.type_substitute(from, to)),
                Box::new(u.type_substitute(from, to)),
            ),
            Self::TypeAbstraction(v, _) if v == from => self.clone(),
            Self::TypeAbstraction(v, t) => {
                let mut vars = self.type_vars();
                vars.extend(to.vars());
                vars.insert(from.to_owned());
                let w = fresh_var(&vars);
                let new_body = Box::new(t.type_rename(v, &w).type_substitute(from, to));
                Self::TypeAbstraction(w, new_body)
            }
            Self::TypeApplication(t, u) => Self::TypeApplication(
                Box::new(t.type_substitute(from, to)),
                u.substitute(from, to),
            ),
        }
    }

    pub fn alpha_equivalent(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Variable(x), Self::Variable(y)) => x == y,
            (Self::Abstraction(x, ty1, t), Self::Abstraction(y, ty2, u)) => {
                let w = fresh_var(&(&self.vars() | &other.vars()));
                ty1.alpha_equivalent(ty2) && t.rename(x, &w).alpha_equivalent(&u.rename(y, &w))
            }
            (Self::Application(t1, u1), Self::Application(t2, u2)) => {
                t1.alpha_equivalent(t2) && u1.alpha_equivalent(u2)
            }
            (Self::TypeAbstraction(x, t), Self::TypeAbstraction(y, u)) => {
                let w = fresh_var(&(&self.type_vars() | &other.type_vars()));
                t.type_rename(x, &w).alpha_equivalent(&u.type_rename(y, &w))
            }
            (Self::TypeApplication(t1, u1), Self::TypeApplication(t2, u2)) => {
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
            Self::TypeAbstraction(a, t) => {
                Self::TypeAbstraction(a.to_owned(), Box::new(t.substitute(from, to)))
            }
            Self::TypeApplication(t, u) => {
                Self::TypeApplication(Box::new(t.substitute(from, to)), u.clone())
            }
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
            Self::TypeApplication(t, u) => {
                if let Self::TypeAbstraction(x, b) = t.as_ref() {
                    Some(b.type_substitute(x, u))
                } else if let Some(t2) = t.beta_reduce_lazy() {
                    Some(Self::TypeApplication(Box::new(t2), u.clone()))
                } else {
                    None
                }
            }
            Self::Abstraction(x, ty, t) => t
                .beta_reduce_lazy()
                .map(|t2| Self::Abstraction(x.to_owned(), ty.clone(), Box::new(t2))),
            Self::TypeAbstraction(x, t) => t
                .beta_reduce_lazy()
                .map(|t2| Self::TypeAbstraction(x.to_owned(), Box::new(t2))),
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
            Self::TypeApplication(t, u) => {
                if let Self::TypeAbstraction(x, b) = t.as_ref() {
                    let b2 = b.parallel_reduct().unwrap_or_else(|| b.as_ref().clone());
                    Some(b2.type_substitute(x, u))
                } else {
                    t.parallel_reduct()
                        .map(|t2| Self::TypeApplication(Box::new(t2), u.clone()))
                }
            }
            Self::Abstraction(x, ty, t) => t
                .parallel_reduct()
                .map(|t2| Self::Abstraction(x.to_owned(), ty.clone(), Box::new(t2))),
            Self::TypeAbstraction(x, t) => t
                .parallel_reduct()
                .map(|t2| Self::TypeAbstraction(x.to_owned(), Box::new(t2))),
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
            Self::TypeAbstraction(x, t) => {
                if let Self::TypeApplication(f, u) = t.as_ref() {
                    if let Type::Variable(y) = u {
                        if x == y && !f.free_vars().contains(x) {
                            return Some(f.as_ref().clone());
                        }
                    }
                }
                t.eta_reduce_lazy()
                    .map(|u2| Self::TypeAbstraction(x.to_owned(), Box::new(u2)))
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
            Self::TypeApplication(t, u) => {
                if let Some(t2) = t.eta_reduce_lazy() {
                    Some(Self::TypeApplication(Box::new(t2), u.clone()))
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
                .term_variables
                .iter()
                .rev()
                .find(|(v, _)| v == x)
                .map(|(_, t)| t.clone()),
            Self::Abstraction(x, ty, t) => {
                if !ty.free_vars().is_subset(&type_env.type_variables) {
                    return None;
                }
                let mut inner_env = type_env.clone();
                inner_env.term_variables.push((x.to_owned(), ty.clone()));
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
            Self::TypeAbstraction(a, t) => {
                let mut inner_env = type_env.clone();
                inner_env.type_variables.insert(a.to_owned());
                t.type_in(&inner_env)
                    .map(|t| Type::ForAll(a.to_owned(), Box::new(t)))
            }
            Self::TypeApplication(t, u) => {
                if let Some(Type::ForAll(v, b)) = t.type_in(type_env) {
                    Some(b.substitute(&v, u))
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
            Self::TypeAbstraction(x, t) => write!(f, "Λ{}. {}", x, t),
            Self::TypeApplication(t, u) => {
                write_func(t, f)?;
                write!(f, " ")?;
                write_ty(u, f)
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
        Term::Variable(_) | Term::Application(_, _) | Term::TypeApplication(_, _) => {
            fmt::Display::fmt(t, f)
        }
        _ => write!(f, "({})", t),
    }
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Base => write!(f, "ι"),
            Self::Variable(x) => x.fmt(f),
            Self::Fn(t, u) => {
                write_ty(t, f)?;
                write!(f, " -> {}", u)
            }
            Self::ForAll(a, t) => write!(f, "∀{}. {}", a, t),
        }
    }
}

fn write_ty(ty: &Type, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match ty {
        Type::Base | Type::Variable(_) => fmt::Display::fmt(ty, f),
        Type::Fn(_, _) | Type::ForAll(_, _) => write!(f, "({})", ty),
    }
}

impl fmt::Display for TypeEnvironment {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut any_elems = false;
        for v in &self.type_variables {
            if any_elems {
                write!(f, ", {v} type")?;
            } else {
                write!(f, "{v} type")?;
                any_elems = true;
            }
        }
        for (v, t) in &self.term_variables {
            if any_elems {
                write!(f, ", {v}: {t}")?;
            } else {
                write!(f, "{v}: {t}")?;
                any_elems = true;
            }
        }
        Ok(())
    }
}

impl Into<untyped::Term> for Term {
    fn into(self) -> untyped::Term {
        match self {
            Self::Variable(x) => untyped::Term::Variable(x),
            Self::Abstraction(x, _, t) => untyped::Term::Abstraction(x, t.into()),
            Self::Application(t, u) => untyped::Term::Application(t.into(), u.into()),
            Self::TypeAbstraction(_, t) => (*t).into(),
            Self::TypeApplication(t, _) => (*t).into(),
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

    #[test]
    fn i_type() {
        let i = Term::TypeAbstraction(
            "a".into(),
            Box::new(Term::Abstraction(
                "x".into(),
                Type::Variable("a".into()),
                Box::new(Term::Variable("x".into())),
            )),
        );
        let expected = Type::ForAll(
            "a".into(),
            Box::new(Type::Fn(
                Box::new(Type::Variable("a".into())),
                Box::new(Type::Variable("a".into())),
            )),
        );
        assert_eq!(i.type_closed(), Some(expected));
    }

    #[test]
    fn k_type() {
        let k = Term::TypeAbstraction(
            "a".into(),
            Box::new(Term::TypeAbstraction(
                "b".into(),
                Box::new(Term::Abstraction(
                    "x".into(),
                    Type::Variable("a".into()),
                    Box::new(Term::Abstraction(
                        "y".into(),
                        Type::Variable("b".into()),
                        Box::new(Term::Variable("x".into())),
                    )),
                )),
            )),
        );
        let expected = Type::ForAll(
            "a".into(),
            Box::new(Type::ForAll(
                "b".into(),
                Box::new(Type::Fn(
                    Box::new(Type::Variable("a".into())),
                    Box::new(Type::Fn(
                        Box::new(Type::Variable("b".into())),
                        Box::new(Type::Variable("a".into())),
                    )),
                )),
            )),
        );
        assert_eq!(k.type_closed(), Some(expected));
    }

    #[test]
    fn s_type() {
        let s = Term::TypeAbstraction(
            "a".into(),
            Box::new(Term::TypeAbstraction(
                "b".into(),
                Box::new(Term::TypeAbstraction(
                    "c".into(),
                    Box::new(Term::Abstraction(
                        "x".into(),
                        Type::Fn(
                            Box::new(Type::Variable("a".into())),
                            Box::new(Type::Fn(
                                Box::new(Type::Variable("b".into())),
                                Box::new(Type::Variable("c".into())),
                            )),
                        ),
                        Box::new(Term::Abstraction(
                            "y".into(),
                            Type::Fn(
                                Box::new(Type::Variable("a".into())),
                                Box::new(Type::Variable("b".into())),
                            ),
                            Box::new(Term::Abstraction(
                                "z".into(),
                                Type::Variable("a".into()),
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
                    )),
                )),
            )),
        );
        let expected = Type::ForAll(
            "a".into(),
            Box::new(Type::ForAll(
                "b".into(),
                Box::new(Type::ForAll(
                    "c".into(),
                    Box::new(Type::Fn(
                        Box::new(Type::Fn(
                            Box::new(Type::Variable("a".into())),
                            Box::new(Type::Fn(
                                Box::new(Type::Variable("b".into())),
                                Box::new(Type::Variable("c".into())),
                            )),
                        )),
                        Box::new(Type::Fn(
                            Box::new(Type::Fn(
                                Box::new(Type::Variable("a".into())),
                                Box::new(Type::Variable("b".into())),
                            )),
                            Box::new(Type::Fn(
                                Box::new(Type::Variable("a".into())),
                                Box::new(Type::Variable("c".into())),
                            )),
                        )),
                    )),
                )),
            )),
        );
        assert_eq!(s.type_closed(), Some(expected));
    }

    #[test]
    fn self_app() {
        let xx = Term::Abstraction(
            "x".into(),
            Type::ForAll(
                "a".into(),
                Box::new(Type::Fn(
                    Box::new(Type::Variable("a".into())),
                    Box::new(Type::Variable("a".into())),
                )),
            ),
            Box::new(Term::Application(
                Box::new(Term::TypeApplication(
                    Box::new(Term::Variable("x".into())),
                    Type::ForAll(
                        "a".into(),
                        Box::new(Type::Fn(
                            Box::new(Type::Variable("a".into())),
                            Box::new(Type::Variable("a".into())),
                        )),
                    ),
                )),
                Box::new(Term::Variable("x".into())),
            )),
        );
        let expected = Type::Fn(
            Box::new(Type::ForAll(
                "a".into(),
                Box::new(Type::Fn(
                    Box::new(Type::Variable("a".into())),
                    Box::new(Type::Variable("a".into())),
                )),
            )),
            Box::new(Type::ForAll(
                "a".into(),
                Box::new(Type::Fn(
                    Box::new(Type::Variable("a".into())),
                    Box::new(Type::Variable("a".into())),
                )),
            )),
        );
        assert_eq!(xx.type_closed(), Some(expected));
    }
}
