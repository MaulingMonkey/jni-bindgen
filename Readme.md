# bindgen-jni

**Work in progress, not yet usable**

Vaguely inspired by, but otherwise unrelated to, [bindgen](https://github.com/rust-lang/rust-bindgen) and
[wasm-bindgen](https://github.com/rustwasm/wasm-bindgen)'s WebIDL stuff.

Generate Rust JVM FFI wrappers around APIs defined by `.jar` or `.class` files, because maintaining your own
hand-written bindings is an exercise in boredom, soundness bugs, and pain.

Goals:
* Provide a means of using Android system APIs specifically.
* Provide a means of using Java, Kotlin, Scala, or other JVM based APIs.
* Automatically link API documentation, so people might actually read it.
* Eliminate the need to manually write unsound, unreviewed, and [unaudited](https://github.com/dpc/crev) `unsafe { ... }` APIs

## License

Licensed under either of

* Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.

<!-- https://doc.rust-lang.org/1.4.0/complement-project-faq.html#why-dual-mit/asl2-license? -->
<!-- https://rust-lang-nursery.github.io/api-guidelines/necessities.html#crate-and-its-dependencies-have-a-permissive-license-c-permissive -->
<!-- https://choosealicense.com/licenses/apache-2.0/ -->
<!-- https://choosealicense.com/licenses/mit/ -->
