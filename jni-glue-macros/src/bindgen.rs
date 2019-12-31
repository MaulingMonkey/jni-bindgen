use proc_macro::TokenStream;

pub fn generate(input: TokenStream) -> TokenStream {
    syn::parse_macro_input!(input as Root).final_tokens().into()
}

mod java {
    use syn::parse::*;

    #[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub enum Visibility { Public, Protected, Private }
    impl Parse for Visibility {
        fn parse(input: ParseStream) -> syn::Result<Self> {
            Ok(match_ident!(input => {
                "public"    => Self::Public,
                "protected" => Self::Protected,
                "private"   => Self::Private,
                _other      => return Err(input.error("expected java visibility specifier (public, protected, or private)")),
            }))
        }
    }

    pub mod ty {
        use super::*;

        pub struct Attribute(syn::Attribute);
        pub struct Jni(syn::LitStr);
        #[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default)] pub struct Modifiers { pub is_static: bool, pub is_final: bool }
        #[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)] pub enum Category { Class, Enum, Interface }

        impl std::fmt::Debug for Attribute  { fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result { write!(fmt, "#[...]") } }
        impl std::fmt::Debug for Jni        { fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result { write!(fmt, "{}", self.0.value()) } }

        impl quote::ToTokens for Attribute  { fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) { self.0.to_tokens(tokens) } }
        impl quote::ToTokens for Jni        { fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) { self.0.to_tokens(tokens) } }

        impl Parse for Jni {
            fn parse(input: ParseStream) -> syn::Result<Self> {
                if input.peek(syn::token::Paren) {
                    let content; syn::parenthesized!(content in input);
                    content.parse::<syn::LitStr>()
                } else {
                    input.parse::<syn::LitStr>()
                }
                .map(|lit| Self(lit))
                .map_err(|_| input.error("Expected a JNI class string"))
            }
        }

        #[derive(Debug)]
        pub struct Binding {
            pub attributes: Vec<Attribute>,
            pub visibility: Visibility,
            pub modifiers:  Modifiers,
            pub category:   Category,
            pub name:       syn::Ident,
            pub jni:        Jni,
            pub extends:    Option<(syn::Type, Jni)>,
            pub implements: Vec<(syn::Type, Jni)>,
            pub body:       proc_macro2::TokenStream,
        }

        impl Parse for Binding {
            fn parse(input: ParseStream) -> syn::Result<Self> {
                let attributes  = input.call(syn::Attribute::parse_outer)?.into_iter().map(|a| Attribute(a)).collect();
                let visibility  = input.parse::<Visibility>()?;
                let mut modifiers = Modifiers::default();
                let mut category = None;
                while category.is_none() {
                    match_ident!(input => {
                        "static"    => modifiers.is_static = true,
                        "final"     => modifiers.is_final = true,
                        "class"     => category = Some(Category::Class),
                        "enum"      => category = Some(Category::Enum),
                        "interface" => category = Some(Category::Interface),
                        _other      => return Err(input.error("expected static, final, class, enum, or interface keyword")),
                    });
                }
                let category = category.unwrap();
                let name = input.parse::<syn::Ident>().map_err(|_| input.error("expected class name"))?;
                let jni = input.parse::<Jni>()?;

                let mut extends = None;
                let mut implements = Vec::new();
                while !input.peek(syn::token::Brace) {
                    match_ident!(input => {
                        "extends" => {
                            if extends.is_some() {
                                return Err(input.error("cannot have multiple extends, JVM only supports having one base class"));
                            }
                            let name = input.parse::<syn::Type>()?;
                            let jni  = input.parse::<Jni>()?;
                            extends = Some((name, jni));
                        },
                        "implements" => {
                            loop {
                                let name = input.parse::<syn::Type>()?;
                                let jni  = input.parse::<Jni>()?;
                                implements.push((name, jni));
                                if input.peek(syn::token::Brace) {
                                    break;
                                } else {
                                    input.parse::<syn::Token![,]>()?;
                                }
                            }
                        },
                        _other => return Err(input.error("expected extends, implements, or {{")),
                    });
                    if !input.peek(syn::token::Brace) {
                        input.parse::<syn::Token![,]>()?;
                    }
                }

                let body;
                if !input.peek(syn::token::Brace) {
                    return Err(input.error("expected class body"));
                }
                syn::braced!(body in input);
                let body = body.parse::<>().map_err(|_| body.error("Error parsing class body"))?;

                Ok(Self{ attributes, visibility, modifiers, category, name, jni, extends, implements, body })
            }
        }

        impl Binding {
            pub fn to_final_tokens(&self, stream: &mut proc_macro2::TokenStream) {
                use quote::quote;

                let Self { attributes, visibility, modifiers, category: _, name, jni, extends, implements, body } = &self;

                let vis = match visibility {
                    Visibility::Public      => quote!{pub},
                    Visibility::Protected   => { stream.extend(quote!(#[cfg(feature = "protected")]));  quote!{pub(crate)} },
                    Visibility::Private     => { stream.extend(quote!(#[cfg(feature = "private")]));    quote!{pub(crate)} },
                };

                for attribute in attributes {
                    stream.extend(quote!{#attribute});
                }

                stream.extend(quote!{#[repr(transparent)] #vis struct #name});
                if !modifiers.is_static {
                    stream.extend(quote!{ (pub(crate) jni_glue::ObjectAndEnv) });
                }
                stream.extend(quote!{;});

                stream.extend(quote!{
                    impl #name {
                        #body
                    }

                    unsafe impl jni_glue::JniType for #name {
                        fn static_with_jni_type<R>(callback: impl FnOnce(&str) -> R) -> R {
                            callback(#jni)
                        }
                    }
                });

                if !modifiers.is_static {
                    stream.extend(quote!{
                        unsafe impl jni_glue::AsValidJObjectAndEnv for #name {}

                        unsafe impl jni_glue::AsJValue for #name {
                            fn as_jvalue(&self) -> jni_glue::jni_sys::jvalue {
                                jni_glue::jni_sys::jvalue { l: self.0.object }
                            }
                        }
                    });
                }

                if let Some((ty, _jni)) = extends {
                    stream.extend(quote!{
                        impl std::ops::Deref for #name {
                            type Target = #ty;
                            fn deref(&self) -> &Self::Target {
                                unsafe { &*(self as *const Self as *const Self::Target) }
                            }
                        }
                    });
                }

                for (ty, _jni) in implements {
                    stream.extend(quote!{
                        impl std::convert::AsRef<#ty> for #name {
                            fn as_ref(&self) -> &#ty {
                                unsafe { &*(self as *const Self as *const #ty) }
                            }
                        }
                    });
                }
            }
        }
    }
}

#[derive(Debug)]
pub struct Root {
    pub bindings: Vec<java::ty::Binding>,
}

impl syn::parse::Parse for Root {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut bindings = Vec::new();
        while !input.is_empty() {
            bindings.push(input.parse()?);
        }
        Ok(Self { bindings })
    }
}

impl Root {
    pub fn to_final_tokens(&self, stream: &mut proc_macro2::TokenStream) {
        for binding in self.bindings.iter() {
            binding.to_final_tokens(stream);
        }
    }

    pub fn final_tokens(&self) -> proc_macro2::TokenStream {
        let mut ts = proc_macro2::TokenStream::new();
        self.to_final_tokens(&mut ts);
        ts
    }
}





#[test] fn bindings() {
    use proc_macro2::TokenStream;
    use quote::quote;
    use java::ty::Binding;

    macro_rules! parse {
        ( Ok($ty:ty): $($tt:tt)* ) => {
            match syn::parse2::<$ty>(TokenStream::from(quote!{$($tt)*})) {
                Err(error) => panic!("Expected:    {}    to parse Ok, got Err:    {}", stringify!($($tt)*), error),
                Ok(result) => result,
            }
        };
        ( Err($ty:ty): $($tt:tt)* ) => {
            if let Ok(result) = syn::parse2::<$ty>(TokenStream::from(quote!{$($tt)*})) {
                panic!("Expected:    {}    to fail to parse, but got Ok:    {:?}", stringify!($($tt)*), result);
            }
        };
    }

    parse!(Ok (Binding): public     static class Foo "a/b/Foo" {});
    parse!(Ok (Binding): protected  static class Foo "a/b/Foo" {});
    parse!(Ok (Binding): private    static class Foo "a/b/Foo" {});
    parse!(Err(Binding): xxx        static class Foo "a/b/Foo" {});

    parse!(Ok (Binding): #[doc("Foo\n")] #[doc("\nBar")] #[asdf] public     class Foo ("a/b/Foo") {});
    parse!(Ok (Binding): #[doc("Foo\n")] #[doc("\nBar")] #[asdf] protected  class Foo ("a/b/Foo") {});
    parse!(Ok (Binding): #[doc("Foo\n")] #[doc("\nBar")] #[asdf] private    class Foo ("a/b/Foo") {});
    parse!(Err(Binding): #[doc("Foo\n")] #[doc("\nBar")] #[asdf] xxx        class Foo ("a/b/Foo") {});

    parse!(Ok (Binding): public     enum Foo "a/b/Foo" {});
    parse!(Ok (Binding): protected  enum Foo "a/b/Foo" {});
    parse!(Ok (Binding): private    enum Foo "a/b/Foo" {});
    parse!(Err(Binding): xxx        enum Foo "a/b/Foo" {});

    parse!(Err(Binding): public     xxx Foo "a/b/Foo" {});
    parse!(Err(Binding): protected  xxx Foo "a/b/Foo" {});
    parse!(Err(Binding): private    xxx Foo "a/b/Foo" {});
    parse!(Err(Binding): xxx        xxx Foo "a/b/Foo" {});

    let binding = parse!(Ok(Root):
        public static class Foo "a/b/Foo" {}
        public static class Bar "a/b/Bar" extends Foo "a/b/Foo" {} // XXX
    );

    let _ = binding.final_tokens();
}
