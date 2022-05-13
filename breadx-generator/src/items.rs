// MIT/Apache2 License

use super::always_available;
use heck::{ToSnakeCase, ToUpperCamelCase};
use std::{
    fmt::{self, Write},
    mem,
    ptr::eq,
};
use xcb_parser::{BinOp, Expression, Field, Header, Request, ToplevelItem};

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
            if do_skip(&req.name) {
                return None;
            }

            let struct_name = camel_case_name(&req.name);

            let (mut return_type, mut fncall_to_use) = match req.reply {
                None => (ReturnType::Empty, "send_void_request"),
                Some((ref fields, _)) => {
                    let ret_ty = format!("types::{}::{}Reply", &header.header, &struct_name);
                    let has_fds = fields.iter().any(|field| match field {
                        Field::Fd(_) => true,
                        Field::List { ty, .. } => ty == "fd",
                        _ => false,
                    });

                    (
                        ReturnType::Reply(ret_ty),
                        if has_fds {
                            "send_reply_fd_request"
                        } else {
                            "send_reply_request"
                        },
                    )
                }
            };

            // discover the parameters for the item
            let mut params = req
                .fields
                .iter()
                .filter_map(|field| match field {
                    Field::Field {
                        ty,
                        name,
                        enum_,
                        altenum,
                        mask,
                        altmask,
                    } => {
                        // determine which type we should use
                        let ty = match (enum_, altenum, mask, altmask, ty) {
                            (Some(ty), _, _, _, _) => Ty::Simple(camel_case_name(ty)),
                            (_, Some(ty), _, _, _)
                            | (_, _, Some(ty), _, _)
                            | (_, _, _, Some(ty), _) => Ty::Into(camel_case_name(ty)),
                            (_, _, _, _, ty) => Ty::Simple(camel_case_name(ty)),
                        };

                        Some(Parameter {
                            name: name.to_snake_case(),
                            ty,
                            is_fd: false,
                        })
                    }
                    Field::List {
                        ty,
                        name,
                        list_length,
                    } => Some(Parameter {
                        name: name.to_snake_case(),
                        ty: match list_length {
                            Some(ll) if const_len(ll).is_some() => {
                                Ty::Array(camel_case_name(ty), const_len(ll).unwrap())
                            }
                            _ => {
                                if ty == "fd" {
                                    Ty::Vector("Fd".to_string())
                                } else if ty.to_lowercase() == "void" {
                                    Ty::Void
                                } else {
                                    Ty::Slice(camel_case_name(ty))
                                }
                            }
                        },
                        is_fd: ty == "fd",
                    }),
                    Field::SwitchCase { name, .. } => {
                        // presence of these fields translates to an Aux param on the x11rb side
                        Some(Parameter {
                            ty: Ty::Borrows(format!("{}::{}Aux", header.header, &struct_name)),
                            name: name.to_snake_case(),
                            is_fd: false,
                        })
                    }
                    Field::Fd(name) => Some(Parameter {
                        ty: Ty::Simple("Fd".to_string()),
                        name: name.to_snake_case(),
                        is_fd: true,
                    }),
                    _ => None,
                })
                .collect::<Vec<_>>();

            clean_inlined_parameters(&mut params, req);
            for param in &mut params {
                param.ty.sanitize_colon();
                param.ty.further_qualify(header);
            }

            // sanitize the names
            params.iter_mut().for_each(|param| param.sanitize_name());

            let mut item = write_initial_send(header, req, &params, &return_type, fncall_to_use);
            item.push_str(&immediate_function(header, req, &params, &return_type));

            Some(item)
        }
        _ => None,
    }
}

fn write_initial_send(
    header: &Header,
    req: &Request,
    params: &[Parameter],
    return_type: &ReturnType,
    fncall_to_use: &'static str,
) -> String {
    let mut item = String::new();

    // output cfg flag
    cfg_flag(header, &mut item);

    let struct_name = camel_case_name(&req.name);
    let snake_case_name = req.name.to_snake_case();

    let fnname = fnname(header, req);

    // emit the function header
    write!(item, "fn {}{}(&mut self, ", fnname, ExtraVoidParam(params)).unwrap();
    write_parameters(params, &mut item);

    write!(item, ") -> Result<Cookie<{}>> {{\n", return_type).unwrap();

    // create a span for it
    writeln!(item, "    let span = tracing::info_span!(").unwrap();
    writeln!(item, r#"        "{}","#, fnname).unwrap();
    for param in params {
        if !matches!(param.ty, Ty::Simple(_)) {
            continue;
        }

        writeln!(
            item,
            "        {0} = ?{0},",
            param.name,
        ).unwrap();
    }
    writeln!(item, "    );").unwrap();
    writeln!(item, "    let _enter = span.enter();").unwrap();

    // build the request in the item
    write!(
        item,
        "    let request = types::{}::{}Request {{\n",
        &header.header, struct_name,
    )
    .unwrap();
    for param in params {
        match param.ty {
            Ty::Borrows(_) | Ty::Array(_, _) => {
                write!(
                    item,
                    "        {0}: Cow::Borrowed({0}.borrow()),\n",
                    param.name
                )
                .unwrap();
            }
            Ty::Slice(_) => {
                write!(
                    item,
                    "        {0}: Cow::Borrowed({0}.as_ref()),\n",
                    param.name
                )
                .unwrap();
            }
            Ty::Simple(_) | Ty::Vector(_) => {
                write!(item, "        {0},\n", param.name).unwrap();
            }
            Ty::Into(_) => {
                writeln!(
                    item,
                    "        {0}: Into::<u32>::into({0}.into()) as _,",
                    param.name
                )
                .unwrap();
            }
            Ty::Void => {
                write!(
                    item,
                    "        {0}: Cow::Borrowed({0}.bytes()),\n",
                    param.name
                ).unwrap();
            }
        }
    }
    write!(item, "    }};\n").unwrap();

    // send the request with the cookie
    writeln!(item, "    self.{}(request)", fncall_to_use).unwrap();

    writeln!(item, "}}").unwrap();

    item
}

fn immediate_function(
    header: &Header,
    req: &Request,
    params: &[Parameter],
    return_type: &ReturnType,
) -> String {
    let mut item = String::new();
    cfg_flag(header, &mut item);

    // determine the function name

    let old_fnname = fnname(header, req);
    let suffix = match return_type {
        ReturnType::Empty => "checked",
        ReturnType::Reply(_) => "immediate",
    };
    let fnname = format!("{}_{}", old_fnname, suffix);

    // emit the function header
    write!(item, "fn {}{}(&mut self, ", fnname, ExtraVoidParam(params)).unwrap();
    write_parameters(params, &mut item);
    writeln!(item, ") -> Result<{}> {{", return_type).unwrap();

    // call the previous function
    writeln!(item, "    let cookie = self.{}(", old_fnname).unwrap();
    for param in params {
        writeln!(item, "        {},", param.name).unwrap();
    }
    writeln!(item, "    )?;").unwrap();

    // resolve the cookie
    writeln!(item, "    self.wait_for_reply(cookie)").unwrap();
    writeln!(item, "}}").unwrap();

    item
}

struct ExtraVoidParam<'a>(&'a [Parameter]);

impl<'a> fmt::Display for ExtraVoidParam<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // see if there is a void param in here
        if self.0.iter().any(|p| matches!(p.ty, Ty::Void)) {
            write!(f, "<T: crate::Void + ?Sized>")
        } else {
            Ok(())
        }
    }
}

fn write_parameters<'a>(params: &[Parameter], item: &mut impl Write) {
    for param in params {
        write!(item, "{}: {}", param.name, param.ty).unwrap();
        if !eq(param, params.last().unwrap()) {
            write!(item, ", ").unwrap();
        }
    }
}

fn fnname(header: &Header, req: &Request) -> String {
    let snake_case_name = req.name.to_snake_case();

    if header.header == "xproto" {
        snake_case_name
    } else {
        format!("{}_{}", header.header, snake_case_name)
    }
}

#[derive(Clone)]
enum ReturnType {
    Empty,
    Reply(String),
}

impl fmt::Display for ReturnType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ReturnType::Empty => f.write_str("()"),
            ReturnType::Reply(ty) => f.write_str(ty),
        }
    }
}

/// Items to skip
fn do_skip(name: &str) -> bool {
    matches!(
        name.to_lowercase().as_str(),
        "setclientdisconnectmode"
            | "getclientdisconnectmode"
            | "getbuffers"
            | "getbufferswithformat"
    )
}

struct Parameter {
    name: String,
    ty: Ty,
    is_fd: bool,
}

impl Parameter {
    fn sanitize_name(&mut self) {
        match self.name.as_str() {
            "type" => {
                self.name = "type_".to_string();
            }
            _ => {}
        }
    }
}

enum Ty {
    Simple(String),
    Slice(String),
    Into(String),
    Borrows(String),
    Vector(String),
    Array(String, usize),
    Void,
}

impl Ty {
    fn sanitize_colon(&mut self) {
        match self {
            Self::Simple(s)
            | Self::Slice(s)
            | Self::Into(s)
            | Self::Borrows(s)
            | Self::Vector(s)
            | Self::Array(s, _) => {
                if s.contains(':') {
                    // split into parts
                    let mut iter = s.split(':');
                    let first_part = iter.next().unwrap().to_snake_case();
                    let second_part = camel_case_name(iter.next().unwrap());
                    if !second_part.is_empty() {
                        *s = format!("{}::{}", first_part, second_part);
                    }
                }
            }
            _ => {},
        }
    }

    fn simple_name(&self) -> &str {
        match self {
            Ty::Simple(name) => name,
            _ => unreachable!(),
        }
    }

    fn further_qualify(&mut self, header: &Header) {
        match self {
            Ty::Simple(s) | Ty::Slice(s) | Ty::Into(s) => {
                let prefix = match s.to_lowercase().as_str() {
                    "window" | "drawable" | "pixmap" | "gc" => {
                        if header.header == "glx" {
                            Some("glx")
                        } else {
                            Some("xproto")
                        }
                    }
                    "event" | "notify" | "eventmask" | "modeinfo" | "modeflag" | "eventtype"
                    | "string8" | "context" => Some(&*header.header),
                    "transform" => Some("render"),
                    "cursor" => Some("xproto"),
                    _ => None,
                };

                if let Some(prefix) = prefix {
                    let mys = mem::take(s);
                    *s = format!("{}::{}", prefix, mys);
                }
            }
            _ => {}
        }
    }
}

impl fmt::Display for Ty {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Simple(s) => write!(f, "types::{}", s),
            Self::Slice(s) => write!(f, "impl AsRef<[types::{}]>", s),
            Self::Borrows(t) => write!(f, "impl Borrow<types::{}>", t),
            Self::Into(t) => write!(f, "impl Into<types::{}>", t),
            Self::Vector(t) => write!(f, "Vec<types::{}>", t),
            Self::Array(t, len) => write!(f, "impl Borrow<[types::{}; {}]>", t, len),
            Self::Void => f.write_str("&T")
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
        // some fields that seem to defy the above rules
        if param.name == "affect_which" {
            return false;
        }

        // determine if the param is inlined
        !req.fields
            .iter()
            .filter_map(|field| match field {
                Field::List { list_length, .. } => list_length.as_ref().and_then(single_field),
                Field::SwitchCase { switch_expr, .. } => single_field(switch_expr),
                _ => None,
            })
            .any(|name| param.name.to_snake_case() == name.to_snake_case())
    });
}

fn single_field(expr: &Expression) -> Option<&str> {
    match expr {
        Expression::Fieldref(name) => Some(name),
        Expression::BinOp { op, left, right } => match (op, &**left, &**right) {
            (BinOp::Multiply, Expression::Fieldref(name), Expression::Value(_))
            | (BinOp::Multiply, Expression::Value(_), Expression::Fieldref(name))
            | (BinOp::Divide, Expression::Fieldref(name), Expression::Value(_)) => Some(name),
            _ => None,
        },
        _ => None,
    }
}

fn const_len(expr: &Expression) -> Option<usize> {
    match expr {
        Expression::Value(n) => Some(*n as usize),
        _ => None,
    }
}

/// Copied from the x11rb generator
fn camel_case_name(name: &str) -> String {
    let mut name = String::from(name);
    if name.bytes().all(|c| !c.is_ascii_lowercase()) {
        name.make_ascii_lowercase();
    }

    // Convert to camel case
    if name.is_empty() {
        return "".to_string();
    }

    if name.len() == 2 {
        return name.to_uppercase();
    }

    let mut r = String::new();
    for chunk in name.split('_') {
        r.push_str(&chunk[..1]);
        let r_len = r.len();
        r[(r_len - 1)..].make_ascii_uppercase();
        r.push_str(&chunk[1..]);
    }
    r
}
