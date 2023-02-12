use super::*;

pub fn id_() -> Term {
    Term::TypeAbstraction(
        "A".into(),
        Box::new(Term::Abstraction(
            "x".into(),
            Type::Variable("A".into()),
            Box::new(Term::Variable("x".into())),
        )),
    )
}

pub fn true_() -> Term {
    Term::TypeAbstraction(
        "A".into(),
        Box::new(Term::Abstraction(
            "x".into(),
            Type::Variable("A".into()),
            Box::new(Term::Abstraction(
                "y".into(),
                Type::Variable("A".into()),
                Box::new(Term::Variable("x".into())),
            )),
        )),
    )
}

pub fn false_() -> Term {
    Term::TypeAbstraction(
        "A".into(),
        Box::new(Term::Abstraction(
            "x".into(),
            Type::Variable("A".into()),
            Box::new(Term::Abstraction(
                "y".into(),
                Type::Variable("A".into()),
                Box::new(Term::Variable("y".into())),
            )),
        )),
    )
}

pub fn not_() -> Term {
    Term::Abstraction(
        "b".into(),
        Type::ForAll(
            "X".into(),
            Box::new(Type::Fn(
                Box::new(Type::Variable("X".into())),
                Box::new(Type::Fn(
                    Box::new(Type::Variable("X".into())),
                    Box::new(Type::Variable("X".into())),
                )),
            )),
        ),
        Box::new(Term::TypeAbstraction(
            "A".into(),
            Box::new(Term::Abstraction(
                "t".into(),
                Type::Variable("A".into()),
                Box::new(Term::Abstraction(
                    "f".into(),
                    Type::Variable("A".into()),
                    Box::new(Term::Application(
                        Box::new(Term::Application(
                            Box::new(Term::TypeApplication(
                                Box::new(Term::Variable("b".into())),
                                Type::Variable("A".into()),
                            )),
                            Box::new(Term::Variable("f".into())),
                        )),
                        Box::new(Term::Variable("t".into())),
                    )),
                )),
            )),
        )),
    )
}

pub fn interpreter() -> Term {
    Term::TypeAbstraction(
        "T".into(),
        Box::new(Term::Abstraction(
            "q".into(),
            Type::Fn(
                Box::new(Type::ForAll(
                    "A".into(),
                    Box::new(Type::Fn(
                        Box::new(Type::Variable("A".into())),
                        Box::new(Type::Variable("A".into())),
                    )),
                )),
                Box::new(Type::Variable("T".into())),
            ),
            Box::new(Term::Application(
                Box::new(Term::Variable("q".into())),
                Box::new(id_()),
            )),
        )),
    )
}

pub fn shallow_encode(t: Term) -> Term {
    Term::Abstraction(
        "id".into(),
        Type::ForAll(
            "A".into(),
            Box::new(Type::Fn(
                Box::new(Type::Variable("A".into())),
                Box::new(Type::Variable("A".into())),
            )),
        ),
        Box::new(id_ify(t, &TypeEnvironment::new())),
    )
}

fn id_ify(t: Term, type_env: &TypeEnvironment) -> Term {
    match t {
        Term::Variable(_) => t,
        Term::Application(t, u) => {
            let t_ty = t
                .type_in(type_env)
                .expect("term to encode should be well-typed");
            Term::Application(
                Box::new(Term::Application(
                    Box::new(Term::TypeApplication(
                        Box::new(Term::Variable("id".into())),
                        t_ty,
                    )),
                    Box::new(id_ify(*t, type_env)),
                )),
                Box::new(id_ify(*u, type_env)),
            )
        }
        Term::Abstraction(v, ty, t) => {
            if !ty.free_vars().is_subset(&type_env.type_variables) {
                panic!("term to encode should be well-typed");
            }
            let mut inner_env = type_env.clone();
            inner_env.term_variables.push((v.to_owned(), ty.clone()));
            Term::Abstraction(v, ty, Box::new(id_ify(*t, &inner_env)))
        }
        Term::TypeAbstraction(a, t) => {
            let mut inner_env = type_env.clone();
            inner_env.type_variables.insert(a.to_owned());
            Term::TypeAbstraction(a, Box::new(id_ify(*t, &inner_env)))
        }
        Term::TypeApplication(t, u) => Term::TypeApplication(Box::new(id_ify(*t, type_env)), u),
    }
}
