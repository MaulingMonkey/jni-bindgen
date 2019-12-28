// TODO: Rename once parsing1 is gone (including function renames)

use super::*;
use proc_macro2::*;

pub(crate) fn expect_ident(tt: impl Into<Option<TokenTree>>) -> Result<Ident, Option<TokenTree>> {
    match tt.into() {
        Some(TokenTree::Ident(ident)) => Ok(ident),
        other => Err(other),
    }
}

pub(crate) fn expect_ident_if(tt: impl Into<Option<TokenTree>>, condition: impl FnOnce(&Ident) -> bool) -> Result<Ident, Option<TokenTree>> {
    match tt.into() {
        Some(TokenTree::Ident(ident)) => if condition(&ident) {
            Ok(ident)
        } else {
            Err(Some(TokenTree::Ident(ident)))
        },
        other => Err(other),
    }
}

pub(crate) fn expect_keyword(tt: impl Into<Option<TokenTree>>, keyword: &str) -> Result<(), Option<TokenTree>> {
    match tt.into() {
        Some(TokenTree::Ident(ref ident)) if ident == keyword => Ok(()),
        other => Err(other),
    }
}

pub(crate) fn expect_punct(tt: impl Into<Option<TokenTree>>, expected: &str) -> Result<(), Option<TokenTree>> {
    match tt.into() {
        Some(TokenTree::Punct(ref punct)) if expected.find(punct.as_char()).is_some() => Ok(()),
        other => Err(other),
    }
}

pub(crate) fn expect_java_ns_class(input: &mut impl TokenIter) -> Result<(String, Ident), Option<TokenTree>> {
    let mut prefix = String::new();
    loop {
        let ident = match input.next() {
            Some(TokenTree::Ident(i)) => i,
            other => return Err(other),
        };

        match input.clone().next() { // peek
            Some(TokenTree::Punct(ref p)) if ".$".find(p.as_char()).is_some() => {
                prefix += &ident.to_string();
                prefix.push(p.as_char());
                let _ = input.next();
            },
            _ => {
                return Ok((prefix, ident));
            },
        }
    }
}
