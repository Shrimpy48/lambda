use std::collections::HashSet;
use std::fmt;

use super::fresh_var;

#[derive(Debug, Clone, Eq)]
pub enum Type {
    Top,
    Variable(String),
    ForAll(String, Box<Type>, Box<Type>),
    Fn(Box<Type>, Box<Type>),
}

impl Type {
    pub fn vars(&self) -> HashSet<String> {
        match self {
            Self::Top => HashSet::new(),
            Self::Variable(v) => {
                let mut vars = HashSet::new();
                vars.insert(v.to_owned());
                vars
            }
            Self::ForAll(v, bound, t) => {
                let mut vars = t.vars();
                vars.extend(bound.vars());
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
            Self::Top => HashSet::new(),
            Self::Variable(v) => {
                let mut vars = HashSet::new();
                vars.insert(v.to_owned());
                vars
            }
            Self::ForAll(v, bound, t) => {
                let mut vars = t.free_vars();
                vars.extend(bound.free_vars());
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
            Self::Top => Self::Top,
            Self::Variable(v) if v == from => Self::Variable(to.to_owned()),
            Self::Variable(_) => self.clone(),
            Self::ForAll(v, bound, t) => Self::ForAll(
                if v == from {
                    to.to_owned()
                } else {
                    v.to_owned()
                },
                Box::new(bound.rename(from, to)),
                Box::new(t.rename(from, to)),
            ),
            Self::Fn(t, u) => Self::Fn(Box::new(t.rename(from, to)), Box::new(u.rename(from, to))),
        }
    }

    /// Substitute the given type avoiding capture.
    pub fn substitute(&self, from: &str, to: &Type) -> Self {
        match self {
            Self::Top => Self::Top,
            Self::Variable(v) if v == from => to.to_owned(),
            Self::Variable(_) => self.clone(),
            Self::ForAll(v, _, _) if v == from => self.clone(),
            Self::ForAll(v, b, t) => {
                let mut vars = self.vars();
                vars.extend(to.vars());
                vars.insert(from.to_owned());
                let w = fresh_var(&vars);
                let new_body = Box::new(t.rename(v, &w).substitute(from, to));
                let new_bound = Box::new(b.rename(v, &w).substitute(from, to));
                Self::ForAll(w, new_bound, new_body)
            }
            Self::Fn(t, u) => Self::Fn(
                Box::new(t.substitute(from, to)),
                Box::new(u.substitute(from, to)),
            ),
        }
    }

    pub fn alpha_equivalent(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Top, Self::Top) => true,
            (Self::Variable(x), Self::Variable(y)) => x == y,
            (Self::ForAll(x, b, t), Self::ForAll(y, c, u)) => {
                let w = fresh_var(&(&self.vars() | &other.vars()));
                b.rename(x, &w).alpha_equivalent(&c.rename(y, &w))
                    && t.rename(x, &w).alpha_equivalent(&u.rename(y, &w))
            }
            (Self::Fn(t1, u1), Self::Fn(t2, u2)) => {
                t1.alpha_equivalent(t2) && u1.alpha_equivalent(u2)
            }
            _ => false,
        }
    }

    pub fn is_subtype_in(&self, type_env: &TypeEnvironment, other: &Self) -> bool {
        if self == other || other == &Self::Top {
            true
        } else {
            match (self, other) {
                (Self::Variable(x), other) => {
                    if let Some(ty) = type_env
                        .type_variables
                        .iter()
                        .rev()
                        .find(|(v, _)| v == x)
                        .map(|(_, t)| t)
                    {
                        ty.is_subtype_in(type_env, other)
                    } else {
                        false
                    }
                }
                (Self::Fn(a1, b1), Self::Fn(a2, b2)) => {
                    a2.is_subtype_in(type_env, a1) && b1.is_subtype_in(type_env, b2)
                }
                (Self::ForAll(x, a1, b1), Self::ForAll(y, a2, b2)) => {
                    let w = fresh_var(&(&self.vars() | &other.vars()));
                    let mut inner_env = type_env.clone();
                    inner_env
                        .type_variables
                        .push((w.to_owned(), a2.as_ref().clone()));
                    a2.is_subtype_in(type_env, a1)
                        && b1
                            .rename(x, &w)
                            .is_subtype_in(&inner_env, &b2.rename(y, &w))
                }
                (_, _) => false,
            }
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
    Top,
    Variable(String),
    Abstraction(String, Type, Box<Term>),
    Application(Box<Term>, Box<Term>),
    TypeAbstraction(String, Type, Box<Term>),
    TypeApplication(Box<Term>, Type),
}

#[derive(Debug, Clone)]
pub struct TypeEnvironment {
    term_variables: Vec<(String, Type)>,
    type_variables: Vec<(String, Type)>,
}

impl TypeEnvironment {
    fn new() -> TypeEnvironment {
        Self {
            term_variables: Vec::new(),
            type_variables: Vec::new(),
        }
    }

    // fn lowest_common_supertype(&self, a: &Type, b: &Type) -> Type {
    //     todo!()
    // }
}

impl Term {
    pub fn vars(&self) -> HashSet<String> {
        match self {
            Self::Top => HashSet::new(),
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
            Self::TypeAbstraction(_, _, t) => t.vars(),
            Self::TypeApplication(t, _) => t.vars(),
        }
    }

    pub fn free_vars(&self) -> HashSet<String> {
        match self {
            Self::Top => HashSet::new(),
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
            Self::TypeAbstraction(_, _, t) => t.free_vars(),
            Self::TypeApplication(t, _) => t.free_vars(),
        }
    }

    /// A simple renaming operation ignoring shadowing.
    fn rename(&self, from: &str, to: &str) -> Self {
        match self {
            Self::Top => self.clone(),
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
            Self::TypeAbstraction(a, b, t) => {
                Self::TypeAbstraction(a.to_owned(), b.clone(), Box::new(t.rename(from, to)))
            }
            Self::TypeApplication(t, a) => {
                Self::TypeApplication(Box::new(t.rename(from, to)), a.to_owned())
            }
        }
    }

    pub fn type_vars(&self) -> HashSet<String> {
        match self {
            Self::Top => HashSet::new(),
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
            Self::TypeAbstraction(v, b, t) => {
                let mut vars = t.type_vars();
                vars.extend(b.vars());
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
            Self::Top => HashSet::new(),
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
            Self::TypeAbstraction(v, b, t) => {
                let mut vars = t.free_type_vars();
                vars.extend(b.free_vars());
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
            Self::Top => self.clone(),
            Self::Variable(_) => self.clone(),
            Self::TypeAbstraction(v, b, t) => Self::TypeAbstraction(
                if v == from {
                    to.to_owned()
                } else {
                    v.to_owned()
                },
                b.rename(from, to),
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
            Self::Top => self.clone(),
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
            Self::TypeAbstraction(v, b, t) if v == from => {
                Self::TypeAbstraction(v.to_owned(), b.substitute(from, to), t.clone())
            }
            Self::TypeAbstraction(v, b, t) => {
                let mut vars = self.type_vars();
                vars.extend(to.vars());
                vars.insert(from.to_owned());
                let w = fresh_var(&vars);
                let new_bound = b.rename(v, &w).substitute(from, to);
                let new_body = Box::new(t.type_rename(v, &w).type_substitute(from, to));
                Self::TypeAbstraction(w, new_bound, new_body)
            }
            Self::TypeApplication(t, u) => Self::TypeApplication(
                Box::new(t.type_substitute(from, to)),
                u.substitute(from, to),
            ),
        }
    }

    pub fn alpha_equivalent(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Top, Self::Top) => true,
            (Self::Variable(x), Self::Variable(y)) => x == y,
            (Self::Abstraction(x, ty1, t), Self::Abstraction(y, ty2, u)) => {
                let w = fresh_var(&(&self.vars() | &other.vars()));
                ty1.alpha_equivalent(ty2) && t.rename(x, &w).alpha_equivalent(&u.rename(y, &w))
            }
            (Self::Application(t1, u1), Self::Application(t2, u2)) => {
                t1.alpha_equivalent(t2) && u1.alpha_equivalent(u2)
            }
            (Self::TypeAbstraction(x, b, t), Self::TypeAbstraction(y, c, u)) => {
                let w = fresh_var(&(&self.type_vars() | &other.type_vars()));
                b.rename(x, &w).alpha_equivalent(&c.rename(y, &w))
                    && t.type_rename(x, &w).alpha_equivalent(&u.type_rename(y, &w))
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
            Self::Top => self.clone(),
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
            Self::TypeAbstraction(a, b, t) => {
                Self::TypeAbstraction(a.to_owned(), b.clone(), Box::new(t.substitute(from, to)))
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
                } else {
                    u.beta_reduce_lazy()
                        .map(|u2| Self::Application(Box::new(t.as_ref().clone()), Box::new(u2)))
                }
            }
            Self::TypeApplication(t, u) => {
                if let Self::TypeAbstraction(x, _, b) = t.as_ref() {
                    Some(b.type_substitute(x, u))
                } else {
                    t.beta_reduce_lazy()
                        .map(|t2| Self::TypeApplication(Box::new(t2), u.clone()))
                }
            }
            Self::Abstraction(x, ty, t) => t
                .beta_reduce_lazy()
                .map(|t2| Self::Abstraction(x.to_owned(), ty.clone(), Box::new(t2))),
            Self::TypeAbstraction(x, b, t) => t
                .beta_reduce_lazy()
                .map(|t2| Self::TypeAbstraction(x.to_owned(), b.clone(), Box::new(t2))),
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
                if let Self::TypeAbstraction(x, _, b) = t.as_ref() {
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
            Self::TypeAbstraction(x, b, t) => t
                .parallel_reduct()
                .map(|t2| Self::TypeAbstraction(x.to_owned(), b.clone(), Box::new(t2))),
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
            Self::TypeAbstraction(x, b, t) => {
                if let Self::TypeApplication(f, Type::Variable(y)) = t.as_ref() {
                    if x == y && !f.free_vars().contains(x) {
                        return Some(f.as_ref().clone());
                    }
                }
                t.eta_reduce_lazy()
                    .map(|u2| Self::TypeAbstraction(x.to_owned(), b.clone(), Box::new(u2)))
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
            Self::TypeApplication(t, u) => t
                .eta_reduce_lazy()
                .map(|t2| Self::TypeApplication(Box::new(t2), u.clone())),
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
            Self::Top => Some(Type::Top),
            Self::Variable(x) => type_env
                .term_variables
                .iter()
                .rev()
                .find(|(v, _)| v == x)
                .map(|(_, t)| t.clone()),
            Self::Abstraction(x, ty, t) => {
                if !ty.free_vars().is_subset(
                    &type_env
                        .type_variables
                        .iter()
                        .map(|(v, _)| v.clone())
                        .collect(),
                ) {
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
                    if a2.is_subtype_in(type_env, a1.as_ref()) {
                        Some(b.as_ref().clone())
                    } else {
                        None
                    }
                } else {
                    None
                }
            }
            Self::TypeAbstraction(a, b, t) => {
                if !b.free_vars().is_subset(
                    &type_env
                        .type_variables
                        .iter()
                        .map(|(v, _)| v.clone())
                        .collect(),
                ) {
                    return None;
                }
                let mut inner_env = type_env.clone();
                inner_env.type_variables.push((a.to_owned(), b.clone()));
                t.type_in(&inner_env)
                    .map(|t| Type::ForAll(a.to_owned(), Box::new(b.clone()), Box::new(t)))
            }
            Self::TypeApplication(t, u) => {
                if let Some(Type::ForAll(v, bound, body)) = t.type_in(type_env) {
                    if u.is_subtype_in(type_env, bound.as_ref()) {
                        Some(body.substitute(&v, u))
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

    // // TODO: check if equalities should be subtypes.
    // pub fn equivalent_at_type(&self, type_env: &TypeEnvironment, other: &Self) -> Option<Type> {
    //     match (self, other) {
    //         (Self::Top, Self::Top) => Some(Type::Top),
    //         (Self::Variable(x), Self::Variable(y)) if x == y => type_env
    //             .term_variables
    //             .iter()
    //             .rev()
    //             .find(|(v, _)| v == x)
    //             .map(|(_, t)| t.clone()),
    //         (Self::Abstraction(x, ty1, t), Self::Abstraction(y, ty2, u)) if ty1 == ty2 => {
    //             let w = fresh_var(&(&self.vars() | &other.vars()));
    //             let mut inner_env = type_env.clone();
    //             inner_env.term_variables.push((w.to_owned(), ty1.clone()));
    //             let b_ty = t
    //                 .rename(x, &w)
    //                 .equivalent_at_type(&inner_env, &u.rename(y, &w))?;
    //             Some(Type::Fn(Box::new(ty1.clone()), Box::new(b_ty)))
    //         }
    //         (Self::Application(t1, u1), Self::Application(t2, u2)) => {
    //             let Type::Fn(a1, b) = t1.equivalent_at_type(type_env, t2)? else {return None };
    //             let a2 = u1.equivalent_at_type(type_env, u2)?;
    //             if *a1 == a2 {
    //                 Some(*b)
    //             } else {
    //                 None
    //             }
    //         }
    //         (Self::TypeAbstraction(x, ty1, t), Self::TypeAbstraction(y, ty2, u)) if ty1 == ty2 => {
    //             let w = fresh_var(&(&self.type_vars() | &other.type_vars()));
    //             let mut inner_env = type_env.clone();
    //             inner_env.type_variables.push((w.to_owned(), ty1.clone()));
    //             let body_ty = t
    //                 .type_rename(x, &w)
    //                 .equivalent_at_type(&inner_env, &u.type_rename(y, &w))?;
    //             Some(Type::ForAll(w, Box::new(ty1.clone()), Box::new(body_ty)))
    //         }
    //         (Self::TypeApplication(t1, u1), Self::TypeApplication(t2, u2)) => {
    //             let Type::ForAll(x, u, b) = t1.equivalent_at_type(type_env, t2)? else {return None };
    //             if u1.is_subtype_in(type_env, &u) && u2.is_subtype_in(type_env, &u) {
    //                 let c = type_env
    //                     .lowest_common_supertype(&b.substitute(&x, u1), &b.substitute(&x, u2));
    //                 Some(c)
    //             } else {
    //                 None
    //             }
    //         }
    //         _ => todo!("eta and beta not yet implemented"),
    //     }
    // }
}

impl PartialEq for Term {
    fn eq(&self, other: &Self) -> bool {
        self.alpha_equivalent(other)
    }
}

impl fmt::Display for Term {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Top => write!(f, "top"),
            Self::Variable(x) => x.fmt(f),
            Self::Abstraction(x, ty, t) => write!(f, "λ{}: {}. {}", x, ty, t),
            Self::Application(t, u) => {
                write_func(t, f)?;
                write!(f, " ")?;
                write_term(u, f)
            }
            Self::TypeAbstraction(x, b, t) => write!(f, "Λ{} <: {}. {}", x, b, t),
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
        Term::Variable(_) | Term::Top => fmt::Display::fmt(t, f),
        _ => write!(f, "({})", t),
    }
}

fn write_func(t: &Term, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match t {
        Term::Variable(_) | Term::Top | Term::Application(_, _) | Term::TypeApplication(_, _) => {
            fmt::Display::fmt(t, f)
        }
        _ => write!(f, "({})", t),
    }
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Top => write!(f, "Top"),
            Self::Variable(x) => x.fmt(f),
            Self::Fn(t, u) => {
                write_ty(t, f)?;
                write!(f, " -> {}", u)
            }
            Self::ForAll(a, b, t) => write!(f, "∀{} <: {}. {}", a, b, t),
        }
    }
}

fn write_ty(ty: &Type, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match ty {
        Type::Top | Type::Variable(_) => fmt::Display::fmt(ty, f),
        Type::Fn(_, _) | Type::ForAll(_, _, _) => write!(f, "({})", ty),
    }
}

impl fmt::Display for TypeEnvironment {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut any_elems = false;
        for (v, b) in &self.type_variables {
            if any_elems {
                write!(f, ", {v} <: {b}")?;
            } else {
                write!(f, "{v} <: {b}")?;
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

#[cfg(test)]
mod tests {}
