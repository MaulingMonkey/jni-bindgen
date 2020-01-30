use std::fmt::{Display, Formatter, Result};

/// [Java SE 7 &sect; 4.1](https://docs.oracle.com/javase/specs/jvms/se7/html/jvms-4.html#jvms-4.1):  ClassFile::major_version.
/// See also [Wikipedia](https://en.wikipedia.org/wiki/Java_class_file#General_layout) which lists versions.
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub struct Major(pub(crate) u16);

#[allow(non_upper_case_globals)]
#[allow(dead_code)]
impl Major {
    pub const Unknown : Major = Major(0);

    pub const JDK_1_1 : Major = Major(0x2D);
    pub const JDK_1_2 : Major = Major(0x2E);
    pub const JDK_1_3 : Major = Major(0x2F);
    pub const JDK_1_4 : Major = Major(0x30);

    pub const JavaSE_5_0 : Major = Major(0x31);
    pub const JavaSE_6_0 : Major = Major(0x32);
    pub const JavaSE_7   : Major = Major(0x33);
    pub const JavaSE_8   : Major = Major(0x34);
    pub const JavaSE_9   : Major = Major(0x35);
    pub const JavaSE_10  : Major = Major(0x36);
    pub const JavaSE_11  : Major = Major(0x37);
    pub const JavaSE_12  : Major = Major(0x38);
    pub const JavaSE_13  : Major = Major(0x39);
    pub const JavaSE_14  : Major = Major(0x3A);
}

impl Default for Major {
    fn default() -> Self { Self::JDK_1_1 }
}

impl Display for Major {
    fn fmt(&self, f: &mut Formatter) -> Result {
        self.0.fmt(f)
    }
}
