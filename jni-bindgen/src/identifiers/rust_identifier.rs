use super::*;

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
            // 2018 edition
            "async"     => RustIdentifier::KeywordRawSafe("r#async"),
            "await"     => RustIdentifier::KeywordRawSafe("r#await"),
            "try"       => RustIdentifier::KeywordRawSafe("r#try"),

            // [Weak keywords](https://doc.rust-lang.org/reference/keywords.html#weak-keywords) that *are* valid
            // [RAW_IDENTIFIER](https://doc.rust-lang.org/reference/identifiers.html)s
            "union"     => RustIdentifier::KeywordRawSafe("r#union"),

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



#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum IdentifierManglingError {
    NotApplicable(&'static str),
    EmptyString,
    NotRustSafe,
    UnexpectedCharacter(char),
}

impl std::error::Error for IdentifierManglingError {}
impl std::fmt::Display for IdentifierManglingError { fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result { std::fmt::Debug::fmt(self, fmt) } }



pub fn javaify_identifier(name: &str) -> Result<String, IdentifierManglingError> {
    if name == "_" {
        return Ok(String::from("__"));
    } else {
        let mut chars = name.chars();

        // First character
        if let Some(ch) = chars.next() {
            match ch {
                'a'..='z'   => {},
                'A'..='Z'   => {},
                '_'         => {},
                _           => { return Err(IdentifierManglingError::UnexpectedCharacter(ch)); },
            }
        }

        // Subsequent characters
        while let Some(ch) = chars.next() {
            match ch {
                'a'..='z'   => {},
                'A'..='Z'   => {},
                '0'..='9'   => {},
                '_'         => {},
                _           => { return Err(IdentifierManglingError::UnexpectedCharacter(ch)); },
            }
        }

        match RustIdentifier::from_str(name) {
            RustIdentifier::Identifier(_)               => Ok(name.to_owned()),
            RustIdentifier::NonIdentifier(_)            => Err(IdentifierManglingError::NotRustSafe),
            RustIdentifier::KeywordRawSafe(s)           => Ok(s.to_owned()),
            RustIdentifier::KeywordUnderscorePostfix(s) => Ok(s.to_owned()),
        }
    }
}

pub fn rustify_identifier(name: &str) -> Result<String, IdentifierManglingError> {
    if name == "_" {
        return Ok(String::from("__"));
    } else {
        let mut chars = name.chars();
        let mut buffer = String::new();
        let mut uppercase = 0;

        // First character
        if let Some(ch) = chars.next() {
            match ch {
                'a'..='z'   => buffer.push(ch),
                'A'..='Z'   => { buffer.push(ch.to_ascii_lowercase()); uppercase = 1; },
                '_'         => buffer.push(ch),
                _           => { return Err(IdentifierManglingError::UnexpectedCharacter(ch)); },
            }
        }

        // Subsequent characters
        while let Some(ch) = chars.next() {
            if ch.is_ascii_uppercase() {
                if uppercase == 0 && !buffer.ends_with('_') {
                    buffer.push('_');
                }
                buffer.push(ch.to_ascii_lowercase());
                uppercase += 1;
            } else if ch.is_ascii_alphanumeric() {
                if uppercase > 1 {
                    buffer.insert(buffer.len()-1, '_');
                }
                buffer.push(ch);
                uppercase = 0;
            } else if ch == '_' {
                buffer.push(ch);
                uppercase = 0;
            } else {
                return Err(IdentifierManglingError::UnexpectedCharacter(ch));
            }
        }

        match RustIdentifier::from_str(&buffer) {
            RustIdentifier::Identifier(_)               => Ok(buffer),
            RustIdentifier::NonIdentifier(_)            => Err(IdentifierManglingError::NotRustSafe),
            RustIdentifier::KeywordRawSafe(s)           => Ok(s.to_owned()),
            RustIdentifier::KeywordUnderscorePostfix(s) => Ok(s.to_owned()),
        }
    }
}

pub fn constify_identifier(name: &str) -> Result<String, IdentifierManglingError> {
    if name == "_" {
        return Ok(String::from("__"));
    } else {
        let mut chars = name.chars();
        let mut buffer = String::new();
        let mut uppercase = 0;
        let mut lowercase = 0;

        // First character
        if let Some(ch) = chars.next() {
            match ch {
                'a'..='z'   => { buffer.push(ch.to_ascii_uppercase()); lowercase = 1; },
                'A'..='Z'   => { buffer.push(ch); uppercase = 1; },
                '_'         => buffer.push(ch),
                _           => { return Err(IdentifierManglingError::UnexpectedCharacter(ch)); },
            }
        }

        // Subsequent characters
        while let Some(ch) = chars.next() {
            let is_upper   = ch.is_ascii_uppercase();
            let is_lower   = ch.is_ascii_lowercase();
            let is_numeric = ch.is_numeric();

            if is_lower && uppercase > 1 {
                buffer.insert(buffer.len()-1, '_');
            } else if is_upper && lowercase > 0 {
                buffer.push('_');
            }

            uppercase = if is_upper { uppercase + 1 } else { 0 };
            lowercase = if is_lower { lowercase + 1 } else { 0 };

            if is_lower {
                buffer.push(ch.to_ascii_uppercase());
            } else if is_upper || is_numeric || ch == '_' {
                buffer.push(ch);
            } else {
                return Err(IdentifierManglingError::UnexpectedCharacter(ch));
            }
        }

        match RustIdentifier::from_str(&buffer) {
            RustIdentifier::Identifier(_)               => Ok(buffer),
            RustIdentifier::NonIdentifier(_)            => Err(IdentifierManglingError::NotRustSafe),
            RustIdentifier::KeywordRawSafe(s)           => Ok(s.to_owned()),
            RustIdentifier::KeywordUnderscorePostfix(s) => Ok(s.to_owned()),
        }
    }
}
