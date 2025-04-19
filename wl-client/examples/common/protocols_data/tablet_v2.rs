//! Wayland protocol for graphics tablets
//!
//! This description provides a high-level overview of the interplay between
//! the interfaces defined this protocol. For details, see the protocol
//! specification.
//!
//! More than one tablet may exist, and device-specifics matter. Tablets are
//! not represented by a single virtual device like wl_pointer. A client
//! binds to the tablet manager object which is just a proxy object. From
//! that, the client requests wp_tablet_manager.get_tablet_seat(wl_seat)
//! and that returns the actual interface that has all the tablets. With
//! this indirection, we can avoid merging wp_tablet into the actual Wayland
//! protocol, a long-term benefit.
//!
//! The wp_tablet_seat sends a "tablet added" event for each tablet
//! connected. That event is followed by descriptive events about the
//! hardware; currently that includes events for name, vid/pid and
//! a wp_tablet.path event that describes a local path. This path can be
//! used to uniquely identify a tablet or get more information through
//! libwacom. Emulated or nested tablets can skip any of those, e.g. a
//! virtual tablet may not have a vid/pid. The sequence of descriptive
//! events is terminated by a wp_tablet.done event to signal that a client
//! may now finalize any initialization for that tablet.
//!
//! Events from tablets require a tool in proximity. Tools are also managed
//! by the tablet seat; a "tool added" event is sent whenever a tool is new
//! to the compositor. That event is followed by a number of descriptive
//! events about the hardware; currently that includes capabilities,
//! hardware id and serial number, and tool type. Similar to the tablet
//! interface, a wp_tablet_tool.done event is sent to terminate that initial
//! sequence.
//!
//! Any event from a tool happens on the wp_tablet_tool interface. When the
//! tool gets into proximity of the tablet, a proximity_in event is sent on
//! the wp_tablet_tool interface, listing the tablet and the surface. That
//! event is followed by a motion event with the coordinates. After that,
//! it's the usual motion, axis, button, etc. events. The protocol's
//! serialisation means events are grouped by wp_tablet_tool.frame events.
//!
//! Two special events (that don't exist in X) are down and up. They signal
//! "tip touching the surface". For tablets without real proximity
//! detection, the sequence is: proximity_in, motion, down, frame.
//!
//! When the tool leaves proximity, a proximity_out event is sent. If any
//! button is still down, a button release event is sent before this
//! proximity event. These button events are sent in the same frame as the
//! proximity event to signal to the client that the buttons were held when
//! the tool left proximity.
//!
//! If the tool moves out of the surface but stays in proximity (i.e.
//! between windows), compositor-specific grab policies apply. This usually
//! means that the proximity-out is delayed until all buttons are released.
//!
//! Moving a tool physically from one tablet to the other has no real effect
//! on the protocol, since we already have the tool object from the "tool
//! added" event. All the information is already there and the proximity
//! events on both tablets are all a client needs to reconstruct what
//! happened.
//!
//! Some extra axes are normalized, i.e. the client knows the range as
//! specified in the protocol (e.g. [0, 65535]), the granularity however is
//! unknown. The current normalized axes are pressure, distance, and slider.
//!
//! Other extra axes are in physical units as specified in the protocol.
//! The current extra axes with physical units are tilt, rotation and
//! wheel rotation.
//!
//! Since tablets work independently of the pointer controlled by the mouse,
//! the focus handling is independent too and controlled by proximity.
//! The wp_tablet_tool.set_cursor request sets a tool-specific cursor.
//! This cursor surface may be the same as the mouse cursor, and it may be
//! the same across tools but it is possible to be more fine-grained. For
//! example, a client may set different cursors for the pen and eraser.
//!
//! Tools are generally independent of tablets and it is
//! compositor-specific policy when a tool can be removed. Common approaches
//! will likely include some form of removing a tool when all tablets the
//! tool was used on are removed.

#![allow(clippy::tabs_in_doc_comments)]
#![allow(clippy::doc_lazy_continuation)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::manual_map)]
#![allow(clippy::module_inception)]
#![allow(unused_imports)]
#![allow(rustdoc::broken_intra_doc_links)]
#![allow(rustdoc::bare_urls)]
#![allow(rustdoc::invalid_rust_codeblocks)]

pub mod zwp_tablet_manager_v2;
pub mod zwp_tablet_pad_group_v2;
pub mod zwp_tablet_pad_ring_v2;
pub mod zwp_tablet_pad_strip_v2;
pub mod zwp_tablet_pad_v2;
pub mod zwp_tablet_seat_v2;
pub mod zwp_tablet_tool_v2;
pub mod zwp_tablet_v2;
