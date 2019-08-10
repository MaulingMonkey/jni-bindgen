use super::*;
use crate::io::*;

use std::io::{self, Read};



#[allow(non_camel_case_types)]
#[derive(Clone, Debug)]
pub(crate) enum Attribute {
    /// https://docs.oracle.com/javase/specs/jvms/se7/html/jvms-4.html#jvms-4.7.2
    ConstantValue_Long(i64),
    /// https://docs.oracle.com/javase/specs/jvms/se7/html/jvms-4.html#jvms-4.7.2
    ConstantValue_Float(f32),
    /// https://docs.oracle.com/javase/specs/jvms/se7/html/jvms-4.html#jvms-4.7.2
    ConstantValue_Double(f64),
    /// https://docs.oracle.com/javase/specs/jvms/se7/html/jvms-4.html#jvms-4.7.2
    ConstantValue_Integer(i32), // int, short, char, byte, boolean
    /// https://docs.oracle.com/javase/specs/jvms/se7/html/jvms-4.html#jvms-4.7.2
    ConstantValue_String(String),

    /// https://docs.oracle.com/javase/specs/jvms/se7/html/jvms-4.html#jvms-4.7.3
    Code { #[doc(hidden)] __nyi: () },

    /// https://docs.oracle.com/javase/specs/jvms/se7/html/jvms-4.html#jvms-4.7.4
    StackMapTable { #[doc(hidden)] __nyi: () },

    /// https://docs.oracle.com/javase/specs/jvms/se7/html/jvms-4.html#jvms-4.7.5
    Exceptions { #[doc(hidden)] __nyi: () },

    /// https://docs.oracle.com/javase/specs/jvms/se7/html/jvms-4.html#jvms-4.7.6
    InnerClasses { #[doc(hidden)] __nyi: () },

    /// https://docs.oracle.com/javase/specs/jvms/se7/html/jvms-4.html#jvms-4.7.7
    EnclosingMethod { #[doc(hidden)] __nyi: () },

    /// https://docs.oracle.com/javase/specs/jvms/se7/html/jvms-4.html#jvms-4.7.8
    Synthetic { #[doc(hidden)] __nyi: () },

    /// https://docs.oracle.com/javase/specs/jvms/se7/html/jvms-4.html#jvms-4.7.9
    Signature { #[doc(hidden)] __nyi: () },

    /// https://docs.oracle.com/javase/specs/jvms/se7/html/jvms-4.html#jvms-4.7.10
    SourceFile { #[doc(hidden)] __nyi: () },

    /// https://docs.oracle.com/javase/specs/jvms/se7/html/jvms-4.html#jvms-4.7.11
    SourceDebugExtension { #[doc(hidden)] __nyi: () },

    /// https://docs.oracle.com/javase/specs/jvms/se7/html/jvms-4.html#jvms-4.7.12
    LineNumberTable { #[doc(hidden)] __nyi: () },

    /// https://docs.oracle.com/javase/specs/jvms/se7/html/jvms-4.html#jvms-4.7.13
    LocalVariableTable { #[doc(hidden)] __nyi: () },

    /// https://docs.oracle.com/javase/specs/jvms/se7/html/jvms-4.html#jvms-4.7.14
    LocalVariableTypeTable { #[doc(hidden)] __nyi: () },

    /// https://docs.oracle.com/javase/specs/jvms/se7/html/jvms-4.html#jvms-4.7.15
    Deprecated { #[doc(hidden)] __nyi: () },

    /// https://docs.oracle.com/javase/specs/jvms/se7/html/jvms-4.html#jvms-4.7.16
    RuntimeVisibleAnnotations { #[doc(hidden)] __nyi: () },

    /// https://docs.oracle.com/javase/specs/jvms/se7/html/jvms-4.html#jvms-4.7.17
    RuntimeInvisibleAnnotations { #[doc(hidden)] __nyi: () },

    /// https://docs.oracle.com/javase/specs/jvms/se7/html/jvms-4.html#jvms-4.7.18
    RuntimeVisibleParameterAnnotations { #[doc(hidden)] __nyi: () },

    /// https://docs.oracle.com/javase/specs/jvms/se7/html/jvms-4.html#jvms-4.7.19
    RuntimeInvisibleParameterAnnotations { #[doc(hidden)] __nyi: () },

    /// https://docs.oracle.com/javase/specs/jvms/se7/html/jvms-4.html#jvms-4.7.20
    AnnotationDefault { #[doc(hidden)] __nyi: () },

    /// https://docs.oracle.com/javase/specs/jvms/se7/html/jvms-4.html#jvms-4.7.21
    BootstrapMethods { #[doc(hidden)] __nyi: () },

    /// An unrecognized attribute was used!
    Unknown,

    #[doc(hidden)] __NonExhaustive,
}

impl Attribute {
    pub(crate) fn read(read: &mut impl Read, constants: &Constants) -> io::Result<Self> {
        let attribute_name_index    = read_u2(read)?;
        let attribute_length        = read_u4(read)? as usize;

        let name = constants.get_utf8(attribute_name_index)?;
        match name {
            "ConstantValue" => {
                // https://docs.oracle.com/javase/specs/jvms/se7/html/jvms-4.html#jvms-4.7.2
                io_assert!(attribute_length == 2);
                let constantvalue_index = read_u2(read)?;
                let constant = constants.get(constantvalue_index)?;
                match constant {
                    Constant::Long(value)               => Ok(Attribute::ConstantValue_Long(*value)),
                    Constant::Float(value)              => Ok(Attribute::ConstantValue_Float(*value)),
                    Constant::Double(value)             => Ok(Attribute::ConstantValue_Double(*value)),
                    Constant::Integer(value)            => Ok(Attribute::ConstantValue_Integer(*value)),
                    Constant::String { string_index }   => Ok(Attribute::ConstantValue_String(constants.get_utf8(*string_index)?.to_owned())),
                    c                                   => io_data_err!("Expected Constant::{{Long, Float, Double, Integer, String}}, got {:?}", c),
                }
            },
            "Code"                                  => { read_ignore(read, attribute_length)?; Ok(Attribute::Code                                  {__nyi:()}) },
            "StackMapTable"                         => { read_ignore(read, attribute_length)?; Ok(Attribute::StackMapTable                         {__nyi:()}) },
            "Exceptions"                            => { read_ignore(read, attribute_length)?; Ok(Attribute::Exceptions                            {__nyi:()}) },
            "InnerClasses"                          => { read_ignore(read, attribute_length)?; Ok(Attribute::InnerClasses                          {__nyi:()}) },
            "EnclosingMethod"                       => { read_ignore(read, attribute_length)?; Ok(Attribute::EnclosingMethod                       {__nyi:()}) },
            "Synthetic"                             => { read_ignore(read, attribute_length)?; Ok(Attribute::Synthetic                             {__nyi:()}) },
            "Signature"                             => { read_ignore(read, attribute_length)?; Ok(Attribute::Signature                             {__nyi:()}) },
            "SourceFile"                            => { read_ignore(read, attribute_length)?; Ok(Attribute::SourceFile                            {__nyi:()}) },
            "SourceDebugExtension"                  => { read_ignore(read, attribute_length)?; Ok(Attribute::SourceDebugExtension                  {__nyi:()}) },
            "LineNumberTable"                       => { read_ignore(read, attribute_length)?; Ok(Attribute::LineNumberTable                       {__nyi:()}) },
            "LocalVariableTable"                    => { read_ignore(read, attribute_length)?; Ok(Attribute::LocalVariableTable                    {__nyi:()}) },
            "LocalVariableTypeTable"                => { read_ignore(read, attribute_length)?; Ok(Attribute::LocalVariableTypeTable                {__nyi:()}) },
            "Deprecated"                            => { read_ignore(read, attribute_length)?; Ok(Attribute::Deprecated                            {__nyi:()}) },
            "RuntimeVisibleAnnotations"             => { read_ignore(read, attribute_length)?; Ok(Attribute::RuntimeVisibleAnnotations             {__nyi:()}) },
            "RuntimeInvisibleAnnotations"           => { read_ignore(read, attribute_length)?; Ok(Attribute::RuntimeInvisibleAnnotations           {__nyi:()}) },
            "RuntimeVisibleParameterAnnotations"    => { read_ignore(read, attribute_length)?; Ok(Attribute::RuntimeVisibleParameterAnnotations    {__nyi:()}) },
            "RuntimeInvisibleParameterAnnotations"  => { read_ignore(read, attribute_length)?; Ok(Attribute::RuntimeInvisibleParameterAnnotations  {__nyi:()}) },
            "AnnotationDefault"                     => { read_ignore(read, attribute_length)?; Ok(Attribute::AnnotationDefault                     {__nyi:()}) },
            "BootstrapMethods"                      => { read_ignore(read, attribute_length)?; Ok(Attribute::BootstrapMethods                      {__nyi:()}) },
            _                                       => { read_ignore(read, attribute_length)?; Ok(Attribute::Unknown) },
        }
    }
}
