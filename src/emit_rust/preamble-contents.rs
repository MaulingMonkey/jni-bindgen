// GENERATED WITH bindgen-jni, I DO NOT RECOMMEND EDITNIG THIS BY HAND

#[allow(dead_code)]
mod jni {
    pub use ::std as std;
    pub use ::jni_sys as jni_sys;

    use jni_sys::*;
    use lazy_static::*;

    use std::ffi::*;
    use std::ptr::*;
    use std::marker::PhantomData;
    use std::ops::Deref;
    use std::sync::*;

    /// Represents a "safe" JNIEnv.  Construct ala:
    /// 
    /// ```no_run
    /// #[no_mangle] pub extern "system"
    /// fn Java_example_Class_method(env: *const JNIEnv, this: jobject) -> jobject {
    ///     let env = unsafe { Env::from_jni_local(&*env) };
    ///     // Env::from_jni_local is marked unsafe, as:
    ///     //   1) There is no guarantee `env` was a valid pointer.
    ///     //   2) There is no guarantee `env` will remain valid for the duration of Env's existence.
    ///     // In an attempt to reduce the chances of misuse, Env::from requires env be passed by
    ///     // reference, and limits the resulting env's lifetime to the lifetime of that pointer.
    /// 
    ///     // BAD, NO, STOP IT:
    ///     static env : *mut JNIEnv = std::ptr::null_mut();
    ///     static env : Env<'static> = Enf::from_jni_local(&*env);
    /// }
    /// ```
    #[repr(transparent)]
    pub struct Env(JNIEnv);
    impl Env {
        pub fn as_jni_env(&self) -> *mut JNIEnv { &self.0 as *const _ as *mut _ }
        pub unsafe fn from_jni_local(env: &JNIEnv) -> &Env { &*(env as *const JNIEnv as *const Env) }
        pub unsafe fn from_jni_void_ref(ptr: &*mut c_void) -> &Env { Self::from_jni_local(&*(*ptr as *const c_void as *const JNIEnv)) }

        fn get_gen_vm(&self) -> GenVM {
            let jni_env = self.as_jni_env();
            let mut vm = null_mut();
            let err = unsafe { (**jni_env).GetJavaVM.unwrap()(jni_env, &mut vm) };
            assert_eq!(err, JNI_OK);
            assert_ne!(vm, null_mut());
            VMS.read().unwrap().get_gen_vm(vm)
        }
    }



    /// Represents a "safe" JavaVM.
    #[repr(transparent)]
    pub struct VM(JavaVM);
    impl VM {
        pub fn as_java_vm(&self) -> *const JavaVM { &self.0 }
        pub unsafe fn from_jni_local(vm: &JavaVM) -> &VM { &*(vm as *const JavaVM as *const VM) }

        pub fn with_env(&self, callback: impl FnOnce(&Env)) {
            let mut java_vm = self.0;
            let mut env = null_mut();
            match unsafe { (*java_vm).GetEnv.unwrap()(&mut java_vm, &mut env, JNI_VERSION_1_2) } {
                JNI_OK => callback(unsafe { Env::from_jni_void_ref(&env) }),
                JNI_EDETACHED => match unsafe { (*java_vm).AttachCurrentThread.unwrap()(&mut java_vm, &mut env, null_mut()) } {
                    JNI_OK => callback(unsafe { Env::from_jni_void_ref(&env) }),
                    unexpected => panic!("AttachCurrentThread returned unknown error: {}", unexpected),
                },
                JNI_EVERSION => panic!("GetEnv returned JNI_EVERSION"),
                unexpected => panic!("GetEnv returned unknown error: {}", unexpected),
            }
        }
    }



    #[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
    struct GenVM {
        pub(crate) gen: usize,
        pub(crate) vm:  *const JavaVM,
    }

    unsafe impl Send for GenVM {}
    unsafe impl Sync for GenVM {}



    struct SingleVmBackend {
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

    // XXX: Implement more backend options - "Permanent" VM Backend (Android), MultipleVmBackend (Windows)



    type VmBackend = SingleVmBackend;
    lazy_static! { // RwLock::new is not const
        static ref VMS : RwLock<VmBackend> = RwLock::new(VmBackend::new());
    }

    #[allow(non_snake_case)] // Called by the JVM... must match this case.
    extern "system" fn JNI_OnLoad(vm: *const JavaVM, _reserved: *const c_void) -> jint {
        unsafe { VMS.write().unwrap().on_load(vm) };
        JNI_OK
    }

    #[allow(non_snake_case)] // Called by the JVM... must match this case.
    extern "system" fn JNI_OnUnload(vm: *const JavaVM, _reserved: *const c_void) {
        unsafe { VMS.write().unwrap().on_load(vm) };
    }



    pub struct ObjectAndEnv {
        object: jobject,
        env:    *const JNIEnv,
    }

    /// This is hideously unsafe to implement:
    /// 
    /// 1) You assert the type is a #[repr(transparent)] wrapper around ObjectAndEnv.
    /// 2) You assert the type cannot exist with a dangling object or env.
    ///     2.1) Do not implement Copy or Clone.
    ///     2.2) Do not allow value access.
    ///     2.3) Do not allow &mut T access.
    ///     2.4) Only allow &T access, which cannot be moved from.
    pub unsafe trait AsValidJObjectAndEnv {}


    pub struct Global<Class: AsValidJObjectAndEnv> {
        global: jobject,
        gen_vm: GenVM,
        pd:     PhantomData<Class>,
    }

    unsafe impl<Class: AsValidJObjectAndEnv> Send for Global<Class> {}
    unsafe impl<Class: AsValidJObjectAndEnv> Sync for Global<Class> {}

    impl<Class: AsValidJObjectAndEnv> Global<Class> {
        pub fn to_local<'env>(&'_ self, env: &'env Env) -> Local<'env, Class> {
            let env = env.as_jni_env();
            let object = unsafe { (**env).NewLocalRef.unwrap()(env, self.global) };
            assert!(!object.is_null());
            Local {
                oae: ObjectAndEnv {
                    object,
                    env,
                },
                pd: PhantomData,
            }
        }
    }

    impl<Class: AsValidJObjectAndEnv> Drop for Global<Class> {
        fn drop(&mut self) {
            VMS.read().unwrap().use_vm(self.gen_vm, |vm|{
                vm.with_env(|env|{
                    let env = env.as_jni_env();
                    unsafe { (**env).DeleteGlobalRef.unwrap()(env, self.global); }
                });
            });
        }
    }



    pub struct Local<'env, Class: AsValidJObjectAndEnv> {
        oae:    ObjectAndEnv,
        pd:     PhantomData<&'env Class>,
    }

    impl<'env, Class: AsValidJObjectAndEnv> Deref for Local<'env, Class> {
        type Target = Class;
        fn deref(&self) -> &Self::Target {
            unsafe { &*(&self.oae as *const ObjectAndEnv as *const Self::Target) }
        }
    }

    impl<'env, Class: AsValidJObjectAndEnv> Drop for Local<'env, Class> {
        fn drop(&mut self) {
            let env = self.oae.env as *mut JNIEnv;
            unsafe { (**env).DeleteLocalRef.unwrap()(env, self.oae.object); }
        }
    }
}

#[doc(hidden)] use jni as __bindgen_jni;

// For easier review, codegen uses this macro, to ensure all output is consistent.
macro_rules! __bindgen_jni {
    () => {};



    (@deref $from:ty => (); $($rest:tt)*) => {
        __bindgen_jni! { $($rest)* }
    };

    (@deref $from:ty => $target:ty; $($rest:tt)*) => {
        impl __bindgen_jni::std::ops::Deref for $from {
            type Target = $target;
            fn deref(&self) -> &Self::Target {
                unsafe { &*(self as *const Self as *const Self::Target) }
            }
        }
        __bindgen_jni! { $($rest)* }
    };

    (@implements $from:ty => $target:ty; $($rest:tt)*) => {
        impl __bindgen_jni::std::convert::AsRef<$target> for $from {
            fn as_ref(&self) -> &$target {
                unsafe { &*(self as *const Self as *const $target) }
            }
        }
        __bindgen_jni! { $($rest)* }
    };



    ($(#[$attr:meta])* private static class $name:ident extends $parent:ty $(, implements $($interface:ty),+)* {} $($rest:tt)*) => {
        $(#[$attr])* #[repr(transparent)] struct $name;
        __bindgen_jni! {
            // static
            $($rest)*
        }
    };

    ($(#[$attr:meta])* private class $name:ident extends $parent:ty $(, implements $($interface:ty),+)* {} $($rest:tt)*) => {
        $(#[$attr])* #[repr(transparent)] struct $name(__bindgen_jni::ObjectAndEnv);
        unsafe impl __bindgen_jni::AsValidJObjectAndEnv for $name {}
        __bindgen_jni! {
            $($(@implements $name => $interface;)*)*
            @deref $name => $parent;
            $($rest)*
        }
    };

    ($(#[$attr:meta])* private enum $name:ident extends $parent:ty $(, implements $($interface:ty),+)*{} $($rest:tt)*) => {
        $(#[$attr])* #[repr(transparent)] struct $name(__bindgen_jni::ObjectAndEnv);
        unsafe impl __bindgen_jni::AsValidJObjectAndEnv for $name {}
        __bindgen_jni! {
            $($(@implements $name => $interface;)*)*
            @deref $name => $parent;
            $($rest)*
        }
    };

    ($(#[$attr:meta])* private interface $name:ident extends $parent:ty $(, implements $($interface:ty),+)*{} $($rest:tt)*) => {
        $(#[$attr])* #[repr(transparent)] struct $name(__bindgen_jni::ObjectAndEnv);
        unsafe impl __bindgen_jni::AsValidJObjectAndEnv for $name {}
        __bindgen_jni! {
            $($(@implements $name => $interface;)*)*
            @deref $name => $parent;
            $($rest)*
        }
    };



    ($(#[$attr:meta])* public static class $name:ident extends $parent:ty $(, implements $($interface:ty),+)*{} $($rest:tt)*) => {
        $(#[$attr])* #[repr(transparent)] pub struct $name;
        __bindgen_jni! {
            // static
            $($rest)*
        }
    };

    ($(#[$attr:meta])* public class $name:ident extends $parent:ty $(, implements $($interface:ty),+)*{} $($rest:tt)*) => {
        $(#[$attr])* #[repr(transparent)] pub struct $name(__bindgen_jni::ObjectAndEnv);
        unsafe impl __bindgen_jni::AsValidJObjectAndEnv for $name {}
        __bindgen_jni! {
            $($(@implements $name => $interface;)*)*
            @deref $name => $parent;
            $($rest)*
        }
    };

    ($(#[$attr:meta])* public enum $name:ident extends $parent:ty $(, implements $($interface:ty),+)*{} $($rest:tt)*) => {
        $(#[$attr])* #[repr(transparent)] pub struct $name(__bindgen_jni::ObjectAndEnv);
        unsafe impl __bindgen_jni::AsValidJObjectAndEnv for $name {}
        __bindgen_jni! {
            $($(@implements $name => $interface;)*)*
            @deref $name => $parent;
            $($rest)*
        }
    };

    ($(#[$attr:meta])* public interface $name:ident extends $parent:ty $(, implements $($interface:ty),+)*{} $($rest:tt)*) => {
        $(#[$attr])* #[repr(transparent)] pub struct $name(__bindgen_jni::ObjectAndEnv);
        unsafe impl __bindgen_jni::AsValidJObjectAndEnv for $name {}
        __bindgen_jni! {
            $($(@implements $name => $interface;)*)*
            @deref $name => $parent;
            $($rest)*
        }
    };
}
