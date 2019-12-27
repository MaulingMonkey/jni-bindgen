// TODO: Remove this entire file.
// This is the older, panic-based parsing stuff.

use proc_macro2::*;
use crate::TokenIter;

pub(crate) fn consume_java_identifier(input: &mut impl TokenIter) -> String {
    let mut buffer = String::new();
    loop {
        buffer.push_str(&expect_ident_str(input.next(), "java identifier fragment", |id| Ok(id.to_string())));
        if !is_punct_next(input, ".$") { return buffer; }
        buffer.push(expect_punct(input.next(), ".$"));
    }
}

pub(crate) fn expect_ident_str<R>(token: impl Into<Option<TokenTree>>, expected: &'static str, f: impl FnOnce(&str) -> std::result::Result<R,()>) -> R {
    expect_ident(token, expected, |kw| f(format!("{}", kw).as_str()))
}

pub(crate) fn expect_ident<R>(token: impl Into<Option<TokenTree>>, expected: &'static str, f: impl FnOnce(&Ident) -> std::result::Result<R,()>) -> R {
    match token.into() {
        Some(TokenTree::Ident(ident)) => f(&ident).unwrap_or_else(|_| panic!("error: expected {}, but got '{}'", expected, ident)),
        other => expect_failed(other, expected),
    }
}

pub(crate) fn expect_punct(token: impl Into<Option<TokenTree>>, expected: &'static str) -> char {
    match token.into() {
        Some(TokenTree::Punct(ref punct)) if expected.find(punct.as_char()).is_some() => punct.as_char(),
        other => expect_failed(other, &format!("one of {:?}", expected)),
    }
}

// XXX: Make this take TokenTree by ref to simplify the heck out of expect_group ?
pub(crate) fn expect_failed(token: impl Into<Option<TokenTree>>, expected: &str) -> ! {
    match token.into() {
        Some(TokenTree::Ident(ident))   => panic!("error: expected {}, but got Ident({:?})", expected, ident),
        Some(TokenTree::Group(group))   => panic!("error: expected {}, but got Group {{ delimiter = {:?} }}", expected, group.delimiter()),
        Some(TokenTree::Literal(lit))   => panic!("error: expected {}, but got Literal({})", expected, lit),
        Some(TokenTree::Punct(punct))   => panic!("error: expected {}, but got Punct('{}')", expected, punct.as_char()),
        None                            => panic!("error: expected {}, but reached end of macro", expected),
    }
}

pub(crate) fn is_punct_next(input: &mut impl TokenIter, punct: &str) -> bool {
    input.clone().next().map(|t| match t {
        TokenTree::Punct(p) => punct.contains(p.as_char()),
        _                   => false,
    }).unwrap_or(false)
}
