//#![cfg_attr(feature = "nightly", feature(external_doc))]
//#![cfg_attr(feature = "nightly", doc(include = "../Readme.md"))]

include!("generated/jdk1.8.0_231.rs");

mod extras {
    use super::*;

    mod strings;
    mod throwable;
}
