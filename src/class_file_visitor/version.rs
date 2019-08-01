use std::fmt::{Display, Formatter, Result};

#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub struct MajorVersion(u16);

#[allow(non_upper_case_globals)]
impl MajorVersion {
    pub const Unknown : MajorVersion = MajorVersion(0);

    pub const JDK_1_1 : MajorVersion = MajorVersion(0x2D);
    pub const JDK_1_2 : MajorVersion = MajorVersion(0x2E);
    pub const JDK_1_3 : MajorVersion = MajorVersion(0x2F);
    pub const JDK_1_4 : MajorVersion = MajorVersion(0x30);

    pub const JavaSE_5_0 : MajorVersion = MajorVersion(0x31);
    pub const JavaSE_6_0 : MajorVersion = MajorVersion(0x32);
    pub const JavaSE_7   : MajorVersion = MajorVersion(0x33);
    pub const JavaSE_8   : MajorVersion = MajorVersion(0x34);
    pub const JavaSE_9   : MajorVersion = MajorVersion(0x35);
    pub const JavaSE_10  : MajorVersion = MajorVersion(0x36);
    pub const JavaSE_11  : MajorVersion = MajorVersion(0x37);
    pub const JavaSE_12  : MajorVersion = MajorVersion(0x38);
    pub const JavaSE_13  : MajorVersion = MajorVersion(0x39);
    pub const JavaSE_14  : MajorVersion = MajorVersion(0x3A);
}

impl Default for MajorVersion {
    fn default() -> Self { Self::JDK_1_1 }
}

impl Display for MajorVersion {
    fn fmt(&self, f: &mut Formatter) -> Result {
        self.0.fmt(f)
    }
}

impl From<u16> for MajorVersion {
    fn from(value: u16) -> Self {
        Self(value)
    }
}
