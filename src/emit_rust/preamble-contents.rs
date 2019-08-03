// GENERATED WITH bindgen-jni, I DO NOT RECOMMEND EDITNIG THIS BY HAND
use jni_sys as __bindgen_jni_jni_sys;



// For easier review, codegen uses this macro, to ensure all output is consistent.
macro_rules! __bindgen_jni {
    () => {};



    (@deref $from:ty => (); $($rest:tt)*) => {
        __bindgen_jni! { $($rest)* }
    };

    (@deref $from:ty => $target:ty; $($rest:tt)*) => {
        impl ::std::ops::Deref for $from {
            type Target = $target;
            fn deref(&self) -> &Self::Target {
                unsafe { &*(self as *const Self as *const Self::Target) }
            }
        }
        __bindgen_jni! { $($rest)* }
    };

    (@implements $from:ty => $target:ty; $($rest:tt)*) => {
        impl ::std::convert::AsRef<$target> for $from {
            fn as_ref(&self) -> &$target {
                unsafe { &*(self as *const Self as *const $target) }
            }
        }
        __bindgen_jni! { $($rest)* }
    };



    ($(#[$attr:meta])* private static class $name:ident extends $parent:ty $(, implements $($interface:ty),+)* {} $($rest:tt)*) => {
        $(#[$attr])* #[repr(transparent)] struct $name;
        __bindgen_jni! {
            $($(@implements $name => $interface;)*)*
            @deref $name => $parent;
            $($rest)*
        }
    };

    ($(#[$attr:meta])* private class $name:ident extends $parent:ty $(, implements $($interface:ty),+)* {} $($rest:tt)*) => {
        $(#[$attr])* #[repr(transparent)] struct $name(__bindgen_jni_jni_sys::jobject);
        __bindgen_jni! {
            $($(@implements $name => $interface;)*)*
            @deref $name => $parent;
            $($rest)*
        }
    };

    ($(#[$attr:meta])* private enum $name:ident extends $parent:ty $(, implements $($interface:ty),+)*{} $($rest:tt)*) => {
        $(#[$attr])* #[repr(transparent)] struct $name(__bindgen_jni_jni_sys::jobject);
        __bindgen_jni! {
            $($(@implements $name => $interface;)*)*
            @deref $name => $parent;
            $($rest)*
        }
    };

    ($(#[$attr:meta])* private interface $name:ident extends $parent:ty $(, implements $($interface:ty),+)*{} $($rest:tt)*) => {
        $(#[$attr])* #[repr(transparent)] struct $name(__bindgen_jni_jni_sys::jobject);
        __bindgen_jni! {
            $($(@implements $name => $interface;)*)*
            @deref $name => $parent;
            $($rest)*
        }
    };



    ($(#[$attr:meta])* public static class $name:ident extends $parent:ty $(, implements $($interface:ty),+)*{} $($rest:tt)*) => {
        $(#[$attr])* #[repr(transparent)] pub struct $name;
        $($(__bindgen_jni! { @implements $name => $interface; })*)*
        __bindgen_jni! { @deref $name => $parent; $($rest)* }
    };

    ($(#[$attr:meta])* public class $name:ident extends $parent:ty $(, implements $($interface:ty),+)*{} $($rest:tt)*) => {
        $(#[$attr])* #[repr(transparent)] pub struct $name(__bindgen_jni_jni_sys::jobject);
        __bindgen_jni! {
            $($(@implements $name => $interface;)*)*
            @deref $name => $parent;
            $($rest)*
        }
    };

    ($(#[$attr:meta])* public enum $name:ident extends $parent:ty $(, implements $($interface:ty),+)*{} $($rest:tt)*) => {
        $(#[$attr])* #[repr(transparent)] pub struct $name(__bindgen_jni_jni_sys::jobject);
        __bindgen_jni! {
            $($(@implements $name => $interface;)*)*
            @deref $name => $parent;
            $($rest)*
        }
    };

    ($(#[$attr:meta])* public interface $name:ident extends $parent:ty $(, implements $($interface:ty),+)*{} $($rest:tt)*) => {
        $(#[$attr])* #[repr(transparent)] pub struct $name(__bindgen_jni_jni_sys::jobject);
        __bindgen_jni! {
            $($(@implements $name => $interface;)*)*
            @deref $name => $parent;
            $($rest)*
        }
    };
}
