// MIT/Apache2 License

use crate::syn_util::*;

#[inline]
pub fn is_vec_of_char(ty: &syn::Type) -> bool {
  match ty {
    syn::Type::Path(syn::TypePath { ref path, .. }) => match &path.segments[0] {
      syn::PathSegment {
        ident,
        arguments: syn::PathArguments::None,
        ..
      } => format!("{}", ident).as_str() == "Str",
      syn::PathSegment {
        ident,
        arguments:
          syn::PathArguments::AngleBracketed(syn::AngleBracketedGenericArguments { args, .. }),
      } => {
        if format!("{}", ident).as_str() == "Vec" {
          if let syn::GenericArgument::Type(syn::Type::Path(syn::TypePath { path, .. })) = &args[0]
          {
            let rep = format!("{}", path.get_ident().unwrap());
            rep.as_str() == "Char" || rep.as_str() == "bool"
          } else {
            false
          }
        } else {
          false
        }
      }
      _ => false,
    },
    _ => false,
  }
}

// Convert Vec<Char> and instances of Str to string
#[inline]
pub fn stringify(file: &mut syn::File) {
  file.items.iter_mut().for_each(|item| match item {
    syn::Item::Struct(syn::ItemStruct { ident, fields, .. }) => {
      if &format!("{}", ident) == "Str" {
        *item = syn::Item::Verbatim("".parse().unwrap());
      } else {
        match fields {
          syn::Fields::Named(syn::FieldsNamed { named, .. }) => {
            named.iter_mut().for_each(|f| {
              if is_vec_of_char(&f.ty) {
                f.ty = str_to_ty("String");
              }
            });
          }
          _ => (),
        }
      }
    }
    syn::Item::Impl(syn::ItemImpl { self_ty, .. }) => match *self_ty.clone() {
      syn::Type::Path(syn::TypePath { path, .. }) => {
        let id = format!(
          "{}",
          match path.get_ident() {
            Some(id) => id,
            None => return,
          }
        );
        if id.as_str() == "String" {
          *item = syn::Item::Verbatim(proc_macro2::TokenStream::new());
        }
      }
      _ => (),
    },
    _ => (),
  });
}
