use crate::*;
use proc_macro2::*;
use quote::{quote, quote_spanned};
use std::collections::HashMap;

mod on_import;
mod on_root;
mod on_unsafe_impl_class;

#[derive(Default)]
pub struct Context {
    imports:    HashMap<String, String>, // "Inner" => "com.maulingmonkey.jni_bindgen.example_jni_java.RustJniGlueAddr$Inner"
    output:     TokenStream,
}

impl Context {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn output(self) -> TokenStream {
        let Self { output, .. } = self;
        output
    }

    pub fn java(&mut self, input: TokenStream) {
        let mut input = input.into_iter();
        self.on_root(&mut input)
    }

    fn error_at(&mut self, at: &impl ErrorLocation, msg: &(impl AsRef<str> + ?Sized)) {
        let at = at.opt_span();
        let msg : &str = msg.as_ref();

        if let Some(at) = at {
            self.output.extend(quote_spanned!{ at => compile_error!(#msg); });
        } else {
            self.output.extend(quote!{ compile_error!(#msg); });
        }
    }
}

trait ErrorLocation                         { fn opt_span(&self) -> Option<Span>; }
impl ErrorLocation for Option<TokenTree>    { fn opt_span(&self) -> Option<Span> { self.as_ref().map(|tt| tt.span()) } }
impl ErrorLocation for TokenTree            { fn opt_span(&self) -> Option<Span> { Some(self.span()) } }
impl ErrorLocation for Ident                { fn opt_span(&self) -> Option<Span> { Some(self.span()) } }
impl ErrorLocation for Group                { fn opt_span(&self) -> Option<Span> { Some(self.span()) } }
