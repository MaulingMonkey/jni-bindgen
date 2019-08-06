use super::*;

/// The result of calling a JNI function - either Ok(return_value) or Err(exception).
pub type Result<R> = std::result::Result<R, jni_sys::jthrowable>; // XXX: Wrap jthrowable better
