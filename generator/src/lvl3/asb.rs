// MIT/Apache2 License

use super::{
    syn_util::{str_to_path, str_to_ty},
    InputParameter, Method, ParameterUsage, Statement, SumOfSizes, SumStatement, ToSyn, Type,
};

/// An implementation of the as-byte-sequence trait.
#[derive(Default, Debug)]
pub struct Asb {
    pub is_none: bool,
    /// Collection of statements defining the as_bytes function to create a vector of bytes representing
    /// this item.
    pub as_bytes_stmts: Vec<SumStatement>,
    /// Collection of statements defining the from_bytes function to create an instance of this object
    /// from a series of bytes.
    pub from_bytes_stmts: Vec<SumStatement>,
    /// Sum of sizes for the size.
    pub size: SumOfSizes,
    /// Name of the field used for FD getting.
    pub fd_getting: Option<String>,
}

impl Asb {
    #[inline]
    pub fn none() -> Self {
        Self {
            is_none: true,
            ..Default::default()
        }
    }

    #[inline]
    pub fn to_syn_item(self, tyname: &str) -> Vec<syn::Item> {
        let Self {
            is_none,
            as_bytes_stmts,
            from_bytes_stmts,
            fd_getting,
            size,
        } = self;

        if is_none {
            return vec![];
        }

        let mut as_bytes_method = Method::new(
            "as_bytes".into(),
            Some(ParameterUsage::Ref),
            vec![InputParameter {
                name: "bytes".into(),
                ty: Type::Slice(Box::new(Type::Basic("u8".into()))),
                usage: ParameterUsage::MutRef,
            }],
            Some(Type::Basic("usize".into())),
        );
        as_bytes_method.statements = as_bytes_stmts;
        let mut from_bytes_method = Method::new(
            "from_bytes".into(),
            None,
            vec![InputParameter {
                name: "bytes".into(),
                ty: Type::Slice(Box::new(Type::Basic("u8".into()))),
                usage: ParameterUsage::Ref,
            }],
            Some(Type::Opt(Box::new(Type::Tuple(vec![
                Type::Basic("Self".into()),
                Type::Basic("usize".into()),
            ])))),
        );
        from_bytes_method.statements = from_bytes_stmts;
        let mut size_method = Method::new(
            "size".into(),
            Some(ParameterUsage::Ref),
            vec![],
            Some(Type::Basic("usize".into())),
        );
        size_method.statements = vec![size.into()];
        let mut file_descriptors_method = match fd_getting {
            None => None,
            Some(ref fd_getting) => Some({
                let mut fdm = Method::new(
                    "file_descriptors".into(),
                    Some(ParameterUsage::MutRef),
                    vec![],
                    Some(Type::Opt(Box::new(Type::Ref(
                        Box::new(Type::Vector(Box::new(Type::Basic("Fd".into())))),
                        true,
                        None,
                    )))),
                );
                fdm.statements = vec![super::GetFdRef(fd_getting.clone()).into()];
                fdm.to_syn_impl_item(true)
            }),
        };
        vec![syn::Item::Impl(syn::ItemImpl {
            attrs: vec![],
            defaultness: None,
            unsafety: None,
            impl_token: Default::default(),
            generics: Default::default(),
            trait_: Some((None, str_to_path("AsByteSequence"), Default::default())),
            self_ty: Box::new(str_to_ty(tyname)),
            brace_token: Default::default(),
            items: {
                let mut v = vec![
                    as_bytes_method.to_syn_impl_item(true),
                    from_bytes_method.to_syn_impl_item(true),
                    size_method.to_syn_impl_item(true),
                ];
                v.extend(file_descriptors_method.into_iter());
                v
            },
        })]
    }
}
