use minhook::MinHook;
use std::{
    ffi::c_void,
    mem::transmute,
    panic::catch_unwind,
    ptr::null_mut,
};
use windows::core::HRESULT;
use crate::rbr::Rbr;

/*
 * Most of this code is semantically the same as you would do in C++
 * See Plugin.h in Countdown plugin for C++ version
 */

const END_SCENE_ADDRESS: usize = 0x0040_E890;

type EndSceneFn = unsafe extern "fastcall" fn(*mut c_void) -> HRESULT;

type UpdateFn = unsafe fn(*mut c_void, &Rbr);

static mut ORIGINAL_END_SCENE: Option<EndSceneFn> = None;
static mut UPDATE_CALLBACK: Option<UpdateFn> = None;
static mut PLUGIN_STATE: *mut c_void = null_mut();
static mut RBR_INSTANCE: *mut Rbr = null_mut();

unsafe extern "fastcall" fn custom_end_scene(object_pointer: *mut c_void) -> HRESULT {
    let _ = catch_unwind(|| unsafe {
        update();

        if !RBR_INSTANCE.is_null() {
            crate::overlay::render(&*RBR_INSTANCE);
        }
    });

    unsafe {
        if let Some(original) = ORIGINAL_END_SCENE {
            return original(object_pointer);
        }
    }

    HRESULT(0)
}

unsafe fn update() {
    unsafe {
        if PLUGIN_STATE.is_null() || RBR_INSTANCE.is_null() {
            return;
        }

        if let Some(callback) = UPDATE_CALLBACK {
            callback(PLUGIN_STATE, &*RBR_INSTANCE);
        }
    }
}


pub(crate) unsafe fn install(plugin_state: *mut c_void, update_callback: UpdateFn, draw_callback: crate::overlay::DrawCallback) -> Result<(), String> {
    unsafe {
        if !RBR_INSTANCE.is_null() {
            return Err("Hook is already installed".to_owned());
        }

        let rbr = Rbr::initialize().map_err(|error| format!("{error:?}"))?;

        RBR_INSTANCE = Box::into_raw(Box::new(rbr));

        crate::overlay::initialize(&*RBR_INSTANCE, plugin_state, draw_callback)?;

        PLUGIN_STATE = plugin_state;
        UPDATE_CALLBACK = Some(update_callback);

        let target = END_SCENE_ADDRESS as *mut c_void;

        let trampoline = match MinHook::create_hook(target, custom_end_scene as *mut c_void) {
            Ok(trampoline) => trampoline,

            Err(status) => {
                clear_state();

                return Err(format!("create_hook failed: {status:?}"));
            }
        };

        ORIGINAL_END_SCENE = Some(transmute(
            trampoline,
        ));

        if let Err(status) = MinHook::enable_hook(target) {
            ORIGINAL_END_SCENE = None;

            let _ = MinHook::remove_hook(target);

            clear_state();

            return Err(format!(
                "enable_hook failed: {status:?}"
            ));
        }

        Ok(())
    }
}

pub(crate) unsafe fn rbr() -> Option<&'static Rbr> {
    unsafe {
        if RBR_INSTANCE.is_null() {
            None
        } else {
            Some(&*RBR_INSTANCE)
        }
    }
}

pub(crate) unsafe fn uninstall() {
    unsafe {
        let target = END_SCENE_ADDRESS as *mut c_void;

        let _ = MinHook::disable_hook(target);
        let _ = MinHook::remove_hook(target);

        ORIGINAL_END_SCENE = None;

        clear_state();
    }
}

unsafe fn clear_state() {
    unsafe {
        crate::overlay::shutdown();
        UPDATE_CALLBACK = None;
        PLUGIN_STATE = null_mut();

        if !RBR_INSTANCE.is_null() {
            drop(Box::from_raw(RBR_INSTANCE));
            RBR_INSTANCE = null_mut();
        }
    }
}