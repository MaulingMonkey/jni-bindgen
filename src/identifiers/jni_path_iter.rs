
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
