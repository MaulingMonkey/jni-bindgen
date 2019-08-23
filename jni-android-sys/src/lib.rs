#![cfg_attr(feature = "nightly", feature(external_doc))]
#![cfg_attr(feature = "nightly", doc(include = "../Readme.md"))]

use cfg_if::*;

cfg_if! {if #[cfg(any(target_os = "android", feature = "force-define"))] {
    cfg_if! {
        if      #[cfg(feature = "api-level-29")] { include!("generated/api-level-29.rs"); }
        else if #[cfg(feature = "api-level-28")] { include!("generated/api-level-28.rs"); }
        else if #[cfg(feature = "api-level-27")] { include!("generated/api-level-27.rs"); }
        else if #[cfg(feature = "api-level-26")] { include!("generated/api-level-26.rs"); }
        else if #[cfg(feature = "api-level-25")] { include!("generated/api-level-25.rs"); }
        else if #[cfg(feature = "api-level-24")] { include!("generated/api-level-24.rs"); }
        else if #[cfg(feature = "api-level-23")] { include!("generated/api-level-23.rs"); }
        else if #[cfg(feature = "api-level-22")] { include!("generated/api-level-22.rs"); }
        else if #[cfg(feature = "api-level-21")] { include!("generated/api-level-21.rs"); }
        else if #[cfg(feature = "api-level-20")] { include!("generated/api-level-20.rs"); }
        else if #[cfg(feature = "api-level-19")] { include!("generated/api-level-19.rs"); }
        else if #[cfg(feature = "api-level-18")] { include!("generated/api-level-18.rs"); }
        else if #[cfg(feature = "api-level-17")] { include!("generated/api-level-17.rs"); }
        else if #[cfg(feature = "api-level-16")] { include!("generated/api-level-16.rs"); }
        else if #[cfg(feature = "api-level-15")] { include!("generated/api-level-15.rs"); }
        else if #[cfg(feature = "api-level-14")] { include!("generated/api-level-14.rs"); }
        else if #[cfg(feature = "api-level-13")] { include!("generated/api-level-13.rs"); }
        else if #[cfg(feature = "api-level-12")] { include!("generated/api-level-12.rs"); }
        else if #[cfg(feature = "api-level-11")] { include!("generated/api-level-11.rs"); }
        else if #[cfg(feature = "api-level-10")] { include!("generated/api-level-10.rs"); }
        else if #[cfg(feature = "api-level-9" )] { include!("generated/api-level-9.rs" ); }
        else if #[cfg(feature = "api-level-8" )] { include!("generated/api-level-8.rs" ); }
        else if #[cfg(feature = "api-level-7" )] { include!("generated/api-level-7.rs" ); }
        else if #[cfg(feature = "api-level-6" )] { include!("generated/api-level-6.rs" ); }
        else if #[cfg(feature = "api-level-5" )] { include!("generated/api-level-5.rs" ); }
        else if #[cfg(feature = "api-level-4" )] { include!("generated/api-level-4.rs" ); }
        else if #[cfg(feature = "api-level-3" )] { include!("generated/api-level-3.rs" ); }
        else if #[cfg(feature = "api-level-2" )] { include!("generated/api-level-2.rs" ); }
        else if #[cfg(feature = "api-level-1" )] { include!("generated/api-level-1.rs" ); }
    }
}}
