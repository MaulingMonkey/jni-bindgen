use super::*;

pub(crate) struct KnownDocsUrl {
    pub(crate) label:  String,
    pub(crate) url:    String,
}

impl KnownDocsUrl {
    pub(crate) fn from_class(context: &Context, java_class: jar_parser::class::Id) -> Option<KnownDocsUrl> {
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

    pub(crate) fn from_method(context: &Context, java_class: &str, java_method: &str, java_descriptor: &str) -> Option<KnownDocsUrl> {
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

        let java_descriptor = JniDescriptor::new(java_descriptor).ok()?;

        let java_class = java_class
            .replace("/", pattern.class_namespace_separator.as_str())
            .replace("$", pattern.class_inner_class_seperator.as_str());

        let mut java_args = String::new();

        for component in java_descriptor {
            match component {
                JniDescriptorSegment::Parameter(param) => {
                    if !java_args.is_empty() {
                        java_args.push_str(&pattern.method_argument_seperator[..]);
                    }

                    match param {
                        JniField::Single(JniBasicType::Void)    => { java_args.push_str("void");    },
                        JniField::Single(JniBasicType::Boolean) => { java_args.push_str("boolean"); },
                        JniField::Single(JniBasicType::Byte)    => { java_args.push_str("byte");    },
                        JniField::Single(JniBasicType::Char)    => { java_args.push_str("char");    },
                        JniField::Single(JniBasicType::Short)   => { java_args.push_str("short");   },
                        JniField::Single(JniBasicType::Int)     => { java_args.push_str("int");     },
                        JniField::Single(JniBasicType::Long)    => { java_args.push_str("long");    },
                        JniField::Single(JniBasicType::Float)   => { java_args.push_str("float");   },
                        JniField::Single(JniBasicType::Double)  => { java_args.push_str("double");  },
                        JniField::Single(JniBasicType::Class(class)) => {
                            let class = class.as_str()
                                .replace("/", pattern.method_argument_seperator.as_str())
                                .replace("$", pattern.method_inner_class_seperator.as_str());
                            java_args.push_str(&class);
                        }
                        JniField::Array { .. }                  => {
                            return None; // XXX
                        }
                    }
                },
                JniDescriptorSegment::Return(_) => {
                    // {RETURN} not currently supported.  Yet.
                }
            }
        }

        Some(KnownDocsUrl{
            label:  java_method.to_owned(),
            url: method_url_pattern
                .replace("{CLASS}",     java_class.as_str())
                .replace("{METHOD}",    java_method)
                .replace("{ARGUMENTS}", java_args.as_str()),
        })
    }
}
