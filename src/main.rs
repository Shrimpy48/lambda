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
    let term_1 = Term::Application(
        Box::new(Term::Application(Box::new(s.clone()), Box::new(k.clone()))),
        Box::new(k.clone()),
    );
    println!("{}", term_1);
    let term_2: de_bruijn::Term = term_1.clone().try_into().unwrap();
    println!("{}", term_2);
    let term_1_red = term_1.beta_reduce_lazy().unwrap();
    println!("{}", term_1_red);
    let term_2_red = term_2.beta_reduce_lazy().unwrap();
    println!("{}", term_2_red);
    let term_3: de_bruijn::Term = term_1_red.try_into().unwrap();
    println!("{}", term_3);
    assert_eq!(term_2_red, term_3);
}
