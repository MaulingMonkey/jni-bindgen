use super::*;



#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum JniDescriptorSegment<'a> {
    Parameter(JniField<'a>),
    Return(JniField<'a>),
}

pub struct JniDescriptor<'a> {
    remaining: &'a str,
}

impl<'a> JniDescriptor<'a> {
    pub fn new(remaining: &'a str) -> Result<JniDescriptor<'a>, &'static str> {
        if remaining.starts_with('(') {
            Ok(JniDescriptor { remaining: &remaining[1..] })
        } else {
            Err("Invalid JniDescriptor, does not start with '('")
        }
    }
}

impl<'a> Iterator for JniDescriptor<'a> {
    type Item = JniDescriptorSegment<'a>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.remaining.starts_with(')') {
            let field = JniField::from_str(&self.remaining[1..]);
            self.remaining = "";
            // XXX: Actually, failing silent here isn't just lame, it's kinda scary.  Fix it.
            field.ok().map(|f| JniDescriptorSegment::Return(f))
        } else {
            let field = JniField::read_next(&mut self.remaining);
            // XXX: Actually, failing silent here isn't just lame, it's kinda scary.  Fix it.
            field.ok().map(|f| JniDescriptorSegment::Parameter(f))
        }
    }
}

#[test] fn jni_descriptor_from_str() {
    use jar_parser::class;

    let mut d = JniDescriptor::new("(Landroid/net/Uri;[Ljava/lang/String;Ljava/lang/String;[Ljava/lang/String;Ljava/lang/String;)Landroid/database/Cursor;").unwrap();
    assert_eq!(d.next(), Some(JniDescriptorSegment::Parameter(JniField::Single(JniBasicType::Class(class::Id("android/net/Uri"))))));
    assert_eq!(d.next(), Some(JniDescriptorSegment::Parameter(JniField::Array { levels: 1, inner: JniBasicType::Class(class::Id("java/lang/String")) })));
    assert_eq!(d.next(), Some(JniDescriptorSegment::Parameter(JniField::Single(JniBasicType::Class(class::Id("java/lang/String"))))));
    assert_eq!(d.next(), Some(JniDescriptorSegment::Parameter(JniField::Array { levels: 1, inner: JniBasicType::Class(class::Id("java/lang/String")) })));
    assert_eq!(d.next(), Some(JniDescriptorSegment::Parameter(JniField::Single(JniBasicType::Class(class::Id("java/lang/String"))))));
    assert_eq!(d.next(), Some(JniDescriptorSegment::Return(JniField::Single(JniBasicType::Class(class::Id("android/database/Cursor"))))));
    assert_eq!(d.next(), None);
    assert_eq!(d.next(), None);

    let mut d = JniDescriptor::new("(Landroid/net/Uri;[Ljava/lang/String;Ljava/lang/String;[Ljava/lang/String;Ljava/lang/String;Landroid/os/CancellationSignal;)Landroid/database/Cursor;").unwrap();
    assert_eq!(d.next(), Some(JniDescriptorSegment::Parameter(JniField::Single(JniBasicType::Class(class::Id("android/net/Uri"))))));
    assert_eq!(d.next(), Some(JniDescriptorSegment::Parameter(JniField::Array { levels: 1, inner: JniBasicType::Class(class::Id("java/lang/String")) })));
    assert_eq!(d.next(), Some(JniDescriptorSegment::Parameter(JniField::Single(JniBasicType::Class(class::Id("java/lang/String"))))));
    assert_eq!(d.next(), Some(JniDescriptorSegment::Parameter(JniField::Array { levels: 1, inner: JniBasicType::Class(class::Id("java/lang/String")) })));
    assert_eq!(d.next(), Some(JniDescriptorSegment::Parameter(JniField::Single(JniBasicType::Class(class::Id("java/lang/String"))))));
    assert_eq!(d.next(), Some(JniDescriptorSegment::Parameter(JniField::Single(JniBasicType::Class(class::Id("android/os/CancellationSignal"))))));
    assert_eq!(d.next(), Some(JniDescriptorSegment::Return(JniField::Single(JniBasicType::Class(class::Id("android/database/Cursor"))))));
    assert_eq!(d.next(), None);
    assert_eq!(d.next(), None);

    let mut d = JniDescriptor::new("(Landroid/net/Uri;[Ljava/lang/String;Landroid/os/Bundle;Landroid/os/CancellationSignal;)Landroid/database/Cursor;").unwrap();
    assert_eq!(d.next(), Some(JniDescriptorSegment::Parameter(JniField::Single(JniBasicType::Class(class::Id("android/net/Uri"))))));
    assert_eq!(d.next(), Some(JniDescriptorSegment::Parameter(JniField::Array { levels: 1, inner: JniBasicType::Class(class::Id("java/lang/String")) })));
    assert_eq!(d.next(), Some(JniDescriptorSegment::Parameter(JniField::Single(JniBasicType::Class(class::Id("android/os/Bundle"))))));
    assert_eq!(d.next(), Some(JniDescriptorSegment::Parameter(JniField::Single(JniBasicType::Class(class::Id("android/os/CancellationSignal"))))));
    assert_eq!(d.next(), Some(JniDescriptorSegment::Return(JniField::Single(JniBasicType::Class(class::Id("android/database/Cursor"))))));
    assert_eq!(d.next(), None);
    assert_eq!(d.next(), None);

    let mut d = JniDescriptor::new("([Ljava/lang/String;)V").unwrap();
    assert_eq!(d.next(), Some(JniDescriptorSegment::Parameter(JniField::Array { levels: 1, inner: JniBasicType::Class(class::Id("java/lang/String")) })));
    assert_eq!(d.next(), Some(JniDescriptorSegment::Return(JniField::Single(JniBasicType::Void))));
    assert_eq!(d.next(), None);
    assert_eq!(d.next(), None);

    let mut d = JniDescriptor::new("()V").unwrap();
    assert_eq!(d.next(), Some(JniDescriptorSegment::Return(JniField::Single(JniBasicType::Void))));
    assert_eq!(d.next(), None);
    assert_eq!(d.next(), None);
}
