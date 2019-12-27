use super::*;

impl Context {
    /// Matching:   "import ..."
    /// Goal:       "import some.java.ClassName$InnerClass;"
    pub(super) fn on_import(&mut self, input: &mut impl TokenIter) {
        match expect_java_ns_class(input) {
            Ok((prefix, class)) => {
                if let Err(bad) = expect_punct_2(input.next(), ";") {
                    skip(bad.as_ref(), input, ";}");
                    return self.error_at(&bad, "Expected '.', '$', or ';' in import statement");
                }

                let fqn = format!("{}{}", prefix, class);
                let key = class.to_string();
                if let Some(_prev) = self.imports.insert(key, fqn) {
                    return self.error_at(&class, "Ambiguous symbol, was already previously imported!");
                }

            },
            Err(bad) => {
                skip(bad.as_ref(), input, ";}");
                return self.error_at(&bad, "Expected:  some.java.ClassName { ... }");
            }
        }
    }
}
