// MIT/Apache2 License

use crate::lvl1::Expression as Lvl1Expression;
use heck::SnakeCase;
use std::{ops::Deref, str::FromStr};
use tinyvec::{tiny_vec, TinyVec};

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum BinaryOp {
    Add,
    Sub,
    Mult,
    Div,
    And,
}

impl FromStr for BinaryOp {
    type Err = ();

    #[inline]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "+" => Self::Add,
            "-" => Self::Sub,
            "*" => Self::Mult,
            "/" => Self::Div,
            "&" => Self::And,
            _ => return Err(()),
        })
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum UnaryOp {
    Not,
    /// Count the ones in this item.
    OneCount,
}

impl FromStr for UnaryOp {
    type Err = ();

    #[inline]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "~" => Self::Not,
            _ => return Err(()),
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ExpressionItem {
    FieldRef(Box<str>),
    Value(i64),
    BinaryOp(BinaryOp),
    UnaryOp(UnaryOp),
    /// ((length as usize) * 4) - index
    Remainder,
    SumOf(Box<str>),
}

impl Default for ExpressionItem {
    #[inline]
    fn default() -> Self {
        Self::Value(0)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct Expression {
    postfix: TinyVec<[ExpressionItem; 1]>,
}

impl Expression {
    /// Generate a list length based on the number of 1's in another field.
    #[inline]
    pub fn one_count(field: String) -> Self {
        Self {
            postfix: tiny_vec!([ExpressionItem; 1] => ExpressionItem::UnaryOp(UnaryOp::OneCount), ExpressionItem::FieldRef(field.into_boxed_str())),
        }
    }

    /// An expression dedicated to the remainder.
    #[inline]
    pub fn remainder() -> Self {
        Self {
            postfix: tiny_vec!([ExpressionItem; 1] => ExpressionItem::Remainder),
        }
    }

    /// Tell if this item involves a certain field.
    #[inline]
    pub fn involves_field(&self, f: &str) -> bool {
        self.postfix.iter().any(|t| match t {
            ExpressionItem::FieldRef(ft) => f == ft.deref(),
            ExpressionItem::SumOf(ls) => f == ls.deref(),
            _ => false,
        })
    }

    /// If this is a one-field list, return that one field.
    #[inline]
    pub fn single_item(&self) -> Option<&str> {
        if self.postfix.len() != 1 {
            None
        } else {
            match &self.postfix[0] {
                ExpressionItem::FieldRef(s) => Some(s),
                _ => None,
            }
        }
    }

    /// If this is a fixed-value list, return that value.
    #[inline]
    pub fn fixed_size(&self) -> Option<i64> {
        if self.postfix.len() != 1 {
            None
        } else {
            match &self.postfix[0] {
                ExpressionItem::Value(v) => Some(*v),
                _ => None,
            }
        }
    }

    /// Iterate over items in postfix notation.
    #[inline]
    pub fn iter(&self) -> impl Iterator<Item = &ExpressionItem> {
        self.postfix.iter()
    }
}

impl From<Lvl1Expression> for Expression {
    #[inline]
    fn from(ll: Lvl1Expression) -> Expression {
        Self {
            postfix: convert_ll(ll),
        }
    }
}

// helper recursive function to build a list from the lvl1 tree
#[inline]
fn convert_ll(length: Lvl1Expression) -> TinyVec<[ExpressionItem; 1]> {
    match length {
        Lvl1Expression::Value(v) => TinyVec::from([ExpressionItem::Value(v)]),
        Lvl1Expression::FieldReference(f) => {
            TinyVec::from([ExpressionItem::FieldRef(f.to_snake_case().into_boxed_str())])
        }
        Lvl1Expression::BinaryOp { op, left, right } => {
            // parse the op
            let op: BinaryOp = op.parse().unwrap();
            let left = convert_ll(*left);
            let right = convert_ll(*right);

            let mut res = Vec::with_capacity(1 + left.len() + right.len());
            res.push(ExpressionItem::BinaryOp(op));
            res.extend(left);
            res.extend(right);
            TinyVec::Heap(res)
        }
        Lvl1Expression::UnaryOp { op, target } => {
            // parse the op
            let op: UnaryOp = op.parse().unwrap();
            let target = convert_ll(*target);

            let mut res = TinyVec::Heap(Vec::with_capacity(1 + target.len()));
            res.push(ExpressionItem::UnaryOp(op));
            res.extend(target);
            res
        }
        Lvl1Expression::SumOf(t) => {
            TinyVec::from([ExpressionItem::SumOf(t.to_snake_case().into_boxed_str())])
        }
        Lvl1Expression::OneCount(ll) => TinyVec::Heap({
            let mut v = vec![ExpressionItem::UnaryOp(UnaryOp::OneCount)];
            v.extend(convert_ll(*ll));
            v
        }),
    }
}
