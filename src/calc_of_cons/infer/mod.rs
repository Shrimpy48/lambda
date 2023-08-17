use std::collections::HashSet;
use std::fmt;

pub mod parser;

use super::*;

#[derive(Debug, Clone, Eq)]
pub enum Term {
    Variable(String),
    Sort(Sort),
    Abstraction(String, Option<Box<Term>>, Box<Term>),
    Application(Box<Term>, Box<Term>),
    Product(String, Box<Term>, Box<Term>),
    Annotation(Box<Term>, Box<Term>),
}

#[derive(Debug, Clone)]
pub enum TypeError {
    Mismatch { expected: Term, actual: Term },
    NotFunType { term: Term, ty: Term },
    NotType(Term),
    NotTypeable(Term),
    UndefinedVar(String),
    CouldNotInfer(Term),
}

pub type Environment = Vec<(String, Term)>;

impl Term {
    pub fn vars(&self) -> HashSet<String> {
        match self {
            Self::Sort(_) => HashSet::new(),
            Self::Variable(v) => {
                let mut vars = HashSet::new();
                vars.insert(v.to_owned());
                vars
            }
            Self::Abstraction(v, ty, t) => {
                let mut vars = t.vars();
                if let Some(ty) = ty.as_ref() {
                    vars.extend(ty.vars());
                }
                vars.insert(v.to_owned());
                vars
            }
            Self::Product(v, ty, t) => {
                let mut vars = t.vars();
                vars.extend(ty.vars());
                vars.insert(v.to_owned());
                vars
            }
            Self::Application(t, u) => {
                let mut vars = t.vars();
                vars.extend(u.vars());
                vars
            }
            Self::Annotation(e, ty) => {
                let mut vars = e.vars();
                vars.extend(ty.vars());
                vars
            }
        }
    }

    pub fn free_vars(&self) -> HashSet<String> {
        match self {
            Self::Sort(_) => HashSet::new(),
            Self::Variable(v) => {
                let mut vars = HashSet::new();
                vars.insert(v.to_owned());
                vars
            }
            Self::Product(v, ty, t) => {
                let mut vars = t.free_vars();
                vars.remove(v);
                vars.extend(ty.free_vars());
                vars
            }
            Self::Abstraction(v, ty, t) => {
                let mut vars = t.free_vars();
                vars.remove(v);
                if let Some(ty) = ty.as_ref() {
                    vars.extend(ty.free_vars());
                }
                vars
            }
            Self::Application(t, u) => {
                let mut vars = t.free_vars();
                vars.extend(u.free_vars());
                vars
            }
            Self::Annotation(e, ty) => {
                let mut vars = e.free_vars();
                vars.extend(ty.free_vars());
                vars
            }
        }
    }

    /// A simple renaming operation ignoring shadowing.
    fn rename(&self, from: &str, to: &str) -> Self {
        match self {
            Self::Sort(_) => self.clone(),
            Self::Variable(v) if v == from => Self::Variable(to.to_owned()),
            Self::Variable(_) => self.clone(),
            Self::Abstraction(v, ty, t) => Self::Abstraction(
                if v == from {
                    to.to_owned()
                } else {
                    v.to_owned()
                },
                ty.as_ref().map(|ty| Box::new(ty.rename(from, to))),
                Box::new(t.rename(from, to)),
            ),
            Self::Product(v, ty, t) => Self::Product(
                if v == from {
                    to.to_owned()
                } else {
                    v.to_owned()
                },
                Box::new(ty.rename(from, to)),
                Box::new(t.rename(from, to)),
            ),
            Self::Application(t, u) => {
                Self::Application(Box::new(t.rename(from, to)), Box::new(u.rename(from, to)))
            }
            Self::Annotation(e, ty) => {
                Self::Annotation(Box::new(e.rename(from, to)), Box::new(ty.rename(from, to)))
            }
        }
    }

    pub fn alpha_equivalent(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Sort(x), Self::Sort(y)) => x == y,
            (Self::Variable(x), Self::Variable(y)) => x == y,
            (Self::Abstraction(x, Some(ty1), t), Self::Abstraction(y, Some(ty2), u)) => {
                let w = fresh_var(&(&self.vars() | &other.vars()));
                ty1.alpha_equivalent(ty2) && t.rename(x, &w).alpha_equivalent(&u.rename(y, &w))
            }
            (Self::Abstraction(x, None, t), Self::Abstraction(y, None, u)) => {
                let w = fresh_var(&(&self.vars() | &other.vars()));
                t.rename(x, &w).alpha_equivalent(&u.rename(y, &w))
            }
            (Self::Product(x, ty1, t), Self::Product(y, ty2, u)) => {
                let w = fresh_var(&(&self.vars() | &other.vars()));
                ty1.alpha_equivalent(ty2) && t.rename(x, &w).alpha_equivalent(&u.rename(y, &w))
            }
            (Self::Application(t1, u1), Self::Application(t2, u2)) => {
                t1.alpha_equivalent(t2) && u1.alpha_equivalent(u2)
            }
            (Self::Annotation(t1, u1), Self::Annotation(t2, u2)) => {
                t1.alpha_equivalent(t2) && u1.alpha_equivalent(u2)
            }
            _ => false,
        }
    }

    /// Substitute the given term avoiding capture.
    pub fn substitute(&self, from: &str, to: &Term) -> Self {
        match self {
            Self::Sort(_) => self.clone(),
            Self::Variable(v) if v == from => to.clone(),
            Self::Variable(_) => self.clone(),
            Self::Abstraction(v, ty, t) => {
                let w;
                let new_body;
                if v == from {
                    w = v.to_owned();
                    new_body = Box::new(t.as_ref().clone());
                } else {
                    let mut vars = self.vars();
                    vars.extend(to.vars());
                    vars.insert(from.to_owned());
                    w = fresh_var(&vars);
                    new_body = Box::new(t.rename(v, &w).substitute(from, to));
                }
                Self::Abstraction(
                    w,
                    ty.as_ref().map(|ty| Box::new(ty.substitute(from, to))),
                    new_body,
                )
            }
            Self::Product(v, ty, t) => {
                let w;
                let new_body;
                if v == from {
                    w = v.to_owned();
                    new_body = Box::new(t.as_ref().clone());
                } else {
                    let mut vars = self.vars();
                    vars.extend(to.vars());
                    vars.insert(from.to_owned());
                    w = fresh_var(&vars);
                    new_body = Box::new(t.rename(v, &w).substitute(from, to));
                }
                Self::Product(w, Box::new(ty.substitute(from, to)), new_body)
            }
            Self::Application(t, u) => Self::Application(
                Box::new(t.substitute(from, to)),
                Box::new(u.substitute(from, to)),
            ),
            Self::Annotation(t, u) => Self::Annotation(
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
            Self::Abstraction(v, ty, t) => {
                if let Some(t2) = t.beta_reduce_lazy() {
                    Some(Self::Abstraction(
                        v.to_owned(),
                        ty.as_ref().map(|ty| Box::new(ty.as_ref().clone())),
                        Box::new(t2),
                    ))
                } else {
                    ty.as_ref().and_then(|ty| ty.beta_reduce_lazy()).map(|ty2| {
                        Self::Abstraction(
                            v.to_owned(),
                            Some(Box::new(ty2)),
                            Box::new(t.as_ref().clone()),
                        )
                    })
                }
            }
            Self::Product(v, ty, t) => {
                if let Some(t2) = t.beta_reduce_lazy() {
                    Some(Self::Product(
                        v.to_owned(),
                        Box::new(ty.as_ref().clone()),
                        Box::new(t2),
                    ))
                } else {
                    ty.beta_reduce_lazy().map(|ty2| {
                        Self::Product(v.to_owned(), Box::new(ty2), Box::new(t.as_ref().clone()))
                    })
                }
            }
            Self::Annotation(e, _ty) => Some(e.as_ref().clone()),
            Self::Variable(_) | Self::Sort(_) => None,
        }
    }

    /// Fully beta-reduce the term. If the term is well-typed, this will halt.
    pub fn evaluate(&self) -> Self {
        let mut res = self.clone();
        while let Some(new_res) = res.beta_reduce_lazy() {
            res = new_res;
        }
        res
    }

    // TODO: make sure types are evaluated when necessary.
    /// The type of this term in the given environment, if the term is well-typed and the type is inferrable.
    pub fn synthesise_type_in(&self, env: &Environment) -> Result<Term, TypeError> {
        match self {
            Self::Sort(Sort::Type) => Ok(Term::Sort(Sort::Universal)),
            Self::Variable(x) => env
                .iter()
                .rev()
                .find(|(v, _)| v == x)
                .map(|(_, t)| t.clone())
                .ok_or(TypeError::UndefinedVar(x.clone())),
            Self::Application(t, u) => match t.synthesise_type_in(env)? {
                Term::Product(v, a1, b) => {
                    u.check_type_in(env, &a1)?;
                    Ok(b.substitute(&v, u).evaluate())
                }
                ty => Err(TypeError::NotFunType {
                    term: t.as_ref().clone(),
                    ty,
                }),
            },
            Self::Product(x, ty, t) => {
                if !matches!(ty.synthesise_type_in(env)?, Term::Sort(_)) {
                    return Err(TypeError::NotType(ty.as_ref().clone()));
                }
                let mut inner_env = env.clone();
                inner_env.push((x.to_owned(), ty.as_ref().clone()));
                let b = t.synthesise_type_in(&inner_env)?;
                if !matches!(b, Term::Sort(_)) {
                    return Err(TypeError::NotType(b));
                }
                Ok(b)
            }
            Self::Abstraction(x, ty, t) => {
                if let Some(ty) = ty {
                    if !matches!(ty.synthesise_type_in(env)?, Term::Sort(_)) {
                        return Err(TypeError::NotType(ty.as_ref().clone()));
                    }
                    let mut inner_env = env.clone();
                    inner_env.push((x.to_owned(), ty.as_ref().clone()));
                    let b = t.synthesise_type_in(&inner_env)?;
                    if !matches!(b.synthesise_type_in(&inner_env)?, Term::Sort(_)) {
                        return Err(TypeError::NotType(b));
                    }
                    Ok(Term::Product(
                        x.to_owned(),
                        Box::new(ty.as_ref().clone()),
                        Box::new(b),
                    ))
                } else {
                    Err(TypeError::CouldNotInfer(self.clone()))
                }
            }
            Self::Annotation(e, ty) => {
                if !matches!(ty.synthesise_type_in(env)?, Term::Sort(_)) {
                    return Err(TypeError::NotType(ty.as_ref().clone()));
                }
                e.check_type_in(env, ty)?;
                return Ok(ty.as_ref().clone());
            }
            Self::Sort(Sort::Universal) => Err(TypeError::NotTypeable(self.clone())),
        }
    }

    /// Assert that this term inhabits the given type.
    pub fn check_type_in(&self, env: &Environment, ty: &Term) -> Result<(), TypeError> {
        match self {
            Self::Abstraction(x, None, t) => {
                if let Term::Product(_, a, b) = ty {
                    let mut inner_env = env.clone();
                    inner_env.push((x.to_owned(), a.as_ref().clone()));
                    return t.check_type_in(&inner_env, b);
                } else {
                    return Err(TypeError::NotFunType {
                        term: self.clone(),
                        ty: ty.clone(),
                    });
                }
            }
            _ => {}
        }
        let actual = self.synthesise_type_in(env)?;
        if ty == &actual {
            Ok(())
        } else {
            Err(TypeError::Mismatch {
                expected: ty.clone(),
                actual,
            })
        }
    }

    /// The type of this term in the empty environment, if it is well-typed.
    pub fn synthesise_type_closed(&self) -> Result<Term, TypeError> {
        self.synthesise_type_in(&Environment::new())
    }

    /// Assert that this term inhabits the given type in the empty environment.
    pub fn check_type_closed(&self, ty: &Term) -> Result<(), TypeError> {
        self.check_type_in(&Environment::new(), ty)
    }
}

impl PartialEq for Term {
    /// Alpha equivalence.
    fn eq(&self, other: &Self) -> bool {
        self.alpha_equivalent(other)
    }
}

impl fmt::Display for Term {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Sort(Sort::Type) => write!(f, "*"),
            Self::Sort(Sort::Universal) => write!(f, "□"),
            Self::Variable(x) => x.fmt(f),
            Self::Abstraction(x, Some(ty), t) => write!(f, "λ{}: {}. {}", x, ty, t),
            Self::Abstraction(x, None, t) => write!(f, "λ{}. {}", x, t),
            Self::Product(x, ty, t) if !t.free_vars().contains(x) => {
                write_term(ty, f)?;
                write!(f, " -> {}", t)
            }
            Self::Product(x, ty, t) => write!(f, "Π{}: {}. {}", x, ty, t),
            Self::Application(t, u) => {
                write_func(t, f)?;
                write!(f, " ")?;
                write_term(u, f)
            }
            Self::Annotation(e, ty) => {
                write_term(e, f)?;
                write!(f, ": ")?;
                write_term(ty, f)
            }
        }
    }
}

fn write_term(t: &Term, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match t {
        Term::Variable(_) | Term::Sort(_) => fmt::Display::fmt(t, f),
        _ => write!(f, "({})", t),
    }
}

fn write_func(t: &Term, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match t {
        Term::Variable(_) | Term::Sort(_) | Term::Application(_, _) => fmt::Display::fmt(t, f),
        _ => write!(f, "({})", t),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}
