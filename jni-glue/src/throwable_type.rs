use super::*;

/// A marker type indicating this is a valid exception type that all exceptions thrown by java should be compatible with
pub trait ThrowableType : AsValidJObjectAndEnv {}
