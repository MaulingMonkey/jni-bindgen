//! JNI Android API Bindings.

use cfg_if::*;

cfg_if! {if #[cfg(any(target_os = "android", feature = "force-define"))] {
    cfg_if! {if #[cfg(feature = "locally-generate")] {
        cfg_if! {
            if      #[cfg(feature = "api-level-29")] { include!("locally-generated/api-level-29.rs"); }
            else if #[cfg(feature = "api-level-28")] { include!("locally-generated/api-level-28.rs"); }
            else if #[cfg(feature = "api-level-27")] { include!("locally-generated/api-level-27.rs"); }
            else if #[cfg(feature = "api-level-26")] { include!("locally-generated/api-level-26.rs"); }
            else if #[cfg(feature = "api-level-25")] { include!("locally-generated/api-level-25.rs"); }
            else if #[cfg(feature = "api-level-24")] { include!("locally-generated/api-level-24.rs"); }
            else if #[cfg(feature = "api-level-23")] { include!("locally-generated/api-level-23.rs"); }
            else if #[cfg(feature = "api-level-22")] { include!("locally-generated/api-level-22.rs"); }
            else if #[cfg(feature = "api-level-21")] { include!("locally-generated/api-level-21.rs"); }
            else if #[cfg(feature = "api-level-20")] { include!("locally-generated/api-level-20.rs"); }
            else if #[cfg(feature = "api-level-19")] { include!("locally-generated/api-level-19.rs"); }
            else if #[cfg(feature = "api-level-18")] { include!("locally-generated/api-level-18.rs"); }
            else if #[cfg(feature = "api-level-17")] { include!("locally-generated/api-level-17.rs"); }
            else if #[cfg(feature = "api-level-16")] { include!("locally-generated/api-level-16.rs"); }
            else if #[cfg(feature = "api-level-15")] { include!("locally-generated/api-level-15.rs"); }
            else if #[cfg(feature = "api-level-14")] { include!("locally-generated/api-level-14.rs"); }
            else if #[cfg(feature = "api-level-13")] { include!("locally-generated/api-level-13.rs"); }
            else if #[cfg(feature = "api-level-12")] { include!("locally-generated/api-level-12.rs"); }
            else if #[cfg(feature = "api-level-11")] { include!("locally-generated/api-level-11.rs"); }
            else if #[cfg(feature = "api-level-10")] { include!("locally-generated/api-level-10.rs"); }
            else if #[cfg(feature = "api-level-9" )] { include!("locally-generated/api-level-9.rs" ); }
            else if #[cfg(feature = "api-level-8" )] { include!("locally-generated/api-level-8.rs" ); }
            else if #[cfg(feature = "api-level-7" )] { include!("locally-generated/api-level-7.rs" ); }
            else if #[cfg(feature = "api-level-6" )] { include!("locally-generated/api-level-6.rs" ); }
            else if #[cfg(feature = "api-level-5" )] { include!("locally-generated/api-level-5.rs" ); }
            else if #[cfg(feature = "api-level-4" )] { include!("locally-generated/api-level-4.rs" ); }
            else if #[cfg(feature = "api-level-3" )] { include!("locally-generated/api-level-3.rs" ); }
            else if #[cfg(feature = "api-level-2" )] { include!("locally-generated/api-level-2.rs" ); }
            else if #[cfg(feature = "api-level-1" )] { include!("locally-generated/api-level-1.rs" ); }
        }
    } else {
        cfg_if! {
            if      #[cfg(feature = "api-level-29")] { include!("reference/api-level-29.rs"); }
            else if #[cfg(feature = "api-level-28")] { include!("reference/api-level-28.rs"); }
            else if #[cfg(feature = "api-level-27")] { include!("reference/api-level-27.rs"); }
            else if #[cfg(feature = "api-level-26")] { include!("reference/api-level-26.rs"); }
            else if #[cfg(feature = "api-level-25")] { include!("reference/api-level-25.rs"); }
            else if #[cfg(feature = "api-level-24")] { include!("reference/api-level-24.rs"); }
            else if #[cfg(feature = "api-level-23")] { include!("reference/api-level-23.rs"); }
            else if #[cfg(feature = "api-level-22")] { include!("reference/api-level-22.rs"); }
            else if #[cfg(feature = "api-level-21")] { include!("reference/api-level-21.rs"); }
            else if #[cfg(feature = "api-level-20")] { include!("reference/api-level-20.rs"); }
            else if #[cfg(feature = "api-level-19")] { include!("reference/api-level-19.rs"); }
            else if #[cfg(feature = "api-level-18")] { include!("reference/api-level-18.rs"); }
            else if #[cfg(feature = "api-level-17")] { include!("reference/api-level-17.rs"); }
            else if #[cfg(feature = "api-level-16")] { include!("reference/api-level-16.rs"); }
            else if #[cfg(feature = "api-level-15")] { include!("reference/api-level-15.rs"); }
            else if #[cfg(feature = "api-level-14")] { include!("reference/api-level-14.rs"); }
            else if #[cfg(feature = "api-level-13")] { include!("reference/api-level-13.rs"); }
            else if #[cfg(feature = "api-level-12")] { include!("reference/api-level-12.rs"); }
            else if #[cfg(feature = "api-level-11")] { include!("reference/api-level-11.rs"); }
            else if #[cfg(feature = "api-level-10")] { include!("reference/api-level-10.rs"); }
            else if #[cfg(feature = "api-level-9" )] { include!("reference/api-level-9.rs" ); }
            else if #[cfg(feature = "api-level-8" )] { include!("reference/api-level-8.rs" ); }
            else if #[cfg(feature = "api-level-7" )] { include!("reference/api-level-7.rs" ); }
            else if #[cfg(feature = "api-level-6" )] { include!("reference/api-level-6.rs" ); }
            else if #[cfg(feature = "api-level-5" )] { include!("reference/api-level-5.rs" ); }
            else if #[cfg(feature = "api-level-4" )] { include!("reference/api-level-4.rs" ); }
            else if #[cfg(feature = "api-level-3" )] { include!("reference/api-level-3.rs" ); }
            else if #[cfg(feature = "api-level-2" )] { include!("reference/api-level-2.rs" ); }
            else if #[cfg(feature = "api-level-1" )] { include!("reference/api-level-1.rs" ); }
        }
    }}
}}
