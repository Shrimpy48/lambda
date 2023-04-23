use super::*;

/// Convert a term to continuation-passing style, following the strategy of http://churchturing.org/y/90-min-scc.pdf.
pub fn cps_convert(t: &Term, cont: &Term) -> Term {
    match t {
        Term::Variable(_) => Term::Application(Box::new(cont.clone()), Box::new(t.clone())),
        Term::Abstraction(x, b) => {
            let k = fresh_var(&t.vars());
            Term::Application(
                Box::new(cont.clone()),
                Box::new(Term::Abstraction(
                    x.to_owned(),
                    Box::new(Term::Abstraction(
                        k.clone(),
                        Box::new(cps_convert(b, &Term::Variable(k))),
                    )),
                )),
            )
        }
        Term::Application(a, b) => {
            if let Term::Abstraction(x, c) = a.as_ref() {
                cps_convert(
                    b,
                    &Term::Abstraction(x.to_owned(), Box::new(cps_convert(c, cont))),
                )
            } else {
                let mut vars = cont.vars();
                vars.extend(t.vars());
                let b2 = fresh_var(&vars);
                vars.insert(b2.clone());
                let a2 = fresh_var(&vars);
                cps_convert(
                    a,
                    &Term::Abstraction(
                        a2.clone(),
                        Box::new(cps_convert(
                            b,
                            &Term::Abstraction(
                                b2.clone(),
                                Box::new(Term::Application(
                                    Box::new(Term::Application(
                                        Box::new(Term::Variable(a2)),
                                        Box::new(Term::Variable(b2)),
                                    )),
                                    Box::new(cont.clone()),
                                )),
                            ),
                        )),
                    ),
                )
            }
        }
    }
}
