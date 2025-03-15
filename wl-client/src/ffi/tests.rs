use {
    crate::ffi::{interface_compatible, wl_interface, wl_message},
    std::ptr,
};

macro_rules! interface {
    ($name:ident, $interface:expr) => {
        static $name: &'static wl_interface = &$interface;
    };
}

macro_rules! messages {
    ($($m1:expr $(, $m2:expr)* $(,)?)?) => {{
        static MSGS: &'static [wl_message] = &[
            $(
                $m1,
                $($m2,)*
            )?
        ];
        MSGS.as_ptr()
    }};
}

macro_rules! types {
    ($($m1:expr $(, $m2:expr)* $(,)?)?) => {{
        static TYPES: &'static [Option<&'static wl_interface>] = &[
            $(
                $m1,
                $($m2,)*
            )?
        ];
        TYPES.as_ptr().cast()
    }};
}

#[test]
fn compatible_same_ptr() {
    let p1 = wl_interface {
        name: c"".as_ptr(),
        version: 0,
        method_count: 0,
        methods: [].as_ptr(),
        event_count: 0,
        events: [].as_ptr(),
    };
    unsafe {
        assert!(interface_compatible(&p1, &p1));
    }
}

#[test]
fn compatible_different_method_count() {
    let p1 = wl_interface {
        name: c"".as_ptr(),
        version: 0,
        method_count: 0,
        methods: [].as_ptr(),
        event_count: 0,
        events: [].as_ptr(),
    };
    let p2 = wl_interface {
        name: c"".as_ptr(),
        version: 0,
        method_count: 1,
        methods: messages![wl_message {
            name: c"".as_ptr(),
            signature: c"".as_ptr(),
            types: ptr::null(),
        }],
        event_count: 0,
        events: [].as_ptr(),
    };
    unsafe {
        assert!(!interface_compatible(&p1, &p2));
        assert!(!interface_compatible(&p2, &p1));
    }
}

#[test]
fn compatible_different_event_count() {
    let p1 = wl_interface {
        name: c"".as_ptr(),
        version: 0,
        method_count: 0,
        methods: [].as_ptr(),
        event_count: 0,
        events: [].as_ptr(),
    };
    let p2 = wl_interface {
        name: c"".as_ptr(),
        version: 0,
        method_count: 0,
        methods: ptr::null(),
        event_count: 1,
        events: messages![wl_message {
            name: c"".as_ptr(),
            signature: c"".as_ptr(),
            types: ptr::null(),
        }],
    };
    unsafe {
        assert!(!interface_compatible(&p1, &p2));
        assert!(!interface_compatible(&p2, &p1));
    }
}

#[test]
fn compatible_different_signature() {
    let p1 = wl_interface {
        name: c"".as_ptr(),
        version: 0,
        method_count: 2,
        methods: messages![
            wl_message {
                name: c"".as_ptr(),
                signature: c"sss".as_ptr(),
                types: ptr::null(),
            },
            wl_message {
                name: c"".as_ptr(),
                signature: c"ssi".as_ptr(),
                types: ptr::null(),
            },
        ],
        event_count: 0,
        events: [].as_ptr(),
    };
    let p2 = wl_interface {
        name: c"".as_ptr(),
        version: 0,
        method_count: 2,
        methods: messages![
            wl_message {
                name: c"".as_ptr(),
                signature: c"sss".as_ptr(),
                types: ptr::null(),
            },
            wl_message {
                name: c"".as_ptr(),
                signature: c"sss".as_ptr(),
                types: ptr::null(),
            },
        ],
        event_count: 0,
        events: [].as_ptr(),
    };
    unsafe {
        assert!(!interface_compatible(&p1, &p2));
        assert!(!interface_compatible(&p2, &p1));
    }
}

#[test]
fn compatible_same_type() {
    interface!(
        P0_0,
        wl_interface {
            name: c"".as_ptr(),
            version: 0,
            method_count: 0,
            methods: messages![],
            event_count: 0,
            events: messages![],
        }
    );
    interface!(
        P0_1,
        wl_interface {
            name: c"".as_ptr(),
            version: 0,
            method_count: 0,
            methods: messages![],
            event_count: 0,
            events: messages![],
        }
    );
    interface!(
        P1,
        wl_interface {
            name: c"".as_ptr(),
            version: 0,
            method_count: 2,
            methods: messages![
                wl_message {
                    name: c"".as_ptr(),
                    signature: c"sss".as_ptr(),
                    types: ptr::null(),
                },
                wl_message {
                    name: c"".as_ptr(),
                    signature: c"o".as_ptr(),
                    types: types![Some(P0_0)],
                },
            ],
            event_count: 0,
            events: [].as_ptr(),
        }
    );
    interface!(
        P2,
        wl_interface {
            name: c"".as_ptr(),
            version: 0,
            method_count: 2,
            methods: messages![
                wl_message {
                    name: c"".as_ptr(),
                    signature: c"sss".as_ptr(),
                    types: ptr::null(),
                },
                wl_message {
                    name: c"".as_ptr(),
                    signature: c"o".as_ptr(),
                    types: types![Some(P0_1)],
                },
            ],
            event_count: 0,
            events: [].as_ptr(),
        }
    );
    unsafe {
        assert!(interface_compatible(&P1, &P2));
        assert!(interface_compatible(&P2, &P1));
    }
}

#[test]
fn compatible_different_type() {
    interface!(
        P0_0,
        wl_interface {
            name: c"".as_ptr(),
            version: 0,
            method_count: 0,
            methods: messages![],
            event_count: 0,
            events: messages![],
        }
    );
    interface!(
        P0_1,
        wl_interface {
            name: c"".as_ptr(),
            version: 0,
            method_count: 1,
            methods: messages![wl_message {
                name: c"".as_ptr(),
                signature: c"".as_ptr(),
                types: ptr::null(),
            },],
            event_count: 0,
            events: messages![],
        }
    );
    interface!(
        P1,
        wl_interface {
            name: c"".as_ptr(),
            version: 0,
            method_count: 0,
            methods: [].as_ptr(),
            event_count: 2,
            events: messages![
                wl_message {
                    name: c"".as_ptr(),
                    signature: c"sss".as_ptr(),
                    types: ptr::null(),
                },
                wl_message {
                    name: c"".as_ptr(),
                    signature: c"o".as_ptr(),
                    types: types![Some(P0_0)],
                },
            ],
        }
    );
    interface!(
        P2,
        wl_interface {
            name: c"".as_ptr(),
            version: 0,
            method_count: 0,
            methods: [].as_ptr(),
            event_count: 2,
            events: messages![
                wl_message {
                    name: c"".as_ptr(),
                    signature: c"sss".as_ptr(),
                    types: ptr::null(),
                },
                wl_message {
                    name: c"".as_ptr(),
                    signature: c"o".as_ptr(),
                    types: types![Some(P0_1)],
                },
            ],
        }
    );
    unsafe {
        assert!(!interface_compatible(&P1, &P2));
        assert!(!interface_compatible(&P2, &P1));
    }
}

#[test]
fn compatible_different_null_type() {
    interface!(
        P0_0,
        wl_interface {
            name: c"".as_ptr(),
            version: 0,
            method_count: 0,
            methods: messages![],
            event_count: 0,
            events: messages![],
        }
    );
    interface!(
        P1,
        wl_interface {
            name: c"".as_ptr(),
            version: 0,
            method_count: 0,
            methods: [].as_ptr(),
            event_count: 2,
            events: messages![
                wl_message {
                    name: c"".as_ptr(),
                    signature: c"sss".as_ptr(),
                    types: ptr::null(),
                },
                wl_message {
                    name: c"".as_ptr(),
                    signature: c"o".as_ptr(),
                    types: types![Some(P0_0)],
                },
            ],
        }
    );
    interface!(
        P2,
        wl_interface {
            name: c"".as_ptr(),
            version: 0,
            method_count: 0,
            methods: [].as_ptr(),
            event_count: 2,
            events: messages![
                wl_message {
                    name: c"".as_ptr(),
                    signature: c"sss".as_ptr(),
                    types: ptr::null(),
                },
                wl_message {
                    name: c"".as_ptr(),
                    signature: c"o".as_ptr(),
                    types: types![None],
                },
            ],
        }
    );
    unsafe {
        assert!(!interface_compatible(&P1, &P2));
        assert!(!interface_compatible(&P2, &P1));
    }
}

#[test]
fn compatible_same_null_type() {
    interface!(
        P1,
        wl_interface {
            name: c"".as_ptr(),
            version: 0,
            method_count: 0,
            methods: [].as_ptr(),
            event_count: 2,
            events: messages![
                wl_message {
                    name: c"".as_ptr(),
                    signature: c"sss".as_ptr(),
                    types: ptr::null(),
                },
                wl_message {
                    name: c"".as_ptr(),
                    signature: c"o".as_ptr(),
                    types: types![None],
                },
            ],
        }
    );
    interface!(
        P2,
        wl_interface {
            name: c"".as_ptr(),
            version: 0,
            method_count: 0,
            methods: [].as_ptr(),
            event_count: 2,
            events: messages![
                wl_message {
                    name: c"".as_ptr(),
                    signature: c"sss".as_ptr(),
                    types: ptr::null(),
                },
                wl_message {
                    name: c"".as_ptr(),
                    signature: c"o".as_ptr(),
                    types: types![None],
                },
            ],
        }
    );
    unsafe {
        assert!(interface_compatible(&P1, &P2));
        assert!(interface_compatible(&P2, &P1));
    }
}

#[test]
fn compatible_questionmark_same_type() {
    interface!(
        P0_0,
        wl_interface {
            name: c"".as_ptr(),
            version: 0,
            method_count: 0,
            methods: messages![],
            event_count: 0,
            events: messages![],
        }
    );
    interface!(
        P1,
        wl_interface {
            name: c"".as_ptr(),
            version: 0,
            method_count: 0,
            methods: [].as_ptr(),
            event_count: 1,
            events: messages![wl_message {
                name: c"".as_ptr(),
                signature: c"?o".as_ptr(),
                types: types![Some(P0_0), Some(P0_0)],
            },],
        }
    );
    interface!(
        P2,
        wl_interface {
            name: c"".as_ptr(),
            version: 0,
            method_count: 0,
            methods: [].as_ptr(),
            event_count: 1,
            events: messages![wl_message {
                name: c"".as_ptr(),
                signature: c"?o".as_ptr(),
                types: types![Some(P0_0), None],
            },],
        }
    );
    unsafe {
        assert!(interface_compatible(&P1, &P2));
        assert!(interface_compatible(&P2, &P1));
    }
}

#[test]
fn compatible_questionmark_different_type() {
    interface!(
        P0_0,
        wl_interface {
            name: c"".as_ptr(),
            version: 0,
            method_count: 0,
            methods: messages![],
            event_count: 0,
            events: messages![],
        }
    );
    interface!(
        P1,
        wl_interface {
            name: c"".as_ptr(),
            version: 0,
            method_count: 0,
            methods: [].as_ptr(),
            event_count: 1,
            events: messages![wl_message {
                name: c"".as_ptr(),
                signature: c"?o".as_ptr(),
                types: types![None, Some(P0_0)],
            },],
        }
    );
    interface!(
        P2,
        wl_interface {
            name: c"".as_ptr(),
            version: 0,
            method_count: 0,
            methods: [].as_ptr(),
            event_count: 1,
            events: messages![wl_message {
                name: c"".as_ptr(),
                signature: c"?o".as_ptr(),
                types: types![Some(P0_0), Some(P0_0)],
            },],
        }
    );
    unsafe {
        assert!(!interface_compatible(&P1, &P2));
        assert!(!interface_compatible(&P2, &P1));
    }
}

#[test]
fn compatible_multiple_types() {
    interface!(
        P0_0,
        wl_interface {
            name: c"".as_ptr(),
            version: 0,
            method_count: 0,
            methods: messages![],
            event_count: 0,
            events: messages![],
        }
    );
    interface!(
        P1,
        wl_interface {
            name: c"".as_ptr(),
            version: 0,
            method_count: 0,
            methods: [].as_ptr(),
            event_count: 1,
            events: messages![wl_message {
                name: c"".as_ptr(),
                signature: c"on".as_ptr(),
                types: types![Some(P0_0), Some(P0_0)],
            },],
        }
    );
    interface!(
        P2,
        wl_interface {
            name: c"".as_ptr(),
            version: 0,
            method_count: 0,
            methods: [].as_ptr(),
            event_count: 1,
            events: messages![wl_message {
                name: c"".as_ptr(),
                signature: c"on".as_ptr(),
                types: types![Some(P0_0), Some(P0_0)],
            },],
        }
    );
    unsafe {
        assert!(interface_compatible(&P1, &P2));
        assert!(interface_compatible(&P2, &P1));
    }
}

#[test]
fn compatible_multiple_different_types() {
    interface!(
        P0_0,
        wl_interface {
            name: c"".as_ptr(),
            version: 0,
            method_count: 0,
            methods: messages![],
            event_count: 0,
            events: messages![],
        }
    );
    interface!(
        P1,
        wl_interface {
            name: c"".as_ptr(),
            version: 0,
            method_count: 0,
            methods: [].as_ptr(),
            event_count: 1,
            events: messages![wl_message {
                name: c"".as_ptr(),
                signature: c"on".as_ptr(),
                types: types![Some(P0_0), Some(P0_0)],
            },],
        }
    );
    interface!(
        P2,
        wl_interface {
            name: c"".as_ptr(),
            version: 0,
            method_count: 0,
            methods: [].as_ptr(),
            event_count: 1,
            events: messages![wl_message {
                name: c"".as_ptr(),
                signature: c"on".as_ptr(),
                types: types![Some(P0_0), None],
            },],
        }
    );
    unsafe {
        assert!(!interface_compatible(&P1, &P2));
        assert!(!interface_compatible(&P2, &P1));
    }
}

#[test]
fn compatible_multiple_different_types_mixed() {
    interface!(
        P0_0,
        wl_interface {
            name: c"".as_ptr(),
            version: 0,
            method_count: 0,
            methods: messages![],
            event_count: 0,
            events: messages![],
        }
    );
    interface!(
        P1,
        wl_interface {
            name: c"".as_ptr(),
            version: 0,
            method_count: 0,
            methods: [].as_ptr(),
            event_count: 1,
            events: messages![wl_message {
                name: c"".as_ptr(),
                signature: c"osn".as_ptr(),
                types: types![Some(P0_0), None, Some(P0_0)],
            },],
        }
    );
    interface!(
        P2,
        wl_interface {
            name: c"".as_ptr(),
            version: 0,
            method_count: 0,
            methods: [].as_ptr(),
            event_count: 1,
            events: messages![wl_message {
                name: c"".as_ptr(),
                signature: c"osn".as_ptr(),
                types: types![Some(P0_0), None, None],
            },],
        }
    );
    unsafe {
        assert!(!interface_compatible(&P1, &P2));
        assert!(!interface_compatible(&P2, &P1));
    }
}

#[test]
fn compatible_multiple_same_types_mixed() {
    interface!(
        P0_0,
        wl_interface {
            name: c"".as_ptr(),
            version: 0,
            method_count: 0,
            methods: messages![],
            event_count: 0,
            events: messages![],
        }
    );
    interface!(
        P1,
        wl_interface {
            name: c"".as_ptr(),
            version: 0,
            method_count: 0,
            methods: [].as_ptr(),
            event_count: 1,
            events: messages![wl_message {
                name: c"".as_ptr(),
                signature: c"osn".as_ptr(),
                types: types![Some(P0_0), None, Some(P0_0)],
            },],
        }
    );
    interface!(
        P2,
        wl_interface {
            name: c"".as_ptr(),
            version: 0,
            method_count: 0,
            methods: [].as_ptr(),
            event_count: 1,
            events: messages![wl_message {
                name: c"".as_ptr(),
                signature: c"osn".as_ptr(),
                types: types![Some(P0_0), None, Some(P0_0)],
            },],
        }
    );
    unsafe {
        assert!(interface_compatible(&P1, &P2));
        assert!(interface_compatible(&P2, &P1));
    }
}
