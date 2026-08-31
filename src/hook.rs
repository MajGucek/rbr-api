use minhook::MinHook;

use std::{
    any::Any,
    ffi::c_void,
    mem::transmute,
    panic::catch_unwind,
    ptr::null_mut,
};

use windows::core::{HRESULT};


use crate::rbr::Rbr;

/*
 * IDirect3DDevice9 vtable indices.
 */
const PRESENT_INDEX: usize = 17;
const BEGIN_SCENE_INDEX: usize = 41;
const END_SCENE_INDEX: usize = 42;

const D3D_DEVICE_ROOT: usize = 0x007E_A990;

type PresentFn = unsafe extern "system" fn(
    device: *mut c_void,
    source_rect: *const c_void,
    destination_rect: *const c_void,
    destination_window: *mut c_void,
    dirty_region: *const c_void,
) -> HRESULT;

type SceneFn = unsafe extern "system" fn(*mut c_void) -> HRESULT;

type UpdateFn = unsafe fn(*mut c_void, &Rbr);

static mut ORIGINAL_PRESENT: Option<PresentFn> = None;
static mut PRESENT_TARGET: *mut c_void = null_mut();

static mut UPDATE_CALLBACK: Option<UpdateFn> = None;
static mut PLUGIN_STATE: *mut c_void = null_mut();
static mut RBR_INSTANCE: *mut Rbr = null_mut();



unsafe extern "system" fn custom_present(
    device: *mut c_void,
    source_rect: *const c_void,
    destination_rect: *const c_void,
    destination_window: *mut c_void,
    dirty_region: *const c_void,
) -> HRESULT {
    let update_result = catch_unwind(|| unsafe {
        update();
    });

    if let Err(panic) = update_result {
        log_panic("Plugin update", panic);
    }
    
    let vtable = unsafe {
        *(device as *const *const *mut c_void)
    };

    let begin_scene: SceneFn = unsafe {
        transmute(
            *vtable.add(BEGIN_SCENE_INDEX),
        )
    };

    let end_scene: SceneFn = unsafe {
        transmute(
            *vtable.add(END_SCENE_INDEX),
        )
    };

    let begin_result = unsafe {
        begin_scene(device)
    };



    if begin_result.is_ok() {
        let render_result = catch_unwind(|| unsafe {
            if !RBR_INSTANCE.is_null() {

                crate::overlay::render(
                    &*RBR_INSTANCE,
                );
            }
        });
        let end_result = unsafe {
            end_scene(device)
        };

        if end_result.is_err() {
            log::error!(
                "IDirect3DDevice9::EndScene failed: \
                 {end_result:?}"
            );
        }

        if let Err(panic) = render_result {
            log_panic("Overlay render", panic);
        }
    }

    /*
     * Pass it forward so that it actually draws after me
     */
    unsafe {
        match ORIGINAL_PRESENT {
            Some(original) => original(
                device,
                source_rect,
                destination_rect,
                destination_window,
                dirty_region,
            ),

            None => HRESULT(0),
        }
    }
}


fn log_panic(
    location: &str,
    panic: Box<dyn Any + Send>,
) {
    let message =
        if let Some(message) =
            panic.downcast_ref::<&str>()
        {
            *message
        } else if let Some(message) =
            panic.downcast_ref::<String>()
        {
            message.as_str()
        } else {
            "unknown panic"
        };

    log::error!(
        "{location} panicked: {message}"
    );
}

unsafe fn update() {
    unsafe {
        if PLUGIN_STATE.is_null()
            || RBR_INSTANCE.is_null()
        {
            return;
        }

        if let Some(callback) = UPDATE_CALLBACK {
            callback(
                PLUGIN_STATE,
                &*RBR_INSTANCE,
            );
        }
    }
}

unsafe fn get_d3d_device()
    -> Result<*mut c_void, String>
{
    unsafe {
        let first = (
            D3D_DEVICE_ROOT as *const usize
        )
            .read_unaligned();

        if first == 0 {
            return Err(
                "D3D device root is null".to_owned(),
            );
        }

        let second = (
            (first + 0x28) as *const usize
        )
            .read_unaligned();

        if second == 0 {
            return Err(
                "D3D device owner is null".to_owned(),
            );
        }

        let device = (
            (second + 0xF4)
                as *const *mut c_void
        )
            .read_unaligned();

        if device.is_null() {
            return Err(
                "IDirect3DDevice9 is null".to_owned(),
            );
        }

        Ok(device)
    }
}

unsafe fn get_present_target(
    device: *mut c_void,
) -> Result<*mut c_void, String> {
    unsafe {
        if device.is_null() {
            return Err(
                "IDirect3DDevice9 is null".to_owned(),
            );
        }

        let vtable =
            *(device as *const *const *mut c_void);

        if vtable.is_null() {
            return Err(
                "IDirect3DDevice9 vtable is null"
                    .to_owned(),
            );
        }

        let present =
            *vtable.add(PRESENT_INDEX);

        if present.is_null() {
            return Err(
                "IDirect3DDevice9::Present is null"
                    .to_owned(),
            );
        }

        Ok(present)
    }
}

pub(crate) unsafe fn install(
    plugin_state: *mut c_void,
    update_callback: UpdateFn,
    draw_callback: crate::overlay::DrawCallback,
) -> Result<(), String> {
    unsafe {
        if !RBR_INSTANCE.is_null() {
            return Err(
                "Hook is already installed".to_owned(),
            );
        }

        let rbr = Rbr::initialize()
            .map_err(|error| {
                format!("{error:?}")
            })?;

        RBR_INSTANCE =
            Box::into_raw(Box::new(rbr));

        if let Err(error) =
            crate::overlay::initialize(
                &*RBR_INSTANCE,
                plugin_state,
                draw_callback,
            )
        {
            clear_state();
            return Err(error);
        }

        PLUGIN_STATE = plugin_state;
        UPDATE_CALLBACK = Some(update_callback);

        let device = match get_d3d_device() {
            Ok(device) => device,

            Err(error) => {
                clear_state();
                return Err(error);
            }
        };

        let target =
            match get_present_target(device) {
                Ok(target) => target,

                Err(error) => {
                    clear_state();
                    return Err(error);
                }
            };

        let trampoline =
            match MinHook::create_hook(
                target,
                custom_present as *mut c_void,
            ) {
                Ok(trampoline) => trampoline,

                Err(status) => {
                    clear_state();

                    return Err(format!(
                        "Present create_hook failed: \
                         {status:?}",
                    ));
                }
            };

        ORIGINAL_PRESENT = Some(
            transmute(trampoline),
        );

        PRESENT_TARGET = target;

        if let Err(status) =
            MinHook::enable_hook(target)
        {
            ORIGINAL_PRESENT = None;
            PRESENT_TARGET = null_mut();

            let _ =
                MinHook::remove_hook(target);

            clear_state();

            return Err(format!(
                "Present enable_hook failed: \
                 {status:?}",
            ));
        }

        Ok(())
    }
}

pub(crate) unsafe fn rbr()
    -> Option<&'static Rbr>
{
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
        if !PRESENT_TARGET.is_null() {
            let target = PRESENT_TARGET;

            let _ =
                MinHook::disable_hook(target);

            let _ =
                MinHook::remove_hook(target);
        }

        ORIGINAL_PRESENT = None;
        PRESENT_TARGET = null_mut();

        clear_state();
    }
}

unsafe fn clear_state() {
    unsafe {
        crate::overlay::shutdown();

        UPDATE_CALLBACK = None;
        PLUGIN_STATE = null_mut();

        if !RBR_INSTANCE.is_null() {
            drop(Box::from_raw(
                RBR_INSTANCE,
            ));

            RBR_INSTANCE = null_mut();
        }
    }
}