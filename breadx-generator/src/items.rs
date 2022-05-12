// MIT/Apache2 License

use super::always_available;
use std::{fmt::{self, Write}, ptr::eq};
use heck::{ToUpperCamelCase, ToSnakeCase};
use xcb_parser::{Header, Field, ToplevelItem, Request};

#[derive(Copy, Clone)]
pub enum Mode {
    Sync,
    Async,
}

pub fn generate_item(header: &Header, item: &ToplevelItem, mode: Mode) -> Option<String> {
    if matches!(mode, Mode::Async) {
        // TODO
        return None;
    }

    match item {
        ToplevelItem::Request(req) => {
            let mut item = String::new();

            // add cfg flag to this item
            cfg_flag(header, &mut item);

            let camel_case_name = req.name.to_upper_camel_case();
            let snake_case_name = req.name.to_snake_case();

            let fnname = if header.header == "xproto" { 
                snake_case_name
            } else {
                format!("{}_{}", header.header, snake_case_name)
            };

            let (return_type, fncall_to_use) = match req.reply {
                None => ("()".into(), "send_void_request"),
                Some((ref fields, _)) => {
                    let mut ret_ty = format!("types::{}Reply", &camel_case_name);
                    let has_fds = fields.iter().any(|field| match field {
                        Field::Fd(_) => true,
                        Field::List { ty, .. } => ty == "fd",
                        _ => false,
                    });

                    (ret_ty, if has_fds {
                        "send_reply_fd_request"
                    } else {
                        "send_reply_request"
                    })
                }
            };

            // discover the parameters for the item
            let mut params = req.fields.iter().filter_map(|field| match field {
                Field::Field { ty, name, enum_, altenum,  mask, altmask } => {
                    // determine which type we should use
                    let ty = match (enum_, altenum, mask, altmask, ty) {
                        (Some(ty), _, _, _, _) => Ty::Simple(ty.to_upper_camel_case()),
                        (_, Some(ty), _, _, _) | (_, _, Some(ty), _, _) | (_, _, _, Some(ty), _) => Ty::Into(ty.to_upper_camel_case()),
                        (_, _, _, _, ty) => Ty::Simple(ty.to_upper_camel_case()),
                    };

                    Some(Parameter {
                        name: name.to_snake_case(),
                        ty,
                        is_fd: false,
                    })
                },
                Field::List { ty, name, .. } => {
                    Some(Parameter {
                        name: name.to_snake_case(),
                        ty: Ty::Slice(ty.to_upper_camel_case()),
                        is_fd: ty == "fd",
                    })
                },
                Field::SwitchCase { .. } | Field::ValueParam { .. } => {
                    // presence of these fields translates to an Aux param on the x11rb side
                    Some(Parameter {
                        ty: Ty::Borrows(format!("{}Aux", &camel_case_name)),
                        name: "value_list".into(),
                        is_fd: false,
                    })
                },
                Field::Fd(name) => {
                    Some(Parameter {
                        ty: Ty::Simple("Fd".to_string()),
                        name: name.to_snake_case(),
                        is_fd: true,
                    })
                }
                _ => None,
            }).collect::<Vec<_>>();

            clean_inlined_parameters(&mut params, req);

            // sanitize the names
            params.iter_mut().for_each(|param| param.sanitize_name());

            // emit the function header
            write!(item, "fn {}(display: &mut impl Display, ", fnname).unwrap();
            for param in &params {
                write!(item, "{}: {}", param.name, param.ty).unwrap();
                if !eq(param, params.last().unwrap()) {
                    write!(item, ", ").unwrap();
                }
            }
            write!(item, ") -> Result<Cookie<{}>> {{\n", return_type).unwrap();

            // build the request in the item
            write!(item, "    let mut request = types::{}Request {{\n", camel_case_name).unwrap();
            for param in &params {
                match param.ty {
                    Ty::Borrows(_) => {
                        write!(item, "        {0}: Cow::Borrowed({0}.borrow()),\n", param.name).unwrap();
                    }
                    Ty::Slice(_) => {
                        write!(item, "        {0}: Cow::Borrowed({0}.as_ref()),\n", param.name).unwrap();                       
                    }
                    Ty::Simple(_) => {
                        write!(item, "        {0},\n", param.name).unwrap();
                    }
                    Ty::Into(_) => {
                        writeln!(item, "        {0}: Into::<u32>::into({0}.into()) as _,", param.name).unwrap();
                    }
                }
            }
            write!(item, "    }};\n").unwrap();

            // send the request along the display
            writeln!(item, "    display.{}(request)", fncall_to_use).unwrap();

            writeln!(item, "}}").unwrap();

            Some(item)
        }
        _ => None,
    }
}

struct Parameter {
    name: String,
    ty: Ty,
    is_fd: bool,
}

impl Parameter {
    fn sanitize_name(&mut self) {
        match self.name.as_str() {
            "type" => { self.name = "type_".to_string(); },
            _ => {},
        }
    }
}

enum Ty {
    Simple(String),
    Slice(String),
    Into(String),
    Borrows(String),
}

impl fmt::Display for Ty {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Simple(s) => write!(f, "types::{}", s),
            Self::Slice(s) => write!(f, "impl AsRef<[types::{}]>", s),
            Self::Borrows(t) => write!(f, "impl Borrow<types::{}>", t),
            Self::Into(t) => write!(f, "impl Into<types::{}>", t),
        }
    }
}

fn cfg_flag(header: &Header, target: &mut impl Write) {
    if !always_available(&header.header) {
        writeln!(target, r#"#[cfg(feature = "{}")]"#, &header.header).ok();
    }
}

fn clean_inlined_parameters(params: &mut Vec<Parameter>, req: &Request) {
    params.retain(|param| {
        // determine if the param is inlined
        true
    });
}