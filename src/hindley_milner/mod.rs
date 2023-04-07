use std::collections::{HashMap, HashSet};
use std::fmt;

use super::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Type {
    Variable(String),
    GenVariable(String),
    Fn(Box<Type>, Box<Type>),
}

impl Type {
    pub fn substitute(&self, mapping: &HashMap<String, Self>) -> Self {
        match self {
            Self::Variable(v) => {
                if let Some(t) = mapping.get(v) {
                    t.clone()
                } else {
                    self.clone()
                }
            }
            Self::GenVariable(_) => self.clone(),
            Self::Fn(t, u) => Self::Fn(
                Box::new(t.substitute(mapping)),
                Box::new(u.substitute(mapping)),
            ),
        }
    }

    pub fn gen_substitute(&self, mapping: &HashMap<String, Self>) -> Self {
        match self {
            Self::GenVariable(v) => {
                if let Some(t) = mapping.get(v) {
                    t.clone()
                } else {
                    self.clone()
                }
            }
            Self::Variable(_) => self.clone(),
            Self::Fn(t, u) => Self::Fn(
                Box::new(t.gen_substitute(mapping)),
                Box::new(u.gen_substitute(mapping)),
            ),
        }
    }

    pub fn vars(&self) -> HashSet<String> {
        match self {
            Self::Variable(v) => [v.to_owned()].into(),
            Self::GenVariable(_) => HashSet::new(),
            Self::Fn(t, u) => {
                let mut vs = t.vars();
                vs.extend(u.vars());
                vs
            }
        }
    }

    pub fn gen_vars(&self) -> HashSet<String> {
        match self {
            Self::GenVariable(v) => [v.to_owned()].into(),
            Self::Variable(_) => HashSet::new(),
            Self::Fn(t, u) => {
                let mut vs = t.gen_vars();
                vs.extend(u.gen_vars());
                vs
            }
        }
    }

    pub fn contains_var(&self, var: &str) -> bool {
        match self {
            Self::Variable(v) => v == var,
            Self::GenVariable(_) => false,
            Self::Fn(t, u) => t.contains_var(var) || u.contains_var(var),
        }
    }

    fn instantiate(&self, var_count: &mut usize) -> Self {
        let mapping = self
            .gen_vars()
            .into_iter()
            .map(|gv| {
                let tv = Self::Variable(format!("t{}", var_count));
                *var_count += 1;
                (gv, tv)
            })
            .collect();
        self.gen_substitute(&mapping)
    }

    fn generalise(&self, free_vars: &HashSet<String>) -> Self {
        match self {
            Self::Variable(v) if !free_vars.contains(v) => Self::GenVariable(v.to_owned()),
            Self::Fn(t, u) => Self::Fn(
                Box::new(t.generalise(free_vars)),
                Box::new(u.generalise(free_vars)),
            ),
            _ => self.clone(),
        }
    }
}

fn unify(mut equalities: Vec<(Type, Type)>) -> Option<HashMap<String, Type>> {
    let mut mapping: HashMap<String, Type> = HashMap::new();
    while let Some(equality) = equalities.pop() {
        match equality {
            (lhs, rhs) if lhs == rhs => continue,
            (Type::Fn(t1, u1), Type::Fn(t2, u2)) => {
                equalities.push((t1.as_ref().clone(), t2.as_ref().clone()));
                equalities.push((u1.as_ref().clone(), u2.as_ref().clone()));
            }
            (Type::Variable(v), t) | (t, Type::Variable(v)) if !t.contains_var(&v) => {
                let new_mapping = [(v, t)].into();
                for (a, b) in equalities.iter_mut() {
                    *a = a.substitute(&new_mapping);
                    *b = b.substitute(&new_mapping);
                }
                for b in mapping.values_mut() {
                    *b = b.substitute(&new_mapping);
                }
                mapping.extend(new_mapping);
            }
            _ => return None,
        }
    }
    Some(mapping)
}

#[derive(Debug, Clone, Eq)]
pub enum Term {
    Variable(String),
    Abstraction(String, Box<Term>),
    Application(Box<Term>, Box<Term>),
    Let(String, Box<Term>, Box<Term>),
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
            Self::Let(v, b, t) => {
                let mut vars = t.vars();
                vars.extend(b.vars());
                vars.insert(v.to_owned());
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
            Self::Let(v, b, t) => {
                let mut vars = t.free_vars();
                vars.remove(v);
                vars.extend(b.free_vars());
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
            Self::Let(v, b, t) => Self::Let(
                if v == from {
                    to.to_owned()
                } else {
                    v.to_owned()
                },
                Box::new(b.rename(from, to)),
                Box::new(t.rename(from, to)),
            ),
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
            (Self::Let(x, b1, t), Self::Let(y, b2, u)) => {
                let w = fresh_var(&(&self.vars() | &other.vars()));
                b1.alpha_equivalent(b2) && t.rename(x, &w).alpha_equivalent(&u.rename(y, &w))
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
            Self::Let(v, _, _) if v == from => self.clone(),
            Self::Let(v, b, t) => {
                let mut vars = self.vars();
                vars.extend(to.vars());
                vars.insert(from.to_owned());
                let w = fresh_var(&vars);
                let new_body = Box::new(t.rename(v, &w).substitute(from, to));
                Self::Let(w, Box::new(b.substitute(from, to)), new_body)
            }
        }
    }

    /// Beta-reduce the left-outermost redex if one exists ("normal order").
    /// This does not check the type.
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
            Self::Let(x, b, t) => Some(t.substitute(x, b)),
            _ => None,
        }
    }

    /// Beta-reduce all redexes simultaneously if any exist.
    /// This does not check the type.
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
            Self::Let(x, b, t) => {
                let b2 = b.parallel_reduct().unwrap_or_else(|| b.as_ref().clone());
                let t2 = t.parallel_reduct().unwrap_or_else(|| t.as_ref().clone());
                Some(t2.substitute(x, &b2))
            }
            _ => None,
        }
    }

    /// Eta-reduce the left-outermost simplification if one exists.
    /// This does not check the type.
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
            Self::Let(x, b, t) => {
                if let Some(b2) = b.eta_reduce_lazy() {
                    Some(Self::Let(
                        x.to_owned(),
                        Box::new(b2),
                        Box::new(t.as_ref().clone()),
                    ))
                } else {
                    t.eta_reduce_lazy().map(|t2| {
                        Self::Let(x.to_owned(), Box::new(b.as_ref().clone()), Box::new(t2))
                    })
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

    /// Infer the most general type of this term in the given type environment, if it is well-typed.
    pub fn type_in(&self, type_env: &TypeEnvironment) -> Option<Type> {
        let mut constraints = Vec::new();
        let ty = self.type_constraints(type_env, &mut constraints, &mut 0)?;
        let solution = unify(constraints)?;
        Some(ty.substitute(&solution))
    }

    fn type_constraints(
        &self,
        type_env: &TypeEnvironment,
        constraints: &mut Vec<(Type, Type)>,
        var_count: &mut usize,
    ) -> Option<Type> {
        match self {
            Self::Variable(x) => {
                let t = type_env
                    .iter()
                    .rev()
                    .find(|(v, _)| v == x)
                    .map(|(_, t)| t.clone())?;
                let t2 = t.instantiate(var_count);
                Some(t2)
            }
            Self::Abstraction(x, t) => {
                let a = Type::Variable(format!("t{}", var_count));
                *var_count += 1;
                let mut inner_env = type_env.clone();
                inner_env.push((x.to_owned(), a.clone()));
                let b = t.type_constraints(&inner_env, constraints, var_count)?;
                Some(Type::Fn(Box::new(a), Box::new(b)))
            }
            Self::Application(t, u) => {
                let f = t.type_constraints(type_env, constraints, var_count)?;
                let a = u.type_constraints(type_env, constraints, var_count)?;
                let b = Type::Variable(format!("t{}", var_count));
                *var_count += 1;
                constraints.push((f, Type::Fn(Box::new(a), Box::new(b.clone()))));
                Some(b)
            }
            Self::Let(x, b, o) => {
                let bt = b.type_constraints(type_env, constraints, var_count)?;
                let gen = bt.generalise(&free_vars(type_env));
                let mut inner_env = type_env.clone();
                inner_env.push((x.to_owned(), gen));
                let ot = o.type_constraints(&inner_env, constraints, var_count)?;
                Some(ot)
            }
        }
    }

    /// The type of this term in the empty type environment, if it is well-typed.
    pub fn type_closed(&self) -> Option<Type> {
        self.type_in(&TypeEnvironment::new())
    }
}

fn free_vars(type_env: &TypeEnvironment) -> HashSet<String> {
    type_env
        .iter()
        .fold(HashSet::new(), |vs, (_, t)| &vs | &t.vars())
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
            Self::Let(x, b, t) => write!(f, "let {} = {} in {}", x, b, t),
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
        let gen_vars = self.gen_vars();
        let mut var_iter = gen_vars.into_iter();
        if let Some(v) = var_iter.next() {
            write!(f, "∀{}", v)?;
            for v in var_iter {
                write!(f, " {}", v)?;
            }
            write!(f, ".")?;
        }
        write_ty_unparen(self, f)
    }
}

fn write_ty_unparen(ty: &Type, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match ty {
        Type::Variable(v) => fmt::Display::fmt(v, f),
        Type::GenVariable(v) => fmt::Display::fmt(v, f),
        Type::Fn(t, u) => {
            write_ty(t, f)?;
            write!(f, " -> {}", u)
        }
    }
}

fn write_ty(ty: &Type, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match ty {
        Type::Variable(_) | Type::GenVariable(_) => fmt::Display::fmt(ty, f),
        Type::Fn(_, _) => write!(f, "({})", ty),
    }
}

impl From<Term> for untyped::Term {
    fn from(val: Term) -> Self {
        match val {
            Term::Variable(x) => untyped::Term::Variable(x),
            Term::Abstraction(x, t) => untyped::Term::Abstraction(x, t.into()),
            Term::Application(t, u) => untyped::Term::Application(t.into(), u.into()),
            Term::Let(x, b, t) => untyped::Term::Application(
                Box::new(untyped::Term::Abstraction(x, t.into())),
                b.into(),
            ),
        }
    }
}

impl From<Box<Term>> for Box<untyped::Term> {
    fn from(val: Box<Term>) -> Self {
        Box::new((*val).into())
    }
}

impl From<untyped::Term> for Term {
    fn from(val: untyped::Term) -> Self {
        match val {
            untyped::Term::Variable(x) => Term::Variable(x),
            untyped::Term::Abstraction(x, t) => Term::Abstraction(x, t.into()),
            untyped::Term::Application(t, u) => Term::Application(t.into(), u.into()),
        }
    }
}

impl From<Box<untyped::Term>> for Box<Term> {
    fn from(val: Box<untyped::Term>) -> Self {
        Box::new((*val).into())
    }
}

#[cfg(test)]
mod tests {}
