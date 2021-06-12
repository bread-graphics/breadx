// MIT/Apache2 License

use super::{
    syn_util::{item_field, str_to_exprpath},
    Asb, ExprWrapper, InputParameter, Method, ParameterUsage, RStruct, StructureItem, Trait,
    Type as Lvl3Type,
};
use crate::lvl2::{Field, StructureItem as Lvl2StructureItem, Type, XidType};

impl From<XidType> for RStruct {
    #[inline]
    fn from(xty: XidType) -> Self {
        let XidType { name, from_impls } = xty;

        // general struct defn
        let mut rstr = RStruct {
            name,
            derives: vec![
                "Default",
                "Copy",
                "Clone",
                "Debug",
                "PartialEq",
                "Eq",
                "PartialOrd",
                "Ord",
                "Hash",
            ],
            fds: vec![],
            is_transparent: true,
            fields: vec![StructureItem::from_lvl2(Lvl2StructureItem::Field(Field {
                name: "xid".into(),
                ty: Type::BasicType("XID".into()),
                ..Default::default()
            }))],
            methods: vec![],
            other_impl_items: vec![],
            traits: vec![],
            asb: Asb::none(),
            lifetimes: vec![],
        };

        // it needs a const. method for initialization of constants
        let mut method = Method::new(
            "const_from_xid".into(),
            None,
            vec![InputParameter {
                name: "xid".into(),
                ty: Lvl3Type::Basic("XID".into()),
                usage: ParameterUsage::Owned,
            }],
            Some(Lvl3Type::Basic("Self".into())),
        );
        method.is_const = true;
        method.statements = vec![super::CreateXidTypeStatement.into()];
        rstr.methods.push(method);

        rstr.traits.push(Trait::Xid);
        rstr.traits
            .extend(from_impls.into_iter().map(|f| Trait::FromXid(f.unwrap())));

        rstr
    }
}
