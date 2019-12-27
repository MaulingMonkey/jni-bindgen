use super::*;
use proc_macro2::*;

struct SkipCondition {
    until_punct_comma:  bool,
    until_punct_semi:   bool,
    until_group_parens: bool,
    until_group_braces: bool,
}

impl SkipCondition {
    pub fn new(until: &str) -> Self {
        for ch in until.chars() { debug_assert!(",;})".find(ch).is_some()); }
        Self {
            until_punct_comma:    until.contains(','),
            until_punct_semi:     until.contains(';'),
            until_group_parens:   until.contains(')'),
            until_group_braces:   until.contains('}'),
        }
    }

    pub fn terminates_skip(&self, tt: &TokenTree) -> bool {
        match tt {
            TokenTree::Group(group) => {
                match group.delimiter() {
                    Delimiter::Brace        => self.until_group_braces,
                    Delimiter::Parenthesis  => self.until_group_parens,
                    _                       => false,
                }
            },
            TokenTree::Punct(punct) => {
                match punct.as_char() {
                    ',' => self.until_punct_comma,
                    ';' => self.until_punct_semi,
                    _   => false,
                }
            },
            _ => false,
        }
    }
}

pub(crate) fn skip(other: Option<&TokenTree>, input: &mut impl TokenIter, until: &str) {
    let cond = SkipCondition::new(until);
    if let Some(tt) = other {
        if cond.terminates_skip(tt) {
            return;
        }
    }

    while let Some(tt) = input.next() {
        if cond.terminates_skip(&tt) {
            return;
        }
    }
}
