//! Parse .jar s and  .class es to generate Rust FFI bindings using JNI.

#![allow(dead_code)] // XXX

pub(crate) mod class_file_visitor;

/// Configuration formats for invoking bindgen_jni
pub mod config { // Part of the actual official API of this crate.
    #[allow(unused_imports)] use super::*;

    pub mod runtime;
    pub mod toml;
}

/// Rust generation logic
pub(crate) mod emit_rust {
    #[allow(unused_imports)] use super::*;
    use class_file_visitor::*;
    use gather_java::*;

    mod context;
    mod known_docs_url;
    mod modules;
    mod preamble;
    mod structs;

    pub use context::Context;
    use known_docs_url::*;
    use modules::*;
    use preamble::*;
    use structs::*;
}

/// Java enumeration/collection logic
pub(crate) mod gather_java {
    #[allow(unused_imports)] use super::*;
    use class_file_visitor::*;

    use std::collections::*;
    use std::io;

    mod class;
    mod class_constants;
    mod method_ref;

    pub use class::*;
    pub use class_constants::*;
    pub use method_ref::*;
}

/// JNI and Rust identifier parsing and categorizing utilities
mod identifiers {
    #[allow(unused_imports)] use super::*;
    use std::iter::*;

    mod jni_descriptor;
    mod jni_field;
    mod jni_path_iter;
    mod rust_identifier;

    pub use jni_descriptor::*;
    pub use jni_field::*;
    pub use jni_path_iter::*;
    pub use rust_identifier::*;
}

/// Core generation logic
mod run {
    #[allow(unused_imports)] use super::*;

    mod run;

    pub use run::run;
}

/// Visitors for use with class_file_visitor
#[allow(dead_code)]
mod visitors {
    #[allow(unused_imports)] use super::*;

    mod noop;
    mod print;
}

pub(crate) use identifiers::*;
pub use run::run;
