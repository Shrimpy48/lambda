use std::collections::HashSet;
use std::fmt;
use std::iter;

use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Sort {
    Type,
    Universal,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Prim {
    Lit(Lit),
    Func(PrimFunc),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Lit {
    IntLit(isize),
    BoolLit(bool),
    IntType,
    BoolType,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PrimFunc {
    Add,
    Sub,
    Mul,
    IntEq,
    Gt,
    Or,
    Not,
    If,
    For,
    Rec,
}

#[derive(Debug, Clone, Eq)]
pub enum Term {
    Primitive(Prim),
    Variable(String),
    Sort(Sort),
    Abstraction(String, Box<Term>, Box<Term>),
    Application(Box<Term>, Box<Term>),
    Product(String, Box<Term>, Box<Term>),
}

pub type Environment = Vec<(String, Term)>;

impl PrimFunc {
    pub fn is_const_safe(self) -> bool {
        match self {
            Self::Rec => false,
            _ => true,
        }
    }
}

impl Prim {
    pub fn is_const_safe(self) -> bool {
        match self {
            Self::Lit(_) => true,
            Self::Func(f) => f.is_const_safe(),
        }
    }

    pub fn ty(self) -> Term {
        match self {
            Self::Lit(l) => l.ty(),
            Self::Func(f) => f.ty(),
        }
    }
}

impl Lit {
    pub fn ty(self) -> Term {
        match self {
            Self::IntLit(_) => Term::Primitive(Prim::Lit(Lit::IntType)),
            Self::BoolLit(_) => Term::Primitive(Prim::Lit(Lit::BoolType)),
            Self::IntType | Self::BoolType => Term::Sort(Sort::Type),
        }
    }
}

impl PrimFunc {
    pub fn ty(self) -> Term {
        match self {
            Self::Add | Self::Sub | Self::Mul => Term::Product(
                "x".into(),
                Box::new(Term::Primitive(Prim::Lit(Lit::IntType))),
                Box::new(Term::Product(
                    "y".into(),
                    Box::new(Term::Primitive(Prim::Lit(Lit::IntType))),
                    Box::new(Term::Primitive(Prim::Lit(Lit::IntType))),
                )),
            ),
            Self::IntEq | Self::Gt => Term::Product(
                "x".into(),
                Box::new(Term::Primitive(Prim::Lit(Lit::IntType))),
                Box::new(Term::Product(
                    "y".into(),
                    Box::new(Term::Primitive(Prim::Lit(Lit::IntType))),
                    Box::new(Term::Primitive(Prim::Lit(Lit::BoolType))),
                )),
            ),
            Self::Or => Term::Product(
                "x".into(),
                Box::new(Term::Primitive(Prim::Lit(Lit::BoolType))),
                Box::new(Term::Product(
                    "y".into(),
                    Box::new(Term::Primitive(Prim::Lit(Lit::BoolType))),
                    Box::new(Term::Primitive(Prim::Lit(Lit::BoolType))),
                )),
            ),
            Self::Not => Term::Product(
                "x".into(),
                Box::new(Term::Primitive(Prim::Lit(Lit::BoolType))),
                Box::new(Term::Primitive(Prim::Lit(Lit::BoolType))),
            ),
            Self::Rec => Term::Product(
                "A".into(),
                Box::new(Term::Sort(Sort::Type)),
                Box::new(Term::Product(
                    "f".into(),
                    Box::new(Term::Product(
                        "x".into(),
                        Box::new(Term::Variable("A".into())),
                        Box::new(Term::Variable("A".into())),
                    )),
                    Box::new(Term::Variable("A".into())),
                )),
            ),
            Self::For => Term::Product(
                "A".into(),
                Box::new(Term::Sort(Sort::Type)),
                Box::new(Term::Product(
                    "n".into(),
                    Box::new(Term::Primitive(Prim::Lit(Lit::IntType))),
                    Box::new(Term::Product(
                        "f".into(),
                        Box::new(Term::Product(
                            "x".into(),
                            Box::new(Term::Variable("A".into())),
                            Box::new(Term::Variable("A".into())),
                        )),
                        Box::new(Term::Product(
                            "x".into(),
                            Box::new(Term::Variable("A".into())),
                            Box::new(Term::Variable("A".into())),
                        )),
                    )),
                )),
            ),
            Self::If => Term::Product(
                "A".into(),
                Box::new(Term::Sort(Sort::Type)),
                Box::new(Term::Product(
                    "c".into(),
                    Box::new(Term::Primitive(Prim::Lit(Lit::BoolType))),
                    Box::new(Term::Product(
                        "t".into(),
                        Box::new(Term::Variable("A".into())),
                        Box::new(Term::Product(
                            "f".into(),
                            Box::new(Term::Variable("A".into())),
                            Box::new(Term::Variable("A".into())),
                        )),
                    )),
                )),
            ),
        }
    }

    pub fn reduce_unary(self, arg: &Term) -> Option<Term> {
        match self {
            Self::Add
            | Self::Sub
            | Self::Mul
            | Self::IntEq
            | Self::Gt
            | Self::Or
            | Self::Rec
            | Self::For
            | Self::If => None,
            Self::Not => {
                if let Term::Primitive(Prim::Lit(Lit::BoolLit(b))) = arg {
                    Some(Term::Primitive(Prim::Lit(Lit::BoolLit(!b))))
                } else {
                    None
                }
            }
        }
    }

    pub fn reduce_binary(self, a: &Term, b: &Term) -> Option<Term> {
        match self {
            Self::Add => {
                if let (
                    Term::Primitive(Prim::Lit(Lit::IntLit(x))),
                    Term::Primitive(Prim::Lit(Lit::IntLit(y))),
                ) = (a, b)
                {
                    Some(Term::Primitive(Prim::Lit(Lit::IntLit(x + y))))
                } else {
                    None
                }
            }
            Self::Mul => {
                if let (
                    Term::Primitive(Prim::Lit(Lit::IntLit(x))),
                    Term::Primitive(Prim::Lit(Lit::IntLit(y))),
                ) = (a, b)
                {
                    Some(Term::Primitive(Prim::Lit(Lit::IntLit(x * y))))
                } else {
                    None
                }
            }
            Self::Sub => {
                if let (
                    Term::Primitive(Prim::Lit(Lit::IntLit(x))),
                    Term::Primitive(Prim::Lit(Lit::IntLit(y))),
                ) = (a, b)
                {
                    Some(Term::Primitive(Prim::Lit(Lit::IntLit(x - y))))
                } else {
                    None
                }
            }
            Self::IntEq => {
                if let (
                    Term::Primitive(Prim::Lit(Lit::IntLit(x))),
                    Term::Primitive(Prim::Lit(Lit::IntLit(y))),
                ) = (a, b)
                {
                    Some(Term::Primitive(Prim::Lit(Lit::BoolLit(x == y))))
                } else {
                    None
                }
            }
            Self::Gt => {
                if let (
                    Term::Primitive(Prim::Lit(Lit::IntLit(x))),
                    Term::Primitive(Prim::Lit(Lit::IntLit(y))),
                ) = (a, b)
                {
                    Some(Term::Primitive(Prim::Lit(Lit::BoolLit(x > y))))
                } else {
                    None
                }
            }
            Self::Or => {
                if let (
                    Term::Primitive(Prim::Lit(Lit::BoolLit(x))),
                    Term::Primitive(Prim::Lit(Lit::BoolLit(y))),
                ) = (a, b)
                {
                    Some(Term::Primitive(Prim::Lit(Lit::BoolLit(x | y))))
                } else {
                    None
                }
            }
            Self::If => {
                if let Term::Primitive(Prim::Lit(Lit::BoolLit(cond))) = b {
                    Some(Term::Abstraction(
                        "t".into(),
                        Box::new(a.clone()),
                        Box::new(Term::Abstraction(
                            "f".into(),
                            Box::new(a.clone()),
                            Box::new(if *cond {
                                Term::Variable("t".into())
                            } else {
                                Term::Variable("f".into())
                            }),
                        )),
                    ))
                } else {
                    None
                }
            }
            Self::For => {
                if let Term::Primitive(Prim::Lit(Lit::IntLit(n))) = b {
                    Some(Term::Abstraction(
                        "f".into(),
                        Box::new(Term::Product(
                            fresh_var(&a.free_vars()),
                            Box::new(a.clone()),
                            Box::new(a.clone()),
                        )),
                        Box::new(Term::Abstraction(
                            "x".into(),
                            Box::new(a.clone()),
                            Box::new(
                                iter::repeat(Term::Variable("f".into()))
                                    .take((*n).try_into().unwrap_or(0))
                                    .fold(Term::Variable("x".into()), |x, f| {
                                        Term::Application(Box::new(f), Box::new(x))
                                    }),
                            ),
                        )),
                    ))
                } else {
                    None
                }
            }
            Self::Rec => Some(Term::Application(
                Box::new(b.clone()),
                Box::new(Term::Application(
                    Box::new(Term::Application(
                        Box::new(Term::Primitive(Prim::Func(self))),
                        Box::new(a.clone()),
                    )),
                    Box::new(b.clone()),
                )),
            )),
            Self::Not => None,
        }
    }
}

impl Term {
    pub fn is_const_safe(&self) -> bool {
        match self {
            Self::Sort(_) | Self::Variable(_) => true,
            Self::Primitive(p) => p.is_const_safe(),
            Self::Abstraction(_, _, b) => b.is_const_safe(),
            Self::Application(a, b) => a.is_const_safe() && b.is_const_safe(),
            Self::Product(_, a, b) => a.is_const_safe() && b.is_const_safe(),
        }
    }

    pub fn vars(&self) -> HashSet<String> {
        match self {
            Self::Sort(_) | Self::Primitive(_) => HashSet::new(),
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
            Self::Sort(_) | Self::Primitive(_) => HashSet::new(),
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
            Self::Sort(_) | Self::Primitive(_) => self.clone(),
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
            (Self::Primitive(x), Self::Primitive(y)) => x == y,
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
    pub fn substitute(&self, from: &str, to: &Term) -> Self {
        match self {
            Self::Sort(_) | Self::Primitive(_) => self.clone(),
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
                    return Some(b.substitute(x, u));
                }
                if let Self::Primitive(Prim::Func(f)) = t.as_ref() {
                    if let Some(res) = f.reduce_unary(u) {
                        return Some(res);
                    }
                }
                if let Some(t2) = t.beta_reduce_lazy() {
                    return Some(Self::Application(
                        Box::new(t2),
                        Box::new(u.as_ref().clone()),
                    ));
                }
                if let Self::Application(op, a) = t.as_ref() {
                    if let Self::Primitive(Prim::Func(f)) = op.as_ref() {
                        if let Some(res) = f.reduce_binary(a, u) {
                            return Some(res);
                        }
                    }
                }
                if let Some(u2) = u.beta_reduce_lazy() {
                    return Some(Self::Application(
                        Box::new(t.as_ref().clone()),
                        Box::new(u2),
                    ));
                }
                None
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
            Self::Variable(_) | Self::Sort(_) | Self::Primitive(_) => None,
        }
    }

    /// Fully beta-reduce the term. This will halt if the term is well-typed and const-safe.
    pub fn evaluate(&self) -> Self {
        let mut res = self.clone();
        while let Some(new_res) = res.beta_reduce_lazy() {
            res = new_res;
        }
        res
    }

    /// The type of this term in the given environment, if it is well-typed. This will always halt
    /// as rec may not appear in types.
    pub fn type_in(&self, env: &Environment) -> Option<Term> {
        match self {
            Self::Primitive(p) => Some(p.ty()),
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
                        let res = b.substitute(&v, &u);
                        if !res.is_const_safe() {
                            return None;
                        }
                        Some(res.evaluate())
                    } else {
                        None
                    }
                } else {
                    None
                }
            }
            Self::Product(x, ty, t) => {
                if !matches!(ty.type_in(env)?, Term::Sort(_)) {
                    return None;
                }
                if !ty.is_const_safe() {
                    return None;
                }
                let ty = ty.evaluate();
                let mut inner_env = env.clone();
                inner_env.push((x.to_owned(), ty.clone()));
                let b = t.type_in(&inner_env)?;
                if !matches!(b, Term::Sort(_)) {
                    return None;
                }
                Some(b)
            }
            Self::Abstraction(x, ty, t) => {
                if !matches!(ty.type_in(env)?, Term::Sort(_)) {
                    return None;
                }
                if !ty.is_const_safe() {
                    return None;
                }
                let ty = ty.evaluate();
                let mut inner_env = env.clone();
                inner_env.push((x.to_owned(), ty.clone()));
                let b = t.type_in(&inner_env)?;
                if !matches!(b.type_in(&inner_env)?, Term::Sort(_)) {
                    return None;
                }
                Some(Term::Product(
                    x.to_owned(),
                    Box::new(ty.clone()),
                    Box::new(b),
                ))
            }
            Self::Sort(Sort::Universal) => None,
        }
    }

    /// The type of this term in the empty environment, if it is well-typed.
    pub fn type_closed(&self) -> Option<Term> {
        self.type_in(&Environment::new())
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
            Self::Primitive(p) => p.fmt(f),
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

impl fmt::Display for Prim {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Lit(l) => l.fmt(f),
            Self::Func(o) => o.fmt(f),
        }
    }
}

impl fmt::Display for Lit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::IntLit(i) => i.fmt(f),
            Self::BoolLit(b) => b.fmt(f),
            Self::IntType => write!(f, "int"),
            Self::BoolType => write!(f, "bool"),
        }
    }
}

impl fmt::Display for PrimFunc {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Add => write!(f, "+"),
            Self::Sub => write!(f, "-"),
            Self::Mul => write!(f, "*"),
            Self::IntEq => write!(f, "="),
            Self::Gt => write!(f, ">"),
            Self::Or => write!(f, "|"),
            Self::Not => write!(f, "!"),
            Self::Rec => write!(f, "rec"),
            Self::If => write!(f, "if"),
            Self::For => write!(f, "for"),
        }
    }
}

fn write_term(t: &Term, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match t {
        Term::Variable(_) | Term::Sort(_) | Term::Primitive(_) => fmt::Display::fmt(t, f),
        _ => write!(f, "({})", t),
    }
}

fn write_func(t: &Term, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match t {
        Term::Variable(_) | Term::Sort(_) | Term::Primitive(_) | Term::Application(_, _) => {
            fmt::Display::fmt(t, f)
        }
        _ => write!(f, "({})", t),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}
