use {
    crate::common::{
        protocols::wayland::{
            wl_display::WlDisplay,
            wl_keyboard::{
                WlKeyboard, WlKeyboardEventHandler, WlKeyboardKeyState, WlKeyboardKeymapFormat,
                WlKeyboardRef,
            },
            wl_registry::{WlRegistry, WlRegistryEventHandler, WlRegistryRef},
            wl_seat::{WlSeat, WlSeatCapability, WlSeatEventHandler, WlSeatRef},
            wl_surface::WlSurfaceRef,
        },
        singletons::get_singletons,
    },
    common::simple_window,
    std::{cell::RefCell, collections::HashMap, os::fd::OwnedFd, rc::Rc},
    wl_client::{
        Libwayland,
        proxy::{self, OwnedProxy},
    },
};

#[path = "../common/mod.rs"]
mod common;

fn main() {
    let lib = Libwayland::open().unwrap();
    let con = lib.connect_to_default_display().unwrap();
    let queue = con.create_local_queue(c"keyboard-events");
    let singletons = get_singletons(&queue.display());
    let simple_window = simple_window::prepare(singletons);

    let wl_registry = queue.display::<WlDisplay>().get_registry();
    proxy::set_event_handler_local(
        &wl_registry,
        RegistryEventHandler {
            wl_registry: wl_registry.clone(),
            seats: Default::default(),
        },
    );

    while !simple_window.exit.get() {
        queue.dispatch_blocking().unwrap();
    }
}

struct RegistryEventHandler {
    wl_registry: WlRegistry,
    seats: RefCell<HashMap<u32, Rc<Seat>>>,
}

struct Seat {
    wl_seat: WlSeat,
    wl_keyboard: RefCell<Option<WlKeyboard>>,
}

#[derive(Clone)]
struct SeatEventHandler(Rc<Seat>);

impl WlRegistryEventHandler for RegistryEventHandler {
    fn global(&self, _slf: &WlRegistryRef, name: u32, interface: &str, version: u32) {
        if interface == WlSeat::INTERFACE {
            let wl_seat = self.wl_registry.bind::<WlSeat>(name, version.min(5));
            let seat = Rc::new(Seat {
                wl_seat: wl_seat.clone(),
                wl_keyboard: Default::default(),
            });
            self.seats.borrow_mut().insert(name, seat.clone());
            proxy::set_event_handler_local(&wl_seat, SeatEventHandler(seat));
        }
    }

    fn global_remove(&self, _slf: &WlRegistryRef, name: u32) {
        if let Some(seat) = self.seats.borrow_mut().remove(&name) {
            if let Some(kb) = seat.wl_keyboard.take() {
                if proxy::version(&*kb) >= WlKeyboard::REQ__RELEASE__SINCE {
                    kb.release();
                } else {
                    proxy::destroy(&kb);
                }
            }
            if proxy::version(&*seat.wl_seat) >= WlSeat::REQ__RELEASE__SINCE {
                seat.wl_seat.release();
            } else {
                proxy::destroy(&seat.wl_seat);
            }
        }
    }
}

impl WlSeatEventHandler for SeatEventHandler {
    fn capabilities(&self, _slf: &WlSeatRef, capabilities: WlSeatCapability) {
        let kb = &mut *self.0.wl_keyboard.borrow_mut();
        if capabilities.contains(WlSeatCapability::KEYBOARD) {
            if kb.is_none() {
                let wl_keyboard = self.0.wl_seat.get_keyboard();
                proxy::set_event_handler_local(&wl_keyboard, self.clone());
                *kb = Some(wl_keyboard);
            }
        } else {
            if let Some(kb) = kb.take() {
                if proxy::version(&*kb) >= WlKeyboard::REQ__RELEASE__SINCE {
                    kb.release();
                } else {
                    proxy::destroy(&kb);
                }
            }
        }
    }
}

impl WlKeyboardEventHandler for SeatEventHandler {
    fn keymap(
        &self,
        _slf: &WlKeyboardRef,
        format: WlKeyboardKeymapFormat,
        _fd: OwnedFd,
        size: u32,
    ) {
        println!("keymap format: {format:?}");
        println!("keymap size: {size:?}");
    }

    fn enter(
        &self,
        _slf: &WlKeyboardRef,
        _serial: u32,
        surface: Option<&WlSurfaceRef>,
        keys: &[u8],
    ) {
        println!("enter surface {:?}", surface.map(proxy::id));
        println!("keys {keys:?}");
    }

    fn leave(&self, _slf: &WlKeyboardRef, _serial: u32, surface: Option<&WlSurfaceRef>) {
        println!("leave surface {:?}", surface.map(proxy::id));
    }

    fn key(
        &self,
        _slf: &WlKeyboardRef,
        _serial: u32,
        _time: u32,
        key: u32,
        state: WlKeyboardKeyState,
    ) {
        println!("key {key:?} {state:?}");
    }

    fn modifiers(
        &self,
        _slf: &WlKeyboardRef,
        _serial: u32,
        mods_depressed: u32,
        mods_latched: u32,
        mods_locked: u32,
        group: u32,
    ) {
        println!("modifiers {mods_depressed:x?}, {mods_latched:x?}, {mods_locked:x?}, {group}");
    }

    fn repeat_info(&self, _slf: &WlKeyboardRef, rate: i32, delay: i32) {
        println!("repeat info {rate:?} {delay:?}");
    }
}
