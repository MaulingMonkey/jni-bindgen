extern crate proc_macro;

#[macro_use] mod macros;
mod bindgen;

#[doc(hidden)] // For codegen use only, not (yet?) an otherwise stable part of the glue interface.
#[proc_macro]
pub fn __jni_bindgen(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    bindgen::generate(input.into()).into()
}
