use super::*;

/// A simple imperative language.
pub mod target {
    use std::fmt;
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
        pub ret_type: Type,
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

    impl fmt::Display for Ast {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            for (name, def) in &self.type_defs {
                match def {
                    TypeDef::Struct { fields } => {
                        writeln!(f, "struct {name} {{")?;
                        for (field_name, field_type) in fields {
                            writeln!(f, "    {field_name}: {field_type},")?;
                        }
                        write!(f, "}}\n\n")?;
                    }
                }
            }
            for (name, def) in &self.func_defs {
                write!(f, "fn {name}(")?;
                write!(f, "{}: {}", def.args[0].0, def.args[0].1)?;
                for (arg_name, arg_type) in &def.args[1..] {
                    write!(f, ", {arg_name}: {arg_type}")?;
                }
                writeln!(f, ") -> {} {{", def.ret_type)?;
                for stmt in &def.body {
                    for line in stmt.to_string().lines() {
                        writeln!(f, "    {line}")?;
                    }
                }
                for line in def.ret.to_string().lines() {
                    writeln!(f, "    {line}")?;
                }
                write!(f, "}}\n\n")?;
            }
            Ok(())
        }
    }

    impl fmt::Display for Type {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                Self::Named { ident } => ident.fmt(f),
                Self::Int => write!(f, "int"),
                Self::FnPtr => write!(f, "fn_ptr"),
                Self::Any => write!(f, "any"),
            }
        }
    }

    impl fmt::Display for Stmt {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                Self::Assign { lhs, rhs } => write!(f, "let {lhs} = {rhs};"),
            }
        }
    }

    impl fmt::Display for Expr {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                Self::Read { path } => path.fmt(f),
                Self::Call { func, args } => {
                    write!(f, "{func}(")?;
                    write!(f, "{}", args[0])?;
                    for arg in &args[1..] {
                        write!(f, ", {arg}")?;
                    }
                    write!(f, ")")
                }
                Self::StructLit { name, fields } => {
                    writeln!(f, "{name} {{")?;
                    for (field_name, field_expr) in fields {
                        let expr_string = field_expr.to_string();
                        let mut lines = expr_string.lines();
                        write!(f, "    {field_name}: {}", lines.next().unwrap())?;
                        for line in lines {
                            write!(f, "\n    {}", line)?;
                        }
                        writeln!(f, ",")?;
                    }
                    write!(f, "}}")
                }
            }
        }
    }

    impl fmt::Display for Path {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{}", self.parts[0])?;
            for part in &self.parts[1..] {
                write!(f, ".{part}")?;
            }
            Ok(())
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
        ret_type: target::Type::Any,
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
            let closure_num = ast.type_defs.len();

            // Build a struct type representing the closure state
            let ty_name = format!("Closure{}", closure_num);
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
                ret_type: target::Type::Any,
                body,
                ret,
            };
            let fn_name = format!("closure_{}", closure_num);
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
