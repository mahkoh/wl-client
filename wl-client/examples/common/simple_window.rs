use {
    crate::common::{
        protocols::{
            cursor_shape_v1::{
                wp_cursor_shape_device_v1::{WpCursorShapeDeviceV1, WpCursorShapeDeviceV1Shape},
                wp_cursor_shape_manager_v1::WpCursorShapeManagerV1,
            },
            viewporter::{wp_viewport::WpViewport, wp_viewporter::WpViewporter},
            wayland::{
                wl_buffer::WlBuffer,
                wl_compositor::WlCompositor,
                wl_display::WlDisplay,
                wl_pointer::{WlPointer, WlPointerEventHandler, WlPointerRef},
                wl_registry::{WlRegistry, WlRegistryEventHandler, WlRegistryRef},
                wl_seat::{WlSeat, WlSeatCapability, WlSeatEventHandler, WlSeatRef},
                wl_shm::{WlShm, WlShmFormat},
                wl_surface::{WlSurface, WlSurfaceRef},
            },
            xdg_shell::{
                xdg_surface::{XdgSurface, XdgSurfaceEventHandler, XdgSurfaceRef},
                xdg_toplevel::{XdgToplevel, XdgToplevelEventHandler, XdgToplevelRef},
                xdg_wm_base::{XdgWmBase, XdgWmBaseEventHandler, XdgWmBaseRef},
            },
        },
        singletons::Singletons,
    },
    std::{
        cell::{Cell, RefCell},
        collections::HashMap,
        io::Write,
        os::fd::AsFd,
        rc::Rc,
    },
    tempfile::tempfile,
    wl_client::{
        Fixed, Libwayland,
        proxy::{self, OwnedProxy},
    },
};

pub struct SimpleWindow {
    pub exit: Rc<Cell<bool>>,
    wl_compositor: WlCompositor,
    wl_shm: WlShm,
    xdg_wm_base: XdgWmBase,
    wp_viewporter: WpViewporter,
    wl_surface: WlSurface,
    wp_viewport: WpViewport,
    xdg_surface: XdgSurface,
    xdg_toplevel: XdgToplevel,
}

pub fn prepare(singletons: Singletons) -> SimpleWindow {
    let lib = Libwayland::open().unwrap();
    let con = lib.connect_to_default_display().unwrap();
    let queue = con.create_local_queue(c"simple-window");
    let display: WlDisplay = queue.display();

    // We have a hard dependency on these globals.
    let wl_compositor = singletons.get::<WlCompositor>(1, 1);
    let wl_shm = singletons.get::<WlShm>(1, 1);
    let xdg_wm_base = singletons.get::<XdgWmBase>(1, 1);
    let wp_viewporter = singletons.get::<WpViewporter>(1, 1);

    proxy::set_event_handler_local(&xdg_wm_base, XdgWmPingPong);

    // Create the window.
    let buffer = {
        let mut tempfile = tempfile().unwrap();
        tempfile.write_all(&[0, 0, 0, 255]).unwrap();
        let pool = wl_shm.create_pool(tempfile.as_fd(), 4);
        let buffer = pool.create_buffer(0, 1, 1, 4, WlShmFormat::ARGB8888);
        pool.destroy();
        buffer
    };
    let wl_surface = wl_compositor.create_surface();
    let wp_viewport = wp_viewporter.get_viewport(&wl_surface);
    let xdg_surface = xdg_wm_base.get_xdg_surface(&wl_surface);
    let xdg_toplevel = xdg_surface.get_toplevel();
    xdg_toplevel.set_title("simple-window");
    wl_surface.commit();

    let exit = Rc::new(Cell::new(false));

    let event_handler = WindowEventHandler {
        state: Rc::new(WindowState {
            surface: wl_surface.clone(),
            buffer,
            viewport: wp_viewport.clone(),
            exit: exit.clone(),
            attached_buffer: Cell::new(false),
            pending_size: Cell::new((DEFAULT_WIDTH, DEFAULT_HEIGHT)),
            current_size: Cell::new((0, 0)),
        }),
    };
    proxy::set_event_handler_local(&xdg_surface, event_handler.clone());
    proxy::set_event_handler_local(&xdg_toplevel, event_handler.clone());

    // If we have the cursor shape manager, create a new registry to handle seats
    // and give the pointers the default shape when they enter our window.
    if let Some(cursor_shape) = singletons.get_opt::<WpCursorShapeManagerV1>(1, 1) {
        let registry = display.get_registry();
        proxy::set_event_handler_local(
            &registry,
            SeatRegistryHandler {
                registry: registry.clone(),
                wp_cursor_shape_manager_v1: cursor_shape,
                seats: Default::default(),
            },
        );
    }

    SimpleWindow {
        exit,
        wl_compositor,
        wl_shm,
        xdg_wm_base,
        wp_viewporter,
        wl_surface,
        wp_viewport,
        xdg_surface,
        xdg_toplevel,
    }
}

struct SeatRegistryHandler {
    registry: WlRegistry,
    wp_cursor_shape_manager_v1: WpCursorShapeManagerV1,
    seats: RefCell<HashMap<u32, Rc<RefCell<SeatData>>>>,
}

impl WlRegistryEventHandler for SeatRegistryHandler {
    fn global(&self, _slf: &WlRegistryRef, name: u32, interface: &str, version: u32) {
        if interface != WlSeat::INTERFACE {
            return;
        }
        let seat: WlSeat = self.registry.bind(name, version.min(5));
        let data = Rc::new(RefCell::new(SeatData {
            seat: seat.clone(),
            pointer: None,
            shape: None,
        }));
        proxy::set_event_handler_local(
            &seat,
            SeatEventHandler {
                wp_cursor_shape_manager_v1: self.wp_cursor_shape_manager_v1.clone(),
                data: data.clone(),
            },
        );
        self.seats.borrow_mut().insert(name, data);
    }

    fn global_remove(&self, _slf: &WlRegistryRef, name: u32) {
        let seats = &mut *self.seats.borrow_mut();
        let Some(seat) = seats.remove(&name) else {
            return;
        };
        let data = &mut *seat.borrow_mut();
        data.destroy_pointer();
        if proxy::version(&*data.seat) >= WlSeat::REQ__RELEASE__SINCE {
            data.seat.release();
        } else {
            proxy::destroy(&data.seat);
        }
    }
}

struct SeatEventHandler {
    wp_cursor_shape_manager_v1: WpCursorShapeManagerV1,
    data: Rc<RefCell<SeatData>>,
}

struct SeatData {
    seat: WlSeat,
    pointer: Option<WlPointer>,
    shape: Option<WpCursorShapeDeviceV1>,
}

impl SeatData {
    fn destroy_pointer(&mut self) {
        if let Some(shape) = self.shape.take() {
            shape.destroy();
        }
        if let Some(pointer) = self.pointer.take() {
            if proxy::version(&*pointer) >= WlPointer::REQ__RELEASE__SINCE {
                pointer.release();
            } else {
                proxy::destroy(&pointer);
            }
        }
    }
}

impl WlSeatEventHandler for SeatEventHandler {
    fn capabilities(&self, _slf: &WlSeatRef, capabilities: WlSeatCapability) {
        let data = &mut *self.data.borrow_mut();
        if capabilities.contains(WlSeatCapability::POINTER) {
            if data.pointer.is_some() {
                return;
            }
            let pointer = data.seat.get_pointer();
            let shape = self.wp_cursor_shape_manager_v1.get_pointer(&pointer);
            proxy::set_event_handler_local(
                &pointer,
                PointerEventHandler {
                    shape: shape.clone(),
                },
            );
            data.pointer = Some(pointer);
            data.shape = Some(shape);
        } else {
            data.destroy_pointer();
        }
    }
}

#[derive(Clone)]
struct PointerEventHandler {
    shape: WpCursorShapeDeviceV1,
}

impl WlPointerEventHandler for PointerEventHandler {
    fn enter(
        &self,
        _slf: &WlPointerRef,
        serial: u32,
        _surface: Option<&WlSurfaceRef>,
        _surface_x: Fixed,
        _surface_y: Fixed,
    ) {
        self.shape
            .set_shape(serial, WpCursorShapeDeviceV1Shape::DEFAULT);
    }
}

#[derive(Clone)]
struct WindowEventHandler {
    state: Rc<WindowState>,
}

struct WindowState {
    surface: WlSurface,
    buffer: WlBuffer,
    viewport: WpViewport,
    exit: Rc<Cell<bool>>,
    attached_buffer: Cell<bool>,
    pending_size: Cell<(i32, i32)>,
    current_size: Cell<(i32, i32)>,
}

const DEFAULT_WIDTH: i32 = 800;
const DEFAULT_HEIGHT: i32 = 600;

impl XdgToplevelEventHandler for WindowEventHandler {
    fn configure(&self, _slf: &XdgToplevelRef, mut width: i32, mut height: i32, _states: &[u8]) {
        if width == 0 {
            width = DEFAULT_WIDTH;
        }
        if height == 0 {
            height = DEFAULT_HEIGHT;
        }
        self.state.pending_size.set((width, height));
    }

    fn close(&self, _slf: &XdgToplevelRef) {
        self.state.exit.set(true);
    }
}

impl XdgSurfaceEventHandler for WindowEventHandler {
    fn configure(&self, slf: &XdgSurfaceRef, serial: u32) {
        slf.ack_configure(serial);
        let state = &*self.state;
        let mut need_commit = false;
        if state.current_size.get() != state.pending_size.get() {
            let (width, height) = state.pending_size.get();
            state.current_size.set((width, height));
            state.viewport.set_destination(width, height);
            need_commit = true;
        }
        if !state.attached_buffer.get() {
            state.attached_buffer.set(true);
            state.surface.attach(Some(&state.buffer), 0, 0);
            need_commit = true;
        }
        if need_commit {
            state.surface.commit();
        }
    }
}

struct XdgWmPingPong;

impl XdgWmBaseEventHandler for XdgWmPingPong {
    fn ping(&self, slf: &XdgWmBaseRef, serial: u32) {
        slf.pong(serial);
    }
}
