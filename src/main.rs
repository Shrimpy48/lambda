use lambda::untyped::*;

fn main() {
    let i = Term::Abstraction("x".into(), Box::new(Term::Variable("x".into())));
    println!("{}", i);
    // let i = de_bruijn::Term::try_from(i).unwrap();
    // println!("{}", i);
    let k = Term::Abstraction(
        "x".into(),
        Box::new(Term::Abstraction(
            "y".into(),
            Box::new(Term::Variable("x".into())),
        )),
    );
    println!("{}", k);
    // let k = de_bruijn::Term::try_from(k).unwrap();
    // println!("{}", k);
    let s = Term::Abstraction(
        "x".into(),
        Box::new(Term::Abstraction(
            "y".into(),
            Box::new(Term::Abstraction(
                "z".into(),
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
    );
    println!("{}", s);
    // let s = de_bruijn::Term::try_from(s).unwrap();
    // println!("{}", s);
    let mut term_1 = Term::Application(
        Box::new(Term::Application(Box::new(s), Box::new(k.clone()))),
        Box::new(k),
    );
    let mut term_2: de_bruijn::Term = term_1.clone().try_into().unwrap();
    loop {
        match (term_1.beta_reduce_lazy(), term_2.beta_reduce_lazy()) {
            (None, None) => break,
            (None, Some(_)) | (Some(_), None) => panic!("different number of beta-reductions"),
            (Some(t1), Some(t2)) => {
                println!("{}\n{}", t1, t2);
                let t1_conv: de_bruijn::Term = t1.clone().try_into().unwrap();
                assert_eq!(t1_conv, t2);
                (term_1, term_2) = (t1, t2);
            }
        }
    }
}
