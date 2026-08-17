use std::{
    ffi::c_void,
    mem::transmute,
    ptr::null_mut,
};
use egui_d3d9::EguiDx9;

use windows::Win32::{
    Foundation::{
        HWND,
        LPARAM,
        LRESULT,
        POINT,
        WPARAM,
    },
    Graphics::Gdi::ScreenToClient,
    UI::{
        Input::KeyboardAndMouse::{
            ReleaseCapture,
            SetCapture,
        },
        WindowsAndMessaging::{
            CallWindowProcW,
            DefWindowProcW,
            GetCursorPos,
            SetWindowLongPtrW,
            GWLP_WNDPROC,
            WNDPROC,
            WM_LBUTTONDBLCLK,
            WM_LBUTTONDOWN,
            WM_LBUTTONUP,
            WM_MBUTTONDBLCLK,
            WM_MBUTTONDOWN,
            WM_MBUTTONUP,
            WM_MOUSEHWHEEL,
            WM_MOUSEMOVE,
            WM_MOUSEWHEEL,
            WM_RBUTTONDBLCLK,
            WM_RBUTTONDOWN,
            WM_RBUTTONUP,
            WM_XBUTTONDBLCLK,
            WM_XBUTTONDOWN,
            WM_XBUTTONUP,
        },
    },
};

use crate::rbr::Rbr;



/*
 * ---WARNING---
 * This code was in majority written by GPT, so it is not to be trusted blindly
 * I just simply haven't done something like this before
 * The code will remain as such until it crashes, so I must trust GPT for now
 * The static mut looks terrible, but any other attempt to reprompt becomes way to complicated to even follow
 * ---WARNING---
*/


pub(crate) type DrawCallback = unsafe fn(
    plugin_state: *mut c_void,
    egui_context: &egui::Context,
    rbr: &Rbr
);

struct OverlayState {
    plugin_state: *mut c_void,
    rbr: *const Rbr,
    draw_callback: DrawCallback,
}

struct Overlay {
    egui: EguiDx9<OverlayState>,
    window: HWND,
    original_window_proc: isize,
    open: bool,
}

static mut OVERLAY: *mut Overlay = null_mut();

pub(crate) unsafe fn initialize(rbr: &Rbr, plugin_state: *mut c_void, draw_callback: DrawCallback) -> Result<(), String> {
    unsafe {
        if !OVERLAY.is_null() {
            return Ok(());
        }

        let device = rbr
            .raw_device()
            .ok_or("RBR D3D9 device is unavailable")?;

        let window = rbr.window_handle();

        if window.is_invalid() {
            return Err("RBR window handle is invalid".to_owned(), );
        }

        let egui = EguiDx9::init(
            &device,
            window,
            draw,
            OverlayState {
                plugin_state,
                rbr,
                draw_callback,
            },
            false,
        );

        let overlay = Box::new(Overlay {
            egui,
            window,
            original_window_proc: 0,
            open: true,
        });

        OVERLAY = Box::into_raw(overlay);

        let new_window_proc = overlay_window_proc as *const () as usize as i32;

        let previous = SetWindowLongPtrW(
            window,
            GWLP_WNDPROC,
            new_window_proc,
        );

        if previous == 0 {
            drop(Box::from_raw(OVERLAY));
            OVERLAY = null_mut();

            return Err("Failed to replace RBR window procedure".to_owned());
        }

        (*OVERLAY).original_window_proc = previous as isize;

        Ok(())
    }
}

pub(crate) unsafe fn render(rbr: &Rbr) {
    unsafe {
        let overlay = OVERLAY;

        if overlay.is_null() {
            return;
        }

        let Some(device) = rbr.raw_device() else {
            return;
        };

        poll_cursor(&mut *overlay);

        (*overlay).egui.present(&device);
    }
}


pub(crate) unsafe fn shutdown() {
    unsafe {
        let overlay = OVERLAY;
        OVERLAY = null_mut();

        if overlay.is_null() {
            return;
        }

        let overlay = Box::from_raw(overlay);

        if !overlay.window.is_invalid() && overlay.original_window_proc != 0 {
            SetWindowLongPtrW(
                overlay.window,
                GWLP_WNDPROC,
                overlay.original_window_proc as i32,
            );
        }

        let _ = ReleaseCapture();
    }
}

fn draw(egui_context: &egui::Context, state: &mut OverlayState) {
    if state.plugin_state.is_null() || state.rbr.is_null() {
        return;
    }

    unsafe {
        (state.draw_callback)(
            state.plugin_state,
            egui_context,
            &*state.rbr,
        );
    }
}

unsafe fn poll_cursor(overlay: &mut Overlay) {
    unsafe {
        let mut cursor = POINT::default();

        if GetCursorPos(&mut cursor).is_err() {
            return;
        }

        if !ScreenToClient(overlay.window, &mut cursor).as_bool() {
            return;
        }

        overlay.egui.wnd_proc(
            WM_MOUSEMOVE,
            WPARAM(0),
            mouse_lparam(cursor.x, cursor.y),
        );
    }
}

unsafe extern "system" fn overlay_window_proc(
    window: HWND,
    message: u32,
    wparam: WPARAM,
    lparam: LPARAM,
) -> LRESULT {
    unsafe {
        let overlay = OVERLAY;

        if overlay.is_null() {
            return DefWindowProcW(
                window,
                message,
                wparam,
                lparam,
            );
        }

        let overlay = &mut *overlay;

        overlay.egui.wnd_proc(
            message,
            wparam,
            lparam,
        );

        if overlay.open && consume_mouse_message(window, message) {
            return LRESULT(0);
        }

        call_original_window_proc(
            overlay,
            window,
            message,
            wparam,
            lparam,
        )
    }
}

unsafe fn call_original_window_proc(
    overlay: &Overlay,
    window: HWND,
    message: u32,
    wparam: WPARAM,
    lparam: LPARAM,
) -> LRESULT {
    unsafe {
        if overlay.original_window_proc == 0 {
            return DefWindowProcW(
                window,
                message,
                wparam,
                lparam,
            );
        }

        let original: WNDPROC = transmute(overlay.original_window_proc);

        CallWindowProcW(
            original,
            window,
            message,
            wparam,
            lparam,
        )
    }
}

unsafe fn consume_mouse_message(
    window: HWND,
    message: u32,
) -> bool {
    unsafe {
        match message {
            WM_LBUTTONDOWN => {
                let _ = SetCapture(window);
                true
            }

            WM_LBUTTONUP => {
                let _ = ReleaseCapture();
                true
            }

            WM_MOUSEMOVE
            | WM_LBUTTONDBLCLK
            | WM_RBUTTONDOWN
            | WM_RBUTTONUP
            | WM_RBUTTONDBLCLK
            | WM_MBUTTONDOWN
            | WM_MBUTTONUP
            | WM_MBUTTONDBLCLK
            | WM_MOUSEWHEEL
            | WM_MOUSEHWHEEL
            | WM_XBUTTONDOWN
            | WM_XBUTTONUP
            | WM_XBUTTONDBLCLK => true,

            _ => false,
        }
    }
}

fn mouse_lparam(x: i32, y: i32) -> LPARAM {
    let packed = (x as u16 as u32) | ((y as u16 as u32) << 16);

    LPARAM(packed as i32 as isize)
}