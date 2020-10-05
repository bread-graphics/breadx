// MIT/Apache2 License

#![forbid(unsafe_code)]
#![warn(clippy::pedantic)]
#![allow(clippy::default_trait_access)]
#![allow(clippy::map_err_ignore)]

use heck::CamelCase;
use proc_macro2::Span;
use std::{
  boxed::Box,
  env,
  error::Error,
  fmt::{self, Write},
  fs,
};
use syn::export::ToTokens;
use treexml::{Document, Element};

pub mod field;
pub mod syn_util;
use syn_util::*;

pub mod state;
mod stringify;
pub mod xenum;
mod xidtype;
mod xidunion;
mod xstruct;

fn main() -> Result<(), Box<dyn Error>> {
  env_logger::init();

  let mut state = state::State::new();

  // open the XCB description file
  let fname = env::args_os()
    .nth(1)
    .unwrap_or_else(|| "xcb.xml".to_owned().into());
  let outname = env::args_os()
    .nth(2)
    .unwrap_or_else(|| "exbound.rs".to_owned().into());
  let file = fs::File::open(&fname)?;
  let document = Document::parse(file).expect("Unable to parse XML document");

  let root = document.root.unwrap();
  root
    .children
    .iter()
    .map(|e| determine_unresolved(e, &mut state))
    .collect::<Result<(), Failures>>()?;

  let mut output = syn::File {
    shebang: None,
    attrs: Vec::new(),
    items: root
      .children
      .into_iter()
      .map(|e| translate(e, &mut state))
      .collect::<Result<Vec<Vec<syn::Item>>, Failures>>()?
      .into_iter()
      .flatten()
      .collect(),
  };

  output.items.extend(state.resolved());

  stringify::stringify(&mut output);

  let mut outtokens = proc_macro2::TokenStream::new();
  output.to_tokens(&mut outtokens);
  let mut outstring = String::new();
  write!(&mut outstring, "{}", outtokens)?;
  let mut outfile = fs::File::create(&outname)?;
  use std::io::Write;
  Write::write_all(&mut outfile, outstring.as_bytes())?;

  Ok(())
}

#[inline]
fn determine_unresolved(elem: &Element, state: &mut state::State) -> Result<(), Failures> {
  match elem.name.to_lowercase().as_str() {
    "enum" => {
      state.add_unresolved_enum(xenum::xenum(
        elem.attributes.get("name").unwrap(),
        elem.children.clone(),
      )?);
      Ok(())
    }
    _ => Ok(()),
  }
}

#[inline]
fn translate(elem: Element, state: &mut state::State) -> Result<Vec<syn::Item>, Failures> {
  // match the event name
  match elem.name.to_lowercase().as_str() {
    "xidtype" => {
      if let Some(name) = elem.attributes.get("name") {
        Ok(xidtype::xidtype(&name.to_camel_case()))
      } else {
        Err(Failures::MalformedXidtype)
      }
    }
    "xidunion" => {
      if let Some(name) = elem.attributes.get("name") {
        Ok(xidunion::xidunion(&name.to_camel_case(), elem.children)?)
      } else {
        Err(Failures::MalformedXidtype)
      }
    }
    "typedef" => {
      match (
        elem.attributes.get("oldname"),
        elem.attributes.get("newname"),
      ) {
        (Some(oldname), Some(newname)) => Ok(vec![syn::Item::Type(syn::ItemType {
          attrs: vec![],
          vis: pub_vis(),
          type_token: Default::default(),
          ident: syn::Ident::new(&newname.to_camel_case(), Span::call_site()),
          generics: Default::default(),
          eq_token: Default::default(),
          ty: Box::new(str_to_ty(&oldname.to_camel_case())),
          semi_token: Default::default(),
        })]),
        _ => Err(Failures::MalformedTypedef),
      }
    }
    "struct" => {
      if let Some(name) = elem.attributes.get("name") {
        Ok(xstruct::xstruct(
          &name.to_camel_case(),
          elem.children,
          state,
        )?)
      } else {
        Err(Failures::MalformedStruct(MalformedStruct::NoName))
      }
    }
    _ => Ok(vec![]),
  }
}

#[derive(Debug, thiserror::Error)]
pub enum Failures {
  #[error("xidtype tag did not have name attr")]
  MalformedXidtype,
  #[error("xidunion tag contained no elements")]
  MalformedXidunion,
  #[error("typedef did not have newname or oldname")]
  MalformedTypedef,
  #[error("{0}")]
  MalformedStruct(#[from] MalformedStruct),
  #[error("enum did not have any valid variants")]
  MalformedTrueEnum,
  #[error("unexpected non-zero and non-bits value in bitsflag")]
  MalformedBitflags,
  #[error("unexpected end of document")]
  UnexpectedEnd,
  #[error("{0}")]
  FromUtf8(#[from] std::string::FromUtf8Error),
}

#[derive(Debug, thiserror::Error)]
pub enum MalformedStruct {
  #[error("struct tag did not have name attr")]
  NoName,
  #[error("struct tag did not have any fields")]
  NoFields,
}
