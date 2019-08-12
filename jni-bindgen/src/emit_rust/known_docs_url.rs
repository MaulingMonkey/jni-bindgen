use super::*;
use java::method;

pub(crate) struct KnownDocsUrl {
    pub(crate) label:  String,
    pub(crate) url:    String,
}

impl KnownDocsUrl {
    pub(crate) fn from_class(context: &Context, java_class: java::class::Id) -> Option<KnownDocsUrl> {
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
            url:    pattern.class_url_pattern.replace("{CLASS}", java_class.as_str()),
        })
    }

    pub(crate) fn from_method(context: &Context, java_class: &str, java_method: &str, java_descriptor: method::Descriptor) -> Option<KnownDocsUrl> {
        use method::*;

        let pattern = context.config.doc_patterns.iter().find(|pattern| java_class.starts_with(pattern.jni_prefix.as_str()))?;
        let method_url_pattern = pattern.method_url_pattern.as_ref()?;

        for ch in java_class.chars() {
            match ch {
                'a'..='z' => {},
                'A'..='Z' => {},
                '0'..='9' => {},
                '_' | '$' | '/' => {},
                _ch => return None,
            }
        }

        for ch in java_method.chars() {
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

        let mut java_args = String::new();

        for arg in java_descriptor.arguments() {
            if !java_args.is_empty() {
                java_args.push_str(&pattern.method_argument_seperator[..]);
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
                        .replace("/", pattern.method_argument_seperator.as_str())
                        .replace("$", pattern.method_inner_class_seperator.as_str());
                    java_args.push_str(&class);
                }
                Type::Array { .. } => {
                    return None; // XXX
                }
            }
        }

        // No {RETURN} support... yet?

        Some(KnownDocsUrl{
            label:  java_method.to_owned(),
            url: method_url_pattern
                .replace("{CLASS}",     java_class.as_str())
                .replace("{METHOD}",    java_method)
                .replace("{ARGUMENTS}", java_args.as_str()),
        })
    }
}
