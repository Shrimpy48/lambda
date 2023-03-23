use std::{error, fmt, str::Utf8Error};

use tree_sitter::{Language, Node, Parser, TreeCursor};

use super::*;

extern "C" {
    fn tree_sitter_lambda() -> Language;
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum ParseError {
    Encoding(Utf8Error),
    Other(String),
}

impl ParseError {
    fn msg(s: impl Into<String>) -> Self {
        Self::Other(s.into())
    }
}

impl From<Utf8Error> for ParseError {
    fn from(value: Utf8Error) -> Self {
        Self::Encoding(value)
    }
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Other(s) => write!(f, "parse error: {}", s),
            Self::Encoding(e) => e.fmt(f),
        }
    }
}

impl error::Error for ParseError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            Self::Encoding(e) => Some(e),
            Self::Other(_) => None,
        }
    }
}

pub fn parse(source: impl AsRef<[u8]>) -> Result<Term, ParseError> {
    let mut parser = Parser::new();
    let language = unsafe { tree_sitter_lambda() };
    parser
        .set_language(language)
        .expect("language should be compiled with a compatible version of tree-sitter");

    let source = source.as_ref();

    let tree = parser.parse(source, None).expect("parser should succeed");
    let mut cursor = tree.walk();
    let node = advance(&mut cursor)?.ok_or(ParseError::msg("missing root"))?;
    if node.kind() != "source" {
        return Err(ParseError::msg(format!(
            "expected root node but got {}",
            node.kind()
        )));
    }
    if !cursor.goto_first_child() {
        return Err(ParseError::msg("root node should have children"));
    }
    walk_term(source, &mut cursor)
}

fn walk_identifier<'a>(
    source: &'a [u8],
    cursor: &mut TreeCursor<'a>,
) -> Result<String, ParseError> {
    let node = advance(cursor)?.ok_or(ParseError::msg("missing variable"))?;
    if node.kind() != "variable" {
        return Err(ParseError::msg(format!(
            "expected variable but got {}",
            node.kind()
        )));
    }
    let text = node.utf8_text(source)?.to_owned();
    Ok(text)
}

fn walk_term<'a>(source: &'a [u8], cursor: &mut TreeCursor<'a>) -> Result<Term, ParseError> {
    let node = advance(cursor)?.ok_or(ParseError::msg("missing term"))?;
    match node.kind() {
        "sort" => {
            let text = node.utf8_text(source)?;
            Ok(Term::Sort(match text {
                "*" => Sort::Type,
                "□" => Sort::Universal,
                _ => return Err(ParseError::msg(format!("unexpected sort {}", text))),
            }))
        }
        "variable" => Ok(Term::Variable(walk_identifier(source, cursor)?)),
        "abstraction" => Ok(walk_abstraction(source, cursor)?),
        "product" => Ok(walk_product(source, cursor)?),
        "application" => Ok(walk_application(source, cursor)?),
        _ => Err(ParseError::msg(format!(
            "unknown term type {}",
            node.kind()
        ))),
    }
}

fn walk_application<'a>(source: &'a [u8], cursor: &mut TreeCursor<'a>) -> Result<Term, ParseError> {
    if !cursor.goto_first_child() {
        return Err(ParseError::msg("application should have children"));
    }
    let func = Box::new(walk_term(source, cursor)?);
    if !cursor.goto_next_sibling() {
        return Err(ParseError::msg("application should have RHS"));
    }
    let arg = Box::new(walk_term(source, cursor)?);
    assert!(cursor.goto_parent());
    Ok(Term::Application(func, arg))
}

fn walk_product<'a>(source: &'a [u8], cursor: &mut TreeCursor<'a>) -> Result<Term, ParseError> {
    if !cursor.goto_first_child() {
        return Err(ParseError::msg("product should have children"));
    }
    advance(cursor)?;
    let bound = if matches!(cursor.field_name(), Some("input_name")) {
        let res = Some(walk_identifier(source, cursor)?);
        if !cursor.goto_next_sibling() {
            return Err(ParseError::msg("product should have input type"));
        }
        res
    } else if matches!(cursor.field_name(), Some("input")) {
        None
    } else {
        return Err(ParseError::msg(format!(
            "unexpected node in product: {}",
            cursor.node().to_sexp()
        )));
    };
    let ty = Box::new(walk_term(source, cursor)?);
    if !cursor.goto_next_sibling() {
        return Err(ParseError::msg("product should have output type"));
    }
    let body = Box::new(walk_term(source, cursor)?);
    assert!(cursor.goto_parent());
    let bound = bound.unwrap_or_else(|| fresh_var(&body.free_vars()));
    Ok(Term::Product(bound, ty, body))
}

fn walk_abstraction<'a>(source: &'a [u8], cursor: &mut TreeCursor<'a>) -> Result<Term, ParseError> {
    if !cursor.goto_first_child() {
        return Err(ParseError::msg("abstraction should have children"));
    }
    let bound = walk_identifier(source, cursor)?;
    if !cursor.goto_next_sibling() {
        return Err(ParseError::msg("abstraction should have type"));
    }
    let ty = Box::new(walk_term(source, cursor)?);
    if !cursor.goto_next_sibling() {
        return Err(ParseError::msg("abstraction should have body"));
    }
    let body = Box::new(walk_term(source, cursor)?);
    assert!(cursor.goto_parent());
    Ok(Term::Abstraction(bound, ty, body))
}

fn advance<'a>(cursor: &mut TreeCursor<'a>) -> Result<Option<Node<'a>>, ParseError> {
    while cursor.node().is_extra() || !cursor.node().is_named() {
        if !cursor.goto_next_sibling() {
            return Ok(None);
        }
    }
    let node = cursor.node();
    if node.is_error() {
        Err(ParseError::msg(format!("parse error: {}", node.to_sexp())))
    } else {
        Ok(Some(node))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn printed_output_parses_correctly(f in any::<Term>()) {
            let s = format!("{}", f);
            let f2 = parse(s).expect("printed output should be parseable");
            prop_assert_eq!(f2, f);
        }


        #[test]
        fn parser_doesnt_crash(s in "\\PC*") {
            let _ = parse(s);
        }
    }
}
