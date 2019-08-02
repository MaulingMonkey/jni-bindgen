//! XXX

#![allow(dead_code)] // XXX

pub mod class_file_visitor;

/// Part of the actual official API of this crate.
pub mod config {
    #[allow(unused_imports)] use super::*;

    pub mod toml;
}

pub mod emit_rust { // XXX: To be made private
    #[allow(unused_imports)] use super::*;
    use class_file_visitor::*;
    use gather_java::*;

    mod context;
    mod known_docs_url;
    mod modules;
    mod structs;

    pub use context::Context;
    use known_docs_url::*;
    use modules::*;
    use structs::*;
}

pub mod gather_java { // XXX: To be made private
    #[allow(unused_imports)] use super::*;
    use class_file_visitor::*;

    use std::collections::*;
    use std::io;

    mod class;
    mod class_constants;
    pub use class::*;
    pub use class_constants::*;
}

mod identifiers {
    #[allow(unused_imports)] use super::*;
    use std::iter::*;

    mod jni_field;
    mod jni_path_iter;
    mod rust_identifier;

    pub use jni_field::*;
    pub use jni_path_iter::*;
    pub use rust_identifier::*;
}

pub use identifiers::*; // XXX: To be made private
