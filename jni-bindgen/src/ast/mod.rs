#![allow(dead_code)]
#![allow(unused_imports)]

mod java {
    use std::collections::BTreeSet;

    pub type Id         = String;
    pub type SdkId      = String;
    pub type SdkSet     = BTreeSet<SdkId>;
    pub type ClassId    = String;
    pub type MethodName = String;
    pub type MethodSig  = String;
    pub type FieldName  = String;
    pub type FieldSig   = String;
}

mod rust {
    use super::{java, rust};
    use std::collections::BTreeMap;

    pub type StructId   = String;
    pub type MemberId   = String;
    pub type ModuleId   = String;

    #[derive(Default)]
    pub struct Struct {
        pub sdks:       BTreeMap<java::SdkId, java::ClassId>, // Original name of this struct in that Sdk
        pub members:    BTreeMap<java::Id, rust::Member>,
    }

    #[derive(Default)]
    pub struct Member {
        pub fields:     BTreeMap<java::FieldSig,    java::SdkSet>,
        pub methods:    BTreeMap<java::MethodSig,   java::SdkSet>,
    }
}


use std::collections::{BTreeMap, BTreeSet};

pub struct Ast {
    pub structures: BTreeMap<rust::StructId, rust::Struct>,
}

mod add_classes;
