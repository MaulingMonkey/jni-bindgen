use super::*;

pub(crate) struct SingleVmBackend {
    current: GenVM
}

impl SingleVmBackend {
    pub const fn new() -> Self {
        Self { 
            current: GenVM {
                gen: 0,
                vm:  null(),
            }
        }
    }

    // Unsafe - by calling this, you assert that `vm` will be valid until you call on_unload and allow it to return.
    pub unsafe fn on_load(&mut self, vm: *const JavaVM) {
        assert_eq!(self.current.vm, null());
        self.current.gen += 1;
        self.current.vm = vm;
    }

    // Safe - only invalidates existing VMs, doesn't actually use 'em or free them.
    pub fn on_unload(&mut self, vm: *const JavaVM) {
        assert_eq!(self.current.vm, vm);
        self.current.gen += 1;
        self.current.vm = null();
    }

    // Safe - validates against current VM state.
    pub fn use_vm(&self, vm: GenVM, callback: impl FnOnce(&VM)) {
        assert_eq!(self.current, vm);
        callback(unsafe { VM::from_jni_local(&*vm.vm) });
    }

    // Safe - validates against current VM state.
    pub fn get_gen_vm(&self, vm: *mut JavaVM) -> GenVM {
        assert_eq!(self.current.vm, vm);
        self.current
    }
}
