#![allow(dead_code)]

use std::borrow::Cow;
use std::collections::{BTreeMap, btree_map};
use std::fmt::Write;
use crate::{rust, java, identifiers::RustIdentifier};

#[derive(Default)]
pub struct ClassToStruct {
    remaps: BTreeMap<java::class::IdBuf, rust::structure::IdBuf>,
}

impl ClassToStruct {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_crate_classes<E>(&mut self, context: &mut impl Context, krate: &str, classes: impl Iterator<Item = java::class::IdBuf>) {
        // XXX: Ignores renaming
        for class in classes {
            match self.remaps.entry(class) {
                btree_map::Entry::Occupied(mut o) => {
                    match context.on_conflict() {
                        Resolve::Old => {},
                        Resolve::New => { o.insert(j2r(krate, o.key().as_id())); },
                    }
                },
                btree_map::Entry::Vacant(mut v) => {
                    let r = j2r(krate, v.key().as_id());
                    v.insert(r);
                },
            }
        }
    }

    pub fn get(&self, id: &java::class::Id) -> Option<rust::structure::Id> {
        self.remaps.get(&java::class::IdBuf::new(id.as_str().into())).map(|s| s.as_str())
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Resolve {
    Old,
    New,
}

pub trait Context {
    fn on_conflict<'a>(&mut self) -> Resolve;
}

impl<F: FnMut(java::class::Id, &str)> Context for (Resolve, F) {
    fn on_conflict<'a>(&mut self) -> Resolve {
        self.0
    }
}

fn j2r(krate: &str, class: java::class::Id) -> rust::structure::IdBuf {
    let mut rusty = format!("{}::", krate);
    for component in class.iter() {
        match component {
            java::class::IdPart::Namespace(id)        => write!(&mut rusty, "{}::", jc2r(id)),
            java::class::IdPart::ContainingClass(id)  => write!(&mut rusty, "{}_",  jc2r(id)),
            java::class::IdPart::LeafClass(id)        => write!(&mut rusty, "{}",   jc2r(id)), // XXX: Ignores renaming
        };
    }
    rusty
}

fn jc2r<'a>(id: &'a str) -> Cow<'a, str> {
    if id == "" { return "_empty".into(); }
    if id == "_" { return "__".into(); }

    if let Ok(id) = jc2r_basic(id) { return id; }

    let mut rusty = format!("_{}", id);
    match jc2r_basic(rusty.as_str()) {
        Ok(Cow::Owned(o))       => return Cow::Owned(o),
        Ok(Cow::Borrowed(b))    => return Cow::Owned(b.to_owned()),
        _ => {},
    }
    rusty.clear();

    // Okay, do it from scratch
    let mut chars = id.chars();
    let mut rusty = String::new();

    // First char
    match chars.next() {
        None => return "blank".into(),
        Some(ch) => match ch {
            '_'             => rusty.push(ch),
            'a'..='z'       => rusty.push(ch),
            'A'..='Z'       => rusty.push(ch),
            other_unicode   => rusty.push_str(&format!("u{:04x}", other_unicode as u32)),
        },
    }

    // More chars
    while let Some(ch) = chars.next() {
        match ch {
            '_'             => rusty.push(ch),
            'a'..='z'       => rusty.push(ch),
            'A'..='Z'       => rusty.push(ch),
            '0'..='9'       => rusty.push(ch),
            other_unicode   => rusty.push_str(&format!("_u{:04x}", other_unicode as u32)),
        }
    }

    match RustIdentifier::from_str(rusty.as_str()) {
        RustIdentifier::Identifier(_)                   => rusty.into(),
        RustIdentifier::KeywordRawSafe(_)               => format!("r#{}", id).into(),
        RustIdentifier::KeywordUnderscorePostfix(id)    => { rusty.push('_'); rusty.into() },
        RustIdentifier::NonIdentifier(id)               => panic!("Impossible jc2r failure"),
    }
}

fn jc2r_basic<'a>(id: &'a str) -> Result<Cow<'a, str>, &'a str> {
    match RustIdentifier::from_str(id) {
        RustIdentifier::Identifier(_)                   => Ok(id.into()),
        RustIdentifier::KeywordRawSafe(_)               => Ok(format!("r#{}", id).into()),
        RustIdentifier::KeywordUnderscorePostfix(id)    => Ok(format!("{}_", id).into()),
        RustIdentifier::NonIdentifier(id)               => Err(id),
    }
}
