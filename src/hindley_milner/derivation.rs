use console::{measure_text_width, pad_str, Alignment};
use std::{collections::VecDeque, fmt};

use super::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Judgement {
    pub lhs: Vec<(String, Type)>,
    pub rhs: (Term, Type),
}

impl Judgement {
    fn substitute(&self, mapping: &HashMap<String, Type>) -> Self {
        Self {
            lhs: self
                .lhs
                .iter()
                .map(|(v, t)| (v.clone(), t.substitute(mapping)))
                .collect(),
            rhs: (self.rhs.0.clone(), self.rhs.1.substitute(mapping)),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Proof {
    pub premises: Vec<Proof>,
    pub rule: &'static str,
    pub conclusion: Judgement,
}

impl Proof {
    fn substitute(&self, mapping: &HashMap<String, Type>) -> Self {
        Self {
            rule: self.rule,
            premises: self
                .premises
                .iter()
                .map(|p| p.substitute(mapping))
                .collect(),
            conclusion: self.conclusion.substitute(mapping),
        }
    }

    fn ty(&self) -> &Type {
        &self.conclusion.rhs.1
    }
}

fn type_deriv_impl(
    t: &Term,
    type_env: &mut TypeEnvironment,
    subs: &mut HashMap<String, Type>,
    var_count: &mut usize,
) -> Option<Proof> {
    match t {
        Term::Variable(x) => {
            let ty = type_env
                .iter()
                .rev()
                .find(|(v, _)| v == x)
                .map(|(_, t)| t)?;
            let var_p = Proof {
                premises: vec![],
                rule: "Var",
                conclusion: Judgement {
                    lhs: type_env.clone(),
                    rhs: (t.clone(), ty.clone()),
                },
            };
            let inst_ty = ty.instantiate(var_count);
            if &inst_ty != ty {
                let inst_p = Proof {
                    premises: vec![var_p],
                    rule: "Inst",
                    conclusion: Judgement {
                        lhs: type_env.clone(),
                        rhs: (t.clone(), inst_ty),
                    },
                };
                Some(inst_p)
            } else {
                Some(var_p)
            }
        }
        Term::Abstraction(x, body) => {
            let a = Type::Variable(format!("t{}", var_count));
            *var_count += 1;
            type_env.push((x.to_owned(), a.clone()));
            let b = type_deriv_impl(body, type_env, subs, var_count);
            type_env.pop();
            let b = b?;
            let a = a.substitute(subs);
            let ty = Type::Fn(Box::new(a), Box::new(b.ty().clone()));
            Some(Proof {
                premises: vec![b],
                rule: "Abs",
                conclusion: Judgement {
                    lhs: type_env.clone(),
                    rhs: (t.clone(), ty),
                },
            })
        }
        Term::Application(l, r) => {
            let f = type_deriv_impl(l, type_env, subs, var_count)?;
            let a = type_deriv_impl(r, type_env, subs, var_count)?;
            let f = f.substitute(subs);
            let b = Type::Variable(format!("t{}", var_count));
            *var_count += 1;
            let rhs = Type::Fn(Box::new(a.ty().clone()), Box::new(b.clone()));
            unify(vec![(f.ty().clone(), rhs)], subs)?;
            for (_, t) in type_env.iter_mut() {
                *t = t.substitute(subs);
            }
            let f = f.substitute(subs);
            let a = a.substitute(subs);
            let b = b.substitute(subs);
            Some(Proof {
                premises: vec![f, a],
                rule: "App",
                conclusion: Judgement {
                    lhs: type_env.clone(),
                    rhs: (t.clone(), b),
                },
            })
        }
        Term::Let(x, b, o) => {
            let bp = type_deriv_impl(b, type_env, subs, var_count)?;
            let fv = free_vars(type_env);
            let gen_t = bp.ty().generalise(&fv);
            type_env.push((x.to_owned(), gen_t.clone()));
            let op = type_deriv_impl(o, type_env, subs, var_count);
            type_env.pop();
            let op = op?;
            let bp = bp.substitute(subs);
            let bt = bp.ty().clone();
            let let_p = Proof {
                premises: vec![bp, op],
                rule: "Let",
                conclusion: Judgement {
                    lhs: type_env.clone(),
                    rhs: (t.clone(), bt.clone()),
                },
            };
            if gen_t != bt {
                let gen_p = Proof {
                    premises: vec![let_p],
                    rule: "Gen",
                    conclusion: Judgement {
                        lhs: type_env.clone(),
                        rhs: (t.clone(), gen_t),
                    },
                };
                Some(gen_p)
            } else {
                Some(let_p)
            }
        }
    }
}

pub fn type_deriv(t: &Term, type_env: &TypeEnvironment) -> Option<Proof> {
    type_deriv_impl(t, &mut type_env.clone(), &mut HashMap::new(), &mut 0)
}

impl fmt::Display for Judgement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if f.alternate() {
            let top = WriteSep {
                xs: &self.lhs,
                sep: "  ",
                w: |pair: &(String, Type), f: &mut fmt::Formatter<'_>| {
                    write!(f, "{}: {}", pair.0, pair.1)
                },
            }
            .to_string();
            let bottom = format!("{}: {}", self.rhs.0, self.rhs.1);
            let n = measure_text_width(&top).max(measure_text_width(&bottom));
            writeln!(f, "{}", top)?;
            for _ in 0..n {
                write!(f, "─")?;
            }
            write!(f, "\n{}", bottom)
        } else {
            WriteSep {
                xs: &self.lhs,
                sep: ", ",
                w: |pair: &(String, Type), f: &mut fmt::Formatter<'_>| {
                    write!(f, "{}: {}", pair.0, pair.1)
                },
            }
            .fmt(f)?;
            write!(f, " ⊢ ")?;
            write!(f, "{}: {}", self.rhs.0, self.rhs.1)
        }
    }
}

impl fmt::Display for Proof {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        proof_lines(self).join("\n").fmt(f)
    }
}

fn proof_lines(p: &Proof) -> Vec<String> {
    let mut top = join(p.premises.iter().map(proof_lines));
    let top_rule_width = if p.premises.len() == 1 {
        measure_text_width(top[top.len() - 2].trim_start_matches('─'))
    } else {
        0
    };
    let top_width =
        measure_text_width(top.first().map(|s| s.as_str()).unwrap_or("")) - top_rule_width;
    let mut bottom = p.conclusion.to_string();
    let n = top_width.max(measure_text_width(&bottom));
    for line in top.iter_mut() {
        *line = pad_str(line, n + top_rule_width, Alignment::Right, None).to_string();
    }
    bottom = pad_str(&bottom, n, Alignment::Center, None).to_string();
    let rule_tag = format!(" [{}]", p.rule);
    let rule_width = top_rule_width.max(measure_text_width(&rule_tag));
    for line in top.iter_mut() {
        *line = pad_str(line, n + rule_width, Alignment::Left, None).to_string();
    }
    bottom = pad_str(&bottom, n + rule_width, Alignment::Left, None).to_string();
    top.push(format!(
        "{}{}",
        "─".repeat(n),
        pad_str(&rule_tag, rule_width, Alignment::Left, None)
    ));
    top.push(bottom);
    top
}

fn join(blocks: impl Iterator<Item = Vec<String>>) -> Vec<String> {
    let mut lines: VecDeque<String> = VecDeque::new();
    for block in blocks {
        let mut block = VecDeque::from(block);
        let lines_width = measure_text_width(lines.front().map(|s| s.as_str()).unwrap_or(""));
        let block_width = measure_text_width(block.front().map(|s| s.as_str()).unwrap_or(""));
        while lines.len() < block.len() {
            lines.push_front(" ".repeat(lines_width));
        }
        while block.len() < lines.len() {
            block.push_front(" ".repeat(block_width));
        }
        for (left, right) in lines.iter_mut().zip(block.iter()) {
            if !left.is_empty() {
                left.push_str("  ");
            }
            left.push_str(right);
        }
    }
    lines.into()
}

struct WriteSep<'a, T, F> {
    xs: &'a [T],
    w: F,
    sep: &'static str,
}

impl<'a, T, F> fmt::Display for WriteSep<'a, T, F>
where
    F: Fn(&T, &mut fmt::Formatter<'_>) -> fmt::Result,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some((head, tail)) = self.xs.split_first() {
            (self.w)(head, f)?;
            for x in tail {
                write!(f, "{}", self.sep)?;
                (self.w)(x, f)?;
            }
        }
        Ok(())
    }
}
