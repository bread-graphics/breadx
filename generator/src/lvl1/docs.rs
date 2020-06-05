// MIT/Apache2 License

use tinyvec::TinyVec;

#[derive(Default, Debug)]
pub struct FieldDoc {
    pub name: String,
    pub desc: String,
}

#[derive(Default, Debug)]
pub struct ErrorDoc {
    pub ty: String,
    pub reason: String,
}

#[derive(Default, Debug)]
pub struct SeeAlso {
    pub ty: String,
    pub name: String,
}

#[derive(Default, Debug)]
pub struct Docs {
    pub brief: String,
    pub descr: Option<String>,
    pub fields: TinyVec<[FieldDoc; 10]>,
    pub see_alsos: TinyVec<[SeeAlso; 2]>,
    pub errors: TinyVec<[ErrorDoc; 6]>,
}
