use super::*;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[allow(non_camel_case_types)]
#[repr(transparent)] pub struct jchar(pub jni_sys::jchar);

// TODO: Extend with encoding/decoding options.
// Can Java create improper UTF16?  Should encodings be to/from WTF8?  What about on Unix, where OsStr(ing) isn't WTF8?
// Several Java methods accept int s instead of char s for their characters...
// https://docs.oracle.com/javase/7/docs/api/java/lang/Character.html
