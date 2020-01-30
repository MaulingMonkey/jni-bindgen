use crate::emit_rust::*;
use jreflection::method;
use jreflection::field;
use std::fmt::{self, Display, Formatter};

pub(crate) struct KnownDocsUrl {
    pub(crate) label:  String,
    pub(crate) url:    String,
}

impl Display for KnownDocsUrl {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        write!(fmt, "[{}]({})", &self.label, &self.url)
    }
}

impl KnownDocsUrl {
    pub(crate) fn from_class(context: &Context, java_class: jreflection::class::Id) -> Option<KnownDocsUrl> {
        let java_class = java_class.as_str();
        let pattern = context.config.doc_patterns.iter().find(|pattern| java_class.starts_with(pattern.jni_prefix.as_str()))?;

        for ch in java_class.chars() {
            match ch {
                'a'..='z' => {},
                'A'..='Z' => {},
                '0'..='9' => {},
                '_' | '$' | '/' => {},
                _ch => return None,
            }
        }

        let last_slash = java_class.rfind(|ch| ch == '/');
        let no_namespace = if let Some(last_slash) = last_slash {
            &java_class[(last_slash+1)..]
        } else {
            &java_class[..]
        };

        let java_class = java_class
            .replace("/", pattern.class_namespace_separator.as_str())
            .replace("$", pattern.class_inner_class_seperator.as_str());

        Some(KnownDocsUrl{
            label:  no_namespace.to_owned().replace("$","."),
            url:    pattern.class_url_pattern
                .replace("{CLASS}",         java_class.as_str())
                .replace("{CLASS.LOWER}",   java_class.to_ascii_lowercase().as_str()),
        })
    }

    pub(crate) fn from_method(context: &Context, method: &Method) -> Option<KnownDocsUrl> {
        use method::*;

        let is_constructor = method.java.is_constructor();

        let pattern = context.config.doc_patterns.iter().find(|pattern| method.class.path.as_str().starts_with(pattern.jni_prefix.as_str()))?;
        let url_pattern = if is_constructor {
            pattern.constructor_url_pattern.as_ref().or(pattern.method_url_pattern.as_ref())?
        } else {
            pattern.method_url_pattern.as_ref()?
        };

        for ch in method.class.path.as_str().chars() {
            match ch {
                'a'..='z' => {},
                'A'..='Z' => {},
                '0'..='9' => {},
                '_' | '$' | '/' => {},
                _ch => return None,
            }
        }

        let java_class = method.class.path.as_str()
            .replace("/", pattern.class_namespace_separator.as_str())
            .replace("$", pattern.class_inner_class_seperator.as_str());

        let java_outer_class = method.class.path.as_str().rsplitn(2, '/').next().unwrap()
            .replace("$", pattern.class_inner_class_seperator.as_str());

        let java_inner_class = method.class.path.as_str().rsplitn(2, '/').next().unwrap().rsplitn(2, '$').next().unwrap();

        let label = if is_constructor {
            java_inner_class
        } else {
            for ch in method.java.name.as_str().chars() {
                match ch {
                    'a'..='z' => {},
                    'A'..='Z' => {},
                    '0'..='9' => {},
                    '_' => {},
                    _ch => return None,
                }
            }
            method.java.name.as_str()
        };

        let mut java_args = String::new();

        let mut prev_was_array = false;
        for arg in method.java.descriptor().arguments() {
            if prev_was_array {
                prev_was_array = false;
                java_args.push_str("%5B%5D"); // []
            }

            if !java_args.is_empty() {
                java_args.push_str(&pattern.argument_seperator[..]);
            }

            match arg {
                Type::Single(BasicType::Void)    => { java_args.push_str("void");    },
                Type::Single(BasicType::Boolean) => { java_args.push_str("boolean"); },
                Type::Single(BasicType::Byte)    => { java_args.push_str("byte");    },
                Type::Single(BasicType::Char)    => { java_args.push_str("char");    },
                Type::Single(BasicType::Short)   => { java_args.push_str("short");   },
                Type::Single(BasicType::Int)     => { java_args.push_str("int");     },
                Type::Single(BasicType::Long)    => { java_args.push_str("long");    },
                Type::Single(BasicType::Float)   => { java_args.push_str("float");   },
                Type::Single(BasicType::Double)  => { java_args.push_str("double");  },
                Type::Single(BasicType::Class(class)) => {
                    let class = class.as_str()
                        .replace("/", pattern.argument_namespace_separator  .as_str())
                        .replace("$", pattern.argument_inner_class_seperator.as_str());
                    java_args.push_str(&class);
                }
                Type::Array { levels, inner } => {
                    match inner {
                        BasicType::Void     => { return None; },
                        BasicType::Boolean  => java_args.push_str("bool"),
                        BasicType::Byte     => java_args.push_str("byte"),
                        BasicType::Char     => java_args.push_str("char"),
                        BasicType::Short    => java_args.push_str("short"),
                        BasicType::Int      => java_args.push_str("int"),
                        BasicType::Long     => java_args.push_str("long"),
                        BasicType::Float    => java_args.push_str("float"),
                        BasicType::Double   => java_args.push_str("double"),
                        BasicType::Class(class) => {
                            let class = class.as_str()
                                .replace("/", pattern.argument_namespace_separator  .as_str())
                                .replace("$", pattern.argument_inner_class_seperator.as_str());
                            java_args.push_str(&class);
                        },
                    }
                    for _ in 1..levels {
                        java_args.push_str("%5B%5D"); // []
                    }
                    prev_was_array = true; // level 0
                }
            }
        }

        if prev_was_array {
            if method.java.is_varargs() {
                java_args.push_str("...");
            } else {
                java_args.push_str("%5B%5D"); // []
            }
        }

        // No {RETURN} support... yet?

        Some(KnownDocsUrl {
            label: label.to_owned(),
            url: url_pattern
                .replace("{CLASS}",         java_class.as_str())
                .replace("{CLASS.LOWER}",   java_class.to_ascii_lowercase().as_str())
                .replace("{CLASS.OUTER}",   java_outer_class.as_str())
                .replace("{CLASS.INNER}",   java_inner_class)
                .replace("{METHOD}",        label)
                .replace("{ARGUMENTS}",     java_args.as_str())
                ,
        })
    }

    pub(crate) fn from_field(context: &Context, java_class: &str, java_field: &str, _java_descriptor: field::Descriptor) -> Option<KnownDocsUrl> {
        let pattern = context.config.doc_patterns.iter().find(|pattern| java_class.starts_with(pattern.jni_prefix.as_str()))?;
        let field_url_pattern = pattern.field_url_pattern.as_ref()?;

        for ch in java_class.chars() {
            match ch {
                'a'..='z' => {},
                'A'..='Z' => {},
                '0'..='9' => {},
                '_' | '$' | '/' => {},
                _ch => return None,
            }
        }

        for ch in java_field.chars() {
            match ch {
                'a'..='z' => {},
                'A'..='Z' => {},
                '0'..='9' => {},
                '_' => {},
                _ch => return None,
            }
        }

        let java_class = java_class
            .replace("/", pattern.class_namespace_separator.as_str())
            .replace("$", pattern.class_inner_class_seperator.as_str());

        // No {RETURN} support... yet?

        Some(KnownDocsUrl{
            label: java_field.to_owned(),
            url: field_url_pattern
                .replace("{CLASS}",     java_class.as_str())
                .replace("{CLASS.LOWER}",java_class.to_ascii_lowercase().as_str())
                .replace("{FIELD}",     java_field),
        })
    }
}
