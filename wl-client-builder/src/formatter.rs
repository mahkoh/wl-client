use {
    crate::ast::{Arg, ArgType, Description, Interface, Message, MessageType, Protocol},
    debug_fn::debug_fn,
    phf::phf_set,
    std::{
        fmt::{Display, Write as FmtWrite},
        io::{self, Write},
    },
};

macro_rules! define_w {
    ($w:ident) => {
        define_w!($w, $);
    };
    ($w:ident, $dol:tt) => {
        #[allow(unused_macros)]
        macro_rules! w {
            ($dol($arg:tt)*) => {
                write!($w, $dol($arg)*)
            };
        }
        macro_rules! wl {
            ($dol($arg:tt)*) => {
                writeln!($w, $dol($arg)*)
            };
        }
    };
}

fn format_interface_header(
    w: &mut impl Write,
    prefix: &str,
    interface: &Interface,
) -> io::Result<()> {
    define_w!(w);
    if let Some(desc) = &interface.description {
        format_description(w, "//!", desc)?;
        wl!()?;
    }
    wl!("use {prefix}::builder::prelude::*;")?;
    wl!("use super::super::all_types::*;")?;
    Ok(())
}

pub fn format_interface_file(
    w: &mut impl Write,
    root: &str,
    interface: &Interface,
) -> io::Result<()> {
    define_w!(w);
    format_interface_header(w, root, interface)?;
    wl!()?;
    format_wl_interface(w, interface)?;
    wl!()?;
    format_interface_types(w, interface)?;
    wl!()?;
    format_interface_trait_impls(w, interface)?;
    wl!()?;
    format_interface_requests(w, interface)?;
    wl!()?;
    format_interface_event_handler(w, interface)?;
    wl!()?;
    format_event_handler(w, interface)?;
    wl!()?;
    format_interface_enums(w, interface)?;
    wl!()?;
    format_event_handlers(w, interface)?;
    Ok(())
}

fn format_wl_interface(w: &mut impl Write, interface: &Interface) -> io::Result<()> {
    define_w!(w);
    wl!(r#"static INTERFACE: wl_interface = wl_interface {{"#)?;
    wl!(r#"    name: c"{}".as_ptr(),"#, interface.name)?;
    wl!(r#"    version: {},"#, interface.version)?;
    wl!(r#"    method_count: {},"#, interface.requests.len())?;
    wl!(
        r#"    methods: {},"#,
        format_wl_interface_messages(&interface.requests)
    )?;
    wl!(r#"    event_count: {},"#, interface.events.len())?;
    wl!(
        r#"    events: {},"#,
        format_wl_interface_messages(&interface.events)
    )?;
    wl!(r#"}};"#)?;
    Ok(())
}

fn format_wl_interface_messages(messages: &[Message]) -> impl Display + use<'_> {
    debug_fn(move |f| {
        define_w!(f);
        if messages.is_empty() {
            return f.write_str("ptr::null()");
        }
        wl!(r#"{{"#)?;
        wl!(
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

            wl!(r#"            wl_message {{"#)?;
            wl!(r#"                name: c"{}".as_ptr(),"#, message.name)?;
            wl!(
                r#"                signature: c"{}".as_ptr(),"#,
                format_signature(message)
            )?;
            wl!(r#"                types: {{"#)?;
            wl!(
                r#"                    static TYPES: [Option<&'static wl_interface>; {}] = ["#,
                num_types,
            )?;
            for arg in &message.args {
                match &arg.interface {
                    None => {
                        if arg.ty == ArgType::NewId {
                            wl!(r#"                        None,"#)?;
                            wl!(r#"                        None,"#)?;
                        }
                        wl!(r#"                        None,"#)?;
                    }
                    Some(name) => {
                        wl!(
                            r#"                        Some({}::WL_INTERFACE),"#,
                            format_camel(name),
                        )?;
                    }
                }
            }
            wl!(r#"                    ];"#)?;
            wl!(r#"                    TYPES.as_ptr().cast()"#)?;
            wl!(r#"                }},"#)?;
            wl!(r#"            }},"#)?;
        }
        wl!(r#"        ];"#)?;
        wl!(r#"        MESSAGES.as_ptr()"#)?;
        w!(r#"    }}"#)?;
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
    define_w!(w);
    let snake = &interface.name;
    let camel = format_camel(snake).to_string();
    wl!(r#"/// An owned {snake} proxy."#)?;
    wl!(r#"///"#)?;
    wl!(r#"/// See the documentation of [the module][self] for the interface description."#)?;
    wl!(r#"#[derive(Clone, Eq, PartialEq)]"#)?;
    wl!(r#"#[repr(transparent)]"#)?;
    wl!(r#"pub struct {camel} {{"#)?;
    wl!(r#"    /// This proxy has the interface INTERFACE."#)?;
    wl!(r#"    proxy: UntypedOwnedProxy,"#)?;
    wl!(r#"}}"#)?;
    wl!()?;
    wl!(r#"/// A borrowed {snake} proxy."#)?;
    wl!(r#"///"#)?;
    wl!(r#"/// See the documentation of [the module][self] for the interface description."#)?;
    wl!(r#"#[derive(Eq, PartialEq)]"#)?;
    wl!(r#"#[repr(transparent)]"#)?;
    wl!(r#"pub struct {camel}Ref {{"#)?;
    wl!(r#"    /// This proxy has the interface INTERFACE."#)?;
    wl!(r#"    proxy: UntypedBorrowedProxy,"#)?;
    wl!(r#"}}"#)?;
    Ok(())
}

fn format_interface_requests(w: &mut impl Write, interface: &Interface) -> io::Result<()> {
    define_w!(w);
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
            wl!()?;
        }
        wl!(r#"#[allow(dead_code)]"#)?;
        wl!(r#"impl {name} {{"#)?;
        let mut first = true;
        for (idx, request) in interface.requests.iter().enumerate() {
            if skip_request(request) {
                continue;
            }
            if first {
                first = false;
            } else {
                wl!()?;
            }
            let new_id = request.args.iter().find(|a| a.ty == ArgType::NewId);
            if owned {
                format_message_since(w, false, request)?;
                wl!()?;
            }
            format_message_doc(w, true, !owned, request)?;
            wl!(r#"    #[inline]"#)?;
            w!(r#"    pub fn {}"#, escape_name(&request.name))?;
            if let Some(arg) = new_id {
                if arg.interface.is_none() {
                    w!(r#"<P: OwnedProxy>"#)?;
                }
            }
            wl!(r#"("#)?;
            wl!(r#"        &self,"#)?;
            if new_id.is_some() && !owned {
                wl!(r#"        _queue: &Queue,"#)?;
            }
            let mut num_args = request.args.len();
            for arg in &request.args {
                if arg.ty == ArgType::NewId {
                    if arg.interface.is_none() {
                        wl!(r#"        version: u32,"#)?;
                    } else {
                        num_args -= 1;
                    }
                } else {
                    wl!(
                        r#"        {}: {},"#,
                        escape_name(&arg.name),
                        arg_type(interface, arg, true),
                    )?;
                }
            }
            w!(r#"    )"#)?;
            if let Some(arg) = new_id {
                match &arg.interface {
                    None => w!(" -> P")?,
                    Some(i) => w!(" -> {}", format_camel(i))?,
                }
            }
            wl!(r#" {{"#)?;
            if num_args > 0 {
                wl!(r#"        let ("#)?;
                for (idx, arg) in request.args.iter().enumerate() {
                    if arg.ty != ArgType::NewId || arg.interface.is_none() {
                        wl!(r#"            arg{idx},"#)?;
                    }
                }
                wl!(r#"        ) = ("#)?;
                for arg in &request.args {
                    if arg.ty == ArgType::NewId {
                        if arg.interface.is_none() {
                            wl!(r#"            version,"#)?;
                        }
                    } else {
                        wl!(r#"            {},"#, escape_name(&arg.name))?;
                    }
                }
                wl!(r#"        );"#)?;
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
                wl!(r#"        with_cstr_cache(|cache| {{"#)?;
                for (idx, arg) in request.args.iter().enumerate() {
                    if arg.ty != ArgType::String {
                        continue;
                    }
                    wl!(r#"{prefix}let str{idx}_offset = cache.len();"#)?;
                    if arg.allow_null {
                        wl!(r#"{prefix}if let Some(arg{idx}) = arg{idx} {{"#)?;
                        wl!(r#"{prefix}    cache.extend_from_slice(arg{idx}.as_bytes());"#)?;
                        wl!(r#"{prefix}    cache.push(0);"#)?;
                        wl!(r#"{prefix}}}"#)?;
                    } else {
                        wl!(r#"{prefix}cache.extend_from_slice(arg{idx}.as_bytes());"#)?;
                        wl!(r#"{prefix}cache.push(0);"#)?;
                    }
                }
                for (idx, arg) in request.args.iter().enumerate() {
                    if arg.ty != ArgType::String {
                        continue;
                    }
                    if arg.allow_null {
                        wl!(r#"{prefix}let mut str{idx} = ptr::null();"#)?;
                        wl!(r#"{prefix}if arg{idx}.is_some() {{"#)?;
                        wl!(r#"{prefix}    str{idx} = cache[str{idx}_offset..].as_ptr().cast();"#)?;
                        wl!(r#"{prefix}}}"#)?;
                    } else {
                        wl!(r#"{prefix}let str{idx} = cache[str{idx}_offset..].as_ptr().cast();"#)?;
                    }
                }
            }
            if object_args > 0 {
                for (idx, arg) in request.args.iter().enumerate() {
                    if arg.ty != ArgType::Object {
                        continue;
                    }
                    if arg.allow_null {
                        wl!(
                            r#"{prefix}let obj{idx}_lock = arg{idx}.map(|arg{idx}| proxy::lock(arg{idx}));"#
                        )?;
                        wl!(
                            r#"{prefix}let obj{idx} = obj{idx}_lock.map(|obj{idx}_lock| check_argument_proxy("{}", obj{idx}_lock.wl_proxy())).unwrap_or(ptr::null_mut());"#,
                            arg.name,
                        )?;
                    } else {
                        wl!(r#"{prefix}let obj{idx}_lock = proxy::lock(arg{idx});"#)?;
                        wl!(
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
                    wl!(r#"{prefix}let mut arr{idx} = wl_array {{"#)?;
                    wl!(r#"{prefix}    size: arg{idx}.len(),"#)?;
                    wl!(r#"{prefix}    alloc: arg{idx}.len(),"#)?;
                    wl!(r#"{prefix}    data: arg{idx}.as_ptr().cast_mut().cast(),"#)?;
                    wl!(r#"{prefix}}};"#)?;
                }
            }
            if request.args.len() > 0 {
                wl!(r#"{prefix}let mut args = ["#)?;
                for (idx, arg) in request.args.iter().enumerate() {
                    match arg.ty {
                        ArgType::NewId if arg.interface.is_none() => {
                            wl!(r#"{prefix}    wl_argument {{ s: P::WL_INTERFACE.name }},"#)?;
                            wl!(r#"{prefix}    wl_argument {{ u: arg{idx} }},"#)?;
                            wl!(r#"{prefix}    wl_argument {{ n: 0 }},"#)?;
                        }
                        ArgType::NewId => {
                            wl!(r#"{prefix}    wl_argument {{ n: 0 }},"#)?;
                        }
                        ArgType::Int | ArgType::Uint => {
                            if arg.enum_.is_some() {
                                wl!(r#"{prefix}    wl_argument {{ u: arg{idx}.0 }},"#)?;
                            } else if arg.ty == ArgType::Int {
                                wl!(r#"{prefix}    wl_argument {{ i: arg{idx} }},"#)?;
                            } else {
                                wl!(r#"{prefix}    wl_argument {{ u: arg{idx} }},"#)?;
                            }
                        }
                        ArgType::Fixed => {
                            wl!(r#"{prefix}    wl_argument {{ f: arg{idx}.to_wire() }},"#)?;
                        }
                        ArgType::String => {
                            wl!(r#"{prefix}    wl_argument {{ s: str{idx} }},"#)?;
                        }
                        ArgType::Object => {
                            wl!(r#"{prefix}    wl_argument {{ o: obj{idx} }},"#)?;
                        }
                        ArgType::Array => {
                            wl!(r#"{prefix}    wl_argument {{ a: &mut arr{idx} }},"#)?;
                        }
                        ArgType::Fd => {
                            wl!(r#"{prefix}    wl_argument {{ h: arg{idx}.as_raw_fd() }},"#)?;
                        }
                    }
                }
                wl!(r#"{prefix}];"#)?;
            } else {
                wl!(r#"{prefix}let mut args = [];"#)?;
            };
            let is_destructor = request.ty == Some(MessageType::Destructor);
            wl!(r#"{prefix}// SAFETY: - self.proxy has the interface INTERFACE"#,)?;
            wl!(
                r#"{prefix}//         - {idx} < INTERFACE.method_count = {}"#,
                interface.requests.len(),
            )?;
            wl!(
                r#"{prefix}//         - the request signature is `{}`"#,
                format_signature(request),
            )?;
            if let Some(arg) = new_id {
                wl!(
                    r#"{prefix}//         - OwnedProxy::WL_INTERFACE is always a valid interface"#,
                )?;
                wl!(r#"{prefix}let data = unsafe {{"#)?;
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
                    wl!(
                        r#"{prefix}    self.proxy.send_constructor::<{is_destructor}>({idx}, &mut args, {interface}, {version})"#,
                    )?;
                } else {
                    wl!(
                        r#"{prefix}    self.proxy.send_constructor(_queue, {idx}, &mut args, {interface}, {version})"#,
                    )?;
                }
                wl!(r#"{prefix}}};"#)?;
                wl!(r#"{prefix}// SAFETY: data has the interface {interface}"#)?;
                wl!(r#"{prefix}unsafe {{"#)?;
                wl!(r#"{prefix}    proxy::low_level::from_untyped_owned(data)"#)?;
                wl!(r#"{prefix}}}"#)?;
            } else {
                wl!(r#"{prefix}unsafe {{"#)?;
                if owned {
                    assert!(is_destructor);
                    wl!(r#"{prefix}    self.proxy.send_destructor({idx}, &mut args);"#,)?;
                } else {
                    wl!(r#"{prefix}    self.proxy.send_request({idx}, &mut args);"#,)?;
                }
                wl!(r#"{prefix}}}"#)?;
            }
            if string_args > 0 {
                wl!(r#"        }})"#)?;
            }
            wl!(r#"    }}"#)?;
        }
        wl!(r#"}}"#)?;
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
    define_w!(w);
    wl!(r#"    /// Since when the {name} {ty} is available."#,)?;
    wl!(r#"    #[allow(dead_code)]"#)?;
    wl!(
        r#"    pub const {prefix}__{uppercase}__SINCE: u32 = {};"#,
        since.unwrap_or(1),
    )?;
    if let Some(n) = deprecated_since {
        wl!()?;
        wl!(r#"    /// Since when the {name} {ty} is deprecated."#,)?;
        wl!(r#"    #[allow(dead_code)]"#)?;
        wl!(r#"    pub const {prefix}__{uppercase}__DEPRECATED_SINCE: u32 = {n};"#,)?;
    }
    Ok(())
}

fn format_interface_event_handler(w: &mut impl Write, interface: &Interface) -> io::Result<()> {
    define_w!(w);
    let snake = &interface.name;
    let camel = format_camel(snake).to_string();
    if interface.events.len() > 0 {
        wl!(r#"impl {} {{"#, camel)?;
        for (idx, event) in interface.events.iter().enumerate() {
            if idx > 0 {
                wl!()?;
            }
            format_message_since(w, true, event)?;
        }
        wl!(r#"}}"#)?;
        wl!()?;
    }
    wl!(r#"/// An event handler for [{camel}] proxies."#)?;
    wl!(r#"#[allow(dead_code)]"#)?;
    wl!(r#"pub trait {camel}EventHandler {{"#)?;
    for (idx, event) in interface.events.iter().enumerate() {
        if idx > 0 {
            wl!()?;
        }
        format_message_doc(w, false, false, event)?;
        wl!(r#"    #[inline]"#)?;
        wl!(r#"    fn {}("#, escape_name(&event.name))?;
        wl!(r#"        &self,"#)?;
        wl!(r#"        _slf: &{camel}Ref,"#)?;
        for arg in &event.args {
            wl!(
                r#"        {}: {},"#,
                escape_name(&arg.name),
                arg_type(interface, arg, false)
            )?;
        }
        wl!(r#"    ) {{"#)?;
        for arg in &event.args {
            wl!(r#"        let _ = {};"#, escape_name(&arg.name))?;
        }
        wl!(r#"    }}"#)?;
    }
    wl!(r#"}}"#)?;
    wl!()?;
    wl!(r#"impl {camel}EventHandler for private::NoOpEventHandler {{ }}"#)?;
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
    define_w!(w);
    for (protocol, _) in protocols {
        wl!(r#"pub mod {};"#, protocol)?;
    }
    wl!()?;
    wl!("#[allow(unused_imports)]")?;
    wl!("mod all_types {{")?;
    for (proto, interfaces) in protocols {
        for (snake, enums) in interfaces {
            let camel = format_camel(snake).to_string();
            let prefix =
                debug_fn(|f| write!(f, r#"    pub(super) use super::{proto}::{snake}::{camel}"#));
            wl!(r#"{prefix};"#)?;
            wl!(r#"{prefix}Ref;"#)?;
            for enum_ in enums {
                wl!(r#"{prefix}{};"#, format_camel(enum_))?;
            }
        }
    }
    wl!("}}")?;
    Ok(())
}

pub fn format_protocol_file(w: &mut impl Write, protocol: &Protocol) -> io::Result<()> {
    define_w!(w);
    if let Some(description) = &protocol.description {
        format_description(w, "//!", description)?;
        wl!()?;
    }
    wl!("#![allow(clippy::tabs_in_doc_comments)]")?;
    wl!("#![allow(clippy::doc_lazy_continuation)]")?;
    wl!("#![allow(clippy::too_many_arguments)]")?;
    wl!("#![allow(clippy::manual_map)]")?;
    wl!("#![allow(clippy::module_inception)]")?;
    wl!("#![allow(unused_imports)]")?;
    wl!("#![allow(rustdoc::broken_intra_doc_links)]")?;
    wl!("#![allow(rustdoc::bare_urls)]")?;
    wl!("#![allow(rustdoc::invalid_rust_codeblocks)]")?;
    wl!()?;
    for interface in &protocol.interfaces {
        let snake = &interface.name;
        wl!(r#"pub mod {snake};"#)?;
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
    define_w!(w);
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
            wl!("    ///")?;
        }
        wl!("    /// # Arguments")?;
        wl!("    ///")?;
        if is_constructor && on_ref {
            wl!("    /// - `_queue`: The queue that the returned proxy is assigned to.")?;
        }
        for arg in &message.args {
            if request && arg.ty == ArgType::NewId {
                continue;
            }
            let name = escape_name(&arg.name).to_string();
            w!("    /// - `{name}`:")?;
            let prefix = format!("    ///    {:width$}  ", " ", width = name.len());
            let mut first = true;
            let mut needs_newline = false;
            if let Some(summary) = &arg.summary {
                for line in summary.lines() {
                    if first {
                        first = false;
                    } else {
                        w!("{}", prefix)?;
                    }
                    wl!(" {}", line)?;
                }
                needs_newline = true;
            }
            if let Some(desc) = &arg.description {
                if needs_newline {
                    wl!("    ///")?;
                }
                format_description(w, &prefix, desc)?;
            }
            if arg.summary.is_none() && arg.description.is_none() {
                wl!()?;
            }
        }
        if has_object {
            wl!("    ///")?;
            wl!("    /// All borrowed proxies passed to this function are guaranteed to be")?;
            wl!("    /// immutable and non-null.")?;
        }
    }
    Ok(())
}

fn format_description(
    w: &mut impl Write,
    prefix: &str,
    description: &Description,
) -> io::Result<()> {
    define_w!(w);
    let mut needs_newline = false;
    if let Some(summary) = &description.summary {
        for line in summary.lines() {
            wl!("{prefix} {line}")?;
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
                wl!("{prefix}")?;
            }
            empty_lines = 0;
        }
        if needs_newline {
            needs_newline = false;
            wl!("{prefix}")?;
        }
        wl!("{prefix} {}", line)?;
    }
    Ok(())
}

fn format_event_handler(w: &mut impl Write, interface: &Interface) -> io::Result<()> {
    define_w!(w);
    let snake = &interface.name;
    let camel = format_camel(snake).to_string();
    wl!(r#"// SAFETY: INTERFACE is a valid wl_interface"#)?;
    wl!(r#"unsafe impl<H> EventHandler for private::EventHandler<H>"#)?;
    wl!(r#"where"#)?;
    wl!(r#"    H: {camel}EventHandler,"#)?;
    wl!(r#"{{"#)?;
    wl!(r#"    const WL_INTERFACE: &'static wl_interface = &INTERFACE;"#)?;
    wl!()?;
    wl!(r#"    #[allow(unused_variables)]"#)?;
    wl!(r#"    unsafe fn handle_event("#)?;
    wl!(r#"        &self,"#)?;
    wl!(r#"        queue: &Queue,"#)?;
    wl!(r#"        slf: &UntypedBorrowedProxy,"#)?;
    wl!(r#"        opcode: u32,"#)?;
    wl!(r#"        args: *mut wl_argument,"#)?;
    wl!(r#"    ) {{"#)?;
    if interface.events.len() > 0 {
        wl!(r#"        // SAFETY: This function required that slf has the interface INTERFACE"#)?;
        wl!(
            r#"        let slf = unsafe {{ proxy::low_level::from_untyped_borrowed::<{camel}Ref>(slf) }};"#
        )?;
        wl!(r#"        match opcode {{"#)?;
        for (idx, event) in interface.events.iter().enumerate() {
            wl!(r#"            {idx} => {{"#)?;
            let prefix = "                ";
            if event.args.len() > 0 {
                wl!(
                    r#"{prefix}// SAFETY: INTERFACE requires that there are {} arguments"#,
                    event.args.len()
                )?;
                wl!(
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
                    wl!(
                        r#"{prefix}// SAFETY: - INTERFACE requires that args[{idx}] contains {ty}"#
                    )?;
                    match arg.ty {
                        ArgType::NewId => {
                            wl!(
                                r#"{prefix}//         - ownership is transferred to this function"#
                            )?;
                            match &arg.interface {
                                None => {
                                    wl!(
                                        r#"{prefix}let arg{idx} = unsafe {{ args[{idx}].o.cast() }};"#
                                    )?;
                                }
                                Some(i) => {
                                    let camel = format_camel(i).to_string();
                                    wl!(
                                        r#"{prefix}//         - INTERFACE requires that the object has the interface {camel}::WL_INTERFACE"#,
                                    )?;
                                    wl!(r#"{prefix}let arg{idx} = unsafe {{"#)?;
                                    wl!(r#"{prefix}    UntypedOwnedProxy::from_plain_wl_proxy("#)?;
                                    wl!(r#"{prefix}        queue,"#)?;
                                    wl!(
                                        r#"{prefix}        NonNull::new_unchecked(args[{idx}].o.cast()),"#
                                    )?;
                                    wl!(r#"{prefix}        {camel}::WL_INTERFACE,"#)?;
                                    wl!(r#"{prefix}    )"#)?;
                                    wl!(r#"{prefix}}};"#)?;
                                    wl!(
                                        r#"{prefix}// SAFETY: - INTERFACE requires that the object has the interface {camel}::WL_INTERFACE"#
                                    )?;
                                    wl!(
                                        r#"{prefix}let arg{idx} = unsafe {{ proxy::low_level::from_untyped_owned::<{camel}>(arg{idx}) }};"#
                                    )?;
                                }
                            }
                        }
                        ArgType::Int | ArgType::Uint => {
                            w!(r#"{prefix}let arg{idx} = unsafe {{"#)?;
                            let field = match arg.ty {
                                ArgType::Int => "i",
                                _ => "u",
                            };
                            match &arg.enum_ {
                                None => {
                                    w!(r#"args[{idx}].{field}"#)?;
                                }
                                Some(e) => {
                                    if e.contains('.') {
                                        w!(r#"{}(args[{idx}].u)"#, format_camel(e))?;
                                    } else {
                                        w!(r#"{camel}{}(args[{idx}].u)"#, format_camel(e))?;
                                    }
                                }
                            }
                            wl!(r#" }};"#)?;
                        }
                        ArgType::Fixed => {
                            wl!(
                                r#"{prefix}let arg{idx} = unsafe {{ Fixed::from_wire(args[{idx}].f) }};"#
                            )?;
                        }
                        ArgType::String => {
                            let name = match arg.allow_null {
                                true => "convert_optional_string_arg",
                                false => "convert_string_arg",
                            };
                            wl!(
                                r#"{prefix}//         - if the pointer is not null, then it is a c string"#,
                            )?;
                            wl!(
                                r#"{prefix}let arg{idx} = unsafe {{ {name}("{snake}", "{}", args[{idx}].s) }};"#,
                                arg.name,
                            )?;
                        }
                        ArgType::Object => {
                            wl!(r#"{prefix}let arg{idx} = unsafe {{"#)?;
                            wl!(
                                r#"{prefix}    if let Some(p) = NonNull::new(args[{idx}].o.cast()) {{"#
                            )?;
                            wl!(
                                r#"{prefix}        Some(UntypedBorrowedProxy::new_immutable(queue.libwayland(), p))"#
                            )?;
                            wl!(r#"{prefix}    }} else {{"#)?;
                            wl!(r#"{prefix}        None"#)?;
                            wl!(r#"{prefix}    }}"#)?;
                            wl!(r#"{prefix}}};"#)?;
                            if let Some(i) = &arg.interface {
                                wl!(
                                    r#"{prefix}// SAFETY: - INTERFACE requires that the object has the interface {}::WL_INTERFACE"#,
                                    format_camel(i),
                                )?;
                                wl!(
                                    r#"{prefix}let arg{idx} = arg{idx}.as_ref().map(|arg{idx}| unsafe {{ proxy::low_level::from_untyped_borrowed::<{}Ref>(arg{idx}) }});"#,
                                    format_camel(i),
                                )?;
                            } else {
                                wl!(r#"{prefix}let arg{idx} = arg{idx}.as_ref();"#)?;
                            }
                        }
                        ArgType::Array => {
                            wl!(r#"{prefix}let arg{idx} = unsafe {{"#)?;
                            wl!(r#"{prefix}    let a = &*args[{idx}].a;"#)?;
                            wl!(
                                r#"{prefix}    std::slice::from_raw_parts(a.data.cast(), a.size)"#
                            )?;
                            wl!(r#"}};"#)?;
                        }
                        ArgType::Fd => {
                            wl!(
                                r#"{prefix}let arg{idx} = unsafe {{ OwnedFd::from_raw_fd(args[{idx}].h) }};"#
                            )?;
                        }
                    }
                }
            }
            w!(r#"{prefix}self.0.{}(slf"#, escape_name(&event.name))?;
            for idx in 0..event.args.len() {
                w!(r#", arg{idx}"#)?;
            }
            wl!(r#");"#)?;
            wl!(r#"            }}"#)?;
        }
        wl!(r#"            _ => {{"#)?;
        wl!(r#"                invalid_opcode("{snake}", opcode);"#)?;
        wl!(r#"            }}"#)?;
        wl!(r#"        }}"#)?;
    } else {
        wl!(r#"        invalid_opcode("{snake}", opcode);"#)?;
    }
    wl!(r#"    }}"#)?;
    wl!(r#"}}"#)?;
    wl!()?;
    wl!(r#"impl<H> CreateEventHandler<H> for private::ProxyApi"#)?;
    wl!(r#"where"#)?;
    wl!(r#"    H: {camel}EventHandler,"#)?;
    wl!(r#"{{"#)?;
    wl!(r#"    type EventHandler = private::EventHandler<H>;"#)?;
    wl!()?;
    wl!(r#"    #[inline]"#)?;
    wl!(r#"    fn create_event_handler(handler: H) -> Self::EventHandler {{"#)?;
    wl!(r#"        private::EventHandler(handler)"#)?;
    wl!(r#"    }}"#)?;
    wl!(r#"}}"#)?;
    Ok(())
}

fn format_interface_enums(w: &mut impl Write, interface: &Interface) -> io::Result<()> {
    define_w!(w);
    let camel = format_camel(&interface.name).to_string();
    if interface.enums.len() > 0 {
        wl!(r#"impl {camel} {{"#)?;
        for (idx, enum_) in interface.enums.iter().enumerate() {
            if idx > 0 {
                wl!()?;
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
        wl!(r#"}}"#)?;
        wl!()?;
    }
    for (idx, enum_) in interface.enums.iter().enumerate() {
        if idx > 0 {
            wl!()?;
        }
        let camel = format!("{camel}{}", format_camel(&enum_.name));
        if let Some(desc) = &enum_.description {
            format_description(w, "///", desc)?;
        }
        wl!(r#"#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]"#)?;
        if enum_.bitfield {
            wl!(r#"#[derive(Default)]"#)?;
        }
        wl!(r#"#[allow(dead_code)]"#)?;
        wl!(r#"pub struct {camel}(pub u32);"#)?;
        if enum_.bitfield {
            wl!()?;
            wl!(r#"/// An iterator over the set bits in a [{camel}]."#)?;
            wl!(r#"///"#)?;
            wl!(
                r#"/// You can construct this with the `IntoIterator` implementation of `{camel}`."#
            )?;
            wl!(r#"#[derive(Clone, Debug)]"#)?;
            wl!(r#"pub struct {camel}Iter(pub u32);"#)?;
        }
        if enum_.entries.len() > 0 {
            wl!()?;
            wl!(r#"impl {camel} {{"#)?;
            for (idx, entry) in enum_.entries.iter().enumerate() {
                if idx > 0 {
                    wl!()?;
                }
                let mut needs_newline = false;
                if let Some(summary) = &entry.summary {
                    for line in summary.lines() {
                        wl!(r#"    /// {line}"#)?;
                        needs_newline = true;
                    }
                }
                if let Some(desc) = &entry.description {
                    if needs_newline {
                        wl!(r#"    ///"#)?;
                    }
                    format_description(w, "    ///", desc)?;
                }
                wl!(r#"    #[allow(dead_code)]"#)?;
                wl!(
                    r#"    pub const {}: Self = Self({});"#,
                    format_enum_variant(&entry.name),
                    entry.value
                )?;
            }
            wl!(r#"}}"#)?;
        }
        if enum_.bitfield {
            wl!()?;
            wl!(r#"#[allow(dead_code)]"#)?;
            wl!(r#"impl {camel} {{"#)?;
            wl!(r#"    #[inline]"#)?;
            wl!(r#"    pub const fn empty() -> Self {{"#)?;
            wl!(r#"        Self(0)"#)?;
            wl!(r#"    }}"#)?;
            wl!()?;
            wl!(r#"    #[inline]"#)?;
            wl!(r#"    #[must_use]"#)?;
            wl!(r#"    pub const fn is_empty(self) -> bool {{"#)?;
            wl!(r#"        self.0 == 0"#)?;
            wl!(r#"    }}"#)?;
            wl!()?;
            wl!(r#"    #[inline]"#)?;
            wl!(r#"    #[must_use]"#)?;
            wl!(r#"    pub const fn contains(self, other: Self) -> bool {{"#)?;
            wl!(r#"        self.0 & other.0 == other.0"#)?;
            wl!(r#"    }}"#)?;
            wl!()?;
            wl!(r#"    #[inline]"#)?;
            wl!(r#"    #[must_use]"#)?;
            wl!(r#"    pub const fn intersects(self, other: Self) -> bool {{"#)?;
            wl!(r#"        self.0 & other.0 != 0"#)?;
            wl!(r#"    }}"#)?;
            wl!()?;
            wl!(r#"    #[inline]"#)?;
            wl!(r#"    pub const fn insert(&mut self, other: Self) {{"#)?;
            wl!(r#"        *self = self.union(other);"#)?;
            wl!(r#"    }}"#)?;
            wl!()?;
            wl!(r#"    #[inline]"#)?;
            wl!(r#"    pub const fn remove(&mut self, other: Self) {{"#)?;
            wl!(r#"        *self = self.difference(other);"#)?;
            wl!(r#"    }}"#)?;
            wl!()?;
            wl!(r#"    #[inline]"#)?;
            wl!(r#"    pub const fn toggle(&mut self, other: Self) {{"#)?;
            wl!(r#"        *self = self.symmetric_difference(other);"#)?;
            wl!(r#"    }}"#)?;
            wl!()?;
            wl!(r#"    #[inline]"#)?;
            wl!(r#"    pub const fn set(&mut self, other: Self, value: bool) {{"#)?;
            wl!(r#"        if value {{"#)?;
            wl!(r#"            self.insert(other);"#)?;
            wl!(r#"        }} else {{"#)?;
            wl!(r#"            self.remove(other);"#)?;
            wl!(r#"        }}"#)?;
            wl!(r#"    }}"#)?;
            wl!()?;
            wl!(r#"    #[inline]"#)?;
            wl!(r#"    #[must_use]"#)?;
            wl!(r#"    pub const fn intersection(self, other: Self) -> Self {{"#)?;
            wl!(r#"        Self(self.0 & other.0)"#)?;
            wl!(r#"    }}"#)?;
            wl!()?;
            wl!(r#"    #[inline]"#)?;
            wl!(r#"    #[must_use]"#)?;
            wl!(r#"    pub const fn union(self, other: Self) -> Self {{"#)?;
            wl!(r#"        Self(self.0 | other.0)"#)?;
            wl!(r#"    }}"#)?;
            wl!()?;
            wl!(r#"    #[inline]"#)?;
            wl!(r#"    #[must_use]"#)?;
            wl!(r#"    pub const fn difference(self, other: Self) -> Self {{"#)?;
            wl!(r#"        Self(self.0 & !other.0)"#)?;
            wl!(r#"    }}"#)?;
            wl!()?;
            wl!(r#"    #[inline]"#)?;
            wl!(r#"    #[must_use]"#)?;
            wl!(r#"    pub const fn complement(self) -> Self {{"#)?;
            wl!(r#"        Self(!self.0)"#)?;
            wl!(r#"    }}"#)?;
            wl!()?;
            wl!(r#"    #[inline]"#)?;
            wl!(r#"    #[must_use]"#)?;
            wl!(r#"    pub const fn symmetric_difference(self, other: Self) -> Self {{"#)?;
            wl!(r#"        Self(self.0 ^ other.0)"#)?;
            wl!(r#"    }}"#)?;
            wl!()?;
            wl!(r#"    #[inline]"#)?;
            wl!(r#"    pub const fn all_known() -> Self {{"#)?;
            wl!(r#"        #[allow(clippy::eq_op, clippy::identity_op)]"#)?;
            w!(r#"        Self(0"#)?;
            for entry in &enum_.entries {
                w!(r#" | {}"#, entry.value)?;
            }
            wl!(r#")"#)?;
            wl!(r#"    }}"#)?;
            wl!(r#"}}"#)?;
            wl!()?;
            wl!(r#"impl Iterator for {camel}Iter {{"#)?;
            wl!(r#"    type Item = {camel};"#)?;
            wl!()?;
            wl!(r#"    fn next(&mut self) -> Option<Self::Item> {{"#)?;
            wl!(r#"        if self.0 == 0 {{"#)?;
            wl!(r#"            return None;"#)?;
            wl!(r#"        }}"#)?;
            wl!(r#"        let bit = 1 << self.0.trailing_zeros();"#)?;
            wl!(r#"        self.0 &= !bit;"#)?;
            wl!(r#"        Some({camel}(bit))"#)?;
            wl!(r#"    }}"#)?;
            wl!(r#"}}"#)?;
            wl!()?;
            wl!(r#"impl IntoIterator for {camel} {{"#)?;
            wl!(r#"    type Item = {camel};"#)?;
            wl!(r#"    type IntoIter = {camel}Iter;"#)?;
            wl!()?;
            wl!(r#"    fn into_iter(self) -> Self::IntoIter {{"#)?;
            wl!(r#"        {camel}Iter(self.0)"#)?;
            wl!(r#"    }}"#)?;
            wl!(r#"}}"#)?;
            macro_rules! bitop {
                ($capital:literal, $lower:literal, $op:literal) => {{
                    wl!()?;
                    wl!(r#"impl Bit{} for {camel} {{"#, $capital)?;
                    wl!(r#"    type Output = Self;"#)?;
                    wl!()?;
                    wl!(
                        r#"    fn bit{}(self, rhs: Self) -> Self::Output {{"#,
                        $lower
                    )?;
                    wl!(r#"        self.{}(rhs)"#, $op)?;
                    wl!(r#"    }}"#)?;
                    wl!(r#"}}"#)?;
                    wl!()?;
                    wl!(r#"impl Bit{}Assign for {camel} {{"#, $capital)?;
                    wl!(r#"    fn bit{}_assign(&mut self, rhs: Self) {{"#, $lower)?;
                    wl!(r#"        *self = self.{}(rhs);"#, $op)?;
                    wl!(r#"    }}"#)?;
                    wl!(r#"}}"#)?;
                }};
            }
            bitop!("And", "and", "intersection");
            bitop!("Or", "or", "union");
            bitop!("Xor", "xor", "symmetric_difference");
            wl!()?;
            wl!(r#"impl Sub for {camel} {{"#)?;
            wl!(r#"    type Output = Self;"#)?;
            wl!()?;
            wl!(r#"    fn sub(self, rhs: Self) -> Self::Output {{"#)?;
            wl!(r#"        self.difference(rhs)"#)?;
            wl!(r#"    }}"#)?;
            wl!(r#"}}"#)?;
            wl!()?;
            wl!(r#"impl SubAssign for {camel} {{"#)?;
            wl!(r#"    fn sub_assign(&mut self, rhs: Self) {{"#)?;
            wl!(r#"        *self = self.difference(rhs);"#)?;
            wl!(r#"    }}"#)?;
            wl!(r#"}}"#)?;
            wl!()?;
            wl!(r#"impl Not for {camel} {{"#)?;
            wl!(r#"    type Output = Self;"#)?;
            wl!()?;
            wl!(r#"    fn not(self) -> Self::Output {{"#)?;
            wl!(r#"        self.complement()"#)?;
            wl!(r#"    }}"#)?;
            wl!(r#"}}"#)?;
        }
        wl!()?;
        wl!(r#"impl Debug for {camel} {{"#)?;
        wl!(r#"    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {{"#)?;
        if enum_.bitfield {
            wl!(r#"        let mut v = self.0;"#)?;
            wl!(r#"        let mut first = true;"#)?;
            let mut zero_entry = None;
            for entry in &enum_.entries {
                if entry.value_u32 == 0 {
                    zero_entry = Some(entry);
                    continue;
                }
                wl!(r#"        if v & {} == {} {{"#, entry.value, entry.value)?;
                wl!(r#"            v &= !{};"#, entry.value)?;
                wl!(r#"            if first {{"#)?;
                wl!(r#"                first = false;"#)?;
                wl!(r#"            }} else {{"#)?;
                wl!(r#"                f.write_str(" | ")?;"#)?;
                wl!(r#"            }}"#)?;
                wl!(
                    r#"            f.write_str("{}")?;"#,
                    format_enum_variant(&entry.name)
                )?;
                wl!(r#"        }}"#)?;
            }
            wl!(r#"        if v != 0 {{"#)?;
            wl!(r#"            if first {{"#)?;
            wl!(r#"                first = false;"#)?;
            wl!(r#"            }} else {{"#)?;
            wl!(r#"                f.write_str(" | ")?;"#)?;
            wl!(r#"            }}"#)?;
            wl!(r#"            write!(f, "0x{{v:032x}}")?;"#)?;
            wl!(r#"        }}"#)?;
            wl!(r#"        if first {{"#)?;
            if let Some(entry) = zero_entry {
                wl!(
                    r#"            f.write_str("{}")?;"#,
                    format_enum_variant(&entry.name)
                )?;
            } else {
                wl!(r#"            f.write_str("0")?;"#)?;
            }
            wl!(r#"        }}"#)?;
            wl!(r#"        Ok(())"#)?;
        } else {
            wl!(r#"        let name = match *self {{"#)?;
            for entry in &enum_.entries {
                let upper = format_enum_variant(&entry.name);
                wl!(r#"            Self::{upper} => "{upper}","#)?;
            }
            wl!(r#"            _ => return Debug::fmt(&self.0, f),"#)?;
            wl!(r#"        }};"#)?;
            wl!(r#"        f.write_str(name)"#)?;
        }
        wl!(r#"    }}"#)?;
        wl!(r#"}}"#)?;
    }
    Ok(())
}

fn format_event_handlers(w: &mut impl Write, interface: &Interface) -> io::Result<()> {
    define_w!(w);
    let if_camel = format_camel(&interface.name).to_string();
    wl!(r#"/// Functional event handlers."#)?;
    wl!(r#"pub mod event_handlers {{"#)?;
    wl!(r#"    use super::*;"#)?;
    for event in &interface.events {
        let camel = format_camel(&event.name).to_string();
        wl!()?;
        wl!(r#"    /// Event handler for {} events."#, event.name)?;
        wl!(r#"    pub struct {camel}<F>(F);"#)?;
        wl!(r#"    impl<F> {if_camel}EventHandler for {camel}<F>"#)?;
        wl!(r#"    where"#)?;
        w!(r#"        F: Fn(&{if_camel}Ref"#)?;
        for arg in &event.args {
            w!(", {}", arg_type(interface, arg, false))?;
        }
        wl!(r#"),"#)?;
        wl!(r#"    {{"#)?;
        wl!(r#"        #[inline]"#)?;
        w!(
            r#"        fn {}(&self, _slf: &{if_camel}Ref"#,
            escape_name(&event.name)
        )?;
        for arg in &event.args {
            w!(
                ", {}: {}",
                escape_name(&arg.name),
                arg_type(interface, arg, false)
            )?;
        }
        wl!(r#") {{"#)?;
        w!(r#"            self.0(_slf"#)?;
        for arg in &event.args {
            w!(", {}", escape_name(&arg.name))?;
        }
        wl!(r#")"#)?;
        wl!(r#"        }}"#)?;
        wl!(r#"    }}"#)?;
    }
    wl!()?;
    wl!(r#"    impl {if_camel} {{"#)?;
    for (idx, event) in interface.events.iter().enumerate() {
        if idx > 0 {
            wl!()?;
        }
        let camel = format_camel(&event.name).to_string();
        wl!(
            r#"        /// Creates an event handler for {} events."#,
            event.name
        )?;
        wl!(r#"        ///"#)?;
        wl!(r#"        /// The event handler ignores all other events."#)?;
        wl!(r#"        #[allow(dead_code)]"#)?;
        wl!(r#"        pub fn on_{}<F>(f: F) -> {camel}<F>"#, event.name,)?;
        wl!(r#"        where"#)?;
        w!(r#"            F: Fn(&{if_camel}Ref"#)?;
        for arg in &event.args {
            w!(", {}", arg_type(interface, arg, false))?;
        }
        wl!(r#"),"#)?;
        wl!(r#"        {{"#)?;
        wl!(r#"            {camel}(f)"#)?;
        wl!(r#"        }}"#)?;
    }
    wl!(r#"    }}"#)?;
    wl!(r#"}}"#)?;
    Ok(())
}

fn format_interface_trait_impls(w: &mut impl Write, interface: &Interface) -> io::Result<()> {
    define_w!(w);
    let snake = &interface.name;
    let camel = format_camel(snake).to_string();
    wl!(r#"// SAFETY: {camel} is a transparent wrapper around UntypedOwnedProxy"#)?;
    wl!(r#"unsafe impl UntypedOwnedProxyWrapper for {camel} {{ }}"#)?;
    wl!()?;
    wl!(r#"// SAFETY: - INTERFACE is a valid wl_interface"#)?;
    wl!(r#"//         - The only invariant is that self.proxy has a compatible interface"#)?;
    wl!(r#"unsafe impl OwnedProxy for {camel} {{"#)?;
    wl!(r#"    const INTERFACE: &'static str = "{snake}";"#)?;
    wl!(r#"    const WL_INTERFACE: &'static wl_interface = &INTERFACE;"#)?;
    wl!(
        r#"    const NO_OP_EVENT_HANDLER: Self::NoOpEventHandler = private::EventHandler(private::NoOpEventHandler);"#
    )?;
    wl!(r#"    const MAX_VERSION: u32 = {};"#, interface.version)?;
    wl!()?;
    wl!(r#"    type Borrowed = {camel}Ref;"#)?;
    wl!(r#"    type Api = private::ProxyApi;"#)?;
    wl!(r#"    type NoOpEventHandler = private::EventHandler<private::NoOpEventHandler>;"#)?;
    wl!(r#"}}"#)?;
    wl!()?;
    wl!(r#"// SAFETY: {camel}Ref is a transparent wrapper around UntypedBorrowedProxy"#)?;
    wl!(r#"unsafe impl UntypedBorrowedProxyWrapper for {camel}Ref {{ }}"#,)?;
    wl!()?;
    wl!(r#"// SAFETY: - The only invariant is that self.proxy has a compatible interface"#)?;
    wl!(r#"unsafe impl BorrowedProxy for {camel}Ref {{"#)?;
    wl!(r#"    type Owned = {camel};"#)?;
    wl!(r#"}}"#)?;
    wl!()?;
    wl!(r#"impl Deref for {camel} {{"#)?;
    wl!(r#"    type Target = {camel}Ref;"#)?;
    wl!()?;
    wl!(r#"    fn deref(&self) -> &Self::Target {{"#)?;
    wl!(r#"        proxy::low_level::deref(self)"#)?;
    wl!(r#"    }}"#)?;
    wl!(r#"}}"#)?;
    wl!()?;
    wl!(r#"mod private {{"#)?;
    wl!(r#"    pub struct ProxyApi;"#)?;
    wl!()?;
    wl!(r#"    #[allow(dead_code)]"#)?;
    wl!(r#"    pub struct EventHandler<H>(pub(super) H);"#)?;
    wl!()?;
    wl!(r#"    #[allow(dead_code)]"#)?;
    wl!(r#"    pub struct NoOpEventHandler;"#)?;
    wl!(r#"}}"#)?;
    wl!()?;
    wl!(r#"impl Debug for {camel} {{"#)?;
    wl!(r#"    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {{"#)?;
    wl!(r#"        write!(f, "{snake}#{{}}", self.proxy.id())"#)?;
    wl!(r#"    }}"#)?;
    wl!(r#"}}"#)?;
    wl!()?;
    wl!(r#"impl Debug for {camel}Ref {{"#)?;
    wl!(r#"    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {{"#)?;
    wl!(r#"        write!(f, "{snake}#{{}}", self.proxy.id())"#)?;
    wl!(r#"    }}"#)?;
    wl!(r#"}}"#)?;
    wl!()?;
    wl!(r#"impl PartialEq<{camel}Ref> for {camel} {{"#)?;
    wl!(r#"    fn eq(&self, other: &{camel}Ref) -> bool {{"#)?;
    wl!(r#"        self.proxy == other.proxy"#)?;
    wl!(r#"    }}"#)?;
    wl!(r#"}}"#)?;
    wl!()?;
    wl!(r#"impl PartialEq<{camel}> for {camel}Ref {{"#)?;
    wl!(r#"    fn eq(&self, other: &{camel}) -> bool {{"#)?;
    wl!(r#"        self.proxy == other.proxy"#)?;
    wl!(r#"    }}"#)?;
    wl!(r#"}}"#)?;
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
