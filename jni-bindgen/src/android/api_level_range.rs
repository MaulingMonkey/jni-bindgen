use std::str::FromStr;
use std::ops::{Deref, DerefMut, RangeInclusive};

pub type RawApiLevelRange = RangeInclusive<u32>;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ApiLevelRange(pub RawApiLevelRange);

impl ApiLevelRange {
    pub fn start(&self) -> u32 { *self.0.start() }
    pub fn end  (&self) -> u32 { *self.0.end  () }
    pub fn iter(&self) -> impl Iterator<Item = u32> { self.0.clone() }
}

impl From<RawApiLevelRange> for ApiLevelRange { fn from(value: RawApiLevelRange) -> Self { Self(value) } }
impl Deref      for ApiLevelRange { fn deref     (&self)     -> &Self::Target        { &self.0 } type Target = RawApiLevelRange; }
impl DerefMut   for ApiLevelRange { fn deref_mut (&mut self) -> &mut Self::Target    { &mut self.0 } }


impl FromStr for ApiLevelRange {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let err = "Unable to parse API level range:  Expected '7', '7-28', or '7..=28' or similar integer ranges.";

        if let Ok(single) = s.parse::<u32>() {
            Ok(Self(single..=single))
        } else if let Some(dash) = s.find('-') {
            let (start, end) = s.split_at(dash);
            let (start, end) = (start.parse::<u32>().map_err(|_| err)?, end[1..].parse::<u32>().map_err(|_| err)?);
            Ok(Self(start..=end))
        } else if let Some(range) = s.find("..=") {
            let (start, end) = s.split_at(range);
            let (start, end) = (start.parse::<u32>().map_err(|_| err)?, end[3..].parse::<u32>().map_err(|_| err)?);
            Ok(Self(start..=end))
        } else {
            Err(err)
        }
    }
}

#[test] fn parse() {
    assert_eq!(ApiLevelRange(8..=27), "8-27"   .parse::<ApiLevelRange>().unwrap());
    assert_eq!(ApiLevelRange(8..=27), "8..=27" .parse::<ApiLevelRange>().unwrap());

    assert_eq!(ApiLevelRange(8..=8), "8"       .parse::<ApiLevelRange>().unwrap());
    assert_eq!(ApiLevelRange(8..=8), "8-8"     .parse::<ApiLevelRange>().unwrap());
    assert_eq!(ApiLevelRange(8..=8), "8..=8"   .parse::<ApiLevelRange>().unwrap());
}
