use super::*;

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
    Boolean,
    Void, // Only really crops up for method return types.
}



#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum JniField<'a> {
    Single(JniBasicType<'a>),
    Array { levels: usize, inner: JniBasicType<'a> },
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
                Some('V') => { *remaining = chars.as_str(); break JniBasicType::Void     }
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
