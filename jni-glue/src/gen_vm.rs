use super::*;

/// A generation count + VM pointer.  Used to attempt to ensure you're not accidentally mixing and matching different VM
/// instances, even if they happen to have the same pointer.
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) struct GenVM {
    pub(crate) gen: usize,
    pub(crate) vm:  *const JavaVM,
}

unsafe impl Send for GenVM {}
unsafe impl Sync for GenVM {}
