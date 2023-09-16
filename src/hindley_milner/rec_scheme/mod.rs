use std::collections::HashMap;
use std::ops::Index;
use std::rc::Rc;

mod calc;
pub use calc::*;

#[derive(Debug, Clone)]
pub enum Term<A> {
    Var(String),
    App { lhs: A, rhs: A },
    Abs { var: String, body: A },
    Let { var: String, expr: A, body: A },
}

#[derive(Debug, Clone)]
pub enum Type<A> {
    Var(String),
    GenVar(String),
    Fn { lhs: A, rhs: A },
}

impl<A> Term<A> {
    #[inline]
    fn map<B, F: FnMut(A) -> B>(self, mut f: F) -> Term<B> {
        match self {
            Self::Var(v) => Term::Var(v),
            Self::App { lhs, rhs } => Term::App {
                lhs: f(lhs),
                rhs: f(rhs),
            },
            Self::Abs { var, body } => Term::Abs { var, body: f(body) },
            Self::Let { var, expr, body } => Term::Let {
                var,
                expr: f(expr),
                body: f(body),
            },
        }
    }

    #[inline]
    fn map_ref<B, F: FnMut(&A) -> B>(&self, mut f: F) -> Term<B> {
        match self {
            Self::Var(v) => Term::Var(v.clone()),
            Self::App { lhs, rhs } => Term::App {
                lhs: f(lhs),
                rhs: f(rhs),
            },
            Self::Abs { var, body } => Term::Abs {
                var: var.clone(),
                body: f(body),
            },
            Self::Let { var, expr, body } => Term::Let {
                var: var.clone(),
                expr: f(expr),
                body: f(body),
            },
        }
    }
}

impl<A> Type<A> {
    #[inline]
    fn map<B, F: FnMut(A) -> B>(self, mut f: F) -> Type<B> {
        match self {
            Self::Var(v) => Type::Var(v),
            Self::GenVar(v) => Type::GenVar(v),
            Self::Fn { lhs, rhs } => Type::Fn {
                lhs: f(lhs),
                rhs: f(rhs),
            },
        }
    }

    #[inline]
    fn map_ref<B, F: FnMut(&A) -> B>(&self, mut f: F) -> Type<B> {
        match self {
            Self::Var(v) => Type::Var(v.clone()),
            Self::GenVar(v) => Type::GenVar(v.clone()),
            Self::Fn { lhs, rhs } => Type::Fn {
                lhs: f(lhs),
                rhs: f(rhs),
            },
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct FlatTermRef {
    idx: usize,
}

impl FlatTermRef {
    pub fn ref_eq(self, other: FlatTypeRef) -> bool {
        self.idx == other.idx
    }
}

#[derive(Debug, Clone)]
pub struct FlatTerm {
    nodes: Vec<Term<FlatTermRef>>,
}

impl FlatTerm {
    fn root(&self) -> FlatTermRef {
        FlatTermRef {
            idx: self.nodes.len() - 1,
        }
    }

    fn push(&mut self, t: BoxedTerm) -> FlatTermRef {
        let node = t.root.map(|child| self.push(child));
        self.nodes.push(node);
        self.root()
    }

    fn push_rc(
        &mut self,
        m: &mut HashMap<*const Term<RcTerm>, FlatTermRef>,
        t: &RcTerm,
    ) -> FlatTermRef {
        let node = t.root.map_ref(|child| {
            if let Some(idx) = m.get(&Rc::as_ptr(&child.root)) {
                *idx
            } else {
                let idx = self.push_rc(m, child);
                m.insert(Rc::as_ptr(&child.root), idx);
                idx
            }
        });
        self.nodes.push(node);
        self.root()
    }

    fn push_general<A, F: FnMut(A) -> Term<A>>(&mut self, seed: A, f: &mut F) -> FlatTermRef {
        let node = f(seed).map(|child| self.push_general(child, f));
        self.nodes.push(node);
        self.root()
    }

    fn reduce<A, F: FnMut(Term<&A>) -> A>(self, mut f: F) -> A {
        let mut values = Vec::with_capacity(self.nodes.len());
        for node in self.nodes {
            let layer = node.map(|idx| &values[idx.idx]);
            values.push(f(layer));
        }
        values.pop().unwrap()
    }

    fn expand<A, F: FnMut(A) -> Term<A>>(seed: A, mut f: F) -> Self {
        let mut tree = FlatTerm { nodes: Vec::new() };
        tree.push_general(seed, &mut f);
        tree
    }
}

impl Index<FlatTermRef> for FlatTerm {
    type Output = Term<FlatTermRef>;

    fn index(&self, index: FlatTermRef) -> &Self::Output {
        self.nodes.index(index.idx)
    }
}

#[derive(Debug, Clone)]
pub struct BoxedTerm {
    root: Box<Term<BoxedTerm>>,
}

#[derive(Debug, Clone)]
pub struct RcTerm {
    root: Rc<Term<RcTerm>>,
}

impl From<BoxedTerm> for FlatTerm {
    fn from(value: BoxedTerm) -> Self {
        Self::expand(value, |node| *node.root)
    }
}

impl From<RcTerm> for FlatTerm {
    fn from(value: RcTerm) -> Self {
        let mut tree = FlatTerm { nodes: Vec::new() };
        let mut mapping = HashMap::new();
        tree.push_rc(&mut mapping, &value);
        tree
    }
}

impl From<FlatTerm> for RcTerm {
    fn from(value: FlatTerm) -> Self {
        value.reduce(|node| RcTerm {
            root: Rc::new(node.map(RcTerm::clone)),
        })
    }
}

#[derive(Debug, Clone, Copy)]
pub struct FlatTypeRef {
    idx: usize,
}
impl FlatTypeRef {
    pub fn ref_eq(self, other: FlatTypeRef) -> bool {
        self.idx == other.idx
    }
}

#[derive(Debug, Clone)]
pub struct FlatType {
    nodes: Vec<Type<FlatTypeRef>>,
}

impl FlatType {
    fn root(&self) -> FlatTypeRef {
        FlatTypeRef {
            idx: self.nodes.len() - 1,
        }
    }

    fn push(&mut self, t: BoxedType) -> FlatTypeRef {
        let node = t.root.map(|child| self.push(child));
        self.nodes.push(node);
        self.root()
    }

    fn push_rc(
        &mut self,
        m: &mut HashMap<*const Type<RcType>, FlatTypeRef>,
        t: &RcType,
    ) -> FlatTypeRef {
        let node = t.root.map_ref(|child| {
            if let Some(idx) = m.get(&Rc::as_ptr(&child.root)) {
                *idx
            } else {
                let idx = self.push_rc(m, child);
                m.insert(Rc::as_ptr(&child.root), idx);
                idx
            }
        });
        self.nodes.push(node);
        self.root()
    }

    fn push_general<A, F: FnMut(A) -> Type<A>>(&mut self, seed: A, f: &mut F) -> FlatTypeRef {
        let node = f(seed).map(|child| self.push_general(child, f));
        self.nodes.push(node);
        self.root()
    }

    fn reduce<A, F: FnMut(Type<&A>) -> A>(self, mut f: F) -> A {
        let mut values = Vec::with_capacity(self.nodes.len());
        for node in self.nodes {
            let layer = node.map(|idx| &values[idx.idx]);
            values.push(f(layer));
        }
        values.pop().unwrap()
    }

    fn expand<A, F: FnMut(A) -> Type<A>>(seed: A, mut f: F) -> Self {
        let mut tree = FlatType { nodes: Vec::new() };
        tree.push_general(seed, &mut f);
        tree
    }
}

impl Index<FlatTypeRef> for FlatType {
    type Output = Type<FlatTypeRef>;

    fn index(&self, index: FlatTypeRef) -> &Self::Output {
        self.nodes.index(index.idx)
    }
}

#[derive(Debug, Clone)]
pub struct BoxedType {
    root: Box<Type<BoxedType>>,
}

#[derive(Debug, Clone)]
pub struct RcType {
    root: Rc<Type<RcType>>,
}

impl From<BoxedType> for FlatType {
    fn from(value: BoxedType) -> Self {
        Self::expand(value, |node| *node.root)
    }
}

impl From<RcType> for FlatType {
    fn from(value: RcType) -> Self {
        let mut tree = FlatType { nodes: Vec::new() };
        let mut mapping = HashMap::new();
        tree.push_rc(&mut mapping, &value);
        tree
    }
}

impl From<FlatType> for RcType {
    fn from(value: FlatType) -> Self {
        value.reduce(|node| RcType {
            root: Rc::new(node.map(RcType::clone)),
        })
    }
}
