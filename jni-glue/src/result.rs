use super::*;

pub type Result<R> = std::result::Result<R, jni_sys::jthrowable>; // XXX: Wrap jthrowable better
