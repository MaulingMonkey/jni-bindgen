use crate::identifiers::*;
use jreflection::class::IdPart;
use jreflection::method;
use serde_derive::*;



#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "snake_case")]
pub enum MethodManglingStyle {
    /// Leave the original method name alone as much as possible.
    /// Constructors will still be renamed from "<init>" to "new".
    ///
    /// # Examples:
    ///
    /// | Java      | Rust      |
    /// | --------- | --------- |
    /// | getFoo    | getFoo    |
    /// | \<init\>  | new       |
    Java,

    /// Leave the original method name alone as much as possible... with unqualified typenames appended for disambiguation.
    /// Constructors will still be renamed from "<init>" to "new".
    ///
    /// # Examples:
    ///
    /// | Java      | Rust          |
    /// | --------- | ------------- |
    /// | getFoo    | getFoo_int    |
    /// | \<init\>  | new_Object    |
    JavaShortSignature,

    /// Leave the original method name alone as much as possible... with qualified typenames appended for disambiguation.
    /// Constructors will still be renamed from "<init>" to "new".
    ///
    /// # Examples:
    ///
    /// | Java      | Rust                  |
    /// | --------- | --------------------- |
    /// | getFoo    | getFoo_int            |
    /// | \<init\>  | new_java_lang_Object  |
    JavaLongSignature,

    /// Rename the method to use rust style naming conventions.
    ///
    /// # Examples:
    ///
    /// | Java      | Rust      |
    /// | --------- | --------- |
    /// | getFoo    | get_foo   |
    /// | \<init\>  | new       |
    Rustify,

    /// Rename the method to use rust style naming conventions, with unqualified typenames appended for disambiguation.
    ///
    /// # Examples:
    ///
    /// | Java      | Rust          |
    /// | --------- | ------------- |
    /// | getFoo    | get_foo_int   |
    /// | \<init\>  | new_object    |
    RustifyShortSignature,

    /// Rename the method to use rust style naming conventions, with qualified typenames appended for disambiguation.
    ///
    /// # Examples:
    ///
    /// | Java      | Rust                  |
    /// | --------- | --------------------- |
    /// | getFoo    | get_foo_int           |
    /// | \<init\>  | new_java_lang_object  |
    RustifyLongSignature,
}

#[test] fn method_mangling_style_mangle_test() {
    for &(name,    sig,                     java,     java_short,      java_long,                 rust,      rust_short,       rust_long                  ) in &[
        ("getFoo", "()V",                   "getFoo", "getFoo",        "getFoo",                  "get_foo", "get_foo",        "get_foo"                  ),
        ("getFoo", "(I)V",                  "getFoo", "getFoo_int",    "getFoo_int",              "get_foo", "get_foo_int",    "get_foo_int"              ),
        ("getFoo", "(Ljava/lang/Object;)V", "getFoo", "getFoo_Object", "getFoo_java_lang_Object", "get_foo", "get_foo_object", "get_foo_java_lang_object" ),
        ("<init>", "()V",                   "new",    "new",           "new",                     "new",     "new",            "new"                      ),
        ("<init>", "(I)V",                  "new",    "new_int",       "new_int",                 "new",     "new_int",        "new_int"                  ),
        ("<init>", "(Ljava/lang/Object;)V", "new",    "new_Object",    "new_java_lang_Object",    "new",     "new_object",     "new_java_lang_object"     ),
        // TODO: get1DFoo
        // TODO: array types (primitive + non-primitive)
    ] {
        let sig = method::Descriptor::new(sig).unwrap();

        assert_eq!(MethodManglingStyle::Java                            .mangle(name, sig).unwrap(), java);
        assert_eq!(MethodManglingStyle::JavaShortSignature              .mangle(name, sig).unwrap(), java_short);
        assert_eq!(MethodManglingStyle::JavaLongSignature               .mangle(name, sig).unwrap(), java_long);

        assert_eq!(MethodManglingStyle::Rustify                         .mangle(name, sig).unwrap(), rust);
        assert_eq!(MethodManglingStyle::RustifyShortSignature           .mangle(name, sig).unwrap(), rust_short);
        assert_eq!(MethodManglingStyle::RustifyLongSignature            .mangle(name, sig).unwrap(), rust_long);
    }
}

#[test] fn mangle_method_name_test() {
    assert_eq!(MethodManglingStyle::Rustify.mangle("isFooBar",          method::Descriptor::new("()V").unwrap()).unwrap(), "is_foo_bar"         );
    assert_eq!(MethodManglingStyle::Rustify.mangle("XMLHttpRequest",    method::Descriptor::new("()V").unwrap()).unwrap(), "xml_http_request"   );
    assert_eq!(MethodManglingStyle::Rustify.mangle("getFieldID_Input",  method::Descriptor::new("()V").unwrap()).unwrap(), "get_field_id_input" );
}

impl MethodManglingStyle {
    pub fn mangle(&self, name: &str, descriptor: method::Descriptor) -> Result<String, IdentifierManglingError> {
        let name = match name {
            ""          => { return Err(IdentifierManglingError::EmptyString); },
            "<init>"    => "new",
            "<clinit>"  => { return Err(IdentifierManglingError::NotApplicable("Static type ctor")); },
            name        => name,
        };

        match self {
            MethodManglingStyle::Java                   => Ok(javaify_identifier(name)?),
            MethodManglingStyle::JavaShortSignature     => Ok(javaify_identifier(&format!("{}{}", name, short_sig(descriptor) )[..])?),
            MethodManglingStyle::JavaLongSignature      => Ok(javaify_identifier(&format!("{}{}", name, long_sig(descriptor)  )[..])?),

            MethodManglingStyle::Rustify                => Ok(rustify_identifier(name)?),
            MethodManglingStyle::RustifyShortSignature  => Ok(rustify_identifier(&format!("{}{}", name, short_sig(descriptor) )[..])?),
            MethodManglingStyle::RustifyLongSignature   => Ok(rustify_identifier(&format!("{}{}", name, long_sig(descriptor)  )[..])?),
        }
    }
}

fn short_sig(descriptor: method::Descriptor) -> String {
    use method::*;

    let mut buffer = String::new();

    for arg in descriptor.arguments() {
        match arg {
            Type::Single(BasicType::Boolean  ) => { buffer.push_str("_boolean");     },
            Type::Single(BasicType::Byte     ) => { buffer.push_str("_byte");        },
            Type::Single(BasicType::Char     ) => { buffer.push_str("_char");        },
            Type::Single(BasicType::Double   ) => { buffer.push_str("_double");      },
            Type::Single(BasicType::Float    ) => { buffer.push_str("_float");       },
            Type::Single(BasicType::Int      ) => { buffer.push_str("_int");         },
            Type::Single(BasicType::Long     ) => { buffer.push_str("_long");        },
            Type::Single(BasicType::Short    ) => { buffer.push_str("_short");       },
            Type::Single(BasicType::Void     ) => { buffer.push_str("_void");        },
            Type::Single(BasicType::Class(class)) => {
                if let Some(IdPart::LeafClass(leaf)) = class.iter().last() {
                    buffer.push('_');
                    buffer.push_str(leaf);
                } else {
                    buffer.push_str("_unknown");
                }
            },
            Type::Array { levels, inner } => {
                match inner {
                    BasicType::Boolean   => { buffer.push_str("_boolean");   },
                    BasicType::Byte      => { buffer.push_str("_byte");      },
                    BasicType::Char      => { buffer.push_str("_char");      },
                    BasicType::Double    => { buffer.push_str("_double");    },
                    BasicType::Float     => { buffer.push_str("_float");     },
                    BasicType::Int       => { buffer.push_str("_int");       },
                    BasicType::Long      => { buffer.push_str("_long");      },
                    BasicType::Short     => { buffer.push_str("_short");     },
                    BasicType::Void      => { buffer.push_str("_void");      },
                    BasicType::Class(class) => {
                        for component in class.iter() {
                            match component {
                                IdPart::Namespace(_) => {},
                                IdPart::ContainingClass(_) => {},
                                IdPart::LeafClass(cls) => {
                                    buffer.push('_');
                                    buffer.push_str(cls);
                                },
                            }
                        }
                    },
                }

                for _ in 0..levels {
                    buffer.push_str("_array");
                }
            }
        }
    }

    buffer
}

fn long_sig(descriptor: method::Descriptor) -> String {
    use method::*;

    let mut buffer = String::new();

    for arg in descriptor.arguments() {
        match arg {
            Type::Single(BasicType::Boolean  ) => { buffer.push_str("_boolean"); },
            Type::Single(BasicType::Byte     ) => { buffer.push_str("_byte");    },
            Type::Single(BasicType::Char     ) => { buffer.push_str("_char");    },
            Type::Single(BasicType::Double   ) => { buffer.push_str("_double");  },
            Type::Single(BasicType::Float    ) => { buffer.push_str("_float");   },
            Type::Single(BasicType::Int      ) => { buffer.push_str("_int");     },
            Type::Single(BasicType::Long     ) => { buffer.push_str("_long");    },
            Type::Single(BasicType::Short    ) => { buffer.push_str("_short");   },
            Type::Single(BasicType::Void     ) => { buffer.push_str("_void");    },
            Type::Single(BasicType::Class(class)) => {
                for component in class.iter() {
                    buffer.push('_');
                    match component {
                        IdPart::Namespace(namespace) => { buffer.push_str(namespace); },
                        IdPart::ContainingClass(cls) => { buffer.push_str(cls); },
                        IdPart::LeafClass(cls)       => { buffer.push_str(cls); },
                    }
                }
            },
            Type::Array { levels, inner } => {
                match inner {
                    BasicType::Boolean   => { buffer.push_str("_boolean");   },
                    BasicType::Byte      => { buffer.push_str("_byte");      },
                    BasicType::Char      => { buffer.push_str("_char");      },
                    BasicType::Double    => { buffer.push_str("_double");    },
                    BasicType::Float     => { buffer.push_str("_float");     },
                    BasicType::Int       => { buffer.push_str("_int");       },
                    BasicType::Long      => { buffer.push_str("_long");      },
                    BasicType::Short     => { buffer.push_str("_short");     },
                    BasicType::Void      => { buffer.push_str("_void");      },
                    BasicType::Class(class) => {
                        for component in class.iter() {
                            buffer.push('_');
                            match component {
                                IdPart::Namespace(namespace) => { buffer.push_str(namespace); },
                                IdPart::ContainingClass(cls) => { buffer.push_str(cls); },
                                IdPart::LeafClass(cls)       => { buffer.push_str(cls); },
                            }
                        }
                    },
                }

                for _ in 0..levels {
                    buffer.push_str("_array");
                }
            }
        }
    }

    buffer
}
