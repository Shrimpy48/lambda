use std::collections::HashSet;
use std::fmt;

use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Sort {
    Type,
    Universal,
}

/// A term in the lambda cube.
///
/// F means terms may depend on types (parametric polymorphism).
/// W means types may depend on types (type constructors).
/// P means types may depend on terms (dependent types).
#[derive(Debug, Clone, Eq)]
pub enum Term<const F: bool, const W: bool, const P: bool> {
    Variable(String),
    Sort(Sort),
    Abstraction(String, Box<Term<F, W, P>>, Box<Term<F, W, P>>),
    Application(Box<Term<F, W, P>>, Box<Term<F, W, P>>),
    Product(String, Box<Term<F, W, P>>, Box<Term<F, W, P>>),
}

pub type Environment<const F: bool, const W: bool, const P: bool> = Vec<(String, Term<F, W, P>)>;

impl<const F: bool, const W: bool, const P: bool> Term<F, W, P> {
    pub fn vars(&self) -> HashSet<String> {
        match self {
            Self::Sort(_) => HashSet::new(),
            Self::Variable(v) => {
                let mut vars = HashSet::new();
                vars.insert(v.to_owned());
                vars
            }
            Self::Abstraction(v, ty, t) | Self::Product(v, ty, t) => {
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
            Self::Abstraction(v, ty, t) | Self::Product(v, ty, t) => {
                let mut vars = t.free_vars();
                vars.remove(v);
                vars.extend(ty.free_vars());
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
            Self::Sort(_) => self.clone(),
            Self::Variable(v) if v == from => Self::Variable(to.to_owned()),
            Self::Variable(_) => self.clone(),
            Self::Abstraction(v, ty, t) => Self::Abstraction(
                if v == from {
                    to.to_owned()
                } else {
                    v.to_owned()
                },
                Box::new(ty.rename(from, to)),
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
        }
    }

    pub fn alpha_equivalent(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Sort(x), Self::Sort(y)) => x == y,
            (Self::Variable(x), Self::Variable(y)) => x == y,
            (Self::Abstraction(x, ty1, t), Self::Abstraction(y, ty2, u))
            | (Self::Product(x, ty1, t), Self::Product(y, ty2, u)) => {
                let w = fresh_var(&(&self.vars() | &other.vars()));
                ty1.alpha_equivalent(ty2) && t.rename(x, &w).alpha_equivalent(&u.rename(y, &w))
            }
            (Self::Application(t1, u1), Self::Application(t2, u2)) => {
                t1.alpha_equivalent(t2) && u1.alpha_equivalent(u2)
            }
            _ => false,
        }
    }

    /// Substitute the given term avoiding capture.
    pub fn substitute(&self, from: &str, to: &Term<F, W, P>) -> Self {
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
                Self::Abstraction(w, Box::new(ty.substitute(from, to)), new_body)
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
            Self::Abstraction(v, ty, t) => {
                if let Some(t2) = t.beta_reduce_lazy() {
                    Some(Self::Abstraction(
                        v.to_owned(),
                        Box::new(ty.as_ref().clone()),
                        Box::new(t2),
                    ))
                } else if let Some(ty2) = ty.beta_reduce_lazy() {
                    Some(Self::Abstraction(
                        v.to_owned(),
                        Box::new(ty2),
                        Box::new(t.as_ref().clone()),
                    ))
                } else {
                    None
                }
            }
            Self::Product(v, ty, t) => {
                if let Some(t2) = t.beta_reduce_lazy() {
                    Some(Self::Product(
                        v.to_owned(),
                        Box::new(ty.as_ref().clone()),
                        Box::new(t2),
                    ))
                } else if let Some(ty2) = ty.beta_reduce_lazy() {
                    Some(Self::Product(
                        v.to_owned(),
                        Box::new(ty2),
                        Box::new(t.as_ref().clone()),
                    ))
                } else {
                    None
                }
            }
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

    /// The type of this term in the given environment, if it is well-typed.
    pub fn type_in(&self, env: &Environment<F, W, P>) -> Option<Term<F, W, P>> {
        match self {
            Self::Sort(Sort::Type) => Some(Term::Sort(Sort::Universal)),
            Self::Variable(x) => env
                .iter()
                .rev()
                .find(|(v, _)| v == x)
                .map(|(_, t)| t.clone()),
            Self::Application(t, u) => {
                if let Some(Term::Product(v, a1, b)) = t.type_in(env) {
                    let a2 = u.type_in(env)?;
                    if a1.as_ref() == &a2 {
                        Some(b.substitute(&v, &u).evaluate())
                    } else {
                        None
                    }
                } else {
                    None
                }
            }
            Self::Product(x, ty, t) => {
                let s1 = ty.type_in(env)?;
                let mut inner_env = env.clone();
                inner_env.push((x.to_owned(), ty.as_ref().clone()));
                let s2 = t.type_in(&inner_env)?;
                match (&s1, &s2) {
                    (Term::Sort(Sort::Type), Term::Sort(Sort::Type)) => {}
                    (Term::Sort(Sort::Type), Term::Sort(Sort::Universal)) => {
                        if !P {
                            return None;
                        }
                    }
                    (Term::Sort(Sort::Universal), Term::Sort(Sort::Type)) => {
                        if !F {
                            return None;
                        }
                    }
                    (Term::Sort(Sort::Universal), Term::Sort(Sort::Universal)) => {
                        if !W {
                            return None;
                        }
                    }
                    _ => return None,
                }
                Some(s2)
            }
            Self::Abstraction(x, ty, t) => {
                let s1 = ty.type_in(env)?;
                let mut inner_env = env.clone();
                inner_env.push((x.to_owned(), ty.as_ref().clone()));
                let b = t.type_in(&inner_env)?;
                let s2 = b.type_in(&inner_env)?;
                match (&s1, &s2) {
                    (Term::Sort(Sort::Type), Term::Sort(Sort::Type)) => {}
                    (Term::Sort(Sort::Type), Term::Sort(Sort::Universal)) => {
                        if !P {
                            return None;
                        }
                    }
                    (Term::Sort(Sort::Universal), Term::Sort(Sort::Type)) => {
                        if !F {
                            return None;
                        }
                    }
                    (Term::Sort(Sort::Universal), Term::Sort(Sort::Universal)) => {
                        if !W {
                            return None;
                        }
                    }
                    _ => return None,
                }
                Some(Term::Product(
                    x.to_owned(),
                    Box::new(ty.as_ref().clone()),
                    Box::new(b),
                ))
            }
            Self::Sort(Sort::Universal) => None,
        }
    }

    /// The type of this term in the empty environment, if it is well-typed.
    pub fn type_closed(&self) -> Option<Term<F, W, P>> {
        self.type_in(&Environment::new())
    }
}

impl<const F: bool, const W: bool, const P: bool> PartialEq for Term<F, W, P> {
    /// Alpha equivalence.
    fn eq(&self, other: &Self) -> bool {
        self.alpha_equivalent(other)
    }
}

impl<const F: bool, const W: bool, const P: bool> fmt::Display for Term<F, W, P> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Sort(Sort::Type) => write!(f, "*"),
            Self::Sort(Sort::Universal) => write!(f, "□"),
            Self::Variable(x) => x.fmt(f),
            Self::Abstraction(x, ty, t) => write!(f, "λ{}: {}. {}", x, ty, t),
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
        }
    }
}

fn write_term<const F: bool, const W: bool, const P: bool>(
    t: &Term<F, W, P>,
    f: &mut fmt::Formatter<'_>,
) -> fmt::Result {
    match t {
        Term::Variable(_) | Term::Sort(_) => fmt::Display::fmt(t, f),
        _ => write!(f, "({})", t),
    }
}

fn write_func<const F: bool, const W: bool, const P: bool>(
    t: &Term<F, W, P>,
    f: &mut fmt::Formatter<'_>,
) -> fmt::Result {
    match t {
        Term::Variable(_) | Term::Sort(_) | Term::Application(_, _) => fmt::Display::fmt(t, f),
        _ => write!(f, "({})", t),
    }
}

#[cfg(test)]
mod tests {}
