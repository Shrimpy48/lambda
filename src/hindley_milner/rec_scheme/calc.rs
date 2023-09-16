use std::collections::HashSet;

use super::*;

// To be more memory efficient, this could try to free up space after unifying
// since the unified terms would now be identical.
#[derive(Debug, Clone, Default)]
pub struct FlatTypeEnvironment {
    types: Vec<Type<FlatTypeRef>>,
    binds: Vec<(String, FlatTypeRef)>,
    var_count: usize,
}

impl Index<FlatTypeRef> for FlatTypeEnvironment {
    type Output = Type<FlatTypeRef>;

    fn index(&self, index: FlatTypeRef) -> &Self::Output {
        self.types.index(index.idx)
    }
}

impl FlatTypeEnvironment {
    pub fn new() -> Self {
        Self::default()
    }

    fn reduce<A, F: FnMut(&Type<A>) -> A>(&self, t: FlatTypeRef, mut f: F) -> A {
        // This can be made more efficient, but it's a bit of a pain because
        // we don't necessarily have topological ordering.
        let mapped = self[t].map_ref(|r| self.reduce(*r, &mut f));
        f(&mapped)
    }

    fn vars(&self, t: FlatTypeRef) -> HashSet<String> {
        self.reduce(t, |l| match l {
            Type::Var(v) => [v.to_owned()].into(),
            Type::GenVar(_) => HashSet::new(),
            Type::Fn { lhs, rhs } => lhs | rhs,
        })
    }

    fn gen_vars(&self, t: FlatTypeRef) -> HashSet<String> {
        self.reduce(t, |l| match l {
            Type::GenVar(v) => [v.to_owned()].into(),
            Type::Var(_) => HashSet::new(),
            Type::Fn { lhs, rhs } => lhs | rhs,
        })
    }

    fn free_vars(&self) -> HashSet<String> {
        self.binds.iter().flat_map(|(_, t)| self.vars(*t)).collect()
    }

    fn instantiate(&mut self, x: &str) -> Option<FlatTypeRef> {
        let t = self
            .binds
            .iter()
            .rev()
            .find(|(v, _)| v == x)
            .map(|(_, t)| *t)?;
        let mapping = self
            .gen_vars(t)
            .into_iter()
            .map(|gv| (gv, self.new_var()))
            .collect();
        Some(self.inst_gv(&mapping, t))
    }

    fn new_var(&mut self) -> FlatTypeRef {
        let a = format!("t{}", self.var_count);
        self.var_count += 1;
        self.insert_type(Type::GenVar(a))
    }

    fn push_binding(&mut self, var: String, t: FlatTypeRef) {
        self.binds.push((var, t))
    }

    fn pop_binding(&mut self) -> Option<FlatTypeRef> {
        self.binds.pop().map(|(_, t)| t)
    }

    fn unify(&mut self, a: FlatTypeRef, b: FlatTypeRef) -> Option<()> {
        let mut equalities = vec![(a, b)];
        while let Some((lhs_ref, rhs_ref)) = equalities.pop() {
            if lhs_ref.idx == rhs_ref.idx {
                continue;
            }
            match (&self[lhs_ref], &self[rhs_ref]) {
                (Type::Var(x), Type::Var(y)) if x == y => continue,
                (Type::Fn { lhs: t1, rhs: u1 }, Type::Fn { lhs: t2, rhs: u2 }) => {
                    equalities.push((*t1, *t2));
                    equalities.push((*u1, *u2));
                }
                (Type::Var(v), t) if !self.vars(rhs_ref).contains(v) => {
                    self.substitute(v.to_owned(), t.clone());
                }
                (t, Type::Var(v)) if !self.vars(lhs_ref).contains(v) => {
                    self.substitute(v.to_owned(), t.clone());
                }
                _ => return None,
            }
        }
        Some(())
    }

    fn generalise(&mut self, t: FlatTypeRef) -> FlatTypeRef {
        let mapping = self
            .vars(t)
            .difference(&self.free_vars())
            .map(|v| (v.to_owned(), self.insert_type(Type::GenVar(v.to_owned()))))
            .collect();
        self.gen_v(&mapping, t)
    }

    fn get_flat_ty(&self, ty_ref: FlatTypeRef) -> FlatType {
        FlatType::expand(ty_ref, |r| self[r].clone())
    }

    fn inst_gv(&mut self, mapping: &HashMap<String, FlatTypeRef>, t: FlatTypeRef) -> FlatTypeRef {
        match &self[t] {
            Type::GenVar(v) => mapping.get(v).copied().unwrap_or(t),
            Type::Fn { lhs, rhs } => {
                let (lhs, rhs) = (*lhs, *rhs);
                let (new_lhs, new_rhs) = (self.inst_gv(mapping, lhs), self.inst_gv(mapping, rhs));
                if !new_lhs.ref_eq(lhs) || !new_rhs.ref_eq(rhs) {
                    self.insert_type(Type::Fn {
                        lhs: new_lhs,
                        rhs: new_rhs,
                    })
                } else {
                    t
                }
            }
            Type::Var(_) => t,
        }
    }

    fn gen_v(&mut self, mapping: &HashMap<String, FlatTypeRef>, t: FlatTypeRef) -> FlatTypeRef {
        match &self[t] {
            Type::Var(x) => mapping.get(x).copied().unwrap_or(t),
            Type::Fn { lhs, rhs } => {
                let (lhs, rhs) = (*lhs, *rhs);
                let (new_lhs, new_rhs) = (self.gen_v(mapping, lhs), self.gen_v(mapping, rhs));
                if !new_lhs.ref_eq(lhs) || !new_rhs.ref_eq(rhs) {
                    self.insert_type(Type::Fn {
                        lhs: new_lhs,
                        rhs: new_rhs,
                    })
                } else {
                    t
                }
            }
            Type::GenVar(_) => t,
        }
    }

    fn substitute(&mut self, v: String, t: Type<FlatTypeRef>) {
        for ty in self.types.iter_mut() {
            match ty {
                Type::Var(x) if x == &v => {
                    *ty = t.clone();
                }
                _ => {}
            }
        }
    }

    fn insert_type(&mut self, b: Type<FlatTypeRef>) -> FlatTypeRef {
        let idx = self.types.len();
        self.types.push(b);
        FlatTypeRef { idx }
    }
}

impl FlatTerm {
    /// Infer the most general type of this term in the given type environment, if it is well-typed.
    pub fn type_in(
        &self,
        node: FlatTermRef,
        type_env: &mut FlatTypeEnvironment,
    ) -> Option<FlatTypeRef> {
        match &self[node] {
            Term::Var(x) => type_env.instantiate(x),
            Term::Abs { var, body } => {
                let t = type_env.new_var();
                type_env.push_binding(var.to_owned(), t);
                let b = self.type_in(*body, type_env);
                type_env.pop_binding().unwrap();
                Some(type_env.insert_type(Type::Fn { lhs: t, rhs: b? }))
            }
            Term::App { lhs, rhs } => {
                let f = self.type_in(*lhs, type_env)?;
                let a = self.type_in(*rhs, type_env)?;
                let b = type_env.new_var();
                let f_ = type_env.insert_type(Type::Fn { lhs: a, rhs: b });
                type_env.unify(f, f_)?;
                Some(b)
            }
            Term::Let { var, expr, body } => {
                let ex_ty = self.type_in(*expr, type_env)?;
                type_env.generalise(ex_ty);
                type_env.push_binding(var.to_owned(), ex_ty);
                let body_ty = self.type_in(*body, type_env);
                type_env.pop_binding().unwrap();
                body_ty
            }
        }
    }

    /// The type of this term in the empty type environment, if it is well-typed.
    pub fn type_closed(&self) -> Option<FlatType> {
        let mut env = FlatTypeEnvironment::new();
        let ty_ref = self.type_in(self.root(), &mut env)?;
        Some(env.get_flat_ty(ty_ref))
    }
}
