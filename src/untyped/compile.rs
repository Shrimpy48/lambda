use super::*;

/// A simple imperative language.
pub mod target {
    #[derive(Debug, Clone, Default)]
    pub struct Ast {
        pub type_defs: Vec<(String, TypeDef)>,
        pub func_defs: Vec<(String, Func)>,
    }

    #[derive(Debug, Clone)]
    pub enum TypeDef {
        Struct { fields: Vec<(String, Type)> },
    }

    #[derive(Debug, Clone)]
    pub enum Type {
        Int,
        FnPtr,
        Named { ident: String },
        Any,
    }

    #[derive(Debug, Clone)]
    pub struct Func {
        pub args: Vec<(String, Type)>,
        pub body: Vec<Stmt>,
        pub ret: Expr,
    }

    #[derive(Debug, Clone)]
    pub enum Stmt {
        Assign { lhs: String, rhs: Expr },
    }

    #[derive(Debug, Clone)]
    pub enum Expr {
        Read {
            path: Path,
        },
        Call {
            func: Path,
            args: Vec<Expr>,
        },
        StructLit {
            name: String,
            fields: Vec<(String, Expr)>,
        },
    }

    #[derive(Debug, Clone)]
    pub struct Path {
        pub parts: Vec<String>,
    }

    impl Ast {
        pub fn new() -> Self {
            Self::default()
        }
    }
}

pub fn compile_toplevel(term: &Term) -> target::Ast {
    let Term::Abstraction(v, b) = term else { panic!("toplevel should be abstraction") };
    let mut out = target::Ast::new();
    let mut body = Vec::new();
    let ctx = [(
        v.to_string(),
        target::Path {
            parts: vec![v.to_string()],
        },
    )]
    .into();
    let ret = compile_impl(b, &mut out, &ctx, &mut body);
    let main_def = target::Func {
        args: vec![(v.to_string(), target::Type::Any)],
        body,
        ret,
    };
    out.func_defs.push(("main".to_string(), main_def));
    out
}

fn compile_impl(
    term: &Term,
    ast: &mut target::Ast,
    ctx: &HashMap<String, target::Path>,
    stmts: &mut Vec<target::Stmt>,
) -> target::Expr {
    match term {
        Term::Variable(v) => target::Expr::Read {
            path: ctx[v].clone(),
        },
        Term::Abstraction(v, b) => {
            let captured_vars = term.free_vars();

            // Build a struct type representing the closure state
            let ty_name = format!("Closure{}", ast.type_defs.len());
            let mut fields = vec![("code".to_string(), target::Type::FnPtr)];
            fields.extend(
                captured_vars
                    .iter()
                    .map(|v| (v.to_string(), target::Type::Any)),
            );
            ast.type_defs
                .push((ty_name.to_string(), target::TypeDef::Struct { fields }));

            // Build a function definition representing the closure body
            let env_name = if v == "env" {
                fresh_var(&[v.to_string()].into())
            } else {
                "env".to_string()
            };
            let mut new_ctx: HashMap<_, _> = captured_vars
                .iter()
                .map(|v| {
                    (
                        v.to_string(),
                        target::Path {
                            parts: vec![env_name.to_string(), v.to_string()],
                        },
                    )
                })
                .collect();
            new_ctx.insert(
                v.to_string(),
                target::Path {
                    parts: vec![v.to_string()],
                },
            );
            let mut body = Vec::new();
            let ret = compile_impl(b, ast, &new_ctx, &mut body);
            let fn_def = target::Func {
                args: vec![
                    (
                        env_name,
                        target::Type::Named {
                            ident: ty_name.to_string(),
                        },
                    ),
                    (v.to_string(), target::Type::Any),
                ],
                body,
                ret,
            };
            let fn_name = format!("closure_{}", ast.func_defs.len());
            ast.func_defs.push((fn_name.to_string(), fn_def));

            // Build an instance of the struct representing the closure
            let mut fields = vec![(
                "code".to_string(),
                target::Expr::Read {
                    path: target::Path {
                        parts: vec![fn_name],
                    },
                },
            )];
            fields.extend(captured_vars.into_iter().map(|v| {
                let vp = target::Expr::Read {
                    path: ctx[&v].clone(),
                };
                (v, vp)
            }));
            target::Expr::StructLit {
                name: ty_name,
                fields,
            }
        }
        Term::Application(f, a) => {
            let closure_expr = compile_impl(f, ast, ctx, stmts);
            let closure_name = format!("f{}", stmts.len());
            stmts.push(target::Stmt::Assign {
                lhs: closure_name.to_string(),
                rhs: closure_expr,
            });
            let arg = compile_impl(a, ast, ctx, stmts);
            target::Expr::Call {
                func: target::Path {
                    parts: vec![closure_name.to_string(), "code".to_string()],
                },
                args: vec![
                    target::Expr::Read {
                        path: target::Path {
                            parts: vec![closure_name.to_string()],
                        },
                    },
                    arg,
                ],
            }
        }
    }
}
