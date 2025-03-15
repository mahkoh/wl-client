use {
    crate::ast::{Arg, ArgType, Description, Interface, Message, MessageType, Protocol},
    debug_fn::debug_fn,
    phf::phf_set,
    std::{
        fmt::{Display, Write as FmtWrite},
        io::{self, Write},
    },
};

fn format_interface_header(
    w: &mut impl Write,
    prefix: &str,
    interface: &Interface,
) -> io::Result<()> {
    if let Some(desc) = &interface.description {
        format_description(w, "//!", desc)?;
        writeln!(w)?;
    }
    writeln!(w, "use {prefix}::builder::prelude::*;")?;
    writeln!(w, "use super::super::all_types::*;")?;
    Ok(())
}

pub fn format_interface_file(
    w: &mut impl Write,
    root: &str,
    interface: &Interface,
) -> io::Result<()> {
    format_interface_header(w, root, interface)?;
    writeln!(w)?;
    format_wl_interface(w, interface)?;
    writeln!(w)?;
    format_interface_types(w, interface)?;
    writeln!(w)?;
    format_interface_trait_impls(w, interface)?;
    writeln!(w)?;
    format_interface_requests(w, interface)?;
    writeln!(w)?;
    format_interface_event_handler(w, interface)?;
    writeln!(w)?;
    format_event_handler(w, interface)?;
    writeln!(w)?;
    format_interface_enums(w, interface)?;
    writeln!(w)?;
    format_event_handlers(w, interface)?;
    Ok(())
}

fn format_wl_interface(w: &mut impl Write, interface: &Interface) -> io::Result<()> {
    writeln!(w, r#"static INTERFACE: wl_interface = wl_interface {{"#)?;
    writeln!(w, r#"    name: c"{}".as_ptr(),"#, interface.name)?;
    writeln!(w, r#"    version: {},"#, interface.version)?;
    writeln!(w, r#"    method_count: {},"#, interface.requests.len())?;
    writeln!(
        w,
        r#"    methods: {},"#,
        format_wl_interface_messages(&interface.requests)
    )?;
    writeln!(w, r#"    event_count: {},"#, interface.events.len())?;
    writeln!(
        w,
        r#"    events: {},"#,
        format_wl_interface_messages(&interface.events)
    )?;
    writeln!(w, r#"}};"#)?;
    Ok(())
}

fn format_wl_interface_messages(messages: &[Message]) -> impl Display + use<'_> {
    debug_fn(move |f| {
        if messages.is_empty() {
            return f.write_str("ptr::null()");
        }
        writeln!(f, r#"{{"#)?;
        writeln!(
            f,
            r#"        static MESSAGES: [wl_message; {}] = ["#,
            messages.len()
        )?;
        for message in messages {
            let mut num_types = message.args.len();
            for arg in &message.args {
                if arg.ty == ArgType::NewId && arg.interface.is_none() {
                    num_types += 2;
                }
            }

            writeln!(f, r#"            wl_message {{"#)?;
            writeln!(f, r#"                name: c"{}".as_ptr(),"#, message.name)?;
            writeln!(
                f,
                r#"                signature: c"{}".as_ptr(),"#,
                format_signature(message)
            )?;
            writeln!(f, r#"                types: {{"#)?;
            writeln!(
                f,
                r#"                    static TYPES: [Option<&'static wl_interface>; {}] = ["#,
                num_types,
            )?;
            for arg in &message.args {
                match &arg.interface {
                    None => {
                        if arg.ty == ArgType::NewId {
                            writeln!(f, r#"                        None,"#)?;
                            writeln!(f, r#"                        None,"#)?;
                        }
                        writeln!(f, r#"                        None,"#)?;
                    }
                    Some(name) => {
                        writeln!(
                            f,
                            r#"                        Some({}::WL_INTERFACE),"#,
                            format_camel(name),
                        )?;
                    }
                }
            }
            writeln!(f, r#"                    ];"#)?;
            writeln!(f, r#"                    TYPES.as_ptr().cast()"#)?;
            writeln!(f, r#"                }},"#)?;
            writeln!(f, r#"            }},"#)?;
        }
        writeln!(f, r#"        ];"#)?;
        writeln!(f, r#"        MESSAGES.as_ptr()"#)?;
        write!(f, r#"    }}"#)?;
        Ok(())
    })
}

fn format_signature(message: &Message) -> impl Display + use<'_> {
    debug_fn(move |f| {
        for arg in &message.args {
            if arg.allow_null {
                f.write_str("?")?;
            }
            let s = match arg.ty {
                ArgType::NewId if arg.interface.is_none() => "sun",
                ArgType::NewId => "n",
                ArgType::Int => "i",
                ArgType::Uint => "u",
                ArgType::Fixed => "f",
                ArgType::String => "s",
                ArgType::Object => "o",
                ArgType::Array => "a",
                ArgType::Fd => "h",
            };
            f.write_str(s)?;
        }
        Ok(())
    })
}

fn format_camel(s: &str) -> impl Display + use<'_> {
    debug_fn(move |f| {
        let mut need_uppercase = true;
        for &c in s.as_bytes() {
            if c == b'_' || c == b'.' {
                need_uppercase = true;
            } else if need_uppercase {
                need_uppercase = false;
                f.write_char(c.to_ascii_uppercase() as _)?;
            } else {
                f.write_char(c as _)?;
            }
        }
        Ok(())
    })
}

fn format_uppercase(s: &str) -> impl Display + use<'_> {
    debug_fn(move |f| {
        for &c in s.as_bytes() {
            f.write_char(c.to_ascii_uppercase() as _)?;
        }
        Ok(())
    })
}

fn format_enum_variant(s: &str) -> impl Display + use<'_> {
    let need_underscore = s.chars().next().unwrap_or_default().is_ascii_digit();
    debug_fn(move |f| {
        if need_underscore {
            f.write_str("_")?;
        }
        format_uppercase(s).fmt(f)
    })
}

fn format_interface_types(w: &mut impl Write, interface: &Interface) -> io::Result<()> {
    let snake = &interface.name;
    let camel = format_camel(snake).to_string();
    writeln!(w, r#"/// An owned {snake} proxy."#)?;
    writeln!(w, r#"///"#)?;
    writeln!(
        w,
        r#"/// See the documentation of [the module][self] for the interface description."#
    )?;
    writeln!(w, r#"#[derive(Clone, Eq, PartialEq)]"#)?;
    writeln!(w, r#"#[repr(transparent)]"#)?;
    writeln!(w, r#"pub struct {camel} {{"#)?;
    writeln!(w, r#"    /// This proxy has the interface INTERFACE."#)?;
    writeln!(w, r#"    proxy: UntypedOwnedProxy,"#)?;
    writeln!(w, r#"}}"#)?;
    writeln!(w)?;
    writeln!(w, r#"/// A borrowed {snake} proxy."#)?;
    writeln!(w, r#"///"#)?;
    writeln!(
        w,
        r#"/// See the documentation of [the module][self] for the interface description."#
    )?;
    writeln!(w, r#"#[derive(Eq, PartialEq)]"#)?;
    writeln!(w, r#"#[repr(transparent)]"#)?;
    writeln!(w, r#"pub struct {camel}Ref {{"#)?;
    writeln!(w, r#"    /// This proxy has the interface INTERFACE."#)?;
    writeln!(w, r#"    proxy: UntypedBorrowedProxy,"#)?;
    writeln!(w, r#"}}"#)?;
    Ok(())
}

fn format_interface_requests(w: &mut impl Write, interface: &Interface) -> io::Result<()> {
    if interface.requests.is_empty() {
        return Ok(());
    }
    for owned in [true, false] {
        let skip_request = |r: &Message| match owned {
            true => {
                let is_constructor = r.args.iter().any(|a| a.ty == ArgType::NewId);
                let is_destructor = r.ty == Some(MessageType::Destructor);
                !is_constructor && !is_destructor
            }
            false => r.ty == Some(MessageType::Destructor),
        };
        if interface.requests.iter().all(skip_request) {
            continue;
        }
        let snake = &interface.name;
        let mut name = format_camel(snake).to_string();
        if !owned {
            name.push_str("Ref");
        }
        if !owned {
            writeln!(w)?;
        }
        writeln!(w, r#"#[allow(dead_code)]"#)?;
        writeln!(w, r#"impl {name} {{"#)?;
        let mut first = true;
        for (idx, request) in interface.requests.iter().enumerate() {
            if skip_request(request) {
                continue;
            }
            if first {
                first = false;
            } else {
                writeln!(w)?;
            }
            let new_id = request.args.iter().find(|a| a.ty == ArgType::NewId);
            if owned {
                format_message_since(w, false, request)?;
                writeln!(w)?;
            }
            format_message_doc(w, true, !owned, request)?;
            writeln!(w, r#"    #[inline]"#)?;
            write!(w, r#"    pub fn {}"#, escape_name(&request.name))?;
            if let Some(arg) = new_id {
                if arg.interface.is_none() {
                    write!(w, r#"<P: OwnedProxy>"#)?;
                }
            }
            writeln!(w, r#"("#)?;
            writeln!(w, r#"        &self,"#)?;
            if new_id.is_some() && !owned {
                writeln!(w, r#"        _queue: &Queue,"#)?;
            }
            let mut num_args = request.args.len();
            for arg in &request.args {
                if arg.ty == ArgType::NewId {
                    if arg.interface.is_none() {
                        writeln!(w, r#"        version: u32,"#)?;
                    } else {
                        num_args -= 1;
                    }
                } else {
                    writeln!(
                        w,
                        r#"        {}: {},"#,
                        escape_name(&arg.name),
                        arg_type(interface, arg, true),
                    )?;
                }
            }
            write!(w, r#"    )"#)?;
            if let Some(arg) = new_id {
                match &arg.interface {
                    None => write!(w, " -> P")?,
                    Some(i) => write!(w, " -> {}", format_camel(i))?,
                }
            }
            writeln!(w, r#" {{"#)?;
            if num_args > 0 {
                writeln!(w, r#"        let ("#)?;
                for (idx, arg) in request.args.iter().enumerate() {
                    if arg.ty != ArgType::NewId || arg.interface.is_none() {
                        writeln!(w, r#"            arg{idx},"#)?;
                    }
                }
                writeln!(w, r#"        ) = ("#)?;
                for arg in &request.args {
                    if arg.ty == ArgType::NewId {
                        if arg.interface.is_none() {
                            writeln!(w, r#"            version,"#)?;
                        }
                    } else {
                        writeln!(w, r#"            {},"#, escape_name(&arg.name))?;
                    }
                }
                writeln!(w, r#"        );"#)?;
            }
            let mut prefix = "        ";
            let mut string_args = 0;
            let mut object_args = 0;
            let mut array_args = 0;
            for arg in &request.args {
                match arg.ty {
                    ArgType::String => string_args += 1,
                    ArgType::Object => object_args += 1,
                    ArgType::Array => array_args += 1,
                    _ => {}
                }
            }
            if string_args > 0 {
                prefix = "            ";
                writeln!(w, r#"        with_cstr_cache(|cache| {{"#)?;
                for (idx, arg) in request.args.iter().enumerate() {
                    if arg.ty != ArgType::String {
                        continue;
                    }
                    writeln!(w, r#"{prefix}let str{idx}_offset = cache.len();"#)?;
                    if arg.allow_null {
                        writeln!(w, r#"{prefix}if let Some(arg{idx}) = arg{idx} {{"#)?;
                        writeln!(
                            w,
                            r#"{prefix}    cache.extend_from_slice(arg{idx}.as_bytes());"#
                        )?;
                        writeln!(w, r#"{prefix}    cache.push(0);"#)?;
                        writeln!(w, r#"{prefix}}}"#)?;
                    } else {
                        writeln!(
                            w,
                            r#"{prefix}cache.extend_from_slice(arg{idx}.as_bytes());"#
                        )?;
                        writeln!(w, r#"{prefix}cache.push(0);"#)?;
                    }
                }
                for (idx, arg) in request.args.iter().enumerate() {
                    if arg.ty != ArgType::String {
                        continue;
                    }
                    if arg.allow_null {
                        writeln!(w, r#"{prefix}let mut str{idx} = ptr::null();"#)?;
                        writeln!(w, r#"{prefix}if arg{idx}.is_some() {{"#)?;
                        writeln!(
                            w,
                            r#"{prefix}    str{idx} = cache[str{idx}_offset..].as_ptr().cast();"#
                        )?;
                        writeln!(w, r#"{prefix}}}"#)?;
                    } else {
                        writeln!(
                            w,
                            r#"{prefix}let str{idx} = cache[str{idx}_offset..].as_ptr().cast();"#
                        )?;
                    }
                }
            }
            if object_args > 0 {
                for (idx, arg) in request.args.iter().enumerate() {
                    if arg.ty != ArgType::Object {
                        continue;
                    }
                    if arg.allow_null {
                        writeln!(
                            w,
                            r#"{prefix}let obj{idx}_lock = arg{idx}.map(|arg{idx}| proxy::lock(arg{idx}));"#
                        )?;
                        writeln!(
                            w,
                            r#"{prefix}let obj{idx} = obj{idx}_lock.map(|obj{idx}_lock| check_argument_proxy("{}", obj{idx}_lock.wl_proxy())).unwrap_or(ptr::null_mut());"#,
                            arg.name,
                        )?;
                    } else {
                        writeln!(w, r#"{prefix}let obj{idx}_lock = proxy::lock(arg{idx});"#)?;
                        writeln!(
                            w,
                            r#"{prefix}let obj{idx} = check_argument_proxy("{}", obj{idx}_lock.wl_proxy());"#,
                            arg.name
                        )?;
                    }
                }
            }
            if array_args > 0 {
                for (idx, arg) in request.args.iter().enumerate() {
                    if arg.ty != ArgType::Array {
                        continue;
                    }
                    writeln!(w, r#"{prefix}let mut arr{idx} = wl_array {{"#)?;
                    writeln!(w, r#"{prefix}    size: arg{idx}.len(),"#)?;
                    writeln!(w, r#"{prefix}    alloc: arg{idx}.len(),"#)?;
                    writeln!(
                        w,
                        r#"{prefix}    data: arg{idx}.as_ptr().cast_mut().cast(),"#
                    )?;
                    writeln!(w, r#"{prefix}}};"#)?;
                }
            }
            if request.args.len() > 0 {
                writeln!(w, r#"{prefix}let mut args = ["#)?;
                for (idx, arg) in request.args.iter().enumerate() {
                    match arg.ty {
                        ArgType::NewId if arg.interface.is_none() => {
                            writeln!(
                                w,
                                r#"{prefix}    wl_argument {{ s: P::WL_INTERFACE.name }},"#
                            )?;
                            writeln!(w, r#"{prefix}    wl_argument {{ u: arg{idx} }},"#)?;
                            writeln!(w, r#"{prefix}    wl_argument {{ n: 0 }},"#)?;
                        }
                        ArgType::NewId => {
                            writeln!(w, r#"{prefix}    wl_argument {{ n: 0 }},"#)?;
                        }
                        ArgType::Int | ArgType::Uint => {
                            if arg.enum_.is_some() {
                                writeln!(w, r#"{prefix}    wl_argument {{ u: arg{idx}.0 }},"#)?;
                            } else if arg.ty == ArgType::Int {
                                writeln!(w, r#"{prefix}    wl_argument {{ i: arg{idx} }},"#)?;
                            } else {
                                writeln!(w, r#"{prefix}    wl_argument {{ u: arg{idx} }},"#)?;
                            }
                        }
                        ArgType::Fixed => {
                            writeln!(w, r#"{prefix}    wl_argument {{ f: arg{idx}.to_wire() }},"#)?;
                        }
                        ArgType::String => {
                            writeln!(w, r#"{prefix}    wl_argument {{ s: str{idx} }},"#)?;
                        }
                        ArgType::Object => {
                            writeln!(w, r#"{prefix}    wl_argument {{ o: obj{idx} }},"#)?;
                        }
                        ArgType::Array => {
                            writeln!(w, r#"{prefix}    wl_argument {{ a: &mut arr{idx} }},"#)?;
                        }
                        ArgType::Fd => {
                            writeln!(
                                w,
                                r#"{prefix}    wl_argument {{ h: arg{idx}.as_raw_fd() }},"#
                            )?;
                        }
                    }
                }
                writeln!(w, r#"{prefix}];"#)?;
            } else {
                writeln!(w, r#"{prefix}let mut args = [];"#)?;
            };
            let is_destructor = request.ty == Some(MessageType::Destructor);
            writeln!(
                w,
                r#"{prefix}// SAFETY: - self.proxy has the interface INTERFACE"#,
            )?;
            writeln!(
                w,
                r#"{prefix}//         - {idx} < INTERFACE.method_count = {}"#,
                interface.requests.len(),
            )?;
            writeln!(
                w,
                r#"{prefix}//         - the request signature is `{}`"#,
                format_signature(request),
            )?;
            if let Some(arg) = new_id {
                writeln!(
                    w,
                    r#"{prefix}//         - OwnedProxy::WL_INTERFACE is always a valid interface"#,
                )?;
                writeln!(w, r#"{prefix}let data = unsafe {{"#)?;
                let interface = debug_fn(|f| {
                    if let Some(i) = &arg.interface {
                        write!(f, "{}::WL_INTERFACE", format_camel(i))
                    } else {
                        f.write_str("P::WL_INTERFACE")
                    }
                });
                let version = debug_fn(|f| {
                    if arg.interface.is_some() {
                        f.write_str("None")
                    } else {
                        f.write_str("Some(version)")
                    }
                });
                if owned {
                    writeln!(
                        w,
                        r#"{prefix}    self.proxy.send_constructor::<{is_destructor}>({idx}, &mut args, {interface}, {version})"#,
                    )?;
                } else {
                    writeln!(
                        w,
                        r#"{prefix}    self.proxy.send_constructor(_queue, {idx}, &mut args, {interface}, {version})"#,
                    )?;
                }
                writeln!(w, r#"{prefix}}};"#)?;
                writeln!(
                    w,
                    r#"{prefix}// SAFETY: data has the interface {interface}"#
                )?;
                writeln!(w, r#"{prefix}unsafe {{"#)?;
                writeln!(
                    w,
                    r#"{prefix}    proxy::low_level::from_untyped_owned(data)"#
                )?;
                writeln!(w, r#"{prefix}}}"#)?;
            } else {
                writeln!(w, r#"{prefix}unsafe {{"#)?;
                if owned {
                    assert!(is_destructor);
                    writeln!(
                        w,
                        r#"{prefix}    self.proxy.send_destructor({idx}, &mut args);"#,
                    )?;
                } else {
                    writeln!(
                        w,
                        r#"{prefix}    self.proxy.send_request({idx}, &mut args);"#,
                    )?;
                }
                writeln!(w, r#"{prefix}}}"#)?;
            }
            if string_args > 0 {
                writeln!(w, r#"        }})"#)?;
            }
            writeln!(w, r#"    }}"#)?;
        }
        writeln!(w, r#"}}"#)?;
    }
    Ok(())
}

fn format_message_since(w: &mut impl Write, event: bool, message: &Message) -> io::Result<()> {
    let prefix = match event {
        true => "EVT",
        false => "REQ",
    };
    let ty = match event {
        true => "event",
        false => "request",
    };
    format_since(
        w,
        prefix,
        ty,
        &message.name,
        format_uppercase(&message.name),
        message.since,
        message.deprecated_since,
    )
}

fn format_since(
    w: &mut impl Write,
    prefix: &str,
    ty: &str,
    name: impl Display,
    uppercase: impl Display,
    since: Option<u32>,
    deprecated_since: Option<u32>,
) -> io::Result<()> {
    writeln!(w, r#"    /// Since when the {name} {ty} is available."#,)?;
    writeln!(w, r#"    #[allow(dead_code)]"#)?;
    writeln!(
        w,
        r#"    pub const {prefix}__{uppercase}__SINCE: u32 = {};"#,
        since.unwrap_or(1),
    )?;
    if let Some(n) = deprecated_since {
        writeln!(w)?;
        writeln!(w, r#"    /// Since when the {name} {ty} is deprecated."#,)?;
        writeln!(w, r#"    #[allow(dead_code)]"#)?;
        writeln!(
            w,
            r#"    pub const {prefix}__{uppercase}__DEPRECATED_SINCE: u32 = {n};"#,
        )?;
    }
    Ok(())
}

fn format_interface_event_handler(w: &mut impl Write, interface: &Interface) -> io::Result<()> {
    let snake = &interface.name;
    let camel = format_camel(snake).to_string();
    if interface.events.len() > 0 {
        writeln!(w, r#"impl {} {{"#, camel)?;
        for (idx, event) in interface.events.iter().enumerate() {
            if idx > 0 {
                writeln!(w)?;
            }
            format_message_since(w, true, event)?;
        }
        writeln!(w, r#"}}"#)?;
        writeln!(w)?;
    }
    writeln!(w, r#"/// An event handler for [{camel}] proxies."#)?;
    writeln!(w, r#"#[allow(dead_code)]"#)?;
    writeln!(w, r#"pub trait {camel}EventHandler {{"#)?;
    for (idx, event) in interface.events.iter().enumerate() {
        if idx > 0 {
            writeln!(w)?;
        }
        format_message_doc(w, false, false, event)?;
        writeln!(w, r#"    #[inline]"#)?;
        writeln!(w, r#"    fn {}("#, escape_name(&event.name))?;
        writeln!(w, r#"        &self,"#)?;
        writeln!(w, r#"        _slf: &{camel}Ref,"#)?;
        for arg in &event.args {
            writeln!(
                w,
                r#"        {}: {},"#,
                escape_name(&arg.name),
                arg_type(interface, arg, false)
            )?;
        }
        writeln!(w, r#"    ) {{"#)?;
        for arg in &event.args {
            writeln!(w, r#"        let _ = {};"#, escape_name(&arg.name))?;
        }
        // writeln!(
        //     w,
        //     r#"        unimplemented_event_handler("{}", "{}");"#,
        //     interface.name, event.name
        // )?;
        writeln!(w, r#"    }}"#)?;
    }
    writeln!(w, r#"}}"#)?;
    writeln!(w)?;
    writeln!(
        w,
        r#"impl {camel}EventHandler for private::NoOpEventHandler {{ }}"#
    )?;
    Ok(())
}

fn arg_type<'a>(interface: &'a Interface, arg: &'a Arg, request: bool) -> impl Display + use<'a> {
    debug_fn(move |f| {
        if let Some(enum_) = &arg.enum_ {
            if enum_.contains('.') {
                return write!(f, "{}", format_camel(enum_));
            }
            return write!(
                f,
                "{}{}",
                format_camel(&interface.name),
                format_camel(enum_)
            );
        }
        let s = match &arg.ty {
            ArgType::NewId => match &arg.interface {
                None => "*mut wl_proxy",
                Some(s) => {
                    return write!(f, "{}", format_camel(s));
                }
            },
            ArgType::Int => "i32",
            ArgType::Uint => "u32",
            ArgType::Fixed => "Fixed",
            ArgType::String if arg.allow_null => "Option<&str>",
            ArgType::String => "&str",
            ArgType::Object if arg.allow_null || !request => match &arg.interface {
                None => "Option<&UntypedBorrowedProxy>",
                Some(s) => {
                    return write!(f, "Option<&{}Ref>", format_camel(s));
                }
            },
            ArgType::Object => match &arg.interface {
                None => "&UntypedBorrowedProxy",
                Some(s) => {
                    return write!(f, "&{}Ref", format_camel(s));
                }
            },
            ArgType::Array => "&[u8]",
            ArgType::Fd => match request {
                true => "BorrowedFd<'_>",
                false => "OwnedFd",
            },
        };
        f.write_str(s)
    })
}

#[allow(clippy::type_complexity)]
pub fn format_mod_file(
    w: &mut impl Write,
    protocols: &[(String, Vec<(String, Vec<String>)>)],
) -> io::Result<()> {
    for (protocol, _) in protocols {
        writeln!(w, r#"pub mod {};"#, protocol)?;
    }
    writeln!(w)?;
    writeln!(w, "#[allow(unused_imports)]")?;
    writeln!(w, "mod all_types {{")?;
    for (proto, interfaces) in protocols {
        for (snake, enums) in interfaces {
            let camel = format_camel(snake).to_string();
            let prefix =
                debug_fn(|f| write!(f, r#"    pub(super) use super::{proto}::{snake}::{camel}"#));
            writeln!(w, r#"{prefix};"#)?;
            writeln!(w, r#"{prefix}Ref;"#)?;
            for enum_ in enums {
                writeln!(w, r#"{prefix}{};"#, format_camel(enum_))?;
            }
        }
    }
    writeln!(w, "}}")?;
    Ok(())
}

pub fn format_protocol_file(w: &mut impl Write, protocol: &Protocol) -> io::Result<()> {
    if let Some(description) = &protocol.description {
        format_description(w, "//!", description)?;
        writeln!(w)?;
    }
    writeln!(w, "#![allow(clippy::tabs_in_doc_comments)]")?;
    writeln!(w, "#![allow(clippy::doc_lazy_continuation)]")?;
    writeln!(w, "#![allow(clippy::too_many_arguments)]")?;
    writeln!(w, "#![allow(clippy::manual_map)]")?;
    writeln!(w, "#![allow(clippy::module_inception)]")?;
    writeln!(w, "#![allow(unused_imports)]")?;
    writeln!(w, "#![allow(rustdoc::broken_intra_doc_links)]")?;
    writeln!(w, "#![allow(rustdoc::bare_urls)]")?;
    writeln!(w, "#![allow(rustdoc::invalid_rust_codeblocks)]")?;
    writeln!(w)?;
    for interface in &protocol.interfaces {
        let snake = &interface.name;
        writeln!(w, r#"pub mod {snake};"#)?;
    }
    Ok(())
}

fn format_message_doc<W>(
    w: &mut W,
    request: bool,
    on_ref: bool,
    message: &Message,
) -> io::Result<()>
where
    W: Write,
{
    let mut need_newline = false;
    if let Some(desc) = &message.description {
        format_description(w, "    ///", desc)?;
        need_newline = true;
    }
    let mut num_args = message.args.len();
    let mut is_constructor = false;
    let mut has_object = false;
    if request {
        for arg in &message.args {
            if arg.ty == ArgType::NewId {
                num_args -= 1;
                is_constructor = true;
                break;
            }
        }
    } else {
        for arg in &message.args {
            if arg.ty == ArgType::Object {
                has_object = true;
                break;
            }
        }
    }
    if num_args > 0 || (is_constructor && on_ref) {
        if need_newline {
            writeln!(w, "    ///")?;
        }
        writeln!(w, "    /// # Arguments")?;
        writeln!(w, "    ///")?;
        if is_constructor && on_ref {
            writeln!(
                w,
                "    /// - `_queue`: The queue that the returned proxy is assigned to."
            )?;
        }
        for arg in &message.args {
            if request && arg.ty == ArgType::NewId {
                continue;
            }
            let name = escape_name(&arg.name).to_string();
            write!(w, "    /// - `{name}`:")?;
            let prefix = format!("    ///    {:width$}  ", " ", width = name.len());
            let mut first = true;
            let mut needs_newline = false;
            if let Some(summary) = &arg.summary {
                for line in summary.lines() {
                    if first {
                        first = false;
                    } else {
                        write!(w, "{}", prefix)?;
                    }
                    writeln!(w, " {}", line)?;
                }
                needs_newline = true;
            }
            if let Some(desc) = &arg.description {
                if needs_newline {
                    writeln!(w, "    ///")?;
                }
                format_description(w, &prefix, desc)?;
            }
            if arg.summary.is_none() && arg.description.is_none() {
                writeln!(w)?;
            }
        }
        if has_object {
            writeln!(w, "    ///")?;
            writeln!(
                w,
                "    /// All borrowed proxies passed to this function are guaranteed to be"
            )?;
            writeln!(w, "    /// immutable and non-null.")?;
        }
    }
    Ok(())
}

fn format_description(
    w: &mut impl Write,
    prefix: &str,
    description: &Description,
) -> io::Result<()> {
    let mut needs_newline = false;
    if let Some(summary) = &description.summary {
        for line in summary.lines() {
            writeln!(w, "{prefix} {line}")?;
        }
        needs_newline = true;
    }
    let mut trim = None;
    let mut empty_lines = 0;
    'outer: for mut line in description.body.lines() {
        if trim.is_none() {
            let idx = 'idx: {
                for (idx, c) in line.char_indices() {
                    if c != ' ' && c != '\t' {
                        break 'idx idx;
                    }
                }
                continue 'outer;
            };
            trim = Some(&line[..idx]);
        }
        if let Some(stripped) = line.strip_prefix(trim.unwrap()) {
            line = stripped;
        }
        if line.trim_ascii().is_empty() {
            empty_lines += 1;
            continue;
        }
        if empty_lines > 0 {
            for _ in 0..empty_lines {
                writeln!(w, "{prefix}")?;
            }
            empty_lines = 0;
        }
        if needs_newline {
            needs_newline = false;
            writeln!(w, "{prefix}")?;
        }
        writeln!(w, "{prefix} {}", line)?;
    }
    Ok(())
}

fn format_event_handler(w: &mut impl Write, interface: &Interface) -> io::Result<()> {
    let snake = &interface.name;
    let camel = format_camel(snake).to_string();
    writeln!(w, r#"// SAFETY: INTERFACE is a valid wl_interface"#)?;
    writeln!(
        w,
        r#"unsafe impl<H> EventHandler for private::EventHandler<H>"#
    )?;
    writeln!(w, r#"where"#)?;
    writeln!(w, r#"    H: {camel}EventHandler,"#)?;
    writeln!(w, r#"{{"#)?;
    writeln!(
        w,
        r#"    const WL_INTERFACE: &'static wl_interface = &INTERFACE;"#
    )?;
    writeln!(w)?;
    writeln!(w, r#"    #[allow(unused_variables)]"#)?;
    writeln!(w, r#"    unsafe fn handle_event("#)?;
    writeln!(w, r#"        &self,"#)?;
    writeln!(w, r#"        queue: &Queue,"#)?;
    writeln!(w, r#"        slf: &UntypedBorrowedProxy,"#)?;
    writeln!(w, r#"        opcode: u32,"#)?;
    writeln!(w, r#"        args: *mut wl_argument,"#)?;
    writeln!(w, r#"    ) {{"#)?;
    if interface.events.len() > 0 {
        writeln!(
            w,
            r#"        // SAFETY: This function required that slf has the interface INTERFACE"#
        )?;
        writeln!(
            w,
            r#"        let slf = unsafe {{ proxy::low_level::from_untyped_borrowed::<{camel}Ref>(slf) }};"#
        )?;
        writeln!(w, r#"        match opcode {{"#)?;
        for (idx, event) in interface.events.iter().enumerate() {
            writeln!(w, r#"            {idx} => {{"#)?;
            let prefix = "                ";
            if event.args.len() > 0 {
                writeln!(
                    w,
                    r#"{prefix}// SAFETY: INTERFACE requires that there are {} arguments"#,
                    event.args.len()
                )?;
                writeln!(
                    w,
                    r#"{prefix}let args = unsafe {{ &*args.cast::<[wl_argument; {}]>() }};"#,
                    event.args.len()
                )?;
                for (idx, arg) in event.args.iter().enumerate() {
                    let ty = match arg.ty {
                        ArgType::NewId | ArgType::Object => "an object",
                        ArgType::Int => "an int",
                        ArgType::Uint => "a uint",
                        ArgType::Fixed => "a fixed",
                        ArgType::String => "a string",
                        ArgType::Array => "an array",
                        ArgType::Fd => "a file descriptor",
                    };
                    writeln!(
                        w,
                        r#"{prefix}// SAFETY: - INTERFACE requires that args[{idx}] contains {ty}"#
                    )?;
                    match arg.ty {
                        ArgType::NewId => {
                            writeln!(
                                w,
                                r#"{prefix}//         - ownership is transferred to this function"#
                            )?;
                            match &arg.interface {
                                None => {
                                    writeln!(
                                        w,
                                        r#"{prefix}let arg{idx} = unsafe {{ args[{idx}].o.cast() }};"#
                                    )?;
                                }
                                Some(i) => {
                                    let camel = format_camel(i).to_string();
                                    writeln!(
                                        w,
                                        r#"{prefix}//         - INTERFACE requires that the object has the interface {camel}::WL_INTERFACE"#,
                                    )?;
                                    writeln!(w, r#"{prefix}let arg{idx} = unsafe {{"#)?;
                                    writeln!(
                                        w,
                                        r#"{prefix}    UntypedOwnedProxy::from_plain_wl_proxy("#
                                    )?;
                                    writeln!(w, r#"{prefix}        queue,"#)?;
                                    writeln!(
                                        w,
                                        r#"{prefix}        NonNull::new_unchecked(args[{idx}].o.cast()),"#
                                    )?;
                                    writeln!(w, r#"{prefix}        {camel}::WL_INTERFACE,"#)?;
                                    writeln!(w, r#"{prefix}    )"#)?;
                                    writeln!(w, r#"{prefix}}};"#)?;
                                    writeln!(
                                        w,
                                        r#"{prefix}// SAFETY: - INTERFACE requires that the object has the interface {camel}::WL_INTERFACE"#
                                    )?;
                                    writeln!(
                                        w,
                                        r#"{prefix}let arg{idx} = unsafe {{ proxy::low_level::from_untyped_owned::<{camel}>(arg{idx}) }};"#
                                    )?;
                                }
                            }
                        }
                        ArgType::Int | ArgType::Uint => {
                            write!(w, r#"{prefix}let arg{idx} = unsafe {{"#)?;
                            let field = match arg.ty {
                                ArgType::Int => "i",
                                _ => "u",
                            };
                            match &arg.enum_ {
                                None => {
                                    write!(w, r#"args[{idx}].{field}"#)?;
                                }
                                Some(e) => {
                                    if e.contains('.') {
                                        write!(w, r#"{}(args[{idx}].u)"#, format_camel(e))?;
                                    } else {
                                        write!(w, r#"{camel}{}(args[{idx}].u)"#, format_camel(e))?;
                                    }
                                }
                            }
                            writeln!(w, r#" }};"#)?;
                        }
                        ArgType::Fixed => {
                            writeln!(
                                w,
                                r#"{prefix}let arg{idx} = unsafe {{ Fixed::from_wire(args[{idx}].f) }};"#
                            )?;
                        }
                        ArgType::String => {
                            let name = match arg.allow_null {
                                true => "convert_optional_string_arg",
                                false => "convert_string_arg",
                            };
                            writeln!(
                                w,
                                r#"{prefix}//         - if the pointer is not null, then it is a c string"#,
                            )?;
                            writeln!(
                                w,
                                r#"{prefix}let arg{idx} = unsafe {{ {name}("{snake}", "{}", args[{idx}].s) }};"#,
                                arg.name,
                            )?;
                        }
                        ArgType::Object => {
                            writeln!(w, r#"{prefix}let arg{idx} = unsafe {{"#)?;
                            writeln!(
                                w,
                                r#"{prefix}    if let Some(p) = NonNull::new(args[{idx}].o.cast()) {{"#
                            )?;
                            writeln!(
                                w,
                                r#"{prefix}        Some(UntypedBorrowedProxy::new_immutable(queue.libwayland(), p))"#
                            )?;
                            writeln!(w, r#"{prefix}    }} else {{"#)?;
                            writeln!(w, r#"{prefix}        None"#)?;
                            writeln!(w, r#"{prefix}    }}"#)?;
                            writeln!(w, r#"{prefix}}};"#)?;
                            if let Some(i) = &arg.interface {
                                writeln!(
                                    w,
                                    r#"{prefix}// SAFETY: - INTERFACE requires that the object has the interface {}::WL_INTERFACE"#,
                                    format_camel(i),
                                )?;
                                writeln!(
                                    w,
                                    r#"{prefix}let arg{idx} = arg{idx}.as_ref().map(|arg{idx}| unsafe {{ proxy::low_level::from_untyped_borrowed::<{}Ref>(arg{idx}) }});"#,
                                    format_camel(i),
                                )?;
                            } else {
                                writeln!(w, r#"{prefix}let arg{idx} = arg{idx}.as_ref();"#)?;
                            }
                        }
                        ArgType::Array => {
                            writeln!(w, r#"{prefix}let arg{idx} = unsafe {{"#)?;
                            writeln!(w, r#"{prefix}    let a = &*args[{idx}].a;"#)?;
                            writeln!(
                                w,
                                r#"{prefix}    std::slice::from_raw_parts(a.data.cast(), a.size)"#
                            )?;
                            writeln!(w, r#"}};"#)?;
                        }
                        ArgType::Fd => {
                            writeln!(
                                w,
                                r#"{prefix}let arg{idx} = unsafe {{ OwnedFd::from_raw_fd(args[{idx}].h) }};"#
                            )?;
                        }
                    }
                }
            }
            write!(w, r#"{prefix}self.0.{}(slf"#, escape_name(&event.name))?;
            for idx in 0..event.args.len() {
                write!(w, r#", arg{idx}"#)?;
            }
            writeln!(w, r#");"#)?;
            writeln!(w, r#"            }}"#)?;
        }
        writeln!(w, r#"            _ => {{"#)?;
        writeln!(w, r#"                invalid_opcode("{snake}", opcode);"#)?;
        writeln!(w, r#"            }}"#)?;
        writeln!(w, r#"        }}"#)?;
    } else {
        writeln!(w, r#"        invalid_opcode("{snake}", opcode);"#)?;
    }
    writeln!(w, r#"    }}"#)?;
    writeln!(w, r#"}}"#)?;
    writeln!(w)?;
    writeln!(w, r#"impl<H> CreateEventHandler<H> for private::ProxyApi"#)?;
    writeln!(w, r#"where"#)?;
    writeln!(w, r#"    H: {camel}EventHandler,"#)?;
    writeln!(w, r#"{{"#)?;
    writeln!(w, r#"    type EventHandler = private::EventHandler<H>;"#)?;
    writeln!(w)?;
    writeln!(w, r#"    #[inline]"#)?;
    writeln!(
        w,
        r#"    fn create_event_handler(handler: H) -> Self::EventHandler {{"#
    )?;
    writeln!(w, r#"        private::EventHandler(handler)"#)?;
    writeln!(w, r#"    }}"#)?;
    writeln!(w, r#"}}"#)?;
    Ok(())
}

fn format_interface_enums(w: &mut impl Write, interface: &Interface) -> io::Result<()> {
    let camel = format_camel(&interface.name).to_string();
    if interface.enums.len() > 0 {
        writeln!(w, r#"impl {camel} {{"#)?;
        for (idx, enum_) in interface.enums.iter().enumerate() {
            if idx > 0 {
                writeln!(w)?;
            }
            for entry in &enum_.entries {
                format_since(
                    w,
                    "ENM",
                    "enum variant",
                    debug_fn(|f| write!(f, "{}.{}", enum_.name, entry.name)),
                    debug_fn(|f| {
                        write!(
                            f,
                            "{}_{}",
                            format_uppercase(&enum_.name),
                            format_uppercase(&entry.name)
                        )
                    }),
                    entry.since,
                    entry.deprecated_since,
                )?;
            }
        }
        writeln!(w, r#"}}"#)?;
        writeln!(w)?;
    }
    for (idx, enum_) in interface.enums.iter().enumerate() {
        if idx > 0 {
            writeln!(w)?;
        }
        let camel = format!("{camel}{}", format_camel(&enum_.name));
        if let Some(desc) = &enum_.description {
            format_description(w, "///", desc)?;
        }
        writeln!(
            w,
            r#"#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]"#
        )?;
        if enum_.bitfield {
            writeln!(w, r#"#[derive(Default)]"#)?;
        }
        writeln!(w, r#"#[allow(dead_code)]"#)?;
        writeln!(w, r#"pub struct {camel}(pub u32);"#)?;
        if enum_.bitfield {
            writeln!(w)?;
            writeln!(w, r#"/// An iterator over the set bits in a [{camel}]."#)?;
            writeln!(w, r#"///"#)?;
            writeln!(
                w,
                r#"/// You can construct this with the `IntoIterator` implementation of `{camel}`."#
            )?;
            writeln!(w, r#"#[derive(Clone, Debug)]"#)?;
            writeln!(w, r#"pub struct {camel}Iter(pub u32);"#)?;
        }
        if enum_.entries.len() > 0 {
            writeln!(w)?;
            writeln!(w, r#"impl {camel} {{"#)?;
            for (idx, entry) in enum_.entries.iter().enumerate() {
                if idx > 0 {
                    writeln!(w)?;
                }
                let mut needs_newline = false;
                if let Some(summary) = &entry.summary {
                    for line in summary.lines() {
                        writeln!(w, r#"    /// {line}"#)?;
                        needs_newline = true;
                    }
                }
                if let Some(desc) = &entry.description {
                    if needs_newline {
                        writeln!(w, r#"    ///"#)?;
                    }
                    format_description(w, "    ///", desc)?;
                }
                writeln!(w, r#"    #[allow(dead_code)]"#)?;
                writeln!(
                    w,
                    r#"    pub const {}: Self = Self({});"#,
                    format_enum_variant(&entry.name),
                    entry.value
                )?;
            }
            writeln!(w, r#"}}"#)?;
        }
        if enum_.bitfield {
            writeln!(w)?;
            writeln!(w, r#"#[allow(dead_code)]"#)?;
            writeln!(w, r#"impl {camel} {{"#)?;
            writeln!(w, r#"    #[inline]"#)?;
            writeln!(w, r#"    pub const fn empty() -> Self {{"#)?;
            writeln!(w, r#"        Self(0)"#)?;
            writeln!(w, r#"    }}"#)?;
            writeln!(w)?;
            writeln!(w, r#"    #[inline]"#)?;
            writeln!(w, r#"    #[must_use]"#)?;
            writeln!(w, r#"    pub const fn is_empty(self) -> bool {{"#)?;
            writeln!(w, r#"        self.0 == 0"#)?;
            writeln!(w, r#"    }}"#)?;
            writeln!(w)?;
            writeln!(w, r#"    #[inline]"#)?;
            writeln!(w, r#"    #[must_use]"#)?;
            writeln!(
                w,
                r#"    pub const fn contains(self, other: Self) -> bool {{"#
            )?;
            writeln!(w, r#"        self.0 & other.0 == other.0"#)?;
            writeln!(w, r#"    }}"#)?;
            writeln!(w)?;
            writeln!(w, r#"    #[inline]"#)?;
            writeln!(w, r#"    #[must_use]"#)?;
            writeln!(
                w,
                r#"    pub const fn intersects(self, other: Self) -> bool {{"#
            )?;
            writeln!(w, r#"        self.0 & other.0 != 0"#)?;
            writeln!(w, r#"    }}"#)?;
            writeln!(w)?;
            writeln!(w, r#"    #[inline]"#)?;
            writeln!(w, r#"    pub const fn insert(&mut self, other: Self) {{"#)?;
            writeln!(w, r#"        *self = self.union(other);"#)?;
            writeln!(w, r#"    }}"#)?;
            writeln!(w)?;
            writeln!(w, r#"    #[inline]"#)?;
            writeln!(w, r#"    pub const fn remove(&mut self, other: Self) {{"#)?;
            writeln!(w, r#"        *self = self.difference(other);"#)?;
            writeln!(w, r#"    }}"#)?;
            writeln!(w)?;
            writeln!(w, r#"    #[inline]"#)?;
            writeln!(w, r#"    pub const fn toggle(&mut self, other: Self) {{"#)?;
            writeln!(w, r#"        *self = self.symmetric_difference(other);"#)?;
            writeln!(w, r#"    }}"#)?;
            writeln!(w)?;
            writeln!(w, r#"    #[inline]"#)?;
            writeln!(
                w,
                r#"    pub const fn set(&mut self, other: Self, value: bool) {{"#
            )?;
            writeln!(w, r#"        if value {{"#)?;
            writeln!(w, r#"            self.insert(other);"#)?;
            writeln!(w, r#"        }} else {{"#)?;
            writeln!(w, r#"            self.remove(other);"#)?;
            writeln!(w, r#"        }}"#)?;
            writeln!(w, r#"    }}"#)?;
            writeln!(w)?;
            writeln!(w, r#"    #[inline]"#)?;
            writeln!(w, r#"    #[must_use]"#)?;
            writeln!(
                w,
                r#"    pub const fn intersection(self, other: Self) -> Self {{"#
            )?;
            writeln!(w, r#"        Self(self.0 & other.0)"#)?;
            writeln!(w, r#"    }}"#)?;
            writeln!(w)?;
            writeln!(w, r#"    #[inline]"#)?;
            writeln!(w, r#"    #[must_use]"#)?;
            writeln!(w, r#"    pub const fn union(self, other: Self) -> Self {{"#)?;
            writeln!(w, r#"        Self(self.0 | other.0)"#)?;
            writeln!(w, r#"    }}"#)?;
            writeln!(w)?;
            writeln!(w, r#"    #[inline]"#)?;
            writeln!(w, r#"    #[must_use]"#)?;
            writeln!(
                w,
                r#"    pub const fn difference(self, other: Self) -> Self {{"#
            )?;
            writeln!(w, r#"        Self(self.0 & !other.0)"#)?;
            writeln!(w, r#"    }}"#)?;
            writeln!(w)?;
            writeln!(w, r#"    #[inline]"#)?;
            writeln!(w, r#"    #[must_use]"#)?;
            writeln!(w, r#"    pub const fn complement(self) -> Self {{"#)?;
            writeln!(w, r#"        Self(!self.0)"#)?;
            writeln!(w, r#"    }}"#)?;
            writeln!(w)?;
            writeln!(w, r#"    #[inline]"#)?;
            writeln!(w, r#"    #[must_use]"#)?;
            writeln!(
                w,
                r#"    pub const fn symmetric_difference(self, other: Self) -> Self {{"#
            )?;
            writeln!(w, r#"        Self(self.0 ^ other.0)"#)?;
            writeln!(w, r#"    }}"#)?;
            writeln!(w)?;
            writeln!(w, r#"    #[inline]"#)?;
            writeln!(w, r#"    pub const fn all_known() -> Self {{"#)?;
            writeln!(w, r#"        #[allow(clippy::eq_op, clippy::identity_op)]"#)?;
            write!(w, r#"        Self(0"#)?;
            for entry in &enum_.entries {
                write!(w, r#" | {}"#, entry.value)?;
            }
            writeln!(w, r#")"#)?;
            writeln!(w, r#"    }}"#)?;
            writeln!(w, r#"}}"#)?;
            writeln!(w)?;
            writeln!(w, r#"impl Iterator for {camel}Iter {{"#)?;
            writeln!(w, r#"    type Item = {camel};"#)?;
            writeln!(w)?;
            writeln!(w, r#"    fn next(&mut self) -> Option<Self::Item> {{"#)?;
            writeln!(w, r#"        if self.0 == 0 {{"#)?;
            writeln!(w, r#"            return None;"#)?;
            writeln!(w, r#"        }}"#)?;
            writeln!(w, r#"        let bit = 1 << self.0.trailing_zeros();"#)?;
            writeln!(w, r#"        self.0 &= !bit;"#)?;
            writeln!(w, r#"        Some({camel}(bit))"#)?;
            writeln!(w, r#"    }}"#)?;
            writeln!(w, r#"}}"#)?;
            writeln!(w)?;
            writeln!(w, r#"impl IntoIterator for {camel} {{"#)?;
            writeln!(w, r#"    type Item = {camel};"#)?;
            writeln!(w, r#"    type IntoIter = {camel}Iter;"#)?;
            writeln!(w)?;
            writeln!(w, r#"    fn into_iter(self) -> Self::IntoIter {{"#)?;
            writeln!(w, r#"        {camel}Iter(self.0)"#)?;
            writeln!(w, r#"    }}"#)?;
            writeln!(w, r#"}}"#)?;
            macro_rules! bitop {
                ($capital:literal, $lower:literal, $op:literal) => {{
                    writeln!(w)?;
                    writeln!(w, r#"impl Bit{} for {camel} {{"#, $capital)?;
                    writeln!(w, r#"    type Output = Self;"#)?;
                    writeln!(w)?;
                    writeln!(
                        w,
                        r#"    fn bit{}(self, rhs: Self) -> Self::Output {{"#,
                        $lower
                    )?;
                    writeln!(w, r#"        self.{}(rhs)"#, $op)?;
                    writeln!(w, r#"    }}"#)?;
                    writeln!(w, r#"}}"#)?;
                    writeln!(w)?;
                    writeln!(w, r#"impl Bit{}Assign for {camel} {{"#, $capital)?;
                    writeln!(w, r#"    fn bit{}_assign(&mut self, rhs: Self) {{"#, $lower)?;
                    writeln!(w, r#"        *self = self.{}(rhs);"#, $op)?;
                    writeln!(w, r#"    }}"#)?;
                    writeln!(w, r#"}}"#)?;
                }};
            }
            bitop!("And", "and", "intersection");
            bitop!("Or", "or", "union");
            bitop!("Xor", "xor", "symmetric_difference");
            writeln!(w)?;
            writeln!(w, r#"impl Sub for {camel} {{"#)?;
            writeln!(w, r#"    type Output = Self;"#)?;
            writeln!(w)?;
            writeln!(w, r#"    fn sub(self, rhs: Self) -> Self::Output {{"#)?;
            writeln!(w, r#"        self.difference(rhs)"#)?;
            writeln!(w, r#"    }}"#)?;
            writeln!(w, r#"}}"#)?;
            writeln!(w)?;
            writeln!(w, r#"impl SubAssign for {camel} {{"#)?;
            writeln!(w, r#"    fn sub_assign(&mut self, rhs: Self) {{"#)?;
            writeln!(w, r#"        *self = self.difference(rhs);"#)?;
            writeln!(w, r#"    }}"#)?;
            writeln!(w, r#"}}"#)?;
            writeln!(w)?;
            writeln!(w, r#"impl Not for {camel} {{"#)?;
            writeln!(w, r#"    type Output = Self;"#)?;
            writeln!(w)?;
            writeln!(w, r#"    fn not(self) -> Self::Output {{"#)?;
            writeln!(w, r#"        self.complement()"#)?;
            writeln!(w, r#"    }}"#)?;
            writeln!(w, r#"}}"#)?;
        }
        writeln!(w)?;
        writeln!(w, r#"impl Debug for {camel} {{"#)?;
        writeln!(
            w,
            r#"    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {{"#
        )?;
        if enum_.bitfield {
            writeln!(w, r#"        let mut v = self.0;"#)?;
            writeln!(w, r#"        let mut first = true;"#)?;
            let mut zero_entry = None;
            for entry in &enum_.entries {
                if entry.value_u32 == 0 {
                    zero_entry = Some(entry);
                    continue;
                }
                writeln!(w, r#"        if v & {} == {} {{"#, entry.value, entry.value)?;
                writeln!(w, r#"            v &= !{};"#, entry.value)?;
                writeln!(w, r#"            if first {{"#)?;
                writeln!(w, r#"                first = false;"#)?;
                writeln!(w, r#"            }} else {{"#)?;
                writeln!(w, r#"                f.write_str(" | ")?;"#)?;
                writeln!(w, r#"            }}"#)?;
                writeln!(
                    w,
                    r#"            f.write_str("{}")?;"#,
                    format_enum_variant(&entry.name)
                )?;
                writeln!(w, r#"        }}"#)?;
            }
            writeln!(w, r#"        if v != 0 {{"#)?;
            writeln!(w, r#"            if first {{"#)?;
            writeln!(w, r#"                first = false;"#)?;
            writeln!(w, r#"            }} else {{"#)?;
            writeln!(w, r#"                f.write_str(" | ")?;"#)?;
            writeln!(w, r#"            }}"#)?;
            writeln!(w, r#"            write!(f, "0x{{v:032x}}")?;"#)?;
            writeln!(w, r#"        }}"#)?;
            writeln!(w, r#"        if first {{"#)?;
            if let Some(entry) = zero_entry {
                writeln!(
                    w,
                    r#"            f.write_str("{}")?;"#,
                    format_enum_variant(&entry.name)
                )?;
            } else {
                writeln!(w, r#"            f.write_str("0")?;"#)?;
            }
            writeln!(w, r#"        }}"#)?;
            writeln!(w, r#"        Ok(())"#)?;
        } else {
            writeln!(w, r#"        let name = match *self {{"#)?;
            for entry in &enum_.entries {
                let upper = format_enum_variant(&entry.name);
                writeln!(w, r#"            Self::{upper} => "{upper}","#)?;
            }
            writeln!(w, r#"            _ => return Debug::fmt(&self.0, f),"#)?;
            writeln!(w, r#"        }};"#)?;
            writeln!(w, r#"        f.write_str(name)"#)?;
        }
        writeln!(w, r#"    }}"#)?;
        writeln!(w, r#"}}"#)?;
    }
    Ok(())
}

fn format_event_handlers(w: &mut impl Write, interface: &Interface) -> io::Result<()> {
    let if_camel = format_camel(&interface.name).to_string();
    writeln!(w, r#"/// Functional event handlers."#)?;
    writeln!(w, r#"pub mod event_handlers {{"#)?;
    writeln!(w, r#"    use super::*;"#)?;
    for event in &interface.events {
        let camel = format_camel(&event.name).to_string();
        writeln!(w)?;
        writeln!(w, r#"    /// Event handler for {} events."#, event.name)?;
        writeln!(w, r#"    pub struct {camel}<F>(F);"#)?;
        writeln!(w, r#"    impl<F> {if_camel}EventHandler for {camel}<F>"#)?;
        writeln!(w, r#"    where"#)?;
        write!(w, r#"        F: Fn(&{if_camel}Ref"#)?;
        for arg in &event.args {
            write!(w, ", {}", arg_type(interface, arg, false))?;
        }
        writeln!(w, r#"),"#)?;
        writeln!(w, r#"    {{"#)?;
        writeln!(w, r#"        #[inline]"#)?;
        write!(
            w,
            r#"        fn {}(&self, _slf: &{if_camel}Ref"#,
            escape_name(&event.name)
        )?;
        for arg in &event.args {
            write!(
                w,
                ", {}: {}",
                escape_name(&arg.name),
                arg_type(interface, arg, false)
            )?;
        }
        writeln!(w, r#") {{"#)?;
        write!(w, r#"            self.0(_slf"#)?;
        for arg in &event.args {
            write!(w, ", {}", escape_name(&arg.name))?;
        }
        writeln!(w, r#")"#)?;
        writeln!(w, r#"        }}"#)?;
        writeln!(w, r#"    }}"#)?;
    }
    writeln!(w)?;
    writeln!(w, r#"    impl {if_camel} {{"#)?;
    for (idx, event) in interface.events.iter().enumerate() {
        if idx > 0 {
            writeln!(w)?;
        }
        let camel = format_camel(&event.name).to_string();
        writeln!(
            w,
            r#"        /// Creates an event handler for {} events."#,
            event.name
        )?;
        writeln!(w, r#"        ///"#)?;
        writeln!(
            w,
            r#"        /// The event handler ignores all other events."#
        )?;
        writeln!(w, r#"        #[allow(dead_code)]"#)?;
        writeln!(
            w,
            r#"        pub fn on_{}<F>(f: F) -> {camel}<F>"#,
            event.name,
        )?;
        writeln!(w, r#"        where"#)?;
        write!(w, r#"            F: Fn(&{if_camel}Ref"#)?;
        for arg in &event.args {
            write!(w, ", {}", arg_type(interface, arg, false))?;
        }
        writeln!(w, r#"),"#)?;
        writeln!(w, r#"        {{"#)?;
        writeln!(w, r#"            {camel}(f)"#)?;
        writeln!(w, r#"        }}"#)?;
    }
    writeln!(w, r#"    }}"#)?;
    writeln!(w, r#"}}"#)?;
    Ok(())
}

fn format_interface_trait_impls(w: &mut impl Write, interface: &Interface) -> io::Result<()> {
    let snake = &interface.name;
    let camel = format_camel(snake).to_string();
    writeln!(
        w,
        r#"// SAFETY: {camel} is a transparent wrapper around UntypedOwnedProxy"#
    )?;
    writeln!(
        w,
        r#"unsafe impl UntypedOwnedProxyWrapper for {camel} {{ }}"#
    )?;
    writeln!(w)?;
    writeln!(w, r#"// SAFETY: - INTERFACE is a valid wl_interface"#)?;
    writeln!(
        w,
        r#"//         - The only invariant is that self.proxy has a compatible interface"#
    )?;
    writeln!(w, r#"unsafe impl OwnedProxy for {camel} {{"#)?;
    writeln!(w, r#"    const INTERFACE: &'static str = "{snake}";"#)?;
    writeln!(
        w,
        r#"    const WL_INTERFACE: &'static wl_interface = &INTERFACE;"#
    )?;
    writeln!(
        w,
        r#"    const NO_OP_EVENT_HANDLER: Self::NoOpEventHandler = private::EventHandler(private::NoOpEventHandler);"#
    )?;
    writeln!(w, r#"    const MAX_VERSION: u32 = {};"#, interface.version)?;
    writeln!(w)?;
    writeln!(w, r#"    type Borrowed = {camel}Ref;"#)?;
    writeln!(w, r#"    type Api = private::ProxyApi;"#)?;
    writeln!(
        w,
        r#"    type NoOpEventHandler = private::EventHandler<private::NoOpEventHandler>;"#
    )?;
    writeln!(w, r#"}}"#)?;
    writeln!(w)?;
    writeln!(
        w,
        r#"// SAFETY: {camel}Ref is a transparent wrapper around UntypedBorrowedProxy"#
    )?;
    writeln!(
        w,
        r#"unsafe impl UntypedBorrowedProxyWrapper for {camel}Ref {{ }}"#,
    )?;
    writeln!(w)?;
    writeln!(
        w,
        r#"// SAFETY: - The only invariant is that self.proxy has a compatible interface"#
    )?;
    writeln!(w, r#"unsafe impl BorrowedProxy for {camel}Ref {{"#)?;
    writeln!(w, r#"    type Owned = {camel};"#)?;
    writeln!(w, r#"}}"#)?;
    writeln!(w)?;
    writeln!(w, r#"impl Deref for {camel} {{"#)?;
    writeln!(w, r#"    type Target = {camel}Ref;"#)?;
    writeln!(w)?;
    writeln!(w, r#"    fn deref(&self) -> &Self::Target {{"#)?;
    writeln!(w, r#"        proxy::low_level::deref(self)"#)?;
    writeln!(w, r#"    }}"#)?;
    writeln!(w, r#"}}"#)?;
    writeln!(w)?;
    writeln!(w, r#"mod private {{"#)?;
    writeln!(w, r#"    pub struct ProxyApi;"#)?;
    writeln!(w)?;
    writeln!(w, r#"    #[allow(dead_code)]"#)?;
    writeln!(w, r#"    pub struct EventHandler<H>(pub(super) H);"#)?;
    writeln!(w)?;
    writeln!(w, r#"    #[allow(dead_code)]"#)?;
    writeln!(w, r#"    pub struct NoOpEventHandler;"#)?;
    writeln!(w, r#"}}"#)?;
    writeln!(w)?;
    writeln!(w, r#"impl Debug for {camel} {{"#)?;
    writeln!(
        w,
        r#"    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {{"#
    )?;
    writeln!(w, r#"        write!(f, "{snake}#{{}}", self.proxy.id())"#)?;
    writeln!(w, r#"    }}"#)?;
    writeln!(w, r#"}}"#)?;
    writeln!(w)?;
    writeln!(w, r#"impl Debug for {camel}Ref {{"#)?;
    writeln!(
        w,
        r#"    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {{"#
    )?;
    writeln!(w, r#"        write!(f, "{snake}#{{}}", self.proxy.id())"#)?;
    writeln!(w, r#"    }}"#)?;
    writeln!(w, r#"}}"#)?;
    writeln!(w)?;
    writeln!(w, r#"impl PartialEq<{camel}Ref> for {camel} {{"#)?;
    writeln!(w, r#"    fn eq(&self, other: &{camel}Ref) -> bool {{"#)?;
    writeln!(w, r#"        self.proxy == other.proxy"#)?;
    writeln!(w, r#"    }}"#)?;
    writeln!(w, r#"}}"#)?;
    writeln!(w)?;
    writeln!(w, r#"impl PartialEq<{camel}> for {camel}Ref {{"#)?;
    writeln!(w, r#"    fn eq(&self, other: &{camel}) -> bool {{"#)?;
    writeln!(w, r#"        self.proxy == other.proxy"#)?;
    writeln!(w, r#"    }}"#)?;
    writeln!(w, r#"}}"#)?;
    Ok(())
}

fn escape_name(name: &str) -> impl Display + use<'_> {
    static KEYWORDS: phf::Set<&'static str> = phf_set! {
        "abstract", "as", "async", "await", "become", "box", "break", "const",
        "continue", "crate", "do", "dyn", "else", "enum", "extern", "false", "final",
        "fn", "for", "gen", "if", "impl", "in", "let", "loop", "macro", "macro_rules",
        "match", "mod", "move", "mut", "override", "priv", "pub", "raw", "ref",
        "return", "safe", "self", "Self", "static", "struct", "super", "trait", "true",
        "try", "type", "typeof", "union", "unsafe", "unsized", "use", "virtual",
        "where", "while", "yield",
    };
    debug_fn(move |f| {
        if KEYWORDS.contains(name) {
            f.write_str("r#")?;
        }
        f.write_str(name)
    })
}
