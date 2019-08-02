use std::iter::*;



#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum JniIdentifier<'a> {
    Namespace(&'a str),
    ContainingClass(&'a str),
    LeafClass(&'a str),
}



pub struct JniPathIter<'a> {
    rest: &'a str,
}

impl<'a> JniPathIter<'a> {
    pub fn new(path: &'a str) -> Self { JniPathIter { rest: path } }
}

impl<'a> Iterator for JniPathIter<'a> {
    type Item = JniIdentifier<'a>;
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(slash) = self.rest.find('/') {
            let (namespace, rest) = self.rest.split_at(slash);
            self.rest = &rest[1..];
            return Some(JniIdentifier::Namespace(namespace));
        }

        if let Some(dollar) = self.rest.find('$') {
            let (class, rest) = self.rest.split_at(dollar);
            self.rest = &rest[1..];
            return Some(JniIdentifier::ContainingClass(class));
        }

        if !self.rest.is_empty() {
            let class = self.rest;
            self.rest = "";
            return Some(JniIdentifier::LeafClass(class));
        }

        None
    }
}

#[test] fn jni_path_iter() {
    assert_eq!(JniPathIter::new("").collect::<Vec<JniIdentifier>>(), &[]);

    assert_eq!(JniPathIter::new("Bar").collect::<Vec<JniIdentifier>>(), &[
        JniIdentifier::LeafClass("Bar"),
    ]);

    assert_eq!(JniPathIter::new("java/foo/Bar").collect::<Vec<JniIdentifier>>(), &[
        JniIdentifier::Namespace("java"),
        JniIdentifier::Namespace("foo"),
        JniIdentifier::LeafClass("Bar"),
    ]);

    assert_eq!(JniPathIter::new("java/foo/Bar$Inner").collect::<Vec<JniIdentifier>>(), &[
        JniIdentifier::Namespace("java"),
        JniIdentifier::Namespace("foo"),
        JniIdentifier::ContainingClass("Bar"),
        JniIdentifier::LeafClass("Inner"),
    ]);

    assert_eq!(JniPathIter::new("java/foo/Bar$Inner$MoreInner").collect::<Vec<JniIdentifier>>(), &[
        JniIdentifier::Namespace("java"),
        JniIdentifier::Namespace("foo"),
        JniIdentifier::ContainingClass("Bar"),
        JniIdentifier::ContainingClass("Inner"),
        JniIdentifier::LeafClass("MoreInner"),
    ]);
}



#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum JniField<'a> {
    Single(JniBasicType<'a>),
    Array { levels: usize, inner: JniBasicType<'a> },
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum JniBasicType<'a> {
    Byte,
    Char,
    Double,
    Float,
    Int,
    Long,
    Class(&'a str),
    Short,
    Boolean
}

impl<'a> JniField<'a> {
    /// Consume a JniField from a string.  Will set `remaining` to parse the *remainder* of the string.
    pub fn read_next(remaining: &mut &'a str) -> Result<JniField<'a>, &'static str> {
        let mut array = 0;
        let mut chars = remaining.chars();

        let leaf = loop {
            match chars.next() {
                None => return Err("Unexpected end of string while parsing for next JNI Field"),
                Some('B') => { *remaining = chars.as_str(); break JniBasicType::Byte     }
                Some('C') => { *remaining = chars.as_str(); break JniBasicType::Char     }
                Some('D') => { *remaining = chars.as_str(); break JniBasicType::Double   }
                Some('F') => { *remaining = chars.as_str(); break JniBasicType::Float    }
                Some('I') => { *remaining = chars.as_str(); break JniBasicType::Int      }
                Some('J') => { *remaining = chars.as_str(); break JniBasicType::Long     }
                Some('L') => {
                    let chars_str = chars.as_str();
                    if let Some(semi) = chars_str.find(';') {
                        *remaining = &chars_str[(semi+1)..];
                        break JniBasicType::Class(&chars_str[..semi])
                    } else {
                        return Err("Unexpected end of string while parsing for terminating ';' of next JNI Field")
                    }
                }
                Some('S') => { *remaining = chars.as_str(); break JniBasicType::Short    }
                Some('Z') => { *remaining = chars.as_str(); break JniBasicType::Boolean  }
                Some('[') => { array += 1; }
                Some(_ch)  => return Err("Unexpected character in JNI type string"),
            }
        };

        match array {
            0   => Ok(JniField::Single(leaf)),
            n   => Ok(JniField::Array { levels: n, inner: leaf }),
        }
    }

    pub fn from_str(mut field: &'a str) -> Result<JniField<'a>, &'static str> {
        let next = Self::read_next(&mut field)?;
        if field.is_empty() {
            Ok(next)
        } else {
            Err("Expected one type field, got multiple")
        }
    }
}

#[test] fn jni_field_from_str() {
    // Single values
    assert_eq!(JniField::from_str("F"),                 Ok(JniField::Single(JniBasicType::Float)));
    assert_eq!(JniField::from_str("Ljava/foo/Bar;"),    Ok(JniField::Single(JniBasicType::Class("java/foo/Bar"))));

    // Arrays
    assert_eq!(JniField::from_str("[[F"),               Ok(JniField::Array { levels: 2, inner: JniBasicType::Float }));
    assert_eq!(JniField::from_str("[[[Ljava/foo/Bar;"), Ok(JniField::Array { levels: 3, inner: JniBasicType::Class("java/foo/Bar") }));

    // Erroneous input
    assert!(JniField::from_str("").is_err());                               // No type
    assert!(JniField::from_str("[[").is_err());                             // No type for array
    assert!(JniField::from_str("Ljava/foo/Bar").is_err());                  // Missing semicolon
    assert!(JniField::from_str("Ljava/foo/Bar;F").is_err());                // More after semicolon
    assert!(JniField::from_str("Ljava/foo/Bar;Ljava/foo/Bar;").is_err());   // More after semicolon

    // Multiple inputs
    let mut class_float = "Ljava/foo/Bar;F";
    assert_eq!(JniField::read_next(&mut class_float),    Ok(JniField::Single(JniBasicType::Class("java/foo/Bar"))));
    assert_eq!(JniField::read_next(&mut class_float),    Ok(JniField::Single(JniBasicType::Float)));
    assert_eq!(class_float, "");
    assert!(   JniField::read_next(&mut class_float).is_err());

    let mut class_class = "Ljava/foo/Bar;Ljava/foo/Bar;";
    assert_eq!(JniField::read_next(&mut class_class),    Ok(JniField::Single(JniBasicType::Class("java/foo/Bar"))));
    assert_eq!(JniField::read_next(&mut class_class),    Ok(JniField::Single(JniBasicType::Class("java/foo/Bar"))));
    assert_eq!(class_class, "");
    assert!(   JniField::read_next(&mut class_class).is_err());
}





/// Categorizes a rust [identifier](https://doc.rust-lang.org/reference/identifiers.html) for use in rust codegen.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum RustIdentifier<'a> {
    /// Meets the criteria for a Rust [NON_KEYWORD_IDENTIFIER](https://doc.rust-lang.org/reference/identifiers.html)
    Identifier(&'a str),

    /// Not a rust-safe [identifier](https://doc.rust-lang.org/reference/identifiers.html).  Unicode, strange ASCII
    /// values, relatively normal ASCII values... you name it.
    NonIdentifier(&'a str),

    /// A [keyword](https://doc.rust-lang.org/reference/keywords.html) that has had `r#` prepended to it, because it can
    /// be used as a [RAW_IDENTIFIER](https://doc.rust-lang.org/reference/identifiers.html)
    KeywordRawSafe(&'a str),

    /// A [keyword](https://doc.rust-lang.org/reference/keywords.html) that has had `_` postpended to it, because it can
    /// *not* be used as a [RAW_IDENTIFIER](https://doc.rust-lang.org/reference/identifiers.html).
    KeywordUnderscorePostfix(&'a str),
}

impl<'a> RustIdentifier<'a> {
    /// Takes an arbitrary string and tries to treat it as a Rust identifier, doing minor escaping for keywords.
    pub fn from_str(s: &'a str) -> RustIdentifier<'a> {
        match s {
            // [Strict keywords](https://doc.rust-lang.org/reference/keywords.html#strict-keywords) that *are not* valid
            // [RAW_IDENTIFIER](https://doc.rust-lang.org/reference/identifiers.html)s
            "crate"     => RustIdentifier::KeywordUnderscorePostfix("crate_"),
            "extern"    => RustIdentifier::KeywordUnderscorePostfix("extern_"),
            "self"      => RustIdentifier::KeywordUnderscorePostfix("self_"),
            "super"     => RustIdentifier::KeywordUnderscorePostfix("super_"),
            "Self"      => RustIdentifier::KeywordUnderscorePostfix("Self_"),

            // [Strict keywords](https://doc.rust-lang.org/reference/keywords.html#strict-keywords) that *are* valid
            // [RAW_IDENTIFIER](https://doc.rust-lang.org/reference/identifiers.html)s
            "as"        => RustIdentifier::KeywordRawSafe("r#as"),
            "break"     => RustIdentifier::KeywordRawSafe("r#break"),
            "const"     => RustIdentifier::KeywordRawSafe("r#const"),
            "continue"  => RustIdentifier::KeywordRawSafe("r#continue"),
            "else"      => RustIdentifier::KeywordRawSafe("r#else"),
            "enum"      => RustIdentifier::KeywordRawSafe("r#enum"),
            "false"     => RustIdentifier::KeywordRawSafe("r#false"),
            "fn"        => RustIdentifier::KeywordRawSafe("r#fn"),
            "for"       => RustIdentifier::KeywordRawSafe("r#for"),
            "if"        => RustIdentifier::KeywordRawSafe("r#if"),
            "impl"      => RustIdentifier::KeywordRawSafe("r#impl"),
            "in"        => RustIdentifier::KeywordRawSafe("r#in"),
            "let"       => RustIdentifier::KeywordRawSafe("r#let"),
            "loop"      => RustIdentifier::KeywordRawSafe("r#loop"),
            "match"     => RustIdentifier::KeywordRawSafe("r#match"),
            "mod"       => RustIdentifier::KeywordRawSafe("r#mod"),
            "move"      => RustIdentifier::KeywordRawSafe("r#move"),
            "mut"       => RustIdentifier::KeywordRawSafe("r#mut"),
            "pub"       => RustIdentifier::KeywordRawSafe("r#pub"),
            "ref"       => RustIdentifier::KeywordRawSafe("r#ref"),
            "return"    => RustIdentifier::KeywordRawSafe("r#return"),
            "static"    => RustIdentifier::KeywordRawSafe("r#static"),
            "struct"    => RustIdentifier::KeywordRawSafe("r#struct"),
            "trait"     => RustIdentifier::KeywordRawSafe("r#trait"),
            "true"      => RustIdentifier::KeywordRawSafe("r#true"),
            "type"      => RustIdentifier::KeywordRawSafe("r#type"),
            "unsafe"    => RustIdentifier::KeywordRawSafe("r#unsafe"),
            "use"       => RustIdentifier::KeywordRawSafe("r#use"),
            "where"     => RustIdentifier::KeywordRawSafe("r#where"),
            "while"     => RustIdentifier::KeywordRawSafe("r#while"),
            "dyn"       => RustIdentifier::KeywordRawSafe("r#dyn"),

            // [Reserved keywords](https://doc.rust-lang.org/reference/keywords.html#reserved-keywords) that *are* valid
            // [RAW_IDENTIFIER](https://doc.rust-lang.org/reference/identifiers.html)s
            "abstract"  => RustIdentifier::KeywordRawSafe("r#abstract"),
            "become"    => RustIdentifier::KeywordRawSafe("r#become"),
            "box"       => RustIdentifier::KeywordRawSafe("r#box"),
            "do"        => RustIdentifier::KeywordRawSafe("r#do"),
            "final"     => RustIdentifier::KeywordRawSafe("r#final"),
            "macro"     => RustIdentifier::KeywordRawSafe("r#macro"),
            "override"  => RustIdentifier::KeywordRawSafe("r#override"),
            "priv"      => RustIdentifier::KeywordRawSafe("r#priv"),
            "typeof"    => RustIdentifier::KeywordRawSafe("r#typeof"),
            "unsized"   => RustIdentifier::KeywordRawSafe("r#unsized"),
            "virtual"   => RustIdentifier::KeywordRawSafe("r#virtual"),
            "yield"     => RustIdentifier::KeywordRawSafe("r#yield"),

            // Not a keyword, but not a valid [IDENTIFIER](https://doc.rust-lang.org/reference/identifiers.html) either.
            ""                          => RustIdentifier::NonIdentifier(s),
            "_"                         => RustIdentifier::NonIdentifier(s),
            s if is_rust_identifier(s)  => RustIdentifier::Identifier(s),
            s                           => RustIdentifier::NonIdentifier(s)
        }
    }
}

#[test] fn rust_identifier_from_str() {
    assert_eq!(RustIdentifier::from_str("foo")  , RustIdentifier::Identifier              ("foo")    );
    assert_eq!(RustIdentifier::from_str("crate"), RustIdentifier::KeywordUnderscorePostfix("crate_") );
    assert_eq!(RustIdentifier::from_str("match"), RustIdentifier::KeywordRawSafe          ("r#match"));
    assert_eq!(RustIdentifier::from_str("föo"),   RustIdentifier::NonIdentifier           ("föo")    );
    assert_eq!(RustIdentifier::from_str(""),      RustIdentifier::NonIdentifier           ("")       );
    assert_eq!(RustIdentifier::from_str("_"),     RustIdentifier::NonIdentifier           ("_")      );
    assert_eq!(RustIdentifier::from_str("_f"),    RustIdentifier::Identifier              ("_f")     );
    assert_eq!(RustIdentifier::from_str("_1"),    RustIdentifier::Identifier              ("_1")     );
    assert_eq!(RustIdentifier::from_str("1_"),    RustIdentifier::NonIdentifier           ("1_")     );
}

fn is_rust_identifier(s: &str) -> bool {
    // https://doc.rust-lang.org/reference/identifiers.html
    let mut chars = s.chars();

    // First char
    let first_char = if let Some(ch) = chars.next() { ch } else { return false };
    match first_char {
        'a'..='z' => {},
        'A'..='Z' => {},
        '_' => { if s == "_" { return false } },
        _ => { return false }
    }

    // Subsequent chars
    while let Some(ch) = chars.next() {
        match ch {
            'a'..='z' => {},
            'A'..='Z' => {},
            '0'..='9' => {},
            '_' => {},
            _ => { return false }
        }
    }

    true
}
