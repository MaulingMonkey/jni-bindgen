use super::*;

pub(crate) struct KnownDocsUrl {
    pub(crate) label:  String,
    pub(crate) url:    String,
}

impl KnownDocsUrl {
    pub(crate) fn from(context: &Context, java_class: &Class) -> Option<KnownDocsUrl> {
        let java_name = java_class.this_class().name();
        let pattern = context.config.doc_patterns.iter().find(|pattern| java_name.starts_with(pattern.jni_prefix.as_str()))?;

        for ch in java_name.chars() {
            match ch {
                'a'..='z' => {},
                'A'..='Z' => {},
                '0'..='9' => {},
                '_' | '$' | '/' => {},
                _ch => return None,
            }
        }

        let last_slash = java_name.rfind(|ch| ch == '/');
        let no_namespace = if let Some(last_slash) = last_slash {
            &java_name[(last_slash+1)..]
        } else {
            &java_name[..]
        };

        let java_name = java_name
            .replace("/", pattern.namespace_separator.as_str())
            .replace("$", pattern.inner_class_seperator.as_str());

        Some(KnownDocsUrl{
            label:  no_namespace.to_owned().replace("$","."),
            url:    pattern.url_pattern.replace("{PATH}", java_name.as_str()),
        })
    }
}
