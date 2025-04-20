pub mod cursor_shape_v1;
pub mod tablet_v2;
pub mod viewporter;
pub mod wayland;
pub mod xdg_shell;

#[allow(unused_imports)]
mod all_types {
    pub(super) use super::{
        cursor_shape_v1::{
            wp_cursor_shape_device_v1::{
                WpCursorShapeDeviceV1, WpCursorShapeDeviceV1Error, WpCursorShapeDeviceV1Ref,
                WpCursorShapeDeviceV1Shape,
            },
            wp_cursor_shape_manager_v1::{WpCursorShapeManagerV1, WpCursorShapeManagerV1Ref},
        },
        tablet_v2::{
            zwp_tablet_manager_v2::{ZwpTabletManagerV2, ZwpTabletManagerV2Ref},
            zwp_tablet_pad_group_v2::{ZwpTabletPadGroupV2, ZwpTabletPadGroupV2Ref},
            zwp_tablet_pad_ring_v2::{
                ZwpTabletPadRingV2, ZwpTabletPadRingV2Ref, ZwpTabletPadRingV2Source,
            },
            zwp_tablet_pad_strip_v2::{
                ZwpTabletPadStripV2, ZwpTabletPadStripV2Ref, ZwpTabletPadStripV2Source,
            },
            zwp_tablet_pad_v2::{ZwpTabletPadV2, ZwpTabletPadV2ButtonState, ZwpTabletPadV2Ref},
            zwp_tablet_seat_v2::{ZwpTabletSeatV2, ZwpTabletSeatV2Ref},
            zwp_tablet_tool_v2::{
                ZwpTabletToolV2, ZwpTabletToolV2ButtonState, ZwpTabletToolV2Capability,
                ZwpTabletToolV2Error, ZwpTabletToolV2Ref, ZwpTabletToolV2Type,
            },
            zwp_tablet_v2::{ZwpTabletV2, ZwpTabletV2Ref},
        },
        viewporter::{
            wp_viewport::{WpViewport, WpViewportError, WpViewportRef},
            wp_viewporter::{WpViewporter, WpViewporterError, WpViewporterRef},
        },
        wayland::{
            wl_buffer::{WlBuffer, WlBufferRef},
            wl_callback::{WlCallback, WlCallbackRef},
            wl_compositor::{WlCompositor, WlCompositorRef},
            wl_data_device::{WlDataDevice, WlDataDeviceError, WlDataDeviceRef},
            wl_data_device_manager::{
                WlDataDeviceManager, WlDataDeviceManagerDndAction, WlDataDeviceManagerRef,
            },
            wl_data_offer::{WlDataOffer, WlDataOfferError, WlDataOfferRef},
            wl_data_source::{WlDataSource, WlDataSourceError, WlDataSourceRef},
            wl_display::{WlDisplay, WlDisplayError, WlDisplayRef},
            wl_fixes::{WlFixes, WlFixesRef},
            wl_keyboard::{WlKeyboard, WlKeyboardKeyState, WlKeyboardKeymapFormat, WlKeyboardRef},
            wl_output::{WlOutput, WlOutputMode, WlOutputRef, WlOutputSubpixel, WlOutputTransform},
            wl_pointer::{
                WlPointer, WlPointerAxis, WlPointerAxisRelativeDirection, WlPointerAxisSource,
                WlPointerButtonState, WlPointerError, WlPointerRef,
            },
            wl_region::{WlRegion, WlRegionRef},
            wl_registry::{WlRegistry, WlRegistryRef},
            wl_seat::{WlSeat, WlSeatCapability, WlSeatError, WlSeatRef},
            wl_shell::{WlShell, WlShellError, WlShellRef},
            wl_shell_surface::{
                WlShellSurface, WlShellSurfaceFullscreenMethod, WlShellSurfaceRef,
                WlShellSurfaceResize, WlShellSurfaceTransient,
            },
            wl_shm::{WlShm, WlShmError, WlShmFormat, WlShmRef},
            wl_shm_pool::{WlShmPool, WlShmPoolRef},
            wl_subcompositor::{WlSubcompositor, WlSubcompositorError, WlSubcompositorRef},
            wl_subsurface::{WlSubsurface, WlSubsurfaceError, WlSubsurfaceRef},
            wl_surface::{WlSurface, WlSurfaceError, WlSurfaceRef},
            wl_touch::{WlTouch, WlTouchRef},
        },
        xdg_shell::{
            xdg_popup::{XdgPopup, XdgPopupError, XdgPopupRef},
            xdg_positioner::{
                XdgPositioner, XdgPositionerAnchor, XdgPositionerConstraintAdjustment,
                XdgPositionerError, XdgPositionerGravity, XdgPositionerRef,
            },
            xdg_surface::{XdgSurface, XdgSurfaceError, XdgSurfaceRef},
            xdg_toplevel::{
                XdgToplevel, XdgToplevelError, XdgToplevelRef, XdgToplevelResizeEdge,
                XdgToplevelState, XdgToplevelWmCapabilities,
            },
            xdg_wm_base::{XdgWmBase, XdgWmBaseError, XdgWmBaseRef},
        },
    };
}
