use super::*;

impl Context {
    /// Matching:   Nothing at all so far
    /// Goal:       Everything.
    pub(super) fn on_root(&mut self, input: &mut impl TokenIter) {
        while let Some(kw) = input.next() {
            match &kw {
                TokenTree::Ident(kw) => {
                    match kw.to_string().as_str() {
                        "import"    => { self.on_import(input); continue; },
                        "unsafe"    => { self.on_unsafe(input); continue; },
                        _other      => skip(None, input, ";}"),
                    }
                },
                other => skip(Some(&other), input, ";}"),
            };

            self.error_at(&kw, concat!(
                "Expected one of:\n",
                "    import ...;\n",
                "    unsafe impl class some.java.ClassName { ... }",
            ));
        }
    }
}
