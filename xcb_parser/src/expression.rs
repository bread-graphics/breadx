// MIT/Apache2 License

use crate::{find_attribute, read_text_or_cdata};
use anyhow::{anyhow, Context, Error, Result};
use quick_xml::{
    events::{BytesStart, Event},
    Reader,
};
use std::{io::BufRead, str::FromStr};

/// An expression, which is used to calculate a value.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Expression {
    /// Binary operation
    BinOp {
        op: BinOp,
        left: Box<Expression>,
        right: Box<Expression>,
    },
    /// Reference to a field
    Fieldref(String),
    /// Reference to a parameter outside of the structure.
    Paramref { ty: String, name: String },
    /// An integer value.
    Value(i64),
    /// A number of bits to shift by.
    Bit(i64),
    /// A reference to a value in an enum.
    EnumRef { ref_: String, name: String },
    /// An unary operation.
    UnOp { op: UnOp, expr: Box<Expression> },
    /// Sum of the items in the referenced list, with a possible
    /// mapping.
    SumOf {
        list: String,
        mapping: Option<Box<Expression>>,
    },
    /// A reference to a value in a list, used inside of `SumOf`.
    ListElementRef,
    /// The number of bits set in an expression.
    Popcount(Box<Expression>),
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum BinOp {
    Add,
    Subtract,
    Multiply,
    Divide,
    BinaryAnd,
    BinaryLeftShift,
}

impl FromStr for BinOp {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        match s {
            "+" => Ok(BinOp::Add),
            "-" => Ok(BinOp::Subtract),
            "*" => Ok(BinOp::Multiply),
            "/" => Ok(BinOp::Divide),
            "&" => Ok(BinOp::BinaryAnd),
            "<<" => Ok(BinOp::BinaryLeftShift),
            _ => Err(anyhow!("invalid binary operator: {}", s)),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum UnOp {
    Not,
}

impl FromStr for UnOp {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        match s {
            "~" => Ok(UnOp::Not),
            _ => Err(anyhow!("invalid unary operator: {}", s)),
        }
    }
}

impl Expression {
    #[inline]
    pub(crate) fn parse_with_start_elem<B: BufRead>(
        start_elem: BytesStart<'_>,
        reader: &mut Reader<B>,
        is_empty: bool,
    ) -> Result<Option<Self>> {
        match start_elem.name() {
            b"fieldref" => {
                let name = read_text_or_cdata(reader)?;
                Ok(Some(Expression::Fieldref(name)))
            }
            b"paramref" => {
                let ty = find_attribute(&start_elem, b"type", reader)?;
                let name = read_text_or_cdata(reader)?;
                Ok(Some(Expression::Paramref { ty, name }))
            }
            b"value" => {
                let value = read_text_or_cdata(reader)?;
                Ok(Some(Expression::Value(i64::from_str(&value)?)))
            }
            b"bit" => {
                let value = read_text_or_cdata(reader)?;
                Ok(Some(Expression::Bit(i64::from_str(&value)?)))
            }
            b"enumref" => {
                let ref_ = find_attribute(&start_elem, b"ref", reader)?;
                let name = read_text_or_cdata(reader)?;
                Ok(Some(Expression::EnumRef { ref_, name }))
            }
            b"unop" => {
                let op = find_attribute(&start_elem, b"op", reader)?;
                let expr = Expression::parse(reader)?;
                Ok(Some(Expression::UnOp {
                    op: UnOp::from_str(&op)?,
                    expr: Box::new(expr),
                }))
            }
            b"op" => {
                let op = find_attribute(&start_elem, b"op", reader)?;
                let left = Expression::parse(reader)?;
                let right = Expression::parse(reader)?;
                Ok(Some(Expression::BinOp {
                    op: BinOp::from_str(&op)?,
                    left: Box::new(left),
                    right: Box::new(right),
                }))
            }
            b"sumof" if !is_empty => {
                let list = find_attribute(&start_elem, b"ref", reader)?;
                let mapping = Expression::parse(reader)?;
                Ok(Some(Expression::SumOf {
                    list,
                    mapping: Some(Box::new(mapping)),
                }))
            }
            b"sumof" => {
                let list = find_attribute(&start_elem, b"ref", reader)?;
                Ok(Some(Expression::SumOf {
                    list,
                    mapping: None,
                }))
            }
            b"popcount" => {
                let expr = Expression::parse(reader)?;
                Ok(Some(Expression::Popcount(Box::new(expr))))
            }
            b"listelement-ref" => Ok(Some(Expression::ListElementRef)),
            _ => Ok(None),
        }
    }

    /// Parse this expression.
    ///
    /// This assumes that we are already in the expression block.
    #[inline]
    pub(crate) fn parse<B: BufRead>(reader: &mut Reader<B>) -> Result<Self> {
        let mut buffer = vec![];

        loop {
            match reader.read_event(&mut buffer)? {
                Event::Start(s) => {
                    if let Some(expr) = Expression::parse_with_start_elem(s, reader, false)? {
                        return Ok(expr);
                    }
                }
                Event::Empty(e) => {
                    if let Some(expr) = Expression::parse_with_start_elem(e, reader, true)? {
                        return Ok(expr);
                    }
                }
                Event::Eof => {
                    return Err(anyhow!("unexpected end of file")).context("expression parsing")
                }
                _ => continue,
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use quick_xml::Reader;

    #[test]
    fn test_parse_fieldref() {
        let mut reader = Reader::from_str("<fieldref>foo</fieldref>");
        let expr = Expression::parse(&mut reader).unwrap();
        assert_eq!(expr, Expression::Fieldref("foo".to_string()));
    }

    #[test]
    fn test_parse_paramref() {
        let mut reader = Reader::from_str("<paramref type=\"int\">foo</paramref>");
        let expr = Expression::parse(&mut reader).unwrap();
        assert_eq!(
            expr,
            Expression::Paramref {
                ty: "int".to_string(),
                name: "foo".to_string(),
            }
        );
    }

    #[test]
    fn test_parse_value() {
        let mut reader = Reader::from_str("<value>42</value>");
        let expr = Expression::parse(&mut reader).unwrap();
        assert_eq!(expr, Expression::Value(42));
    }

    #[test]
    fn test_parse_bit() {
        let mut reader = Reader::from_str("<bit>42</bit>");
        let expr = Expression::parse(&mut reader).unwrap();
        assert_eq!(expr, Expression::Bit(42));
    }

    #[test]
    fn test_parse_enumref() {
        let mut reader = Reader::from_str("<enumref ref=\"foo\">bar</enumref>");
        let expr = Expression::parse(&mut reader).unwrap();
        assert_eq!(
            expr,
            Expression::EnumRef {
                ref_: "foo".to_string(),
                name: "bar".to_string(),
            }
        );
    }

    #[test]
    fn test_parse_unop() {
        let mut reader = Reader::from_str("<unop op=\"~\"><value>42</value></unop>");
        let expr = Expression::parse(&mut reader).unwrap();
        assert_eq!(
            expr,
            Expression::UnOp {
                op: UnOp::Not,
                expr: Box::new(Expression::Value(42)),
            }
        );
    }

    #[test]
    fn test_parse_binop() {
        let mut reader =
            Reader::from_str("<op op=\"+\"><value>42</value><value>43</value></binop>");
        let expr = Expression::parse(&mut reader).unwrap();
        assert_eq!(
            expr,
            Expression::BinOp {
                op: BinOp::Add,
                left: Box::new(Expression::Value(42)),
                right: Box::new(Expression::Value(43)),
            }
        );
    }

    #[test]
    fn test_parse_sumof() {
        let mut reader = Reader::from_str("<sumof ref=\"foo\"><listelement-ref /></sumof>");
        let expr = Expression::parse(&mut reader).unwrap();
        assert_eq!(
            expr,
            Expression::SumOf {
                list: "foo".to_string(),
                mapping: Some(Box::new(Expression::ListElementRef)),
            }
        );
    }

    #[test]
    fn test_parse_sumof_empty() {
        let mut reader = Reader::from_str("<sumof ref=\"foo\"/>");
        let expr = Expression::parse(&mut reader).unwrap();
        assert_eq!(
            expr,
            Expression::SumOf {
                list: "foo".to_string(),
                mapping: None,
            }
        );
    }

    #[test]
    fn test_parse_popcount() {
        let mut reader = Reader::from_str("<popcount><value>42</value></popcount>");
        let expr = Expression::parse(&mut reader).unwrap();
        assert_eq!(expr, Expression::Popcount(Box::new(Expression::Value(42))));
    }

    #[test]
    fn test_parse_listelementref() {
        let mut reader = Reader::from_str("<listelement-ref />");
        let expr = Expression::parse(&mut reader).unwrap();
        assert_eq!(expr, Expression::ListElementRef);
    }
}
