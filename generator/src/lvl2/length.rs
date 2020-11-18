// MIT/Apache2 License

use crate::lvl1::ListLength as Lvl1ListLength;
use std::str::FromStr;
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
pub enum ListLengthItem {
    FieldRef(Box<str>),
    Value(i64),
    BinaryOp(BinaryOp),
    UnaryOp(UnaryOp),
}

impl Default for ListLengthItem {
    #[inline]
    fn default() -> Self {
        Self::Value(0)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct ListLength {
    postfix: TinyVec<[ListLengthItem; 1]>,
}

impl ListLength {
    /// Generate a list length based on the number of 1's in another field.
    #[inline]
    pub fn one_count(field: String) -> Self {
        Self {
            postfix: tiny_vec!([ListLengthItem; 1] => ListLengthItem::UnaryOp(UnaryOp::OneCount), ListLengthItem::FieldRef(field.into_boxed_str())),
        }
    }

    /// If this is a one-field list, return that one field.
    #[inline]
    pub fn single_item(&self) -> Option<&str> {
        if self.postfix.len() != 1 {
            None
        } else {
            match &self.postfix[0] {
                ListLengthItem::FieldRef(s) => Some(s),
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
                ListLengthItem::Value(v) => Some(*v),
                _ => None,
            }
        }
    }

    /// Iterate over items in postfix notation.
    #[inline]
    pub fn iter(&self) -> impl Iterator<Item = &ListLengthItem> {
        self.postfix.iter()
    }
}

impl From<Lvl1ListLength> for ListLength {
    #[inline]
    fn from(ll: Lvl1ListLength) -> ListLength {
        Self {
            postfix: convert_ll(ll),
        }
    }
}

// helper recursive function to build a list from the lvl1 tree
#[inline]
fn convert_ll(length: Lvl1ListLength) -> TinyVec<[ListLengthItem; 1]> {
    match length {
        Lvl1ListLength::Value(v) => TinyVec::from([ListLengthItem::Value(v)]),
        Lvl1ListLength::FieldReference(f) => {
            TinyVec::from([ListLengthItem::FieldRef(f.into_boxed_str())])
        }
        Lvl1ListLength::BinaryOp { op, left, right } => {
            // parse the op
            let op: BinaryOp = op.parse().unwrap();
            let left = convert_ll(*left);
            let right = convert_ll(*right);

            let mut res = TinyVec::Heap(Vec::with_capacity(1 + left.len() + right.len()));
            res.push(ListLengthItem::BinaryOp(op));
            res.extend(left);
            res.extend(right);
            res
        }
        Lvl1ListLength::UnaryOp { op, target } => {
            // parse the op
            let op: UnaryOp = op.parse().unwrap();
            let target = convert_ll(*target);

            let mut res = TinyVec::Heap(Vec::with_capacity(1 + target.len()));
            res.push(ListLengthItem::UnaryOp(op));
            res.extend(target);
            res
        }
    }
}
