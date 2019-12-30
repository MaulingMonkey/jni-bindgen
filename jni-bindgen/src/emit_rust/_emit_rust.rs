//! Rust generation logic

#[allow(unused_imports)] use super::*;

mod context;
mod fields;
mod known_docs_url;
mod methods;
mod modules;
mod preamble;
mod structs;

pub use context::Context;
use fields::*;
use known_docs_url::*;
use methods::*;
use modules::*;
use preamble::*;
use structs::*;
