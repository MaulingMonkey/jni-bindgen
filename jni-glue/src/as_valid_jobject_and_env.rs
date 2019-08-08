use super::*;

#[doc(hidden)] // You should generally not be interacting with this type directly, but it must be public for codegen.
/// This is hideously unsafe to implement:
/// 
/// 1) You assert the type is a #[repr(transparent)] wrapper around ObjectAndEnv.
/// 2) You assert the type cannot exist with a dangling object or env.
///     2.1) Do not implement Copy or Clone.
///     2.2) Do not allow value access.
///     2.3) Do not allow &mut T access.
///     2.4) Only allow &T access, which cannot be moved from.
pub unsafe trait AsValidJObjectAndEnv: AsJValue {}
