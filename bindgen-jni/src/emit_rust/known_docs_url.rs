use super::*;

pub(crate) struct KnownDocsUrl {
    pub(crate) label:  String,
    pub(crate) url:    String,
}

impl KnownDocsUrl {
    pub(crate) fn from(java_class: &Class) -> Option<KnownDocsUrl> {
        let java_name = java_class.this_class().name();

        //let prefix = if java_name.starts_with("android/") {
        //    "https://developer.android.com/reference/kotlin/"
        //} else if java_name.starts_with("java/") {
        //    "https://docs.oracle.com/javase/7/docs/api/index.html?"
        //} else {
        //    return None;
        //};

        let prefix = "https://developer.android.com/reference/kotlin/";

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

        Some(KnownDocsUrl{
            label:  no_namespace.to_owned().replace("$","."),
            url:    format!("{}{}.html", prefix, java_name.replace("$",".")),
        })
    }
}
